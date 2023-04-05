use super::ethereum_tx_receipt::{EthereumTxReceiptMsg, EthereumTxReceiptPlugin};
use crate::{
	error::error::ExpectedError,
	libs::{
		self,
		convert::hex_to_decimal_converter,
		request,
		serde::{filter, get_array, get_object, get_string},
		sync::load_state,
	},
	plugin::{
		postgres::{PostgresMsg, PostgresPlugin},
		slack::SlackPlugin,
	},
	types::{channel::MultiSender, sync::SyncState},
};
use appbase::prelude::*;
use serde_json::{json, Value};

#[derive(Default)]
#[appbase_plugin(PostgresPlugin, SlackPlugin, EthereumTxReceiptPlugin)]
pub struct EthereumBlockPlugin {
	state: Option<SyncState>,
	senders: Option<MultiSender>,
	receiver: Option<Receiver>,
}

impl Plugin for EthereumBlockPlugin {
	fn new() -> Self {
		EthereumBlockPlugin::default()
	}

	fn init(&mut self) {
		let senders = MultiSender::new(vec!["postgres", "slack", "ethereum_tx_receipt"]);
		self.senders = Some(senders.to_owned());
		self.receiver = Some(APP.channels.subscribe("ethereum_block"));
		self.state = Some(load_state("ethereum_block").expect("failed to load ethereum_block."));
	}

	fn startup(&mut self) {
		let receiver = self.receiver.take().unwrap();
		let state = self.state.take().unwrap();
		let senders = self.senders.take().unwrap();
		let app = APP.quit_handle().unwrap();

		Self::recv(receiver, state, senders, app);
	}

	fn shutdown(&mut self) {}
}

impl EthereumBlockPlugin {
	fn recv(mut receiver: Receiver, mut state: SyncState, senders: MultiSender, app: QuitHandle) {
		APP.spawn(async move {
			if let Ok(message) = receiver.try_recv() {
				if let Err(err) = libs::sync::message_handler(message, &mut state) {
					let _ = libs::error::warn(senders.get("slack"), err);
				}
			}
			if state.is_workable() {
				if let Err(e) = Self::process(&mut state, &senders).await {
					libs::sync::error_handler(e, &mut state, &senders)
				} else {
					state.next_idx();
				}
			}
			if !app.is_quitting() {
				tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
				Self::recv(receiver, state, senders, app);
			}
		});
	}

	async fn process(state: &mut SyncState, senders: &MultiSender) -> Result<(), ExpectedError> {
		let req_url = state.active_node();
		let hex_idx = format!("0x{:x}", state.sync_idx);
		let req_body = json!({
			"jsonrpc": "2.0",
			"method": "eth_getBlockByNumber",
			"params": [ hex_idx, true ],
			"id": 1
		});
		let response = request::post(&req_url, &req_body.to_string()).await?;
		if let None = response.get("result") {
			return Err(ExpectedError::BlockHeightError(format!(
				"block number {} has not yet been created.",
				state.sync_idx
			)))
		}
		let block = get_object(&response, "result")?;
		if let false = filter(block, state.get_filter())? {
			return Err(ExpectedError::FilterError("filtered.".to_string()))
		}
		let block = hex_to_decimal_converter(
			block,
			vec!["number", "size", "timestamp", "gasLimit", "gasUsed"],
		)?;
		let pg_sender = senders.get("postgres");
		let _ = pg_sender.send(PostgresMsg::new(
			String::from("ethereum_blocks"),
			Value::Object(block.clone()),
		))?;
		let txs = get_array(&block, "transactions")?;
		for tx in txs.iter() {
			let tx = tx
				.as_object()
				.ok_or(ExpectedError::ParsingError("transaction is not object.".to_string()))?;
			let tx = hex_to_decimal_converter(
				tx,
				vec!["blockNumber", "gas", "gasPrice", "nonce", "transactionIndex", "value"],
			)?;
			let tx_hash = get_string(&tx, "hash")?;
			let _ = pg_sender
				.send(PostgresMsg::new("ethereum_transactions".to_string(), Value::Object(tx)))?;
			let receipt_sender = senders.get("ethereum_tx_receipt");
			let _ = receipt_sender.send(EthereumTxReceiptMsg::new(tx_hash))?;
		}
		libs::sync::save_state(&state)?;
		Ok(())
	}
}

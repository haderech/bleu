use super::postgres::PostgresMsg;
use crate::{
	error::error::ExpectedError,
	libs::{
		self,
		convert::convert_hex_to_decimal,
		request,
		serde::{get_array, get_object},
		sync::load_state,
	},
	message,
	plugin::{postgres::PostgresPlugin, slack::SlackPlugin},
	types::{channel::MultiSender, sync::SyncState},
};
use appbase::prelude::*;
use serde_json::json;

#[derive(Default)]
#[appbase_plugin(PostgresPlugin, SlackPlugin)]
pub struct EthereumTxReceiptPlugin {
	state: Option<SyncState>,
	senders: Option<MultiSender>,
	receiver: Option<Receiver>,
}

message!(EthereumTxReceiptMsg; {tx_hash: String});

impl Plugin for EthereumTxReceiptPlugin {
	fn new() -> Self {
		EthereumTxReceiptPlugin::default()
	}

	fn init(&mut self) {
		let senders = MultiSender::new(vec!["postgres", "slack"]);
		self.senders = Some(senders.to_owned());
		self.receiver = Some(APP.channels.subscribe("ethereum_tx_receipt"));
		self.state =
			Some(load_state("ethereum_tx_receipt").expect("failed to load ethereum_tx_receipt."));
	}

	fn startup(&mut self) {
		let receiver = self.receiver.take().unwrap();
		let senders = self.senders.take().unwrap();
		let state = self.state.take().unwrap();
		let app = APP.quit_handle().unwrap();

		Self::recv(receiver, state, senders, app);
	}

	fn shutdown(&mut self) {}
}

impl EthereumTxReceiptPlugin {
	fn recv(mut receiver: Receiver, mut state: SyncState, senders: MultiSender, app: QuitHandle) {
		APP.spawn(async move {
			if let Ok(message) = receiver.try_recv() {
				let tx_hash =
					serde_json::from_value::<EthereumTxReceiptMsg>(message).unwrap().tx_hash;
				if let Err(e) = Self::process(tx_hash.clone(), &mut state, &senders).await {
					let _ = libs::error::error_message(
						senders.get("slack"),
						format!("{} tx_hash={}", e.to_string(), tx_hash),
					);
				}
			}

			if !app.is_quitting() {
				tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
				Self::recv(receiver, state, senders, app);
			}
		});
	}

	async fn process(
		tx_hash: String,
		state: &mut SyncState,
		senders: &MultiSender,
	) -> Result<(), ExpectedError> {
		let req_url = state.active_node();
		let req_body = json!({
			"jsonrpc": "2.0",
			"method": "eth_getTransactionReceipt",
			"params": [ tx_hash ],
			"id": 1
		});
		let response = request::post(&req_url, &req_body.to_string()).await?;
		let mut receipt = get_object(&response, "result")?.clone();

		convert_hex_to_decimal(
			&mut receipt,
			vec!["blockNumber", "cumulativeGasUsed", "gasUsed", "status", "transactionIndex"],
		)?;
		let pg_sender = senders.get("postgres");
		let _ = pg_sender.send(PostgresMsg::new(
			String::from("ethereum_tx_receipts"),
			serde_json::Value::Object(receipt.to_owned()),
		))?;
		let mut logs = get_array(&receipt, "logs")?.clone();
		for log in logs.iter_mut() {
			let log = log
				.as_object_mut()
				.ok_or(ExpectedError::ParsingError("log is not object.".to_string()))?;
			convert_hex_to_decimal(log, vec!["blockNumber", "transactionIndex", "logIndex"])?;
			let _ = pg_sender.send(PostgresMsg::new(
				String::from("ethereum_logs"),
				serde_json::Value::Object(log.to_owned()),
			))?;
		}
		Ok(())
	}
}

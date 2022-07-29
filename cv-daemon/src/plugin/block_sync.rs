use appbase::prelude::*;
use clap::Arg;
use serde_json::Value;

use crate::error::error::ExpectedError;
use crate::libs;
use crate::libs::request;
use crate::libs::serde::{filter, find_value_by_path, get_array};
use crate::libs::sync_helper::load_sync_state;
use crate::plugin::postgres::{Postgres, PostgresMsg};
use crate::types::channel::MultiSender;
use crate::types::sync::SyncState;

#[appbase_plugin(Postgres)]
pub struct BlockSync {
  sync_state: Option<SyncState>,
  senders: Option<MultiSender>,
  receiver: Option<Receiver>,
}

impl Plugin for BlockSync {
  fn new() -> Self {
    APP.options.arg(Arg::new("block::poll-interval").long("block-poll-interval").takes_value(true));
    BlockSync {
      sync_state: None,
      senders: None,
      receiver: None,
    }
  }

  fn init(&mut self) {
    self.senders = Some(MultiSender::new(vec!("postgres", "slack")));
    self.receiver = Some(APP.channels.subscribe("block_sync"));
    self.sync_state = Some(load_sync_state("block_sync.json").unwrap());
  }

  fn startup(&mut self) {
    let receiver = self.receiver.take().unwrap();
    let sync_state = self.sync_state.take().unwrap();
    let senders = self.senders.take().unwrap();
    let app = APP.quit_handle().unwrap();

    Self::process(receiver, sync_state, senders, app);
  }

  fn shutdown(&mut self) {}
}

impl BlockSync {
  fn process(mut receiver: Receiver, mut sync_state: SyncState, senders: MultiSender, app: QuitHandle) {
    APP.spawn(async move {
      log::debug!("start to block sync process");
      if let Ok(msg) = receiver.try_recv() {
        if let Err(err) = libs::sync_helper::message_handler(msg, &mut sync_state) {
          let _ = libs::error::warn_handler(senders.get("slack"), err);
        }
      }
      if sync_state.is_workable() {
        if let Err(err) = Self::sync(&sync_state, &senders).await {
          libs::sync_helper::error_handler(err, &mut sync_state, &senders);
        } else {
          sync_state.next_idx();
        }
      }
      if !app.is_quitting() {
        let poll_interval = libs::opt::get_value::<u64>("block::poll-interval").unwrap_or(5000);
        tokio::time::sleep(tokio::time::Duration::from_millis(poll_interval)).await;
        Self::process(receiver, sync_state, senders, app);
      }
    });
  }

  async fn sync(sync_state: &SyncState, senders: &MultiSender) -> Result<(), ExpectedError> {
    let sync_block_api = format!("blocks/{}", sync_state.sync_idx);
    let block_req = libs::sync_helper::create_req_url(sync_state.active_node(), sync_block_api);
    let mut block_res = request::get_block_async(block_req.as_str()).await?;
    if !filter(&block_res, sync_state.get_filter())? {
      return Err(ExpectedError::FilterError(format!("not matched filter condition! sync_type={}", sync_state.sync_type)));
    }
    let num_txs = if let Some(txs) = find_value_by_path(&block_res, "block.data.txs").as_array() {
      txs.len()
    } else {
      0
    };
    block_res.insert("num_txs".to_string(), Value::from(num_txs));

    let pg_sender = senders.get("postgres");
    let _ = pg_sender.send(PostgresMsg::new(String::from("cosmos_block"), Value::Object(block_res.clone()), -1))?;

    if num_txs > 0 {
      let total_page = (num_txs / 100) + if num_txs % 100 > 0 { 1 } else { 0 };
      for i in 0..total_page {
        let offset = i * 100;
        let sync_tx_api = format!("cosmos/tx/v1beta1/txs?pagination.limit=100&pagination.offset={}&events=tx.height={}", offset, sync_state.sync_idx);
        let txs_req = libs::sync_helper::create_req_url(sync_state.active_node(), sync_tx_api);
        let txs_res = request::get_async(txs_req.as_str()).await?;
        let tx_responses = get_array(&txs_res, "tx_responses")?;
        let _ = pg_sender.send(PostgresMsg::new(String::from("cosmos_tx"), Value::Array(tx_responses.to_owned()), -1))?;
      }
    }
    libs::sync_helper::save_state(&sync_state)?;
    Ok(())
  }
}

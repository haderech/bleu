use appbase::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::error::ExpectedError;
use crate::libs;
use crate::libs::convert::hex_to_decimal_converter;
use crate::libs::opt::opt_to_result;
use crate::libs::request;
use crate::libs::serde::{get_array, get_object, get_str};
use crate::libs::subscribe::task_loader;
use crate::message;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::plugin::rocks::RocksPlugin;
use crate::types::channel::MultiSender;
use crate::types::subscribe::SubscribeEvent;

#[appbase_plugin(RocksPlugin, PostgresPlugin)]
pub struct L2TxReceiptPlugin {
    sub_event: Option<SubscribeEvent>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";
const TASK_NAME: &str = "l2_tx_receipt";
const TASK_FILE: &str = "task/l2_tx_receipt.json";

message!(L2TxReceiptMsg; {tx_hash: String});

impl Plugin for L2TxReceiptPlugin {
    fn new() -> Self {
        L2TxReceiptPlugin {
            sub_event: None,
            senders: None,
            receiver: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!("postgres" /*"elasticsearch"*/));
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe(TASK_NAME));
        let rocksdb = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        self.sub_event = Some(task_loader(rocksdb, TASK_FILE, CHAIN, TASK_PREFIX, TASK_NAME).expect(format!("failed to load task! task={}", TASK_NAME).as_str()));
    }

    fn startup(&mut self) {
        let receiver = self.receiver.take().unwrap();
        let senders = self.senders.take().unwrap();
        let sub_event = self.sub_event.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(receiver, sub_event, senders, app);
    }

    fn shutdown(&mut self) {}
}

impl L2TxReceiptPlugin {
    fn recv(mut receiver: Receiver, sub_event: SubscribeEvent, senders: MultiSender, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(message) = receiver.try_recv() {
                if let Err(err) = Self::message_handler(message, &sub_event, &senders) {
                    log::error!("{}", err.to_string());
                }
            }
            if !app.is_quitting() {
                Self::recv(receiver, sub_event, senders, app);
            }
        });
    }

    fn message_handler(message: Value, sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let parsed_msg = message.as_object().unwrap();
        let tx_hash = get_str(parsed_msg, "tx_hash")?;
        let req_url = sub_event.active_node();
        let req_body = json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getTransactionReceipt",
                    "params": [ tx_hash ],
                    "id": 1
                });
        let response = request::post(req_url.as_str(), req_body.to_string().as_str())?;
        if !libs::subscribe::is_value_created(&response, "result") {
            return Err(ExpectedError::NoneError(format!("receipt does not created...tx_hash={}", tx_hash)));
        }
        let receipt = get_object(&response, "result")?;
        let converted_receipt = hex_to_decimal_converter(receipt, vec!["blockNumber", "cumulativeGasUsed", "gasUsed", "status", "transactionIndex"])?;
        let pg_sender = senders.get("postgres");
        let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_tx_receipts"), Value::Object(converted_receipt.to_owned())))?;
        let logs = get_array(&receipt, "logs")?;
        for log in logs.iter() {
            let log_map = opt_to_result(log.as_object())?;
            let converted_log = hex_to_decimal_converter(log_map, vec!["blockNumber", "transactionIndex", "logIndex"])?;
            let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_tx_receipt_logs"), Value::Object(converted_log.to_owned())))?;
        }
        Ok(())
    }
}
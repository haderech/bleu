use appbase::prelude::*;
use jsonrpc_core::Params;
use jsonrpc_core::serde_json::Map;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::enumeration;
use crate::error::error::ExpectedError;
use crate::libs::opt::opt_to_result;
use crate::libs::rocks::get_by_prefix_static;
use crate::libs::serde::get_str;
use crate::message;
use crate::plugin::jsonrpc::JsonRpcPlugin;
use crate::plugin::l2_block_tx::L2BlockTxMsg;
use crate::plugin::l2_enqueue::L2EnqueueMsg;
use crate::plugin::l2_state_batch::L2StateBatchMsg;
use crate::plugin::l2_tx_batch::L2TxBatchMsg;
use crate::plugin::rocks::RocksPlugin;
use crate::types::channel::MultiSender;
use crate::types::enumeration::Enumeration;
use crate::types::subscribe::TaskMethod;
use crate::validation::task;

#[appbase_plugin(JsonRpcPlugin, RocksPlugin)]
pub struct TaskPlugin {
    receiver: Option<Receiver>,
    senders: Option<MultiSender>,
}

const TASK_PREFIX: &str = "task:optimism";

enumeration!(TaskType; {L2BlockTx: "l2_block_tx"}, {L2TxBatch: "l2_tx_batch"}, {L2StateBatch: "l2_state_batch"}, {L2Enqueue: "l2_enqueue"});
message!(TaskMsg; {method: String}, {task: String});

impl Plugin for TaskPlugin {
    fn new() -> Self {
        TaskPlugin {
            receiver: None,
            senders: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!["task", "l2_block_tx", "l2_tx_batch", "l2_state_batch", "l2_enqueue"]);
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe("task"));

        self.jsonrpc_register();
    }

    fn startup(&mut self) {
        let receiver = self.receiver.take().unwrap();
        let senders = self.senders.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(receiver, senders, app);
    }

    fn shutdown(&mut self) {}
}

impl TaskPlugin {
    fn recv(mut receiver: Receiver, senders: MultiSender, app: QuitHandle) {
        APP.spawn(async move {
            if let Ok(message) = receiver.try_recv() {
                let _ = Self::message_handler(message, &senders);
            }
            if !app.is_quitting() {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                Self::recv(receiver, senders, app);
            }
        });
    }

    fn jsonrpc_register(&self) {
        let senders = self.senders.as_ref().unwrap();
        let task_sender = senders.get("task");

        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("start_task"), move |params: Params| {
                let response = match Self::task_request_handler(TaskMethod::Start, params, &task_sender) {
                    Ok(response) => response,
                    Err(err) => json!({"error": err.to_string()}),
                };
                Box::new(futures::future::ok(response))
            });
        });

        let task_sender = senders.get("task");
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("stop_task"), move |params: Params| {
                let response = match Self::task_request_handler(TaskMethod::Stop, params, &task_sender) {
                    Ok(response) => response,
                    Err(err) => json!({"error": err.to_string()}),
                };
                Box::new(futures::future::ok(response))
            });
        });

        let task_sender = senders.get("task");
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("remove_task"), move |params: Params| {
                let response = match Self::task_request_handler(TaskMethod::Remove, params, &task_sender) {
                    Ok(response) => response,
                    Err(err) => json!({"error": err.to_string()}),
                };
                Box::new(futures::future::ok(response))
            });
        });

        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("get_tasks"), move |_| {
                let tasks = get_by_prefix_static(&rocks_db, TASK_PREFIX);
                Box::new(futures::future::ok(tasks))
            });
        });
    }

    fn task_request_handler(method: TaskMethod, params: Params, sender: &Sender) -> Result<Value, ExpectedError> {
        let params: Map<String, Value> = params.parse().unwrap();
        let _ = task::verify(&params)?;
        let task_name = get_str(&params, "task")?;
        let task = opt_to_result(TaskType::find(task_name))?;
        let _ = sender.send(TaskMsg::new(method.value(), task.value()))?;

        Ok(Value::String(format!("request registered! task={}", task_name)))
    }

    fn message_handler(message: Value, senders: &MultiSender) -> Result<(), ExpectedError> {
        let parsed_msg = opt_to_result(message.as_object())?;
        let method = opt_to_result(TaskMethod::find(get_str(parsed_msg, "method")?))?;
        let task = opt_to_result(TaskType::find(get_str(parsed_msg, "task")?))?;
        let message = match task {
            TaskType::L2BlockTx => L2BlockTxMsg::new(method.value()),
            TaskType::L2TxBatch => L2TxBatchMsg::new(method.value()),
            TaskType::L2StateBatch => L2StateBatchMsg::new(method.value()),
            TaskType::L2Enqueue => L2EnqueueMsg::new(method.value()),
        };
        let sender = senders.get(&task.value());
        let _ = sender.send(message)?;
        Ok(())
    }
}
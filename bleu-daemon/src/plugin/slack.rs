use std::collections::HashMap;

use appbase::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{enumeration, libs};
use crate::libs::opt;
use crate::libs::serde::get_str;
use crate::message;
use crate::types::enumeration::Enumeration;

#[appbase_plugin]
pub struct SlackPlugin {
    slack_hooks: Option<SlackHooks>,
    monitor: Option<Receiver>,
}

pub type SlackHooks = HashMap<String, String>;

message!(SlackMsg; {msg_level: String}, {msg: String});
enumeration!(SlackMsgLevel; {Info: "info"}, {Warn: "warn"}, {Error: "error"});

impl Plugin for SlackPlugin {
    fn new() -> Self {
        APP.options.arg(clap::Arg::new("slack::activate").long("slack-activate").takes_value(true));
        APP.options.arg(clap::Arg::new("slack::info").long("slack-info").takes_value(true));
        APP.options.arg(clap::Arg::new("slack::warn").long("slack-warn").takes_value(true));
        APP.options.arg(clap::Arg::new("slack::error").long("slack-error").takes_value(true));

        SlackPlugin {
            slack_hooks: None,
            monitor: None,
        }
    }

    fn init(&mut self) {
        let mut slack_hooks: SlackHooks = HashMap::new();
        slack_hooks.insert(String::from("info"), opt::get_value_str("slack::info").unwrap());
        slack_hooks.insert(String::from("warn"), opt::get_value_str("slack::warn").unwrap());
        slack_hooks.insert(String::from("error"), opt::get_value_str("slack::error").unwrap());

        self.slack_hooks = Some(slack_hooks);
        self.monitor = Some(APP.channels.subscribe("slack"));
    }

    fn startup(&mut self) {
        let slack_hooks = self.slack_hooks.take().unwrap();
        let monitor = self.monitor.take().unwrap();
        let app = APP.quit_handle().unwrap();
        Self::recv(slack_hooks, monitor, app);
    }

    fn shutdown(&mut self) {}
}

impl SlackPlugin {
    fn recv(slack_hooks: SlackHooks, mut monitor: Receiver, app: QuitHandle) {
        APP.spawn(async move {
            if let Ok(msg) = monitor.try_recv() {
                if libs::opt::get_value::<bool>("slack::activate").unwrap_or(false) {
                    let parsed_msg = msg.as_object().unwrap();
                    let msg_level = SlackMsgLevel::find(get_str(parsed_msg, "msg_level").unwrap()).unwrap();
                    let msg_level_value = msg_level.value();
                    let slack_hook = slack_hooks.get(&msg_level_value).unwrap();
                    let slack_msg = get_str(parsed_msg, "msg").unwrap();

                    let mut text = HashMap::new();
                    text.insert("text", slack_msg);
                    let client = reqwest::Client::new();
                    let result = client.post(slack_hook)
                        .json(&text)
                        .send()
                        .await;

                    if let Err(err) = result {
                        log::error!("slack error! error={:?}", err);
                    }
                }
            }
            if !app.is_quitting() {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                Self::recv(slack_hooks, monitor, app);
            }
        });
    }
}

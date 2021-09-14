use std::collections::HashMap;

use appbase::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use teloxide::Bot;
use teloxide::prelude::*;

use crate::libs;
use crate::libs::serde::get_str;
use crate::message;

#[appbase_plugin]
pub struct TelegramPlugin {
    token: Option<String>,
    bot: Option<AutoSend<Bot>>,
    monitor: Option<Receiver>,
}

message!(TelegramMsg; {chat_id: String}, {text: String});

impl Plugin for TelegramPlugin {
    fn new() -> Self {
        APP.options.arg(clap::Arg::new("telegram::bot-token").long("telegram-bot-token").takes_value(true));

        TelegramPlugin {
            token: None,
            bot: None,
            monitor: None,
        }
    }

    fn init(&mut self) {
        let token = libs::opts::string("telegram::bot-token").unwrap();
        self.token = Some(token.clone());
        let bot = Bot::new(token).auto_send();
        self.bot = Some(bot);
        self.monitor = Some(APP.channels.subscribe("telegram"));
    }

    fn startup(&mut self) {
        let monitor = self.monitor.take().unwrap();
        let token = self.token.as_ref().unwrap().clone();
        let bot = self.bot.as_ref().unwrap().clone();
        let app = APP.quit_handle().unwrap();
        Self::recv(monitor, token, app);
        Self::api(bot);
    }

    fn shutdown(&mut self) {}
}

impl TelegramPlugin {
    fn recv(mut monitor: Receiver, token: String, app: QuitHandle) {
        APP.spawn(async move {
            if let Ok(msg) = monitor.try_recv() {
                let parsed_msg = msg.as_object().unwrap();
                let chat_id = get_str(parsed_msg, "chat_id").unwrap();
                let text = get_str(parsed_msg, "text").unwrap();

                let mut req_body = HashMap::new();
                req_body.insert("chat_id", chat_id);
                req_body.insert("text", text);

                let client = reqwest::Client::new();
                let result = client.post(format!("https://api.telegram.org/bot{}/sendMessage", token))
                    .json(&req_body)
                    .send()
                    .await;
                if let Err(err) = result {
                    log::error!("telegram_error={:?}", err);
                }
            }
            if !app.is_quitting() {
                Self::recv(monitor, token, app);
            }
        });
    }

    fn api(bot: AutoSend<Bot>) {
        APP.spawn(async move {
            teloxide::repl(bot, |message| async move {
                let _ = message.reply_to(format!("You can use telegram features of UFC with your chat ID! YOUR_CHAT_ID={}", message.chat_id())).await;
                respond(())
            }).await;
        });
    }
}

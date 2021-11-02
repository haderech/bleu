use appbase::prelude::*;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::error::ExpectedError;
use crate::libs;
use crate::libs::serde::get_str;
use crate::message;

#[appbase_plugin]
pub struct EmailPlugin {
    monitor: Option<Receiver>,
}

message!(EmailMsg; {to: String}, {subject: String}, {body: String});

impl Plugin for EmailPlugin {
    fn new() -> Self {
        APP.options.arg(clap::Arg::new("email::smtp-username").long("smtp-username").takes_value(true));
        APP.options.arg(clap::Arg::new("email::smtp-password").long("smtp-password").takes_value(true));
        APP.options.arg(clap::Arg::new("email::smtp-relay").long("smtp-relay").takes_value(true));
        APP.options.arg(clap::Arg::new("email::from").long("email-from").takes_value(true));
        APP.options.arg(clap::Arg::new("email::reply-to").long("email-reply-to").takes_value(true));

        EmailPlugin {
            monitor: None,
        }
    }

    fn init(&mut self) {
        self.monitor = Some(APP.channels.subscribe("email"));
    }

    fn startup(&mut self) {
        let monitor = self.monitor.take().unwrap();
        let app = APP.quit_handle().unwrap();
        Self::recv(monitor, app);
    }

    fn shutdown(&mut self) {}
}

impl EmailPlugin {
    fn recv(mut monitor: Receiver, app: QuitHandle) {
        APP.spawn(async move {
            if let Ok(msg) = monitor.try_recv() {
                let parsed_msg = msg.as_object().unwrap();

                let to = get_str(parsed_msg, "to").unwrap();
                let subject = get_str(parsed_msg, "subject").unwrap();
                let body = get_str(parsed_msg, "body").unwrap();

                if let Err(result) = Self::send(to, subject, body) {
                    log::error!("{}", result);
                }
            }
            if !app.is_quitting() {
                Self::recv(monitor, app);
            }
        });
    }

    pub fn send(to: &str, subject: &str, body: &str) -> Result<(), ExpectedError> {
        let smtp_username = libs::opt::get_value_str("email::smtp-username")?;
        let smtp_password = libs::opt::get_value_str("email::smtp-password")?;
        let credentials = Credentials::new(smtp_username, smtp_password);
        let smtp_relay = libs::opt::get_value_str("email::smtp-relay")?;
        let from = libs::opt::get_value_str("email::from")?;
        let reply_to = libs::opt::get_value_str("email::reply-to")?;

        let email = Message::builder()
            .from(from.as_str().parse().unwrap())
            .reply_to(reply_to.as_str().parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .body(String::from(body))
            .unwrap();

        let mailer = SmtpTransport::relay(smtp_relay.as_str())
            .unwrap()
            .credentials(credentials)
            .build();

        let _ = mailer.send(&email)?;
        Ok(())
    }
}

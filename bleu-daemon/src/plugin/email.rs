use crate::error::error::ExpectedError;
use appbase::prelude::*;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

#[derive(Clone, Debug)]
pub struct EmailConfig {
	username: String,
	password: String,
	relay: String,
	from: String,
	reply_to: String,
}

#[appbase_plugin]
pub struct EmailPlugin {
	receiver: Option<Receiver>,
	config: Option<EmailConfig>,
}

impl Plugin for EmailPlugin {
	fn new() -> Self {
		APP.options
			.arg(clap::Arg::new("email::smtp-username").long("smtp-username").takes_value(true));
		APP.options
			.arg(clap::Arg::new("email::smtp-password").long("smtp-password").takes_value(true));
		APP.options
			.arg(clap::Arg::new("email::smtp-relay").long("smtp-relay").takes_value(true));
		APP.options.arg(clap::Arg::new("email::from").long("from").takes_value(true));
		APP.options
			.arg(clap::Arg::new("email::reply-to").long("reply-to").takes_value(true));

		EmailPlugin { receiver: None, config: None }
	}

	fn init(&mut self) {
		self.receiver = Some(APP.channels.subscribe("email"));

		let username = APP
			.options
			.value_of("email::smtp-username")
			.expect("smtp-username not exist");
		let password = APP
			.options
			.value_of("email::smtp-password")
			.expect("smtp-password not exist");
		let relay = APP.options.value_of("email::smtp-relay").expect("smtp-relay not exist");
		let from = APP.options.value_of("email::from").expect("from not exist");
		let reply_to = APP.options.value_of("email::reply-to").expect("reply-to not exist");
		self.config = Some(EmailConfig { username, password, relay, from, reply_to });
	}

	fn startup(&mut self) {
		let receiver = self.receiver.take().unwrap();
		let config = self.config.take().unwrap();
		let app = APP.quit_handle().unwrap();
		Self::recv(receiver, config, app);
	}

	fn shutdown(&mut self) {}
}

impl EmailPlugin {
	fn recv(mut receiver: Receiver, config: EmailConfig, app: QuitHandle) {
		APP.spawn(async move {
			if let Ok(message) = receiver.try_recv() {
				let message = message.as_object().unwrap();

				let to = message.get("to").unwrap().as_str().unwrap();
				let subject = message.get("subject").unwrap().as_str().unwrap();
				let body = message.get("body").unwrap().as_str().unwrap();

				if let Err(e) = Self::send(&config, to, subject, body) {
					log::error!("this error will be ignored; {}", e);
				}
			}
			if !app.is_quitting() {
				tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
				Self::recv(receiver, config, app);
			}
		});
	}

	pub fn send(
		config: &EmailConfig,
		to: &str,
		subject: &str,
		body: &str,
	) -> Result<(), ExpectedError> {
		let EmailConfig { username, password, relay, from, reply_to } = config;
		let credentials = Credentials::new(username.clone(), password.clone());

		let mail = Message::builder()
			.from(from.parse().unwrap())
			.reply_to(reply_to.parse().unwrap())
			.to(to.parse().unwrap())
			.subject(subject)
			.body(body.to_string())
			.unwrap();

		let mailer = SmtpTransport::relay(relay).unwrap().credentials(credentials).build();

		let _ = mailer.send(&mail)?;
		Ok(())
	}
}

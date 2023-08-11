use appbase::prelude::*;
use std::collections::HashMap;

type SlackHooks = HashMap<String, String>;

#[derive(Clone, Debug)]
pub struct SlackConfig {
	active: bool,
	hooks: SlackHooks,
}

#[appbase_plugin]
pub struct SlackPlugin {
	receiver: Option<Receiver>,
	config: Option<SlackConfig>,
}

impl Plugin for SlackPlugin {
	fn new() -> Self {
		APP.options
			.arg(clap::Arg::new("slack::active").long("slack-active").takes_value(true));
		APP.options
			.arg(clap::Arg::new("slack::info").long("slack-info").takes_value(true));
		APP.options
			.arg(clap::Arg::new("slack::warn").long("slack-warn").takes_value(true));
		APP.options
			.arg(clap::Arg::new("slack::error").long("slack-error").takes_value(true));

		SlackPlugin { receiver: None, config: None }
	}

	fn init(&mut self) {
		let active = APP
			.options
			.value_of_t::<bool>("slack::active")
			.expect("slack::active does not exist");
		let info_hook = APP.options.value_of("slack::info").expect("slack::info does not exist");
		let warn_hook = APP.options.value_of("slack::warn").expect("slack::warn does not exist");
		let error_hook = APP.options.value_of("slack::error").expect("slack::error does not exist");

		let mut hooks = SlackHooks::new();
		hooks.insert("info".to_string(), info_hook);
		hooks.insert("warn".to_string(), warn_hook);
		hooks.insert("error".to_string(), error_hook);
		self.config = Some(SlackConfig { active, hooks });
		self.receiver = Some(APP.channels.subscribe("slack"));
	}

	fn startup(&mut self) {
		let config = self.config.take().unwrap();
		let receiver = self.receiver.take().unwrap();
		let app = APP.quit_handle().unwrap();
		Self::recv(receiver, config, app);
	}

	fn shutdown(&mut self) {}
}

impl SlackPlugin {
	fn recv(mut receiver: Receiver, config: SlackConfig, app: QuitHandle) {
		APP.spawn(async move {
			if let Ok(received) = receiver.try_recv() {
				let received = received.as_object().unwrap();
				let level = received.get("level").unwrap().as_str().unwrap();
				let message = received.get("message").unwrap().as_str().unwrap();

				let SlackConfig {active, hooks } = &config;
				if *active {
					if let Some(hook) = hooks.get(level) {
						let mut body: HashMap<&str, String> = HashMap::new();
						body.insert("text", message.to_string());
						if let Err(e) = reqwest::Client::new().post(hook).json(&body).send().await {
							log::error!("this error will be ignored; {}; level: {level}, message: {message}", e.to_string());
						}
					} else {
						log::error!("this error will be ignored; unsupported level; level: {level}");
					}
				}
			}
			if !app.is_quitting() {
				tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
				Self::recv(receiver, config, app);
			}
		});
	}
}

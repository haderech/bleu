use crate::types::channel::MultiSender;
use appbase::prelude::*;

#[derive(Default)]
#[appbase_plugin()]
pub struct Template {
	senders: Option<MultiSender>,
	receiver: Option<Receiver>,
}

impl Plugin for Template {
	fn new() -> Self {
		Self::default()
	}

	fn init(&mut self) {
		self.senders = Some(MultiSender::new(vec![]));
		self.receiver = Some(APP.channels.subscribe("template"));
	}

	fn startup(&mut self) {
		let receiver = self.receiver.take().unwrap();
		let senders = self.senders.take().unwrap();
		let app = APP.quit_handle().unwrap();

		Self::recv(receiver, senders, app);
	}

	fn shutdown(&mut self) {}
}

impl Template {
	fn recv(mut receiver: Receiver, senders: MultiSender, app: QuitHandle) {
		APP.spawn(async move {
			if let Ok(_message) = receiver.try_recv() {
				// Handling message from other plugins.
			}

			// Execute routine job.

			if !app.is_quitting() {
				tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
				Self::recv(receiver, senders, app);
			}
		});
	}
}

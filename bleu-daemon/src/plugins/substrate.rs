use crate::types::{channel::MultiSender, sync::SyncState};
use appbase::prelude::*;
use subxt::{Config, OnlineClient, PolkadotConfig, SubstrateConfig};
use crate::libs::sync::load_state;

pub enum SubstrateNodeConfig {}

impl Config for SubstrateNodeConfig {
    type Hash = <PolkadotConfig as Config>::Hash;
    type AccountId = <PolkadotConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <PolkadotConfig as Config>::Signature;
    type Hasher = <PolkadotConfig as Config>::Hasher;
    type Header = <PolkadotConfig as Config>::Header;
    type ExtrinsicParams = <SubstrateConfig as Config>::ExtrinsicParams;
}

#[subxt::subxt(runtime_metadata_path = "./metadata/metadata.scale")]
pub mod substrate {}

#[derive(Default)]
#[appbase_plugin()]
pub struct SubstratePlugin {
	state: Option<SyncState>,
	senders: Option<MultiSender>,
	receiver: Option<Receiver>,
}

impl Plugin for SubstratePlugin {
	fn new() -> Self {
		Self::default()
	}

	fn init(&mut self) {
		self.senders = Some(MultiSender::new(vec![]));
		self.receiver = Some(APP.channels.subscribe("substrate"));
		self.state = Some(load_state("substrate").expect("failed to load substrate"));
	}

	fn startup(&mut self) {
		let state = self.state.take().unwrap();
		let receiver = self.receiver.take().unwrap();
		let senders = self.senders.take().unwrap();
		let app = APP.quit_handle().unwrap();

		Self::recv(state, receiver, senders, app);
	}

	fn shutdown(&mut self) {}
}

impl SubstratePlugin {
	fn recv(mut state: SyncState, mut receiver: Receiver, senders: MultiSender, app: QuitHandle) {
		APP.spawn(async move {
			if let Ok(_) = receiver.try_recv() {
				// Handling message from other plugins.
			}
			let number = state.sync_idx;
			let url = state.endpoints[state.endpoint_idx as usize];

			if let Ok(client) = OnlineClient::<SubstrateNodeConfig>::from_url(url).await {

			} else {
				state.handle_error(format!("failed to create connection with url={}", url));
			}

			if !app.is_quitting() {
				tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
				Self::recv(state, receiver, senders, app);
			}
		});
	}
}

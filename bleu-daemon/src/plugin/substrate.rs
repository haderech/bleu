use crate::{
	error::error::ExpectedError,
	libs::sync::{control_state, error_state, init_state, load_state, save_state},
	types::{
		channel::MultiSender,
		sync::{SyncState, SyncStatus},
	},
	SyncRpcPlugin,
};
use appbase::prelude::*;
use subxt::{Config, OnlineClient, PolkadotConfig, SubstrateConfig};

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

#[subxt::subxt(runtime_metadata_path = "metadata/horizon.metadata.scale")]
pub mod substrate_node {}

#[derive(Default)]
#[appbase_plugin(SyncRpcPlugin)]
pub struct SubstratePlugin {
	senders: Option<MultiSender>,
	receiver: Option<Receiver>,
	state: Option<SyncState>,
}

impl Plugin for SubstratePlugin {
	fn new() -> Self {
		Self::default()
	}

	fn init(&mut self) {
		self.senders = Some(MultiSender::new(vec![]));
		self.receiver = Some(APP.channels.subscribe("substrate"));
		self.state = Some(load_state("substrate").unwrap_or_else(|_| {
			let state = SyncState {
				sync_type: "substrate".to_string(),
				chain_id: "dev".to_string(),
				from_idx: 0u64,
				sync_idx: 0u64,
				endpoint: "ws://127.0.0.1:9944".to_string(),
				status: SyncStatus::Working,
				message: "".to_string(),
			};
			init_state(&state).expect("failed to init substrate sync state");
			state
		}));
	}

	fn startup(&mut self) {
		let receiver = self.receiver.take().unwrap();
		let senders = self.senders.take().unwrap();
		let state = self.state.take().unwrap();
		let app = APP.quit_handle().unwrap();
		Self::recv(receiver, senders, state, app);
	}

	fn shutdown(&mut self) {}
}

impl SubstratePlugin {
	fn recv(mut receiver: Receiver, senders: MultiSender, mut state: SyncState, app: QuitHandle) {
		APP.spawn(async move {
			if let Ok(message) = receiver.try_recv() {
				let method = message.as_str().unwrap();
				if let Err(e) = control_state(method, &mut state) {
					log::error!("this error will be ignored; {}", e.to_string());
				}
			}

			if state.status == SyncStatus::Working {
				if let Err(e) = Self::execute(&state).await {
					if e.to_string().contains("UnknownBlock: State already discarded for") {
						log::error!(
							"this error will be ignored; {}; sync_type:{}, sync_idx: {}",
							e.to_string(),
							state.sync_type,
							state.sync_idx
						);
						state.sync_idx += 1;
						if let Err(e) = save_state(&state) {
							log::error!(
								"this error will be ignored; {}; sync_type:{}, sync_idx: {}",
								e.to_string(),
								state.sync_type,
								state.sync_idx
							);
						}
					} else if e.to_string().contains("block does not created") {
						log::error!(
							"this error will be ignored; {}; sync_type:{}, sync_idx: {}",
							e.to_string(),
							state.sync_type,
							state.sync_idx
						);
					} else {
						log::error!(
							"{}; sync_type:{}, sync_idx: {}",
							e.to_string(),
							state.sync_type,
							state.sync_idx
						);
						let _ = error_state(e, &mut state);
					}
				} else {
					state.sync_idx += 1;
					if let Err(e) = save_state(&state) {
						log::error!(
							"this error will be ignored; {}; sync_type:{}, sync_idx: {}",
							e.to_string(),
							state.sync_type,
							state.sync_idx
						);
					}
				}
			}

			if !app.is_quitting() {
				tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
				Self::recv(receiver, senders, state, app);
			}
		});
	}

	async fn execute(state: &SyncState) -> Result<(), ExpectedError> {
		let SyncState { sync_idx, endpoint, .. } = state;
		let api = OnlineClient::<SubstrateNodeConfig>::from_url(endpoint)
			.await
			.map_err(|e| ExpectedError::ConnectionError(e.to_string()))?;
		let rpc = api.rpc().clone();
		if let Some(hash) = rpc
			.block_hash(Some((*sync_idx).into()))
			.await
			.map_err(|e| ExpectedError::UnknownBlockError(e.to_string()))?
		{
			let block = api
				.blocks()
				.at(hash)
				.await
				.map_err(|e| ExpectedError::UnknownBlockError(e.to_string()))?;
			let block_number = block.header().number;
			let block_hash = block.hash();

			let body =
				block.body().await.map_err(|e| ExpectedError::JsonRpcError(e.to_string()))?;
			for extrinsic in body.extrinsics().iter() {
				let extrinsic =
					extrinsic.map_err(|e| ExpectedError::JsonRpcError(e.to_string()))?;
				// let extrinsic_idx = extrinsic.index();
				let events = extrinsic
					.events()
					.await
					.map_err(|e| ExpectedError::JsonRpcError(e.to_string()))?;
				// let bytes_hex = format!("0x{}", hex::encode(extrinsic.bytes()));

				let decoded_extrinsic = extrinsic.as_root_extrinsic::<substrate_node::Call>();

				log::debug!("block_number: {block_number}");
				log::debug!("block_hash: {block_hash}");
				log::debug!("decoded_extrinsic: {decoded_extrinsic:?}");

				for event in events.iter() {
					let event = event.map_err(|e| ExpectedError::JsonRpcError(e.to_string()))?;

					let pallet_name = event.pallet_name();
					let event_name = event.variant_name();
					// let event_values = event
					// .field_values()
					// .map_err(|e| ExpectedError::JsonRpcError(e.to_string()))?;

					if pallet_name == "Balances" && event_name == "Transfer" {
						let transfer_event = event
							.as_event::<substrate_node::balances::events::Transfer>()
							.unwrap()
							.unwrap();

						log::debug!("from: {:?}", transfer_event.from.0 .0);
						log::debug!("to: {:?}", transfer_event.to.0 .0);
						log::debug!("amount: {:?}", transfer_event.amount.to_string());
					}
				}
			}
		} else {
			return Err(ExpectedError::UnknownBlockError("block does not created".to_string()))
		}
		Ok(())
	}
}

use crate::{
	error::error::ExpectedError,
	plugin::{postgres::PostgresPlugin, slack::SlackPlugin},
	types::channel::MultiSender,
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

#[subxt::subxt(runtime_metadata_path = "metadata/metadata.scale")]
pub mod substrate_node {}

#[derive(Default)]
#[appbase_plugin(SlackPlugin)]
pub struct SubstratePlugin {
	senders: Option<MultiSender>,
	receiver: Option<Receiver>,
}

impl Plugin for SubstratePlugin {
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
		Self::recv(receiver, senders, 0u64, app);
	}

	fn shutdown(&mut self) {}
}

impl SubstratePlugin {
	fn recv(mut receiver: Receiver, senders: MultiSender, mut number: u64, app: QuitHandle) {
		APP.spawn(async move {
			if let Ok(_message) = receiver.try_recv() {}

			if let Ok(_) = Self::execute(number).await {
				number += 1;
			}

			if !app.is_quitting() {
				tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
				Self::recv(receiver, senders, number, app);
			}
		});
	}

	async fn execute(number: u64) -> Result<(), ExpectedError> {
		let api = OnlineClient::<SubstrateNodeConfig>::from_url("ws://127.0.0.1:9944")
			.await
			.map_err(|e| ExpectedError::ConnectionError(e.to_string()))?;
		let rpc = api.rpc().clone();
		if let Some(hash) = rpc
			.block_hash(Some(number.into()))
			.await
			.map_err(|e| ExpectedError::BlockHeightError(e.to_string()))?
		{
			let block = api
				.blocks()
				.at(hash)
				.await
				.map_err(|e| ExpectedError::BlockHeightError(e.to_string()))?;
			let block_number = block.header().number;
			let block_hash = block.hash();

			let body = block.body().await.map_err(|e| ExpectedError::TypeError(e.to_string()))?;
			for extrinsic in body.extrinsics().iter() {
				let extrinsic = extrinsic.map_err(|e| ExpectedError::TypeError(e.to_string()))?;
				let extrinsic_index = extrinsic.index();
				let events = extrinsic
					.events()
					.await
					.map_err(|e| ExpectedError::TypeError(e.to_string()))?;
				let bytes_hex = format!("0x{}", hex::encode(extrinsic.bytes()));

				let decoded_extrinsic = extrinsic.as_root_extrinsic::<substrate_node::Call>();

				println!("block_number: {block_number}");
				println!("block_hash: {block_hash}");
				println!("extrinsic_index: #{extrinsic_index}");
				println!("extrinsic_bytes: {bytes_hex}");
				println!("decoded_extrinsic: {decoded_extrinsic:?}");

				for event in events.iter() {
					let event = event.map_err(|e| ExpectedError::TypeError(e.to_string()))?;

					let pallet_name = event.pallet_name();
					let event_name = event.variant_name();
					let event_values = event
						.field_values()
						.map_err(|e| ExpectedError::TypeError(e.to_string()))?;

					if pallet_name == "Balances" && event_name == "Transfer" {
						println!("pallet_name: {pallet_name}");
						println!("event_name: {event_name}");
						println!("event_values: {event_values}");
					}
				}
			}
		} else {
			return Err(ExpectedError::BlockHeightError("block does not created.".to_string()))
		}
		Ok(())
	}
}

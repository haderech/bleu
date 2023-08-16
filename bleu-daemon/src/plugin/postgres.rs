use crate::{
	libs::postgres::{create_table, insert, load_schema},
	message,
	plugin::slack::SlackPlugin,
	types::{channel::MultiSender, postgres::PostgresSchema},
};
use appbase::prelude::*;
use r2d2_postgres::{postgres::NoTls, r2d2, PostgresConnectionManager};
use serde_json::Value;
use std::{collections::HashMap, thread, time::Duration};

#[appbase_plugin(SlackPlugin)]
pub struct PostgresPlugin {
	receiver: Option<Receiver>,
	senders: Option<MultiSender>,
	pool: Option<Pool>,
	schemas: Option<HashMap<String, PostgresSchema>>,
}

pub type Pool = r2d2::Pool<PostgresConnectionManager<NoTls>>;

message!(PostgresMsg; {schema: String}, {value: Value});

impl Plugin for PostgresPlugin {
	fn new() -> Self {
		APP.options
			.arg(clap::Arg::new("postgres::host").long("postgres-host").takes_value(true));
		APP.options
			.arg(clap::Arg::new("postgres::port").long("postgres-port").takes_value(true));
		APP.options
			.arg(clap::Arg::new("postgres::dbname").long("postgres-dbname").takes_value(true));
		APP.options
			.arg(clap::Arg::new("postgres::user").long("postgres-user").takes_value(true));
		APP.options
			.arg(clap::Arg::new("postgres::password").long("postgres-password").takes_value(true));

		PostgresPlugin { receiver: None, senders: None, pool: None, schemas: None }
	}

	fn init(&mut self) {
		let schema_files = vec!["ethereum"];
		let schemas = load_schema(schema_files).expect("failed to load schemas");

		let host = APP.options.value_of("postgres::host").expect("postgres::host does not exist");
		let port = APP.options.value_of("postgres::port").expect("postgres::port does not exist");
		let dbname = APP
			.options
			.value_of("postgres::dbname")
			.expect("postgres::dbname does not exist");
		let user = APP.options.value_of("postgres::user").expect("postgres::user does not exist");
		let password = APP
			.options
			.value_of("postgres::password")
			.expect("postgres::password does not exist");
		let manager = PostgresConnectionManager::new(
			format!("host={host} port={port} dbname={dbname} user={user} password={password}")
				.parse()
				.unwrap(),
			NoTls,
		);
		let pool: Pool = r2d2::Pool::builder().build(manager).expect("failed to create pool");

		create_table(&pool, &schemas).expect("failed to create tables");

		self.senders = Some(MultiSender::new(vec!["slack"]));
		self.receiver = Some(APP.channels.subscribe("postgres"));
		self.pool = Some(pool);
		self.schemas = Some(schemas);
	}

	fn startup(&mut self) {
		let pool = self.pool.as_ref().unwrap().clone();
		let schemas = self.schemas.as_ref().unwrap().clone();
		let receiver = self.receiver.take().unwrap();
		let senders = self.senders.take().unwrap();
		let app = APP.quit_handle().unwrap();

		Self::recv(pool, schemas, senders, receiver, app);
	}

	fn shutdown(&mut self) {}
}

impl PostgresPlugin {
	fn recv(
		pool: Pool,
		schemas: HashMap<String, PostgresSchema>,
		senders: MultiSender,
		mut receiver: Receiver,
		app: QuitHandle,
	) {
		APP.spawn_blocking(move || {
			if let Ok(message) = receiver.try_recv() {
				let message = message.as_object().unwrap();
				let target_schema = message.get("schema").unwrap().as_str().unwrap();
				let schema = schemas.get(target_schema).unwrap();
				let values = message.get("value").unwrap().as_object().unwrap();

				if let Err(e) = insert(&pool, schema, values) {
					log::error!("this error will be ignored; {}", e.to_string());
				}
			}
			if !app.is_quitting() {
				thread::sleep(Duration::from_millis(10));
				Self::recv(pool, schemas, senders, receiver, app);
			}
		});
	}
}

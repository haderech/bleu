use appbase::prelude::*;
use r2d2_postgres::{postgres::NoTls, r2d2, PostgresConnectionManager};
use serde_json::Value;
use std::{collections::HashMap, fs, thread, time::Duration};

use crate::{
	error::error::ExpectedError,
	libs::{
		self,
		postgres::{create_table, insert_value},
		serde::{get_object, get_string},
	},
	message,
	plugin::slack::{SlackMsg, SlackMsgLevel, SlackPlugin},
	types::{channel::MultiSender, enumeration::Enumeration, postgres::PostgresSchema},
};

#[appbase_plugin(SlackPlugin)]
pub struct PostgresPlugin {
	monitor: Option<Receiver>,
	senders: Option<MultiSender>,
	pool: Option<Pool>,
	schema_map: Option<HashMap<String, PostgresSchema>>,
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

		PostgresPlugin { monitor: None, senders: None, pool: None, schema_map: None }
	}

	fn init(&mut self) {
		let schema_map = Self::load_schema().expect("failed to load schema!");
		let pool = Self::create_pool().expect("failed to create pool!");
		create_table(pool.clone(), &schema_map).expect("failed to create tables!");
		let senders = MultiSender::new(vec!["slack"]);
		self.senders = Some(senders.to_owned());
		self.monitor = Some(APP.channels.subscribe("postgres"));
		self.pool = Some(pool);
		self.schema_map = Some(schema_map);
	}

	fn startup(&mut self) {
		let pool = self.pool.as_ref().unwrap().clone();
		let schema_map = self.schema_map.as_ref().unwrap().clone();
		let monitor = self.monitor.take().unwrap();
		let senders = self.senders.take().unwrap();
		let app = APP.quit_handle().unwrap();

		Self::recv(pool, schema_map, senders, monitor, app);
	}

	fn shutdown(&mut self) {}
}

impl PostgresPlugin {
	fn recv(
		pool: Pool,
		schema_map: HashMap<String, PostgresSchema>,
		senders: MultiSender,
		mut monitor: Receiver,
		app: QuitHandle,
	) {
		APP.spawn_blocking(move || {
			if let Ok(msg) = monitor.try_recv() {
				let parsed_msg = msg.as_object().unwrap();
				let schema_name = get_string(parsed_msg, "schema").unwrap();
				let selected_schema = schema_map.get(&schema_name).unwrap();
				let values = get_object(parsed_msg, "value").unwrap();
				if let Err(error) = insert_value(pool.clone(), selected_schema, values) {
					let _ = senders
						.get("slack")
						.send(SlackMsg::new(SlackMsgLevel::Warn.value(), error.to_string()));
				}
			}
			if !app.is_quitting() {
				thread::sleep(Duration::from_millis(10));
				Self::recv(pool, schema_map, senders, monitor, app);
			}
		});
	}

	fn load_schema() -> Result<HashMap<String, PostgresSchema>, ExpectedError> {
		let schema_files = vec!["schema/ethereum.json".to_string()];
		let mut schema_map = HashMap::new();
		for schema_file in schema_files.iter() {
			let json_str = fs::read_to_string(schema_file)?;
			let json_schema = serde_json::from_str::<serde_json::Value>(json_str.as_str())?;
			let json_schema = json_schema
				.as_object()
				.ok_or(ExpectedError::ParsingError("schema is not object.".to_string()))?;
			for (schema_name, values) in json_schema {
				schema_map.insert(
					schema_name.clone(),
					PostgresSchema::from(schema_name.clone(), values)?,
				);
			}
		}
		Ok(schema_map)
	}

	fn create_pool() -> Result<Pool, ExpectedError> {
		let host = libs::opt::get_value::<String>("postgres::host")?;
		let port = libs::opt::get_value::<String>("postgres::port")?;
		let dbname = libs::opt::get_value::<String>("postgres::dbname")?;
		let user = libs::opt::get_value::<String>("postgres::user")?;
		let password = libs::opt::get_value::<String>("postgres::password")?;

		let config = format!(
			"host={host} port={port} dbname={dbname} user={user} password={password}",
			host = host,
			port = port,
			dbname = dbname,
			user = user,
			password = password
		);

		let manager = PostgresConnectionManager::new(config.parse().unwrap(), NoTls);
		let pool: Pool = r2d2::Pool::builder().build(manager).expect("failed to create pool.");
		Ok(pool)
	}
}

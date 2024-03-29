use std::{fs, thread};
use std::collections::HashMap;
use std::time::Duration;

use appbase::prelude::*;
use r2d2_postgres::{PostgresConnectionManager, r2d2};
use r2d2_postgres::postgres::NoTls;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{libs, message};
use crate::error::error::ExpectedError;
use crate::libs::opt::opt_to_result;
use crate::libs::postgres::{create_table, insert_value};
use crate::libs::serde::{get_object, get_str};
use crate::plugin::slack::{SlackMsg, SlackMsgLevel};
use crate::plugin::slack::SlackPlugin;
use crate::types::channel::MultiSender;
use crate::types::enumeration::Enumeration;
use crate::types::postgres::PostgresSchema;

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
        APP.options.arg(clap::Arg::new("postgres::host").long("postgres-host").takes_value(true));
        APP.options.arg(clap::Arg::new("postgres::port").long("postgres-port").takes_value(true));
        APP.options.arg(clap::Arg::new("postgres::dbname").long("postgres-dbname").takes_value(true));
        APP.options.arg(clap::Arg::new("postgres::user").long("postgres-user").takes_value(true));
        APP.options.arg(clap::Arg::new("postgres::password").long("postgres-password").takes_value(true));

        PostgresPlugin {
            monitor: None,
            senders: None,
            pool: None,
            schema_map: None,
        }
    }

    fn init(&mut self) {
        let schema_map = Self::load_schema().expect("failed to load schema!");
        let pool = Self::create_pool().expect("failed to create pool!");
        create_table(pool.clone(), &schema_map).expect("failed to create tables!");
        let senders = MultiSender::new(vec!("slack"));
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
    fn recv(pool: Pool, schema_map: HashMap<String, PostgresSchema>, senders: MultiSender, mut monitor: Receiver, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(msg) = monitor.try_recv() {
                let parsed_msg = msg.as_object().unwrap();
                let schema_name = get_str(parsed_msg, "schema").unwrap();
                let selected_schema = schema_map.get(schema_name).unwrap();
                let values = get_object(parsed_msg, "value").unwrap();
                if let Err(error) = insert_value(pool.clone(), selected_schema, values) {
                    let _ = senders.get("slack").send(SlackMsg::new(SlackMsgLevel::Warn.value(), error.to_string()));
                }
            }
            if !app.is_quitting() {
                thread::sleep(Duration::from_millis(10));
                Self::recv(pool, schema_map, senders, monitor, app);
            }
        });
    }

    fn load_schema() -> Result<HashMap<String, PostgresSchema>, ExpectedError> {
        let schema_files = vec![String::from("schema/optimism.json"), String::from("schema/ethereum.json")];
        let mut schema_map = HashMap::new();
        for schema_file in schema_files.iter() {
            let json_str = fs::read_to_string(schema_file)?;
            let json_schema: Value = serde_json::from_str(json_str.as_str())?;
            let raw_schema_map = opt_to_result(json_schema.as_object())?;
            for (schema_name, values) in raw_schema_map {
                schema_map.insert(schema_name.clone(), PostgresSchema::from(schema_name.clone(), values)?);
            }
        }
        Ok(schema_map)
    }

    fn create_pool() -> Result<Pool, ExpectedError> {
        let host = libs::opt::get_value_str("postgres::host")?;
        let port = libs::opt::get_value_str("postgres::port")?;
        let dbname = libs::opt::get_value_str("postgres::dbname")?;
        let user = libs::opt::get_value_str("postgres::user")?;
        let password = libs::opt::get_value_str("postgres::password")?;

        let config = format!("host={host} port={port} dbname={dbname} user={user} password={password}", host = host, port = port, dbname = dbname, user = user, password = password);

        let manager = PostgresConnectionManager::new(config.parse().unwrap(), NoTls);
        let pool: Pool = r2d2::Pool::builder().build(manager).expect("failed to create pool.");
        Ok(pool)
    }
}
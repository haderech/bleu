use std::collections::HashMap;
use std::fs;

use appbase::prelude::*;
use diesel::{PgConnection, r2d2};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{enumeration, message};
use crate::error::error::ExpectedError;
use crate::libs::opts;
use crate::libs::opts::opt_to_result;
use crate::libs::serde::{find_value, get_object, get_str};
use crate::plugin::slack::{SlackMsg, SlackMsgLevel};
use crate::plugin::slack::SlackPlugin;
use crate::repository::ethereum::{create_eth_block, create_eth_table, create_eth_txs};
use crate::types::channel::MultiChannel;
use crate::types::enumeration::Enumeration;
use crate::types::postgres::PostgresSchema;

#[appbase_plugin(SlackPlugin)]
pub struct PostgresPlugin {
    monitor: Option<Receiver>,
    channels: Option<MultiChannel>,
    pool: Option<Pool>,
    schema_map: Option<HashMap<String, PostgresSchema>>,
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Client = PooledConnection<ConnectionManager<PgConnection>>;

message!((PostgresMsg; {value: Value}); (PostgresMethod; {CreateEthBlock: "create_eth_block"}));

impl Plugin for PostgresPlugin {
    fn new() -> Self {
        APP.options.arg(clap::Arg::new("postgres::url").long("postgres-url").takes_value(true));

        PostgresPlugin {
            monitor: None,
            channels: None,
            pool: None,
            schema_map: None,
        }
    }

    fn init(&mut self) {
        let schema_map = Self::load_schema().expect("failed to load schema!");
        let pool = Self::create_pool().expect("failed to create pool!");
        if let Err(err) = create_eth_table(pool.get().unwrap(), &schema_map) {
            log::warn!("{}", err.to_string());
        }
        let channels = MultiChannel::new(vec!("slack"));

        self.channels = Some(channels.to_owned());
        self.monitor = Some(APP.channels.subscribe("postgres"));
        self.pool = Some(pool);
        self.schema_map = Some(schema_map);
    }

    fn startup(&mut self) {
        let pool = self.pool.as_ref().unwrap().clone();
        let schema_map = self.schema_map.as_ref().unwrap().clone();
        let monitor = self.monitor.take().unwrap();
        let channels = self.channels.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(pool, schema_map, channels, monitor, app);
    }

    fn shutdown(&mut self) {}
}

impl PostgresPlugin {
    fn recv(pool: Pool, schema_map: HashMap<String, PostgresSchema>, channels: MultiChannel, mut monitor: Receiver, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(msg) = monitor.try_recv() {
                let parsed_msg = msg.as_object().unwrap();
                let method = PostgresMethod::find(get_str(parsed_msg, "method").unwrap()).unwrap();
                let value = get_object(parsed_msg, "value").unwrap();

                match method {
                    PostgresMethod::CreateEthBlock => {
                        let conn = pool.get().unwrap();
                        if let Err(err) = create_eth_block(conn, value) {
                            Self::error_handler(err, &channels);
                        };

                        let raw_txs = find_value(value, "transactions");
                        let txs = opt_to_result(raw_txs.as_array()).unwrap();
                        let conn = pool.get().unwrap();
                        if let Err(err) = create_eth_txs(conn, txs) {
                            Self::error_handler(err, &channels);
                        };
                    }
                };
            }
            if !app.is_quitting() {
                Self::recv(pool, schema_map, channels, monitor, app);
            }
        });
    }

    fn load_schema() -> Result<HashMap<String, PostgresSchema>, ExpectedError> {
        let json_str = fs::read_to_string("schema/ethereum.json").unwrap();
        let json_schema: Value = serde_json::from_str(json_str.as_str()).unwrap();
        let raw_schema_map = json_schema.as_object().unwrap();

        let mut schema_map = HashMap::new();
        for (schema_name, values) in raw_schema_map {
            schema_map.insert(schema_name.clone(), PostgresSchema::from(schema_name.clone(), values).unwrap());
        }
        Ok(schema_map)
    }

    fn create_pool() -> Result<Pool, ExpectedError> {
        let url = opts::string("postgres::url")?;

        let manager = ConnectionManager::<PgConnection>::new(url);
        let pool: Pool = r2d2::Pool::builder().build(manager).expect("failed to create pool.");
        Ok(pool)
    }

    fn error_handler(error: ExpectedError, channels: &MultiChannel) {
        let slack_err_msg = match error {
            ExpectedError::DieselError(err) => {
                log::error!("{}", err.to_string());
                SlackMsg::new(SlackMsgLevel::Error.value(), err.clone())
            }
            _ => {
                log::warn!("{}", error.to_string());
                SlackMsg::new(SlackMsgLevel::Warn.value(), error.to_string().clone())
            }
        };
        let slack_channel = channels.get("slack");
        let _ = slack_channel.send(slack_err_msg);
    }
}
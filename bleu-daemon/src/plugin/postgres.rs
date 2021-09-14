use std::collections::HashMap;
use std::fs;

use appbase::prelude::*;
use r2d2_postgres::{PostgresConnectionManager, r2d2};
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::r2d2::PooledConnection;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{enumeration, message};
use crate::error::error::ExpectedError;
use crate::libs::opts;
use crate::libs::opts::opt_to_result;
use crate::libs::serde::{find_value, get_object, get_str};
use crate::repository::ethereum::{create_eth_block, create_eth_table, create_eth_txs};
use crate::types::enumeration::Enumeration;
use crate::types::postgres::PostgresSchema;

#[appbase_plugin]
pub struct PostgresPlugin {
    monitor: Option<Receiver>,
    pool: Option<Pool>,
    schema_map: Option<HashMap<String, PostgresSchema>>,
}

type Pool = r2d2::Pool<PostgresConnectionManager<NoTls>>;
pub type Connection = PooledConnection<PostgresConnectionManager<NoTls>>;

message!((PostgresMsg; {value: Value}); (PostgresMethod; {CreateEthBlock: "create_eth_block"}));

impl Plugin for PostgresPlugin {
    fn new() -> Self {
        APP.options.arg(clap::Arg::new("postgres::host").long("postgres-host").takes_value(true));
        APP.options.arg(clap::Arg::new("postgres::port").long("postgres-port").takes_value(true));
        APP.options.arg(clap::Arg::new("postgres::dbname").long("postgres-dbname").takes_value(true));
        APP.options.arg(clap::Arg::new("postgres::user").long("postgres-user").takes_value(true));
        APP.options.arg(clap::Arg::new("postgres::password").long("postgres-password").takes_value(true));

        PostgresPlugin {
            monitor: None,
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
        self.monitor = Some(APP.channels.subscribe("postgres"));
        self.pool = Some(pool);
        self.schema_map = Some(schema_map);
    }

    fn startup(&mut self) {
        let pool = self.pool.as_ref().unwrap().clone();
        let schema_map = self.schema_map.as_ref().unwrap().clone();
        let monitor = self.monitor.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(pool, schema_map, monitor, app);
    }

    fn shutdown(&mut self) {}
}

impl PostgresPlugin {
    fn recv(pool: Pool, schema_map: HashMap<String, PostgresSchema>, mut monitor: Receiver, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(msg) = monitor.try_recv() {
                let parsed_msg = msg.as_object().unwrap();
                let method = PostgresMethod::find(get_str(parsed_msg, "method").unwrap()).unwrap();
                let value = get_object(parsed_msg, "value").unwrap();

                match method {
                    PostgresMethod::CreateEthBlock => {
                        let conn = pool.get().unwrap();
                        if let Err(err) = create_eth_block(conn, value) {
                            log::error!("{}", err.to_string());
                        };

                        let raw_txs = find_value(value, "transactions");
                        let txs = opt_to_result(raw_txs.as_array()).unwrap();
                        let conn = pool.get().unwrap();
                        if let Err(err) = create_eth_txs(conn, txs) {
                            log::error!("{}", err.to_string());
                        };
                    }
                };
            }
            if !app.is_quitting() {
                Self::recv(pool, schema_map, monitor, app);
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
        let host = opts::string("postgres::host")?;
        let port = opts::string("postgres::port")?;
        let dbname = opts::string("postgres::dbname")?;
        let user = opts::string("postgres::user")?;
        let password = opts::string("postgres::password")?;

        let manager = PostgresConnectionManager::new(
            format!("host={} port={} dbname={} user={} password={}", host, port, dbname, user, password).as_str().parse().unwrap(),
            NoTls,
        );
        Ok(r2d2::Pool::new(manager).unwrap())
    }
}
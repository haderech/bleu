use appbase::prelude::*;
use elasticsearch::{Elasticsearch, IndexParts};
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::http::Url;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::libs;
use crate::libs::serde::get_str;
use crate::message;

#[appbase_plugin]
pub struct ElasticsearchPlugin {
    monitor: Option<Receiver>,
    client: Option<Elasticsearch>,
}

message!(ElasticsearchMsg; {index: String}, {index_id: String}, {body: Value});

impl Plugin for ElasticsearchPlugin {
    fn new() -> Self {
        APP.options.arg(clap::Arg::new("elasticsearch::url").long("elasticsearch-url").takes_value(true));

        ElasticsearchPlugin {
            monitor: None,
            client: None,
        }
    }

    fn init(&mut self) {
        self.monitor = Some(APP.channels.subscribe("elasticsearch"));

        let elasticsearch_url = libs::opts::string("elasticsearch::url").unwrap();
        let url = Url::parse(elasticsearch_url.as_str()).unwrap();
        let conn_pool = SingleNodeConnectionPool::new(url);
        let transport = TransportBuilder::new(conn_pool).disable_proxy().build().unwrap();
        let client = Elasticsearch::new(transport);
        self.client = Some(client);
    }

    fn startup(&mut self) {
        let monitor = self.monitor.take().unwrap();
        let client = self.client.take().unwrap();
        let app = APP.quit_handle().unwrap();
        Self::recv(monitor, client, app);
    }

    fn shutdown(&mut self) {}
}

impl ElasticsearchPlugin {
    fn recv(mut monitor: Receiver, client: Elasticsearch, app: QuitHandle) {
        APP.spawn(async move {
            if let Ok(msg) = monitor.try_recv() {
                let parsed_msg = msg.as_object().unwrap();
                let index = get_str(parsed_msg, "index").unwrap();
                let index_id = get_str(parsed_msg, "index_id").unwrap();
                let body = parsed_msg.get("body").unwrap();

                let response = client.index(IndexParts::IndexId(index, index_id))
                    .body(body.clone())
                    .send()
                    .await
                    .unwrap();
                if let false = response.status_code().is_success() {
                    log::error!("{:?}", response.text().await);
                };
            }
            if !app.is_quitting() {
                Self::recv(monitor, client, app);
            }
        });
    }
}
use std::collections::HashMap;
use std::sync::Arc;

use appbase::prelude::*;
use futures::lock::Mutex as FutureMutex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::enumeration;
use crate::message;
use crate::plugin::postgres::PostgresPlugin;
use crate::types::channel::MultiChannel;
use crate::types::enumeration::Enumeration;
use crate::types::subscribe::SubscribeEvent;

#[appbase_plugin(PostgresPlugin)]
pub struct OptimismPlugin {
    sub_events: Option<SubscribeEvents>,
    channels: Option<MultiChannel>,
    monitor: Option<Receiver>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";

type SubscribeEvents = Arc<FutureMutex<HashMap<String, SubscribeEvent>>>;

message!((OptimismMsg; {value: Value}); (OptimismMethod; {Subscribe: "subscribe"}, {Resubscribe: "resubscribe"}, {Stop: "stop"}, {Unsubscribe: "unsubscribe"}));

impl Plugin for OptimismPlugin {
    fn new() -> Self {
        OptimismPlugin {
            sub_events: None,
            channels: None,
            monitor: None,
        }
    }

    fn init(&mut self) {
        self.sub_events = Some(Arc::new(FutureMutex::new(HashMap::new())));
        let channels = MultiChannel::new(vec!("optimism", "rocks", "postgres", /*"elasticsearch"*/));
        self.channels = Some(channels.to_owned());
        self.monitor = Some(APP.channels.subscribe("optimism"));
    }

    fn startup(&mut self) {
        let monitor = self.monitor.take().unwrap();
        let sub_events = self.sub_events.take().unwrap();
        let channels = self.channels.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(monitor, sub_events, channels, app);
    }

    fn shutdown(&mut self) {}
}

impl OptimismPlugin {
    fn recv(monitor: Receiver, sub_events: SubscribeEvents, channels: MultiChannel, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if !app.is_quitting() {
                Self::recv(monitor, sub_events, channels, app);
            }
        });
    }
}
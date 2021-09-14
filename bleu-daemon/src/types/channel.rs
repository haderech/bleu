use std::collections::HashMap;
use appbase::prelude::*;

#[derive(Clone)]
pub struct MultiChannel {
    channel_map: HashMap<String, Sender>,
}

impl MultiChannel {
    pub fn new(channels: Vec<&str>) -> Self {
        let mut channel_map = HashMap::new();
        for channel in channels.into_iter() {
            channel_map.insert(String::from(channel), APP.channels.get(channel));
        }
        MultiChannel {
            channel_map: channel_map.to_owned(),
        }
    }

    pub fn get(&self, name: &str) -> Sender {
        self.channel_map.get(name).unwrap().clone()
    }
}

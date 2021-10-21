use std::collections::HashMap;
use appbase::prelude::*;

#[derive(Clone)]
pub struct MultiSender {
    sender_map: HashMap<String, Sender>,
}

impl MultiSender {
    pub fn new(senders: Vec<&str>) -> Self {
        let mut sender_map = HashMap::new();
        for sender in senders.into_iter() {
            sender_map.insert(String::from(sender), APP.channels.get(sender));
        }
        MultiSender {
            sender_map: sender_map.to_owned(),
        }
    }

    pub fn get(&self, name: &str) -> Sender {
        self.sender_map.get(name).unwrap().clone()
    }
}

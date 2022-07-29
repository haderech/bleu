use std::env;

#[derive(Clone)]
pub struct NodeConfig {
  endpoint: String,
}

impl NodeConfig {
  pub fn load() -> Self {
    let node_endpoint = env::var("NODE_ENDPOINT").expect("NODE_ENDPOINT does not exist!");
    Self {
      endpoint: node_endpoint
    }
  }

  pub fn get_endpoint(&self) -> String {
    self.endpoint.clone()
  }
}

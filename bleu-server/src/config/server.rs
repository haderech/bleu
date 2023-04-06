use std::env;

pub fn load() -> (String, String) {
	let host = env::var("SERVER_HOST").expect("SERVER_HOST does not exist!");
	let port = env::var("SERVER_PORT").expect("SERVER_PORT does not exist!");
	(host, port)
}

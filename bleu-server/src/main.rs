#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use paperclip::actix::{web, OpenApiExt};
use service::swagger;

mod config;
mod error;
mod library;
mod model;
mod repository;
mod router;
mod schema;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv::dotenv().ok();
	env_logger::init();

	let (server_ip, server_port) = config::server::load();
	let pool = config::postgres::load();
	let swagger = config::swagger::load();

	HttpServer::new(move || {
		App::new()
			.route("/swagger", actix_web::web::get().to(swagger::load_swagger))
			.service(fs::Files::new("/swagger-ui", "./swagger-ui").show_files_listing())
			.wrap(Cors::default().allow_any_origin().send_wildcard())
			.wrap_api_with_spec(swagger.clone())
			.data(pool.clone())
			.service(web::scope("/api/v1").service(router::ethereum::route()))
			.with_json_spec_at("/api/spec")
			.build()
	})
	.bind(format!("{}:{}", server_ip, server_port))?
	.run()
	.await
}

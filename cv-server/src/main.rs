#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use paperclip::actix::{OpenApiExt, web};

use crate::config::postgres::PostgresConfig;
use crate::config::server::ServerConfig;
use crate::config::swagger::SwaggerConfig;
use crate::service::{block, swagger, tx};

mod service;
mod repository;
mod schema;
mod model;
mod error;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let server_config = ServerConfig::load();
  let postgres_config = PostgresConfig::load();
  HttpServer::new(move || {
    let swagger_config = SwaggerConfig::load();

    App::new()
      .route("/swagger", actix_web::web::get().to(swagger::load_swagger))
      .service(fs::Files::new("/swagger-ui", "./swagger-ui").show_files_listing())
      .wrap(Cors::default().allow_any_origin().send_wildcard())
      .wrap_api_with_spec(swagger_config.get_spec())
      .data(postgres_config.get_pool())
      .service(
        web::scope("/api/v1")
          .service(web::resource("/cosmos/block").route(web::get().to(block::get_block_by_page_and_count)))
          .service(web::resource("/cosmos/block/height/{height}").route(web::get().to(block::get_block_by_height)))
          .service(web::resource("/cosmos/tx").route(web::get().to(tx::get_tx_by_page_and_count)))
          .service(web::resource("/cosmos/tx/height/{height}").route(web::get().to(tx::get_tx_by_height_and_page_and_count)))
          .service(web::resource("/cosmos/tx/hash/{hash}").route(web::get().to(tx::get_tx_by_hash)))
      )
      .with_json_spec_at("/api/spec")
      .build()
  })
    .bind(server_config.get_binding_url())?
    .run()
    .await
}


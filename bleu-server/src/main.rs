#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use paperclip::actix::{OpenApiExt, web};

use crate::config::postgres::PostgresConfig;
use crate::config::server::ServerConfig;
use crate::config::swagger::SwaggerConfig;
use crate::service::optimism;

mod service;
mod repository;
mod schema;
mod model;
mod error;
mod libs;
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
            .wrap(Cors::default().allow_any_origin().send_wildcard())
            .wrap_api_with_spec(swagger_config.get_spec())
            .data(postgres_config.get_pool())
            .service(
                web::scope("/api/v1")
                    .service(web::resource("/optimism/batch/tx/latest").route(web::get().to(optimism::get_latest_tx_batch_summary)))
                    .service(web::resource("/optimism/batch/tx/index/{index}").route(web::get().to(optimism::get_tx_batch_by_index)))
                    .service(web::resource("/optimism/batch/tx/page/{page}/count/{count}").route(web::get().to(optimism::get_paginated_tx_batch)))
                    .service(web::resource("/optimism/tx/latest").route(web::get().to(optimism::get_latest_tx_summary)))
                    .service(web::resource("/optimism/tx/hash/{hash}").route(web::get().to(optimism::get_tx_by_hash)))
                    .service(web::resource("/optimism/tx/index/{index}").route(web::get().to(optimism::get_tx_by_index)))
                    .service(web::resource("/optimism/tx/batch/{index}/page/{page}/count/{count}").route(web::get().to(optimism::get_paginated_tx_by_tx_batch_index)))
                    .service(web::resource("/optimism/tx/stateroot/{index}/page/{page}/count/{count}").route(web::get().to(optimism::get_paginated_tx_by_state_batch_index)))
                    .service(web::resource("/optimism/batch/stateroot/page/{page}/count/{count}").route(web::get().to(optimism::get_paginated_state_batch)))
                    .service(web::resource("/optimism/batch/stateroot/index/{index}").route(web::get().to(optimism::get_state_batch_by_index)))
                    .service(web::resource("/optimism/tx/l1tol2/latest").route(web::get().to(optimism::get_latest_l1_to_l2_tx_summary)))
                    .service(web::resource("/optimism/tx/l1tol2/page/{page}/count/{count}").route(web::get().to(optimism::get_l1_to_l2_tx_by_page_count)))
                    .service(web::resource("/optimism/tx/logs/hash/{hash}").route(web::get().to(optimism::get_tx_logs_by_hash)))
                    .service(web::resource("/optimism/board/summary").route(web::get().to(optimism::get_board_summary)))
            )
            .with_json_spec_at(swagger_config.get_resource())
            .build()
    })
        .bind(server_config.get_binding_url())?
        .run()
        .await
}


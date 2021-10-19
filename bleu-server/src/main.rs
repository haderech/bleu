#[macro_use]
extern crate diesel;

use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use paperclip::actix::{OpenApiExt, web};
use paperclip::v2::models::{DefaultApiRaw, Info};

use crate::service::optimism;

mod service;
mod repository;
mod schema;
mod model;
mod error;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL does not exist!");
    let manager = ConnectionManager::<PgConnection>::new(postgres_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let host = env::var("SERVER_HOST").expect("SERVER_HOST does not exist!");
    let port = env::var("SERVER_PORT").expect("SERVER_PORT does not exist!");
    let endpoint = format!("{host}:{port}", host = host, port = port);

    HttpServer::new(move || {
        let mut spec = DefaultApiRaw::default();
        spec.info = Info {
            version: "0.1".into(),
            title: "Bleu Server".into(),
            ..Default::default()
        };

        App::new()
            .wrap(Cors::default().allowed_origin("http://localhost:3000"))
            .wrap_api_with_spec(spec)
            .data(pool.clone())
            .service(
                web::scope("/api/v1")
                    .service(web::resource("/optimism/batch/tx/latest").route(web::get().to(optimism::get_latest_tx_batch_summary)))
                    .service(web::resource("/optimism/batch/tx/index/{index}").route(web::get().to(optimism::get_tx_batch_by_index)))
                    .service(web::resource("/optimism/batch/tx/page/{page}/count/{count}").route(web::get().to(optimism::get_paginated_tx_batch)))
                    .service(web::resource("/optimism/tx/latest").route(web::get().to(optimism::get_latest_tx_summary)))
                    .service(web::resource("/optimism/tx/hash/{hash}").route(web::get().to(optimism::get_tx_by_hash)))
                    .service(web::resource("/optimism/tx/batch/{index}/page/{page}/count/{count}").route(web::get().to(optimism::get_paginated_tx)))
                    .service(web::resource("/optimism/batch/stateroot/page/{page}/count/{count}").route(web::get().to(optimism::get_paginated_state_batch)))
                    .service(web::resource("/optimism/batch/stateroot/index/{index}").route(web::get().to(optimism::get_state_batch_by_index)))
                    .service(web::resource("/optimism/tx/l1tol2/latest").route(web::get().to(optimism::get_latest_l1_to_l2_tx)))
            )
            .with_json_spec_at("/api/spec")
            .build()
    })
        .bind(endpoint)?
        .run()
        .await
}


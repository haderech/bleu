#[macro_use]
extern crate diesel;

use std::env;

use actix_web::{App, HttpServer, web};
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;

use crate::service::ethereum;
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
        App::new()
            .data(pool.clone())
            .service(web::scope("/api/v1")
                .service(ethereum::get_eth_block_by_id)
                .service(optimism::get_latest_batch_summary)
                .service(optimism::get_batch_by_index)
                .service(optimism::get_paginated_batch)
                .service(optimism::get_latest_tx_summary)
                .service(optimism::get_tx_by_hash)
                .service(optimism::get_paginated_tx)
                .service(optimism::get_latest_l1_to_l2_tx)
            )
    })
        .bind(endpoint)?
        .run()
        .await
}


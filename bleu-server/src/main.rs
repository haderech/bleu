#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer, web};
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;

use service::ethereum::get_eth_block_by_id;

mod service;
mod repository;
mod schema;
mod model;
mod error;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<PgConnection>::new("postgres://root:postgresql@localhost:5432/postgres");
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::scope("/api/v1")
                .route("/ethereum/block/{eth_blocks_id}", web::get().to(get_eth_block_by_id))
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


#[macro_use]
extern crate diesel;

use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;
use paperclip::actix::{OpenApiExt, web};
use paperclip::v2::models::{DefaultApiRaw, Info, Tag};

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
        spec.tags = vec![
            Tag {
                name: "TxBatch".to_string(),
                description: None,
                external_docs: None,
            },
            Tag {
                name: "StateRootBatch".to_string(),
                description: None,
                external_docs: None,
            },
            Tag {
                name: "Tx".to_string(),
                description: None,
                external_docs: None,
            },
        ];
        spec.info = Info {
            version: "0.1".into(),
            title: "Bleu Server".into(),
            ..Default::default()
        };
        let swagger_resource = env::var("SWAGGER_RESOURCE").expect("SWAGGER_RESOURCE does not exist!");
        let swagger_port = env::var("SWAGGER_PORT").expect("SWAGGER_PORT does not exist!");

        App::new()
            .wrap(Cors::default().allowed_origin(format!("http://localhost:{}", swagger_port).as_str()))
            .wrap_api_with_spec(spec)
            .data(pool.clone())
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
                    .service(web::resource("/optimism/tx/l1tol2/latest").route(web::get().to(optimism::get_latest_l1_to_l2_tx)))
            )
            .with_json_spec_at(&swagger_resource)
            .build()
    })
        .bind(endpoint)?
        .run()
        .await
}


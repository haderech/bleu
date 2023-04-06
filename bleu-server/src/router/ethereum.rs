use crate::service::ethereum::{
	get_block_by_number, get_blocks_by_page_count, get_logs_by_hash, get_tx_by_hash,
	get_txs_by_page_count,
};
use paperclip::actix::web;

pub fn route() -> web::Scope {
	web::scope("/ethereum")
		.service(web::resource("/blocks").route(web::get().to(get_blocks_by_page_count)))
		.service(web::resource("/block/{number}").route(web::get().to(get_block_by_number)))
		.service(web::resource("/txs").route(web::get().to(get_txs_by_page_count)))
		.service(web::resource("/tx/{hash}").route(web::get().to(get_tx_by_hash)))
		.service(web::resource("/logs/{hash}").route(web::get().to(get_logs_by_hash)))
}

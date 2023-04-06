use crate::service::ethereum::{
	get_block_by_hash, get_block_by_number, get_blocks_by_page_count, get_board_summary,
	get_latest_blocks, get_latest_txs, get_logs_by_hash, get_tx_by_hash, get_txs_by_page_count,
};
use paperclip::actix::web;

pub fn route() -> web::Scope {
	web::scope("/ethereum")
		.service(web::resource("/blocks").route(web::get().to(get_blocks_by_page_count)))
		.service(web::resource("/blocks/latest").route(web::get().to(get_latest_blocks)))
		.service(web::resource("/block/number/{number}").route(web::get().to(get_block_by_number)))
		.service(web::resource("/block/hash/{hash}").route(web::get().to(get_block_by_hash)))
		.service(web::resource("/txs").route(web::get().to(get_txs_by_page_count)))
		.service(web::resource("/txs/latest").route(web::get().to(get_latest_txs)))
		.service(web::resource("/tx/{hash}").route(web::get().to(get_tx_by_hash)))
		.service(web::resource("/logs/{hash}").route(web::get().to(get_logs_by_hash)))
		.service(web::resource("/board/summary").route(web::get().to(get_board_summary)))
}

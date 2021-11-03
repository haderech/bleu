use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestPage {
    pub page: i64,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PageInfo {
    page: i64,
    count: i64,
    total_page: i64,
    total_count: i64,
}

impl PageInfo {
    pub fn new(page: i64, count: i64, total_page: i64, total_count: i64) -> Self {
        Self {
            page,
            count,
            total_page,
            total_count,
        }
    }
}
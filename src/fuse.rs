#![allow(dead_code)]

use serde::Serialize;
use ts_derive::{TsEndpoint, TsResponse};

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "kpl_concept", desc = "获取概念股列表", resp = ConceptListItem)]
pub struct ConceptListRequest {
    pub trade_date: String,
}

#[derive(TsResponse, Debug)]
#[response(api = "kpl_concept")]
pub struct ConceptListItem {
    #[ts_field(0)]
    pub trade_date: String,
    #[ts_field(1)]
    pub ts_code: String,
    #[ts_field(2)]
    pub name: String,
    #[ts_field(3)]
    pub z_t_num: i64,
    #[ts_field(4)]
    pub up_num: String,
}

pub async fn example_usage() -> Vec<ConceptListItem> {
    let res1 = ConceptListRequest {
        trade_date: "20250403".to_string(),
    }
    .execute_typed()
    .await
    .expect("failed to execute concept list request");

    res1
}

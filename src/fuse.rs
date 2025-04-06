#![allow(dead_code)]

use ts_model::{ConceptListRequest, ConceptListItem};

pub async fn example_usage() -> Vec<ConceptListItem> {
    let res1 = ConceptListRequest {
        trade_date: "20250403".to_string(),
    }
    .execute_typed()
    .await
    .expect("failed to execute concept list request");

    res1
}

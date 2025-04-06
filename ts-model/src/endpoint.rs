use serde::Serialize;
use ts_derive::TsEndpoint;

use crate::model::ConceptListItem;

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "kpl_concept", desc = "获取概念股列表", resp = ConceptListItem)]
pub struct ConceptListRequest {
    pub trade_date: String,
}

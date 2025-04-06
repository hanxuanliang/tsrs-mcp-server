use serde::Serialize;
use ts_derive::TsEndpoint;

use crate::{
    ConceptListItem, KplConceptConsItem, KplListItem, LimitCptListItem, LimitStepItem, StkMinsItem,
    ThsHotItem, ThsMoneyflowCptItem, ThsMoneyflowItem,
};

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "limit_step", desc = "获取每天连板个数晋级的股票", resp = LimitStepItem)]
pub struct LimitStepReq {
    pub trade_date: String,
    pub start_date: String,
    pub end_date: String,
    pub nums: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "ths_hot", desc = "获取同花顺App热榜数据", resp = ThsHotItem)]
pub struct ThsHotReq {
    pub trade_date: String,
    pub market: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "kpl_list", desc = "获取涨跌停板数据", resp = KplListItem)]
pub struct KplListReq {
    pub tag: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "kpl_concept", desc = "获取开盘啦概念题材列表", resp = ConceptListItem)]
pub struct KplConceptReq {
    pub trade_date: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "kpl_concept_cons", desc = "获取开盘啦概念题材的成分股", resp = KplConceptConsItem)]
pub struct KplConceptConsReq {
    pub trade_date: String,
    pub ts_code: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "limit_cpt_list", desc = "获取每天涨停股票最多最强的概念板块", resp = LimitCptListItem)]
pub struct LimitCptListReq {
    pub trade_date: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "moneyflow_ths", desc = "获取同花顺个股资金流向数据", resp = ThsMoneyflowItem)]
pub struct ThsMoneyflowReq {
    pub ts_code: String,
    pub trade_date: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "moneyflow_cnt_ths", desc = "获取同花顺概念板块每日资金流向", resp = ThsMoneyflowCptItem)]
pub struct ThsMoneyflowCptReq {
    pub trade_date: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "stk_mins", desc = "获取A股分钟数据", resp = StkMinsItem)]
pub struct StkMinsReq {
    pub ts_code: String,
    pub freq: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

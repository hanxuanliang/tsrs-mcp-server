use serde::{Deserialize, Serialize};
use ts_derive::TsResponse;

#[derive(TsResponse, Serialize, Deserialize, Debug, Default)]
#[response(api = "kpl_list")]
pub struct KplListItem {
    #[ts_field(0)]
    #[serde(default)] 
    pub ts_code: String,
    #[ts_field(1)]
    #[serde(default)] 
    pub name: String,
    #[ts_field(2)]
    #[serde(default)] 
    pub trade_date: String,
    #[ts_field(3)]
    #[serde(default)]
    pub lu_time: String,
    #[ts_field(4)]
    #[serde(default)]
    pub ld_time: String,
    #[ts_field(5)]
    #[serde(default)]
    pub open_time: String,
    #[ts_field(6)]
    #[serde(default)]
    pub last_time: String,
    #[ts_field(7)]
    #[serde(default)]
    pub lu_desc: String,
    #[ts_field(8)]
    #[serde(default)]
    pub tag: String,
    #[ts_field(9)]
    #[serde(default)]
    pub theme: String,
    #[ts_field(10)]
    #[serde(default)]
    pub net_change: f64,
    #[ts_field(11)]
    #[serde(default)]
    pub bid_amount: f64,
    #[ts_field(12)]
    #[serde(default)]
    pub status: String,
    #[ts_field(13)]
    #[serde(default)]
    pub bid_change: f64,
    #[ts_field(14)]
    #[serde(default)]
    pub bid_turnover: f64,
    #[ts_field(15)]
    #[serde(default)]
    pub lu_bid_vol: f64,
    #[ts_field(16)]
    #[serde(default)]
    pub pct_chg: f64,
    #[ts_field(17)]
    #[serde(default)]
    pub bid_pct_chg: f64,
    #[ts_field(18)]
    #[serde(default)]
    pub rt_pct_chg: f64,
    #[ts_field(19)]
    #[serde(default)]
    pub limit_order: f64,
    #[ts_field(20)]
    #[serde(default)]
    pub amount: f64,
    #[ts_field(21)]
    #[serde(default)]
    pub turnover_rate: f64,
    #[ts_field(22)]
    #[serde(default)]
    pub free_float: f64,
    #[ts_field(23)]
    #[serde(default)]
    pub lu_limit_order: f64,
}

#[derive(TsResponse, Serialize, Debug)]
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

#[derive(TsResponse, Serialize, Debug)]
#[response(api = "kpl_concept_cons")]
pub struct KplConceptConsItem {
    #[ts_field(0)]
    pub ts_code: String,
    #[ts_field(1)]
    pub name: String,
    #[ts_field(2)]
    pub con_name: String,
    #[ts_field(3)]
    pub con_code: String,
    #[ts_field(4)]
    pub trade_date: String,
    #[ts_field(5)]
    pub desc: String,
    #[ts_field(6)]
    #[serde(default)]
    pub hot_num: String,
}

#[derive(TsResponse, Serialize, Deserialize, Debug)]
#[response(api = "ths_hot")]
pub struct ThsHotItem {
    #[ts_field(0)]
    pub trade_date: String,
    #[ts_field(1)]
    pub data_type: String,
    #[ts_field(2)]
    pub ts_code: String,
    #[ts_field(3)]
    pub ts_name: String,
    #[ts_field(4)]
    pub rank: i32,
    #[ts_field(5)]
    pub pct_change: f64,
    #[ts_field(6)]
    pub current_price: f64,
    #[ts_field(7)]
    pub concept: String,
    #[ts_field(8)]
    pub rank_reason: String,
    #[ts_field(9)]
    pub hot: f64,
    #[ts_field(10)]
    pub rank_time: String,
}

#[derive(TsResponse, Serialize, Debug)]
#[response(api = "limit_step")]
pub struct LimitStepItem {
    #[ts_field(0)]
    pub ts_code: String,
    #[ts_field(1)]
    pub name: String,
    #[ts_field(2)]
    pub trade_date: String,
    #[ts_field(3)]
    pub nums: String,
}

#[derive(TsResponse, Serialize, Debug)]
#[response(api = "limit_cpt_list")]
pub struct LimitCptListItem {
    #[ts_field(0)]
    pub ts_code: String,
    #[ts_field(1)]
    pub name: String,
    #[ts_field(2)]
    pub trade_date: String,
    #[ts_field(3)]
    pub days: i32,
    #[ts_field(4)]
    pub up_stat: String,
    #[ts_field(5)]
    pub cons_nums: i32,
    #[ts_field(6)]
    pub up_nums: i32,
    #[ts_field(7)]
    pub pct_chg: f64,
    #[ts_field(8)]
    pub rank: String,
}

#[derive(TsResponse, Serialize, Debug)]
#[response(api = "moneyflow_ths")]
pub struct ThsMoneyflowItem {
    #[ts_field(0)]
    pub trade_date: String,
    #[ts_field(1)]
    pub ts_code: String,
    #[ts_field(2)]
    pub name: String,
    #[ts_field(3)]
    pub pct_change: f64,
    #[ts_field(4)]
    pub latest: f64,
    #[ts_field(5)]
    pub net_amount: f64,
    #[ts_field(6)]
    pub net_d5_amount: f64,
    #[ts_field(7)]
    pub buy_lg_amount: f64,
    #[ts_field(8)]
    pub buy_lg_amount_rate: f64,
    #[ts_field(9)]
    pub buy_md_amount: f64,
    #[ts_field(10)]
    pub buy_md_amount_rate: f64,
    #[ts_field(11)]
    pub buy_sm_amount: f64,
    #[ts_field(12)]
    pub buy_sm_amount_rate: f64,
}

#[derive(TsResponse, Serialize, Debug)]
#[response(api = "moneyflow_cnt_ths")]
pub struct ThsMoneyflowCptItem {
    #[ts_field(0)]
    pub trade_date: String,
    #[ts_field(1)]
    pub ts_code: String,
    #[ts_field(2)]
    pub name: String,
    #[ts_field(3)]
    pub lead_stock: String,
    #[ts_field(4)]
    pub close_price: f64,
    #[ts_field(5)]
    pub pct_change: f64,
    #[ts_field(6)]
    pub index_close: f64,
    #[ts_field(7)]
    pub company_num: i32,
    #[ts_field(8)]
    pub pct_change_stock: f64,
    #[ts_field(9)]
    pub net_buy_amount: f64,
    #[ts_field(10)]
    pub net_sell_amount: f64,
    #[ts_field(11)]
    pub net_amount: f64,
}

#[derive(TsResponse, Serialize, Debug)]
#[response(api = "stk_mins")]
pub struct StkMinsItem {
    #[ts_field(0)]
    pub ts_code: String,
    #[ts_field(1)]
    pub trade_time: String,
    #[ts_field(2)]
    pub open: f64,
    #[ts_field(3)]
    pub close: f64,
    #[ts_field(4)]
    pub high: f64,
    #[ts_field(5)]
    pub low: f64,
    #[ts_field(6)]
    pub vol: i64,
    #[ts_field(7)]
    pub amount: f64,
}

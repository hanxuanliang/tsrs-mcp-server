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
#[endpoint(api = "limit_step", desc = "获取每天连板个数晋级的股票", resp = LimitStepItem)]
pub struct HisLimitStepReq {
    pub start_date: String,
    pub end_date: String,
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
    pub trade_date: String,
}

#[derive(TsEndpoint, Debug, Serialize)]
#[endpoint(api = "limit_list_ths", desc = "涨跌停榜单(同花顺)", resp = KplListItem)]
pub struct LimitListThs {
    pub tag: String,
    pub trade_date: String,
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

#[cfg(test)]
mod tests {
    use crate::endpoint::*;

    #[tokio::test]
    async fn test() {
        let res = ThsHotReq {
            trade_date: "20250407".to_string(),
            market: "热股".to_string(),
        }
        .execute()
        .await
        .unwrap_or_default();

        println!("res: {:?}", res);
    }

    #[tokio::test]
    async fn test2() {
        let res: Vec<KplListItem> = KplListReq {
            tag: "涨停".to_string(),
            trade_date: "20250407".to_string(),
        }
        .execute_typed()
        .await
        .unwrap_or_default();

        println!("{:?}", res);
    }

    #[tokio::test]
    async fn test_his_limit_step() {
        let res = HisLimitStepReq {
            start_date: "20250407".to_string(),
            end_date: "20250409".to_string(),
        }
        .execute_typed()
        .await
        .unwrap_or_default();

        for item in res {
            println!(
                "code: {:?} name: {:?}, status: {:?} trade_date: {:?}",
                item.ts_code, item.name, item.nums, item.trade_date
            );
        }
    }
}

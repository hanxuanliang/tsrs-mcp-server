use poem_mcpserver::{McpServer, Tools, stdio::stdio, tool::Json};
use ts_model::*;

struct TsApp {}

#[Tools]
impl TsApp {
    /// 获取每天连板个数晋级的股票
    ///
    /// # Arguments
    /// * `trade_date` - 交易日期(YYYYMMDD格式)
    /// * `start_date` - 开始日期(YYYYMMDD格式)
    /// * `end_date` - 结束日期(YYYYMMDD格式)
    /// * `nums` - 连板个数
    ///
    /// # Returns
    /// - `ts_code`: 股票代码
    /// - `name`: 股票名称
    /// - `trade_date`: 交易日期
    /// - `nums`: 连板次数,支持多个输入,例如nums='2,3'
    async fn limit_step(
        &self,
        trade_date: String,
        start_date: String,
        end_date: String,
        nums: String,
    ) -> Json<Vec<LimitStepItem>> {
        Json(
            LimitStepReq {
                trade_date,
                start_date,
                end_date,
                nums,
            }
            .execute_typed()
            .await
            .unwrap_or_default(),
        )
    }

    /// 获取同花顺App热榜数据
    ///
    /// # Arguments
    /// * `trade_date` - 交易日期(YYYYMMDD格式)
    ///
    /// # Returns
    /// - `trade_date`: 交易日期
    /// - `data_type`: 数据类型
    /// - `ts_code`: 股票代码
    /// - `ts_name`: 股票名称
    /// - `rank`: 排行
    /// - `pct_change`: 涨跌幅%
    /// - `current_price`: 当前价格
    /// - `concept`: 标签
    /// - `rank_reason`: 上榜解读
    /// - `hot`: 热度值
    /// - `rank_time`: 排行榜获取时间
    async fn ths_hot(&self, trade_date: String) -> Json<Vec<ThsHotItem>> {
        Json(
            ThsHotReq {
                trade_date,
                market: "热股".to_string(),
            }
            .execute_typed()
            .await
            .unwrap_or_default(),
        )
    }

    /// 获取涨跌停板数据
    ///
    /// # Arguments
    /// * `tag` - 板单类型(枚举值: 涨停/炸板/跌停/自然涨停/竞价)
    /// * `start_date` - 开始日期(YYYYMMDD格式)
    /// * `end_date` - 结束日期(YYYYMMDD格式)
    ///
    /// # Returns
    /// - `ts_code`: 代码
    /// - `name`: 名称
    /// - `trade_date`: 交易时间
    /// - `lu_time`: 涨停时间
    /// - `ld_time`: 跌停时间
    /// - `open_time`: 开板时间
    /// - `last_time`: 最后涨停时间
    /// - `lu_desc`: 涨停原因
    /// - `tag`: 标签
    /// - `theme`: 板块
    /// - `net_change`: 主力净额(元)
    /// - `bid_amount`: 竞价成交额(元)
    /// - `status`: 状态（N连板）
    /// - `bid_change`: 竞价净额
    /// - `bid_turnover`: 竞价换手%
    /// - `lu_bid_vol`: 涨停委买额
    /// - `pct_chg`: 涨跌幅%
    /// - `bid_pct_chg`: 竞价涨幅%
    /// - `rt_pct_chg`: 实时涨幅%
    /// - `limit_order`: 封单
    /// - `amount`: 成交额
    /// - `turnover_rate`: 换手率%
    /// - `free_float`: 实际流通
    /// - `lu_limit_order`: 最大封单
    async fn kpl_list(
        &self,
        tag: String,
        start_date: String,
        end_date: String,
    ) -> Json<Vec<KplListItem>> {
        Json(
            KplListReq {
                tag,
                start_date,
                end_date,
            }
            .execute_typed()
            .await
            .unwrap_or_default(),
        )
    }

    /// 获取开盘啦概念题材列表
    ///
    /// # Arguments
    /// * `trade_date` - 交易日期(YYYYMMDD格式)
    ///
    /// # Returns
    /// - `trade_date`: 交易日期
    /// - `ts_code`: 题材代码
    /// - `name`: 题材名称
    /// - `z_t_num`: 涨停数量
    /// - `up_num`: 排名上升位数
    async fn kpl_concept(&self, trade_date: String) -> Json<Vec<ConceptListItem>> {
        Json(
            KplConceptReq { trade_date }
                .execute_typed()
                .await
                .unwrap_or_default(),
        )
    }

    /// 获取开盘啦概念题材的成分股
    ///
    /// # Arguments
    /// * `trade_date` - 交易日期(YYYYMMDD格式)
    /// * `ts_code` - 题材代码(xxxxxx.KP格式)
    ///
    /// # Returns
    /// - `ts_code`: 题材ID
    /// - `name`: 题材名称
    /// - `con_name`: 股票名称
    /// - `con_code`: 股票代码
    /// - `trade_date`: 交易日期
    /// - `desc`: 描述
    /// - `hot_num`: 人气值
    async fn kpl_concept_cons(
        &self,
        trade_date: String,
        ts_code: String,
    ) -> Json<Vec<KplConceptConsItem>> {
        Json(
            KplConceptConsReq {
                trade_date,
                ts_code,
            }
            .execute_typed()
            .await
            .unwrap_or_default(),
        )
    }

    /// 获取每天涨停股票最多最强的概念板块
    ///
    /// # Arguments
    /// * `trade_date` - 交易日期(YYYYMMDD格式)
    /// * `start_date` - 开始日期(YYYYMMDD格式)
    /// * `end_date` - 结束日期(YYYYMMDD格式)
    ///
    /// # Returns
    /// - `ts_code`: 板块代码
    /// - `name`: 板块名称
    /// - `trade_date`: 交易日期
    /// - `days`: 上榜天数
    /// - `up_stat`: 连板高度
    /// - `cons_nums`: 连板家数
    /// - `up_nums`: 涨停家数
    /// - `pct_chg`: 涨跌幅%
    /// - `rank`: 板块热点排名
    async fn limit_cpt_list(
        &self,
        trade_date: String,
        start_date: String,
        end_date: String,
    ) -> Json<Vec<LimitCptListItem>> {
        Json(
            LimitCptListReq {
                trade_date,
                start_date,
                end_date,
            }
            .execute_typed()
            .await
            .unwrap_or_default(),
        )
    }

    /// 获取同花顺个股资金流向数据
    ///
    /// # Arguments
    /// * `ts_code` - 股票代码
    /// * `trade_date` - 交易日期(YYYYMMDD格式)
    /// * `start_date` - 开始日期(YYYYMMDD格式)
    /// * `end_date` - 结束日期(YYYYMMDD格式)
    ///
    /// # Returns
    /// - `trade_date`: 交易日期
    /// - `ts_code`: 股票代码
    /// - `name`: 股票名称
    /// - `pct_change`: 涨跌幅
    /// - `latest`: 最新价
    /// - `net_amount`: 资金净流入(万元)
    /// - `net_d5_amount`: 5日主力净额(万元)
    /// - `buy_lg_amount`: 今日大单净流入额(万元)
    /// - `buy_lg_amount_rate`: 今日大单净流入占比(%)
    /// - `buy_md_amount`: 今日中单净流入额(万元)
    /// - `buy_md_amount_rate`: 今日中单净流入占比(%)
    /// - `buy_sm_amount`: 今日小单净流入额(万元)
    /// - `buy_sm_amount_rate`: 今日小单净流入占比(%)
    async fn moneyflow_ths(
        &self,
        ts_code: String,
        trade_date: String,
        start_date: String,
        end_date: String,
    ) -> Json<Vec<ThsMoneyflowItem>> {
        Json(
            ThsMoneyflowReq {
                ts_code,
                trade_date,
                start_date,
                end_date,
            }
            .execute_typed()
            .await
            .unwrap_or_default(),
        )
    }

    /// 获取同花顺概念板块每日资金流向
    ///
    /// # Arguments
    /// * `trade_date` - 交易日期(YYYYMMDD格式)
    /// * `start_date` - 开始日期(YYYYMMDD格式)
    /// * `end_date` - 结束日期(YYYYMMDD格式)
    ///
    /// # Returns
    /// - `trade_date`: 交易日期
    /// - `ts_code`: 板块代码
    /// - `name`: 板块名称
    /// - `lead_stock`: 领涨股票名称
    /// - `close_price`: 最新价
    /// - `pct_change`: 行业涨跌幅
    /// - `index_close`: 板块指数
    /// - `company_num`: 公司数量
    /// - `pct_change_stock`: 领涨股涨跌幅
    /// - `net_buy_amount`: 流入资金(元)
    /// - `net_sell_amount`: 流出资金(元)
    /// - `net_amount`: 净额(元)
    async fn moneyflow_cnt_ths(
        &self,
        trade_date: String,
        start_date: String,
        end_date: String,
    ) -> Json<Vec<ThsMoneyflowCptItem>> {
        Json(
            ThsMoneyflowCptReq {
                trade_date,
                start_date,
                end_date,
            }
            .execute_typed()
            .await
            .unwrap_or_default(),
        )
    }

    /// 获取A股分钟数据
    ///
    /// # Arguments
    /// * `ts_code` - 股票代码
    /// * `freq` - 分钟频度(枚举值: 1min/5min/15min/30min/60min)
    /// * `start_date` - 开始日期
    /// * `end_date` - 结束日期
    ///
    /// # Returns
    /// - `ts_code`: 股票代码
    /// - `trade_time`: 交易时间
    /// - `open`: 开盘价
    /// - `close`: 收盘价
    /// - `high`: 最高价
    /// - `low`: 最低价
    /// - `vol`: 成交量
    /// - `amount`: 成交金额
    async fn stk_mins(
        &self,
        ts_code: String,
        freq: String,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> Json<Vec<StkMinsItem>> {
        Json(
            StkMinsReq {
                ts_code,
                freq,
                start_date,
                end_date,
            }
            .execute_typed()
            .await
            .unwrap_or_default(),
        )
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    stdio(McpServer::new().tools(TsApp {})).await
}

use ts_derive::TsResponse;

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

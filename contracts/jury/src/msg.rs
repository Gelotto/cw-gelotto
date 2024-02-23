use cosmwasm_schema::cw_serde;
use cw_table::lifecycle::LifecycleExecuteMsg;

use crate::state::models::Config;

pub type ArticleID = u16;

#[cw_serde]
pub enum ArticleValue {
    Website(String),
    ImageUrl(String),
    VideoUrl(String),
    Article(ArticleID),
}

#[cw_serde]
pub struct Article {
    pub description: String,
    pub value: ArticleValue,
}

#[cw_serde]
pub struct JurorVoteMsg {
    pub id: String,
    pub rationale: Option<String>,
    pub evidence: Option<Vec<Article>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Lifecycle(LifecycleExecuteMsg),
    SetConfig(Config),
    Vote(JurorVoteMsg),
    Follow {},
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);

use cosmwasm_schema::cw_serde;
use cw_table::lifecycle::LifecycleExecuteMsg;
use gelotto_jury_lib::models::{ArticleID, ArticleValue};

use crate::state::models::Config;

#[cw_serde]
pub struct ArticleMsg {
    pub description: String,
    pub value: ArticleValue,
}

#[cw_serde]
pub struct JurorVoteMsg {
    pub answer_id: String,
    pub rationale: Option<String>,
}

#[cw_serde]
pub enum EvidenceMsg {
    Add(Vec<ArticleMsg>),
    Remove(Vec<ArticleID>),
    Vote { article_id: ArticleID, vote: i8 },
}

#[cw_serde]
pub enum ExecuteMsg {
    Lifecycle(LifecycleExecuteMsg),
    SetConfig(Config),
    Vote(JurorVoteMsg),
    Evidence(EvidenceMsg),
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

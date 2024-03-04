use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

use crate::juror::models::JurorRequirements;

pub const MAX_SPEED_SCORE: u8 = 100;

pub type ArticleID = u16;

/// This is the payload received by the jury pool when a client requests that a new
/// jury be formed around the given charges (tasks).
#[cw_serde]
pub struct JuryRequest {
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub settings: JurySettings,
    pub requirements: JurorRequirements,
    pub task: JuryTask,
}

#[cw_serde]
pub struct JurySettings {
    /// When voting opens to the jury pool
    pub starts_at: Timestamp,
    /// Ideal timeframe for jury to come to its verdict
    pub target_duration_sec: u32,
    /// Time after which the jury times out AKA hung jury
    pub max_duration_sec: u32,
    /// Can the verdict be appealed or is it final?
    pub allow_appeals: bool,
    /// Minimum number of qualified votes for consensus
    pub min_vote_count: u32,
    /// Miniumum majority as percentage in form of u32
    pub min_consensus_pct: u32,
    /// Fixed payout to first N qualified jurors
    pub incentive: Option<BaseIncentive>,
}

#[cw_serde]
pub struct BaseIncentive {
    pub amount: Uint128,
    pub units: u32,
}

#[cw_serde]
pub struct JuryTask {
    pub prompt: String,
    pub answers: Vec<Answer>,
}

#[cw_serde]
pub struct Answer {
    pub text: String,
    pub id: String,
}

#[cw_serde]
pub enum ArticleValue {
    Website(String),
    Image(String),
    Video(String),
}

#[cw_serde]
pub struct Article {
    pub owner: Addr,
    pub description: String,
    pub value: ArticleValue,
    pub rank: i16,
}

#[cw_serde]
pub struct Verdict {
    pub answer_id: String,
    pub created_at: Timestamp,
}

#[cw_serde]
pub struct JuryConfig {}

#[cw_serde]
pub struct JurorVoteMetadata {
    pub qualified: u8,
    pub time: Timestamp,
    pub answer_id: String,
}

#[cw_serde]
pub struct VotingPeriod {
    pub start: Timestamp,
    pub stop: Timestamp,
    pub target: Timestamp,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};
use gelotto_core::models::token::TokenAmount;

pub const MAX_SPEED_SCORE: u8 = 100;

pub type ArticleID = u16;

/// This is the payload received by the jury pool when a client requests that a new
/// jury be formed around the given charges (tasks).
#[cw_serde]
pub struct JuryRequest {
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub settings: JurySettings,
    pub task: JuryTask,
    pub jurors: JurorConfig,
}

#[cw_serde]
pub struct JurorConfig {
    /// Minimum scores necessary for incentive participation
    pub qualifications: JurorQualifications,
    /// Minimum bonding token amounts and/or NFTs
    pub bond_requirements: Vec<Bond>,
}

#[cw_serde]
pub struct JurorQualifications {
    /// Total number of juries participated in
    pub n_juries: Option<u32>,
    /// Score for how promptly juror votes within target window
    pub speed: Option<u8>,
    /// Score for how often juror participates w/o dispute
    pub precision: Option<u8>,
    /// Score for personal identity verification level
    pub identity: Option<u8>,
    /// Score for how often juror proposes correct answer
    pub initiative: Option<u8>,
    /// Score for how much juror cites credible evidence
    pub research: Option<u8>,
    /// Scores for specific named areas of expertise
    pub expertise: Vec<DomainExpertise>,
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
    ImageUrl(String),
    VideoUrl(String),
    Article(ArticleID),
}

#[cw_serde]
pub struct Article {
    pub owner: Addr,
    pub description: String,
    pub value: ArticleValue,
    pub rank: i16,
}

#[cw_serde]
pub struct DomainExpertise {
    pub domain: String,
    pub score: u16,
}

#[cw_serde]
pub struct Verdict {
    pub answer_id: String,
    pub created_at: Timestamp,
}

#[cw_serde]
pub enum Bond {
    Token(TokenAmount),
    Nft { cw721_addr: Addr },
}

impl Bond {
    pub fn get_key(&self) -> String {
        match self {
            Bond::Nft { cw721_addr } => cw721_addr.to_string(),
            Bond::Token(TokenAmount { token, amount }) => {
                format!("{}:{}", token.to_key(), amount.u128())
            }
        }
    }
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};
use cw_utils::Duration;

use super::token::TokenAmount;

#[cw_serde]
pub struct Config {}

/// This is the payload received by the jury pool when a client requests that a new
/// jury be formed around the given charges (tasks).
#[cw_serde]
pub struct JuryRequest {
    pub settings: JuryParams,
    pub charges: Vec<Charge>,
    pub jurors: JurorConfig,
}

#[cw_serde]
pub struct JurorConfig {
    /// Minimum scores necessary for incentive participation
    pub qualifications: JurorQualifications,
    /// Minimum bonding token amounts and/or NFTs
    pub bond: Vec<Bond>,
}

#[cw_serde]
pub struct JurorQualifications {
    /// Score for personal identity verification level
    pub identity: Option<u8>,
    /// Score for how often juror proposes correct work
    pub initiative: Option<u8>,
    /// Score for how often juror participates w/o dispute
    pub precision: Option<u8>,
    /// Score for how promptly juror performs work
    pub speed: Option<u8>,
    /// Score for how much juror offers & cites evidentiary material
    pub research: Option<u8>,
    /// Specific areas of expertise
    pub expertise: Vec<DomainExpertise>,
}

#[cw_serde]
pub struct JuryParams {
    /// When voting opens to the jury pool
    pub starts_at: Timestamp,
    /// Ideal timeframe for jury to come to its verdict
    pub target_duration: Duration,
    /// Time after which the jury times out AKA hung jury
    pub max_duration: Duration,
    /// Can the verdict be appealed or is it final?
    pub allow_appeals: bool,
    /// Minimum number of qualified votes for consensus
    pub min_vote_count: u32,
    /// Miniumum majority as percentage in form of u32
    pub min_consensus: u32,
}

#[cw_serde]
pub enum Charge {
    MultipleChoice {
        prompt: String,
        answers: Vec<Answer>,
    },
    FreeResponse {
        prompt: String,
    },
}

#[cw_serde]
pub struct Answer {
    pub text: String,
    pub id: String,
}

#[cw_serde]
pub struct DomainExpertise {
    pub domain: String,
    pub score: u16,
}

#[cw_serde]
pub enum Bond {
    Token(TokenAmount),
    Nft {
        collection_addr: Addr,
        token_ids: Option<Vec<String>>,
    },
}

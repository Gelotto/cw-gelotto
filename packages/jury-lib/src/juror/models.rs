use cosmwasm_schema::cw_serde;

use crate::bond::Bond;

#[cw_serde]
pub struct JurorConfig {}

#[cw_serde]
pub struct JurorRequirements {
    /// Minimum scores necessary for incentive participation
    pub scores: JurorQualifications,
    /// Minimum bonding token amounts and/or NFTs
    pub bond: Vec<Bond>,
}

#[cw_serde]
pub struct DomainExpertise {
    pub domain: String,
    pub score: u16,
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

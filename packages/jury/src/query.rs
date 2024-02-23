use cosmwasm_schema::cw_serde;

use crate::models::{DomainExpertise, JurorQualifications};

#[cw_serde]
pub struct JurorPerformance {
    /// Total number of juries participated in
    pub xp: u32,
    /// Score for personal identity verification level
    pub identity: u8,
    /// Score for how often juror proposes correct work
    pub initiative: u8,
    /// Score for how often juror participates w/o dispute
    pub precision: u8,
    /// Score for how promptly juror performs work
    pub speed: u8,
    /// Score for how much juror offers & cites evidentiary material
    pub research: u8,
    /// Specific areas of expertise
    pub expertise: Vec<DomainExpertise>,
}

#[cw_serde]
pub enum JurorQueryMsg {
    Config {},
    Performance {},
    Qualifies(JurorQualifications),
}

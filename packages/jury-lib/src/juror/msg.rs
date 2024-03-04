use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_table::lifecycle::LifecycleExecuteMsg;

use super::models::{DomainExpertise, JurorConfig, JurorQualifications};

#[cw_serde]
pub struct InstantiateMsg {
    pub acl: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Lifecycle(LifecycleExecuteMsg),
    SetConfig(JurorConfig),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub JurorConfig);

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
pub enum QueryMsg {
    Config {},
    Performance {},
    Qualifies(JurorQualifications),
}

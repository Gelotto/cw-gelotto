use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use gelotto_jury_lib::models::JuryRequest;

use crate::state::models::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub acl: Addr,
    pub table: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetConfig(Config),
    RequestJury(JuryRequest),
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);

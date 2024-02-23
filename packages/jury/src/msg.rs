use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::models::JuryRequest;

#[cw_serde]
pub struct JuryInstantiateMsg {
    pub acl: Addr,
    pub request: JuryRequest,
}

#[cw_serde]
pub struct JurorInstantiateMsg {
    pub acl: Addr,
}

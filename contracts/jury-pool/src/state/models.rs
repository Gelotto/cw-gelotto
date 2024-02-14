use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint64};

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub enum SubMsgReplyJob {
    JuryInstantiated { jury_id: Uint64, initiator: Addr },
}

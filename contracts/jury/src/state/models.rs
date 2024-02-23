use cosmwasm_schema::cw_serde;
use cosmwasm_std::Timestamp;

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct JurorVoteMetadata {
    pub qualified: u8,
    pub time: Timestamp,
    pub answer_id: String,
}

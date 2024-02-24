use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Timestamp, Uint64};

pub mod owner;
pub mod token;

#[cw_serde]
pub enum Moment {
    Time(Timestamp),
    Height(Uint64),
}

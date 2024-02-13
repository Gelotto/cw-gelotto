use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub enum Token {
    Denom(String),
    Address(Addr),
}

#[cw_serde]
pub struct TokenAmount {
    pub token: Token,
    pub amount: Uint128,
}

impl Token {
    pub fn to_key(&self) -> String {
        match self {
            Self::Address(address) => address.to_string(),
            Self::Denom(denom) => denom.clone(),
        }
    }

    pub fn get_denom(&self) -> Option<String> {
        if let Self::Denom(denom) = self {
            Some(denom.clone())
        } else {
            None
        }
    }

    pub fn get_address(&self) -> Option<Addr> {
        if let Self::Address(addr) = self {
            Some(addr.clone())
        } else {
            None
        }
    }
}

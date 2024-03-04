use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use gelotto_core::models::token::TokenAmount;

#[cw_serde]
pub enum Bond {
    Token(TokenAmount),
    Nft { cw721_addr: Addr },
}

impl Bond {
    pub fn get_key(&self) -> String {
        match self {
            Bond::Nft { cw721_addr } => cw721_addr.to_string(),
            Bond::Token(TokenAmount { token, amount }) => {
                format!("{}:{}", token.to_key(), amount.u128())
            }
        }
    }
}

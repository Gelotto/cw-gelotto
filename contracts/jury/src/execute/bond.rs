use crate::{
    error::ContractError,
    state::storage::{JUROR_BONDS, JUROR_BOND_REQUIREMENTS},
};
use cosmwasm_std::{attr, has_coins, Addr, Attribute, Coin, Response, Uint128};
use gelotto_core::models::token::{Token, TokenAmount};
use gelotto_jury_lib::models::Bond;

use super::Context;

pub fn exec_bond(
    ctx: Context,
    bond: Bond,
    sender: Option<Addr>,
    cw20_amount: Option<Uint128>,
    _nft_token_id: Option<String>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let bond_sender = sender.unwrap_or(info.sender.clone());
    let mut attrs: Vec<Attribute> = vec![attr("action", "bond")];

    if let Some(requirements) = JUROR_BOND_REQUIREMENTS.may_load(deps.storage)? {
        let bond_key = bond.get_key();
        // Process each bonding requirement and ensure that we've actually
        // received the assets received. Bond can be fungible tokens or NFTs
        if requirements
            .iter()
            .find(|b| b.get_key() == bond_key)
            .is_some()
        {
            // Don't re-bond the same thing.
            if !JUROR_BONDS.has(deps.storage, (&bond_sender, &bond_key)) {
                match &bond {
                    // Bonding fungible tokens
                    Bond::Token(TokenAmount { amount, token }) => match token {
                        // If token is native/tokenfactory, ensure at least the
                        // required amount.
                        Token::Denom(denom) => {
                            if !has_coins(&info.funds, &Coin::new(amount.u128(), denom)) {
                                return Err(ContractError::InvalidBond { bond });
                            } else {
                                // Success! We received payment
                                attrs.push(attr("denom", denom));
                                attrs.push(attr("amount", amount.to_string()));
                            }
                        }
                        // If token is CW20, ensure tx sent by expected CW20 and
                        // correct amount.
                        Token::Address(cw20_addr) => {
                            // Is from expected CW20 contract address?
                            if info.sender != cw20_addr {
                                return Err(ContractError::InvalidBond { bond });
                            }
                            // Check token amount
                            if let Some(amount_received) = cw20_amount {
                                if amount_received < *amount {
                                    return Err(ContractError::InvalidBond { bond });
                                } else {
                                    // Success! We received payment
                                    attrs.push(attr("cw20", cw20_addr.to_string()));
                                    attrs.push(attr("amount", amount.to_string()));
                                }
                            } else {
                                return Err(ContractError::InvalidBond { bond });
                            }
                        }
                    },
                    // Are we bonding an NFT?
                    Bond::Nft { cw721_addr: _ } => {
                        // TODO: Ensure that the token ID is valid by querying the CW721
                        return Err(ContractError::ValidationError {
                            reason: "NFT bond not yet supported".into(),
                        });
                    }
                }
                // Save the user's bond
                JUROR_BONDS.save(deps.storage, (&bond_sender, &bond_key), &true)?;
            }
        } else {
            return Err(ContractError::InvalidBond { bond });
        }
    }

    Ok(Response::new().add_attributes(attrs))
}

use crate::{error::ContractError, state::storage::JUROR_HAS_CLAIMED};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_claim(ctx: Context) -> Result<Response, ContractError> {
    let Context { info, deps, .. } = ctx;

    // Prevent double claim
    JUROR_HAS_CLAIMED.update(
        deps.storage,
        &info.sender,
        |maybe_b| -> Result<_, ContractError> {
            if maybe_b.is_some() {
                return Err(ContractError::NotAuthorized {
                    reason: "Already claimed".to_string(),
                });
            }
            Ok(true)
        },
    )?;

    // Send incentive

    Ok(Response::new().add_attributes(vec![attr("action", "claim")]))
}

use crate::error::ContractError;
use cosmwasm_std::{attr, Response};
use gelotto_jury_lib::jury::models::JuryConfig;

use super::Context;

pub fn exec_set_config(ctx: Context, _config: JuryConfig) -> Result<Response, ContractError> {
    let Context { .. } = ctx;
    Ok(Response::new().add_attributes(vec![attr("action", "set_config")]))
}

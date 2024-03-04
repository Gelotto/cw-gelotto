pub mod storage;

use cosmwasm_std::Response;
use gelotto_jury_lib::juror::msg::InstantiateMsg;

use crate::{error::ContractError, execute::Context};

/// Top-level initialization of contract state
pub fn init(ctx: Context, _msg: &InstantiateMsg) -> Result<Response, ContractError> {
    let Context { .. } = ctx;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

use gelotto_jury_lib::jury::{models::JuryConfig, msg::ConfigResponse};

use crate::error::ContractError;

use super::ReadonlyContext;

pub fn query_config(ctx: ReadonlyContext) -> Result<ConfigResponse, ContractError> {
    let ReadonlyContext { .. } = ctx;
    Ok(ConfigResponse(JuryConfig {}))
}

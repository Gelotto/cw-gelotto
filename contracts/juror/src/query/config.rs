use crate::{error::ContractError, msg::ConfigResponse, state::models::Config};

use super::ReadonlyContext;

pub fn query_config(ctx: ReadonlyContext) -> Result<ConfigResponse, ContractError> {
    let ReadonlyContext { .. } = ctx;
    Ok(ConfigResponse(Config {}))
}

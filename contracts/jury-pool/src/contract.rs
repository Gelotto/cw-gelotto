use crate::error::ContractError;
use crate::execute::request_jury::{exec_request_jury, save_jury};
use crate::execute::set_config::exec_set_config;
use crate::execute::{Context, ReplyContext};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{query_config, ReadonlyContext};
use crate::state;
use crate::state::models::SubMsgReplyJob;
use crate::state::storage::REPLY_JOBS;
use cosmwasm_std::{entry_point, to_json_binary, Reply};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-jury-pool";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(state::init(Context { deps, env, info }, &msg)?)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = Context { deps, env, info };
    match msg {
        ExecuteMsg::SetConfig(config) => exec_set_config(ctx, config),
        ExecuteMsg::RequestJury(req) => exec_request_jury(ctx, req),
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    // Pop and process the staged reply job
    let job = REPLY_JOBS.load(deps.storage, reply.id)?;
    REPLY_JOBS.remove(deps.storage, reply.id);
    let ctx = ReplyContext { deps, env };
    return Ok(match job {
        SubMsgReplyJob::JuryInstantiated { jury_id, initiator } => {
            save_jury(ctx, jury_id, initiator)
        }
    }?);
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let ctx = ReadonlyContext { deps, env };
    let result = match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(ctx)?),
    }?;
    Ok(result)
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

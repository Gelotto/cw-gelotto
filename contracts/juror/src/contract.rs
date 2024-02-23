use crate::error::ContractError;
use crate::execute::lifecycle::{exec_resume, exec_setup, exec_suspend, exec_teardown};
use crate::execute::set_config::exec_set_config;
use crate::execute::Context;
use crate::msg::{ExecuteMsg, MigrateMsg};
use crate::query::performance::query_performance;
use crate::query::qualifies::query_qualifies;
use crate::query::{query_config, ReadonlyContext};
use crate::state;
use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use cw_table::lifecycle::LifecycleExecuteMsg;
use gelotto_jury_lib::msg::JurorInstantiateMsg;
use gelotto_jury_lib::query::JurorQueryMsg;

const CONTRACT_NAME: &str = "crates.io:cw-juror";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: JurorInstantiateMsg,
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
        ExecuteMsg::Lifecycle(msg) => match msg {
            LifecycleExecuteMsg::Setup(args) => exec_setup(ctx, args),
            LifecycleExecuteMsg::Teardown(args) => exec_teardown(ctx, args),
            LifecycleExecuteMsg::Suspend(args) => exec_suspend(ctx, args),
            LifecycleExecuteMsg::Resume(args) => exec_resume(ctx, args),
        },
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: JurorQueryMsg) -> Result<Binary, ContractError> {
    let ctx = ReadonlyContext { deps, env };
    let result = match msg {
        JurorQueryMsg::Config {} => to_json_binary(&query_config(ctx)?),
        JurorQueryMsg::Performance {} => to_json_binary(&query_performance(ctx)?),
        JurorQueryMsg::Qualifies(quals) => to_json_binary(&query_qualifies(ctx, quals)?),
    }?;
    Ok(result)
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

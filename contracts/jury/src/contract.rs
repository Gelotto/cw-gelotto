use crate::error::ContractError;
use crate::execute::evidence::{exec_add_evidence, exec_remove_evidence, exec_vote_evidence};
use crate::execute::lifecycle::{exec_resume, exec_setup, exec_suspend, exec_teardown};
use crate::execute::set_config::exec_set_config;
use crate::execute::vote::exec_vote;
use crate::execute::Context;
use crate::msg::{EvidenceMsg, ExecuteMsg, MigrateMsg, QueryMsg};
use crate::query::{query_config, ReadonlyContext};
use crate::state;
use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use cw_table::lifecycle::LifecycleExecuteMsg;
use gelotto_jury_lib::msg::JuryInstantiateMsg;

const CONTRACT_NAME: &str = "crates.io:cw-jury";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: JuryInstantiateMsg,
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
        ExecuteMsg::Vote(msg) => exec_vote(ctx, msg),
        ExecuteMsg::Follow {} => todo!(),
        ExecuteMsg::Evidence(msg) => match msg {
            EvidenceMsg::Add(articles) => exec_add_evidence(ctx, articles),
            EvidenceMsg::Remove(article_ids) => exec_remove_evidence(ctx, article_ids),
            EvidenceMsg::Vote { article_id, vote } => exec_vote_evidence(ctx, article_id, vote),
        },
        ExecuteMsg::Lifecycle(msg) => match msg {
            LifecycleExecuteMsg::Setup(args) => exec_setup(ctx, args),
            LifecycleExecuteMsg::Teardown(args) => exec_teardown(ctx, args),
            LifecycleExecuteMsg::Suspend(args) => exec_suspend(ctx, args),
            LifecycleExecuteMsg::Resume(args) => exec_resume(ctx, args),
        },
    }
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

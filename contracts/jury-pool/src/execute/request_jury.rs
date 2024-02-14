use crate::{
    error::ContractError,
    state::{
        models::SubMsgReplyJob,
        storage::{JURY_CODE_ID, JURY_ID_COUNTER, REPLY_JOBS, TABLE_ADDR},
    },
};
use cosmwasm_std::{
    attr, to_json_binary, Addr, Coin, DepsMut, Env, Reply, Response, Storage, SubMsg, Uint64,
    WasmMsg,
};
use cw_table::{client::Table, msg::PartitionSelector};
use gelotto_jury_lib::{models::JuryRequest, msg::JuryInstantiateMsg};

use super::{Context, ReplyContext};

pub fn exec_request_jury(ctx: Context, req: JuryRequest) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;
    let instantiate_jury_submsg = instantiate_jury(
        deps.storage,
        &req,
        &info.sender,
        &env.contract.address,
        &info.funds,
    )?;
    Ok(Response::new()
        .add_attributes(vec![attr("action", "request_jury")])
        .add_submessage(instantiate_jury_submsg))
}

fn instantiate_jury(
    store: &mut dyn Storage,
    req: &JuryRequest,
    initiator: &Addr,
    jury_pool_addr: &Addr,
    funds: &Vec<Coin>,
) -> Result<SubMsg, ContractError> {
    // Get Jury contract code ID to instantiate
    let code_id = JURY_CODE_ID.load(store)?;

    // Get next available jury ID int
    let jury_id = JURY_ID_COUNTER.update(store, |id| -> Result<_, ContractError> {
        Ok(id + Uint64::one())
    })?;

    // Build CosmWasm smart contract instantiation label
    let label = format!(
        "Jury {} - {}",
        jury_id,
        req.title.clone().unwrap_or_else(|| "(untitled)".to_owned())
    );

    // Build the InstantiateMsg destined for the new Jury contract
    let instantiate_msg = to_json_binary(&JuryInstantiateMsg {
        id: jury_id,
        initiator: initiator.clone(),
        request: req.clone(),
    })?;

    // Instantiate the Jury through its table contract, setting the jury admin
    // to this jury pool contract addr
    let table = Table::new(&TABLE_ADDR.load(store)?, jury_pool_addr);
    let wasm_msg = table.create(
        code_id,
        instantiate_msg,
        label,
        true,
        PartitionSelector::Id(0),
        Some(jury_pool_addr.clone()),
        None,
        None,
    )?;

    // Stage a new ReplyJob for processing on submsg reply, capturing the new
    // Jury contract address
    REPLY_JOBS.save(
        store,
        jury_id.into(),
        &SubMsgReplyJob::JuryInstantiated {
            jury_id,
            initiator: initiator.clone(),
        },
    )?;

    Ok(SubMsg::reply_always(wasm_msg, jury_id.into()))
}

pub fn save_jury(
    ctx: ReplyContext,
    jury_id: Uint64,
    initiator: Addr,
) -> Result<Response, ContractError> {
    let ReplyContext { deps, .. } = ctx;
    Ok(Response::default())
}

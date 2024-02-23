use crate::{
    error::ContractError,
    state::storage::{CREATED_BY, JUROR_ID, TABLE_ADDR},
};
use cosmwasm_std::{attr, Response};
use cw_table::lifecycle::{LifecycleArgs, LifecycleSetupArgs};

use super::Context;

// pub const TABLE_INDEX_ACTIVITY_SCORE: &str = "activity";
// pub const TABLE_INDEX_RANK: &str = "rank";

pub fn exec_setup(ctx: Context, args: LifecycleSetupArgs) -> Result<Response, ContractError> {
    let resp = Response::new().add_attributes(vec![attr("action", "setup")]);
    let Context { deps, .. } = ctx;

    JUROR_ID.save(deps.storage, &args.id)?;
    TABLE_ADDR.save(deps.storage, &args.table)?;
    CREATED_BY.save(deps.storage, &args.initiator)?;

    // let indices = vec![
    //     KeyValue::Int32(TABLE_INDEX_RANK.into(), Some(meta.rank)),
    //     KeyValue::Uint32(TABLE_INDEX_ACTIVITY_SCORE.into(), Some(0)),
    // ];

    // let relationships_to_add: Vec<Relationship> = vec![Relationship {
    //     address: meta.created_by.clone(),
    //     name: "creator".to_owned(),
    //     unique: false,
    // }];

    // let relationshps = RelationshipUpdates {
    //     remove: None,
    //     add: Some(relationships_to_add),
    // };

    // let tags = prepare_tag_updates(deps.storage, ROOT_ID)?;
    // let table = Table::new(&info.sender, &env.contract.address);

    // Ok(resp.add_message(table.update(
    //     &info.sender,
    //     Some(indices),
    //     Some(tags),
    //     Some(relationshps),
    // )?))

    Ok(resp)
}

pub fn exec_teardown(_ctx: Context, _args: LifecycleArgs) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes(vec![attr("action", "teardown")]))
}

pub fn exec_suspend(_ctx: Context, _args: LifecycleArgs) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes(vec![attr("action", "suspend")]))
}

pub fn exec_resume(_ctx: Context, _args: LifecycleArgs) -> Result<Response, ContractError> {
    Ok(Response::new().add_attributes(vec![attr("action", "resume")]))
}

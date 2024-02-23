pub mod models;
pub mod storage;

use cosmwasm_std::Response;
use gelotto_jury_lib::{
    models::{JuryRequest, JurySettings},
    msg::JuryInstantiateMsg,
};

use crate::{error::ContractError, execute::Context};

use self::storage::{
    JUROR_BOND_REQUIREMENTS, JUROR_QUALIFICATIONS, JURY_ALLOW_APPEALS, JURY_MAX_DURATION,
    JURY_MIN_CONSENSUS_PCT, JURY_MIN_VOTE_COUNT, JURY_TAGS, JURY_TARGET_DURATION, JURY_TASK,
    JURY_TITLE, JURY_VOTING_STARTS_AT,
};

/// Top-level initialization of contract state
pub fn init(ctx: Context, msg: &JuryInstantiateMsg) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let JuryRequest {
        title: maybe_title,
        tags: maybe_tags,
        task,
        settings,
        jurors,
    } = &msg.request;

    if let Some(title) = maybe_title {
        JURY_TITLE.save(deps.storage, &title)?;
    }

    for tag in maybe_tags.clone().unwrap_or_default().iter() {
        JURY_TAGS.save(deps.storage, tag, &true)?;
    }

    JURY_TASK.save(deps.storage, task)?;

    let JurySettings {
        starts_at,
        allow_appeals,
        min_consensus,
        min_vote_count,
        max_duration,
        target_duration,
    } = settings;

    JURY_VOTING_STARTS_AT.save(deps.storage, &starts_at)?;
    JURY_ALLOW_APPEALS.save(deps.storage, &allow_appeals)?;
    JURY_MAX_DURATION.save(deps.storage, &max_duration)?;
    JURY_TARGET_DURATION.save(deps.storage, &target_duration)?;
    JURY_MIN_CONSENSUS_PCT.save(deps.storage, &min_consensus)?;
    JURY_MIN_VOTE_COUNT.save(deps.storage, &min_vote_count)?;
    JURY_TARGET_DURATION.save(deps.storage, &target_duration)?;

    JUROR_QUALIFICATIONS.save(deps.storage, &jurors.qualifications)?;
    JUROR_BOND_REQUIREMENTS.save(deps.storage, &jurors.bond_requirements)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

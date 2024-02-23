use gelotto_jury_lib::query::JurorPerformance;

use crate::{
    error::ContractError,
    state::storage::{
        JUROR_EXPERTISE, JUROR_SCORE_EXPERIENCE, JUROR_SCORE_IDENTITY, JUROR_SCORE_INITIATIVE,
        JUROR_SCORE_PRECISION, JUROR_SCORE_RESEARCH, JUROR_SCORE_SPEED,
    },
};

use super::ReadonlyContext;

pub fn query_performance(ctx: ReadonlyContext) -> Result<JurorPerformance, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(JurorPerformance {
        xp: JUROR_SCORE_EXPERIENCE.load(deps.storage)?,
        identity: JUROR_SCORE_IDENTITY.load(deps.storage)?,
        initiative: JUROR_SCORE_INITIATIVE.load(deps.storage)?,
        precision: JUROR_SCORE_PRECISION.load(deps.storage)?,
        research: JUROR_SCORE_RESEARCH.load(deps.storage)?,
        speed: JUROR_SCORE_SPEED.load(deps.storage)?,
        expertise: JUROR_EXPERTISE
            .iter(deps.storage)?
            .map(|r| r.unwrap())
            .collect(),
    })
}

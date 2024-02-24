use std::collections::HashMap;

use gelotto_jury_lib::{models::JurorQualifications, query::JurorPerformance};

use crate::{
    error::ContractError,
    state::storage::{
        JUROR_EXPERTISE, JUROR_SCORE_EXPERIENCE, JUROR_SCORE_IDENTITY, JUROR_SCORE_INITIATIVE,
        JUROR_SCORE_PRECISION, JUROR_SCORE_RESEARCH, JUROR_SCORE_SPEED,
    },
};

use super::ReadonlyContext;

pub fn query_qualifies(
    ctx: ReadonlyContext,
    qual: JurorQualifications,
) -> Result<bool, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let perf = JurorPerformance {
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
    };
    if let Some(min_exp) = qual.n_juries {
        if perf.xp < min_exp {
            return Ok(false);
        }
    }
    if let Some(min_score) = qual.identity {
        if perf.identity < min_score {
            return Ok(false);
        }
    }
    if let Some(min_score) = qual.speed {
        if perf.speed < min_score {
            return Ok(false);
        }
    }
    if let Some(min_score) = qual.initiative {
        if perf.initiative < min_score {
            return Ok(false);
        }
    }
    if let Some(min_score) = qual.research {
        if perf.research < min_score {
            return Ok(false);
        }
    }
    if let Some(min_score) = qual.precision {
        if perf.precision < min_score {
            return Ok(false);
        }
    }
    // Check domain expertise
    if !qual.expertise.is_empty() {
        let mut expertise_map: HashMap<String, u16> = HashMap::with_capacity(perf.expertise.len());
        for expertise in perf.expertise.iter() {
            let domain = expertise.domain.to_lowercase();
            expertise_map.insert(domain, expertise.score);
        }
        for required in qual.expertise.iter() {
            if let Some(score) = expertise_map.get(&required.domain.to_lowercase()) {
                if *score < required.score {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

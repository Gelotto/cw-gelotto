use crate::{
    error::ContractError,
    msg::JurorVoteMsg,
    state::{
        models::{JurorVoteMetadata, VotingPeriod},
        storage::{
            JUROR_BONDS, JUROR_BOND_REQUIREMENTS, JUROR_QUALIFICATIONS, JUROR_SPEED_SCORES,
            JUROR_VOTES, JUROR_VOTE_METADATA, JUROR_VOTE_PROPOSERS, JUROR_VOTE_RATIONALES,
            JUROR_VOTE_TOTALS, JURY_MIN_CONSENSUS_PCT, JURY_MIN_VOTE_COUNT, JURY_VOTING_PERIOD,
            TOTAL_QUALIFIED_VOTE_COUNT, TOTAL_UNQUALIFIED_VOTE_COUNT, VERDICT,
        },
    },
};
use cosmwasm_std::{
    attr, to_json_binary, Addr, Attribute, BlockInfo, Empty, QuerierWrapper, Response, Storage,
    Uint128,
};
use gelotto_jury_lib::{models::Verdict, query::JurorQueryMsg};

use super::Context;

pub const QUALIFIED: u8 = 1;
pub const NOT_QUALIFIED: u8 = 0;

pub fn exec_vote(ctx: Context, msg: JurorVoteMsg) -> Result<Response, ContractError> {
    let Context { deps, info, env } = ctx;
    let BlockInfo { time, .. } = env.block;
    let JurorVoteMsg {
        id: answer_id,
        rationale: maybe_rationale,
        evidence: _maybe_evidence,
    } = msg;

    if VERDICT.exists(deps.storage) {
        return Err(ContractError::NotAuthorized {
            reason: "Verdict cannot be changed".to_string(),
        });
    }

    let vp: VotingPeriod = JURY_VOTING_PERIOD.load(deps.storage)?;

    // Ensure jury has started

    if time < vp.start {
        return Err(ContractError::NotAuthorized {
            reason: "Voting period not started".to_string(),
        });
    }

    // Ensure voting period hasn't ended
    if time >= vp.stop {
        return Err(ContractError::NotAuthorized {
            reason: "Voting closed".to_string(),
        });
    }

    // Determine if juror qualified for official inclusion in verdict decision
    let has_scores = juror_meets_score_requirements(deps.storage, deps.querier, &info.sender)?;
    let has_bond = juror_meets_bonding_requirements(deps.storage, &info.sender)?;

    // `q` implies that this vote counts towards the official outcome/verdict
    let q = if has_scores && has_bond {
        QUALIFIED
    } else {
        NOT_QUALIFIED
    };

    // Handle case where juror is changing an existing vote, by undoing it
    if let Some(JurorVoteMetadata {
        answer_id: old_answer_id,
        ..
    }) = JUROR_VOTE_METADATA.may_load(deps.storage, &info.sender)?
    {
        // Remove the existing juror vote record
        JUROR_VOTES.remove(deps.storage, (q, &info.sender, &old_answer_id));

        // Remove any existing rationale string
        JUROR_VOTE_RATIONALES.remove(deps.storage, &info.sender);

        // Decrement global tally for the undone vote
        JUROR_VOTE_TOTALS.update(
            deps.storage,
            (q, &old_answer_id),
            |maybe_n| -> Result<_, ContractError> { Ok(maybe_n.unwrap_or_default() - 1) },
        )?;

        // Remove juror as proposer if application
        if let Some(addr) = JUROR_VOTE_PROPOSERS.may_load(deps.storage, &answer_id)? {
            if addr == info.sender {
                JUROR_VOTE_PROPOSERS.remove(deps.storage, &answer_id);
            }
        }
    } else {
        // Only do these things if this is the first vote by the juror, not an update.
        // ------------
        // If this is the first time that a juror has voted this way, save the juror
        // as the vote's "proposer"
        JUROR_VOTE_PROPOSERS.update(
            deps.storage,
            &answer_id,
            |maybe_proposer| -> Result<_, ContractError> {
                Ok(maybe_proposer.unwrap_or_else(|| info.sender.clone()))
            },
        )?;
        // Increment total vote count
        (if q == QUALIFIED {
            TOTAL_QUALIFIED_VOTE_COUNT
        } else {
            TOTAL_UNQUALIFIED_VOTE_COUNT
        })
        .update(deps.storage, |n| -> Result<_, ContractError> { Ok(n + 1) })?;
    }

    // Record juror's new vote
    JUROR_VOTES.save(deps.storage, (q, &info.sender, &answer_id), &true)?;

    // Increment global tally for this answer
    let n_votes = JUROR_VOTE_TOTALS.update(
        deps.storage,
        (q, &answer_id),
        |maybe_n| -> Result<_, ContractError> { Ok(maybe_n.unwrap_or_default() + 1) },
    )?;

    // Sore metadata on the juror's vote
    JUROR_VOTE_METADATA.save(
        deps.storage,
        &info.sender,
        &JurorVoteMetadata {
            answer_id: answer_id.clone(),
            time: env.block.time,
            qualified: q,
        },
    )?;

    // Save rationale explaining why the juror voted this way
    if let Some(rationale) = maybe_rationale {
        JUROR_VOTE_RATIONALES.save(deps.storage, &info.sender, &rationale)?;
    }

    // Update the juror's "response speed" score
    let speed_score = compute_speed_score(&vp, &env.block);
    JUROR_SPEED_SCORES.save(deps.storage, &info.sender, &speed_score)?;

    let mut attrs: Vec<Attribute> = vec![
        attr("action", "vote"),
        attr("qualified", has_scores.to_string()),
        attr("bonded", has_bond.to_string()),
        attr("answer_id", answer_id.to_string()),
    ];

    // If we have a verdict, save it so that the juror's clients can query it
    if has_verdict(deps.storage, n_votes).unwrap_or(false) {
        attrs.push(attr("has_verdict", "true"));
        VERDICT.save(
            deps.storage,
            &Verdict {
                answer_id: answer_id.clone(),
                created_at: env.block.time,
            },
        )?;
    }

    Ok(Response::new().add_attributes(attrs))
}

fn juror_meets_bonding_requirements(
    store: &dyn Storage,
    sender: &Addr,
) -> Result<bool, ContractError> {
    if let Some(bonds) = JUROR_BOND_REQUIREMENTS.may_load(store)? {
        for bond in bonds.iter() {
            let key = bond.get_key();
            if !JUROR_BONDS.has(store, (sender, &key)) {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

fn juror_meets_score_requirements(
    store: &dyn Storage,
    querier: QuerierWrapper<Empty>,
    sender: &Addr,
) -> Result<bool, ContractError> {
    Ok(querier.query_wasm_smart(
        sender,
        &to_json_binary(&JurorQueryMsg::Qualifies(JUROR_QUALIFICATIONS.load(store)?))?,
    )?)
}

fn compute_speed_score(vp: &VotingPeriod, block: &BlockInfo) -> u8 {
    // only set a non-zero score if juror response within target time window
    if block.time < vp.target {
        let max_duration = vp.target.nanos() - vp.start.nanos();
        let delta_t = block.time.nanos() - vp.start.nanos();
        100u8
            - (Uint128::from(100u128)
                .multiply_ratio(delta_t, max_duration)
                .u128()
                % 100) as u8
    } else {
        0
    }
}

fn has_verdict(store: &dyn Storage, n_votes: u32) -> Result<bool, ContractError> {
    let n_total_votes: u32 = TOTAL_QUALIFIED_VOTE_COUNT.load(store)?;

    if n_total_votes < JURY_MIN_VOTE_COUNT.load(store)? {
        return Ok(false);
    }

    // Do we have the minimum percentage required for consensus?
    // Scale the totals for integer arithmetic with percentages
    let pct = (u32::MAX * n_votes) / n_total_votes;
    let min_pct_required = JURY_MIN_CONSENSUS_PCT.load(store)?;
    if pct >= min_pct_required {
        return Ok(true);
    }

    Ok(false)
}

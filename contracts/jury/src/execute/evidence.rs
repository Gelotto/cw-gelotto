use crate::{
    error::ContractError,
    state::storage::{
        EVIDENCE_ARTICLES, EVIDENCE_ARTICLE_ID_COUNTER, EVIDENCE_RANKED_ARTICLES,
        JUROR_EVIDENCE_ARTICLE_IDS, JUROR_EVIDENCE_VOTES,
    },
};
use cosmwasm_std::{attr, Response};
use gelotto_jury_lib::jury::{
    models::{Article, ArticleID},
    msg::ArticleMsg,
};

use super::Context;

pub fn exec_add_evidence(
    ctx: Context,
    articles: Vec<ArticleMsg>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    let mut next_id = EVIDENCE_ARTICLE_ID_COUNTER.load(deps.storage)?;
    for article in articles.iter() {
        EVIDENCE_ARTICLES.save(
            deps.storage,
            next_id,
            &Article {
                owner: info.sender.clone(),
                description: article.description.to_owned(),
                value: article.value.to_owned(),
                rank: 0,
            },
        )?;
        JUROR_EVIDENCE_ARTICLE_IDS.save(deps.storage, (&info.sender, next_id), &true)?;
        EVIDENCE_RANKED_ARTICLES.save(deps.storage, (0, next_id), &true)?;
        next_id += 1;
    }
    EVIDENCE_ARTICLE_ID_COUNTER.save(deps.storage, &next_id)?;
    Ok(Response::new().add_attributes(vec![attr("action", "add_evidence")]))
}

pub fn exec_remove_evidence(
    ctx: Context,
    article_ids: Vec<ArticleID>,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;
    for id in article_ids.iter() {
        let id = *id;
        if let Some(article) = EVIDENCE_ARTICLES.may_load(deps.storage, id)? {
            if article.owner != info.sender {
                return Err(ContractError::NotAuthorized {
                    reason: format!("Sender does not own evidence article {}", id),
                });
            }
            EVIDENCE_ARTICLES.remove(deps.storage, id);
            JUROR_EVIDENCE_ARTICLE_IDS.remove(deps.storage, (&info.sender, id));
            EVIDENCE_RANKED_ARTICLES.remove(deps.storage, (article.rank, id));
        }
    }
    Ok(Response::new().add_attributes(vec![attr("action", "remove_evidence")]))
}

pub fn exec_vote_evidence(
    ctx: Context,
    article_id: ArticleID,
    vote: i8,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    let mut attrs = vec![
        attr("action", "vote_evidence"),
        attr("article_id", article_id.to_string()),
        attr("vote", vote.to_string()),
    ];

    if let Some(mut article) = EVIDENCE_ARTICLES.may_load(deps.storage, article_id)? {
        let old_rank = article.rank;

        // Handle case of juror undoing or changing existing vote
        if let Some(old_vote) =
            JUROR_EVIDENCE_VOTES.may_load(deps.storage, (&info.sender, article_id))?
        {
            EVIDENCE_RANKED_ARTICLES.remove(deps.storage, (article.rank, article_id));
            if vote == old_vote || vote == 0 {
                // remove old vote
                article.rank += if old_vote == 1 { -1 } else { 1 };
                JUROR_EVIDENCE_VOTES.remove(deps.storage, (&info.sender, article_id));
            } else {
                // changing vote from up to down or down to up
                article.rank += if old_vote == 1 { -2 } else { 2 };
                JUROR_EVIDENCE_VOTES.save(
                    deps.storage,
                    (&info.sender, article_id),
                    &(if old_vote == 1 { -1 } else { 1 }),
                )?;
            }
        } else if vote != 0 {
            // Handle case of juror voting evidence article for first time
            article.rank += if vote > 0 { 1 } else { -1 };
            JUROR_EVIDENCE_VOTES.save(
                deps.storage,
                (&info.sender, article_id),
                &(if vote > 0 { 1 } else { -1 }),
            )?;
        } else {
            return Err(ContractError::NotAuthorized {
                reason: "Vote value must be non-zero".to_owned(),
            });
        }

        attrs.push(attr("old_rank", old_rank.to_string()));
        attrs.push(attr("new_rank", article.rank.to_string()));

        // Save article with updated rank
        EVIDENCE_ARTICLES.save(deps.storage, article_id, &article)?;
        EVIDENCE_RANKED_ARTICLES.save(deps.storage, (article.rank, article_id), &true)?;
    } else {
        return Err(ContractError::EvidenceNotFound { article_id });
    }

    Ok(Response::new().add_attributes(attrs))
}

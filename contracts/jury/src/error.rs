use cosmwasm_std::StdError;
use gelotto_jury_lib::models::{ArticleID, Bond};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("NotAuthorized: {reason}")]
    NotAuthorized { reason: String },

    #[error("EvidenceNotFound: Article {article_id:?} not found")]
    EvidenceNotFound { article_id: ArticleID },

    #[error("InvalidBond: {bond:?}")]
    InvalidBond { bond: Bond },

    #[error("JurorNotQualified")]
    JurorNotQualified {},

    #[error("ValidationError: {reason:?}")]
    ValidationError { reason: String },
}

impl From<ContractError> for StdError {
    fn from(err: ContractError) -> Self {
        StdError::generic_err(err.to_string())
    }
}

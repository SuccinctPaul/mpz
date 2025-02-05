use mpz_garble_core::ValueError;

use crate::{value::ValueRef, DecodeError, ExecutionError, LoadError, ProveError, VerifyError};

/// Errors that can occur during the DEAP protocol.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum DEAPError {
    #[error("role error: {0}")]
    RoleError(String),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("context error: {0}")]
    ContextError(#[from] mpz_common::ContextError),
    #[error(transparent)]
    GeneratorError(#[from] crate::generator::GeneratorError),
    #[error(transparent)]
    EvaluatorError(#[from] crate::evaluator::EvaluatorError),
    #[error(transparent)]
    ValueError(#[from] ValueError),
    #[error("value does not exist: {0:?}")]
    ValueDoesNotExist(ValueRef),
    #[error("missing encoding for value: {0:?}")]
    MissingEncoding(ValueRef),
    #[error(transparent)]
    FinalizationError(#[from] FinalizationError),
}

#[derive(Debug, thiserror::Error)]
pub enum FinalizationError {
    #[error("DEAP instance already finalized")]
    AlreadyFinalized,
    #[error("Only main thread can finalize DEAP instance")]
    NotMainThread,
    #[error(transparent)]
    CommitmentError(#[from] mpz_core::commit::CommitmentError),
    #[error("invalid encoder seed")]
    InvalidEncoderSeed,
    #[error("invalid equality check")]
    InvalidEqualityCheck,
    #[error("invalid proof")]
    InvalidProof,
}

/// Errors that can occur when accessing peer's encodings.
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum PeerEncodingsError {
    #[error("Encodings not available since DEAP instance already finalized")]
    AlreadyFinalized,
    #[error("Value id was not found in memory: {0:?}")]
    ValueIdNotFound(String),
    #[error("Encoding is not available for value: {0:?}")]
    EncodingNotAvailable(ValueRef),
}

impl From<DEAPError> for LoadError {
    fn from(err: DEAPError) -> Self {
        match err {
            DEAPError::IOError(err) => LoadError::IOError(err),
            err => LoadError::ProtocolError(Box::new(err)),
        }
    }
}

impl From<DEAPError> for ExecutionError {
    fn from(err: DEAPError) -> Self {
        match err {
            DEAPError::IOError(err) => ExecutionError::IOError(err),
            err => ExecutionError::ProtocolError(Box::new(err)),
        }
    }
}

impl From<DEAPError> for ProveError {
    fn from(err: DEAPError) -> Self {
        match err {
            DEAPError::IOError(err) => ProveError::IOError(err),
            err => ProveError::ProtocolError(Box::new(err)),
        }
    }
}

impl From<DEAPError> for VerifyError {
    fn from(err: DEAPError) -> Self {
        match err {
            DEAPError::IOError(err) => VerifyError::IOError(err),
            err => VerifyError::ProtocolError(Box::new(err)),
        }
    }
}

impl From<DEAPError> for DecodeError {
    fn from(err: DEAPError) -> Self {
        match err {
            DEAPError::IOError(err) => DecodeError::IOError(err),
            err => DecodeError::ProtocolError(Box::new(err)),
        }
    }
}

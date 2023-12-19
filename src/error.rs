use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid funds sent")]
    InvalidFunds {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Invalid initial funds provided")]
    InvalidInitialFunds {},

    #[error("Invalid deposit ratio")]
    InvalidDepositRatio {},

    #[error("Withdrawing more than your current balance")]
    WithdrawError {},

    #[error("Deposit too small")]
    DepositTooSmall {},

    #[error("Overflow Error")]
    Overflow(OverflowError),
}

impl From<OverflowError> for ContractError {
    fn from(err: OverflowError) -> ContractError {
        ContractError::Overflow(err)
    }
}
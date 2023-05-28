use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid subdenom: {subdenom:?}")]
    InvalidSubdenom { subdenom: String },

    #[error("Invalid denom: {denom:?} {message:?}")]
    InvalidDenom { denom: String, message: String },

    #[error("denom does not exist: {denom:?}")]
    DenomDoesNotExist { denom: String },

    #[error("Not enough {denom:?} ({funds:?}) in funds. {needed:?} {denom:?} needed")]
    NotEnoughFunds {
        denom: String,
        funds: u128,
        needed: u128,
    },

    #[error("Not enough {action} allowance: attempted to {action} {amount}, but remaining allowance is {allowance}")]
    NotEnoughAllowance {
        action: String,
        amount: Uint128,
        allowance: Uint128,
    },

    #[error("address is not supported yet, was: {address:?}")]
    BurnFromAddressNotSupported { address: String },

    #[error("amount was zero, must be positive")]
    ZeroAmount {},

    #[error("The address '{address}' is blacklisted")]
    Blacklisted { address: String },

    #[error("The contract is frozen for denom {denom:?}")]
    ContractFrozen { denom: String },
}

impl ContractError {
    pub fn not_enough_mint_allowance(
        amount: impl Into<Uint128>,
        allowance: impl Into<Uint128>,
    ) -> ContractError {
        ContractError::NotEnoughAllowance {
            action: "mint".to_string(),
            amount: amount.into(),
            allowance: allowance.into(),
        }
    }

    pub fn not_enough_burn_allowance(
        amount: impl Into<Uint128>,
        allowance: impl Into<Uint128>,
    ) -> ContractError {
        ContractError::NotEnoughAllowance {
            action: "burn".to_string(),
            amount: amount.into(),
            allowance: allowance.into(),
        }
    }
}

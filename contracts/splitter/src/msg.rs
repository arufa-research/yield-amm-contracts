use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Addr, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub red_bank: Addr,
    pub mars_adapter: Addr,
    pub epoch_period: u64,    // length of each epoch in seconds
    pub expiry_period: u64,   // length of time till maturity
    pub underlying_denom: String,
    pub yield_bearing_denom: String,
    pub principle_denom: String,
    pub yield_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Deposit ybToken to get pToken and yToken
    Deposit {},
    // Deposit pToken and yToken to withdraw ybToken
    Withdraw {},

    // Advance the epoch
    // Calculate the yield since last epoch
    // and send it over to rewards contract
    Advance {},

    UpdateRewardsContract {
        rewards_contract: Addr,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // UserDeposit returns the amount deposited by user as a json-encoded number
    #[returns(UserDepositResponse)]
    UserDeposit {},
    // TotalDeposit returns the total amount deposited as a json-encoded number
    #[returns(TotalDepositResponse)]
    TotalDeposit {},
    // Config returns the contract config as a json-encoded number
    #[returns(ConfigResponse)]
    Config {},
    // State returns the contract state as a json-encoded number
    #[returns(StateResponse)]
    State {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct UserDepositResponse {
    pub yb_deposited: Uint128,
}

#[cw_serde]
pub struct TotalDepositResponse {
    pub yb_deposited: Uint128,
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub red_bank: Addr,
    pub mars_adapter: Addr,
    pub underlying_denom: String,
    pub yield_bearing_denom: String,
    pub principle_denom: String,
    pub yield_denom: String,
    pub expiry_period: u64,
    pub epoch_period: u64,
    pub rewards_contract: Addr,
}

#[cw_serde]
pub struct StateResponse {
    pub yb_deposited: Uint128,
    pub p_issued: Uint128,
    pub y_issued: Uint128,
    pub exchange_rate: Decimal,
}

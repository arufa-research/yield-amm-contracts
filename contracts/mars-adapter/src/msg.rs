use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Addr, Decimal};
// use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub red_bank: Addr,
    pub underlying_denom: String,
    pub yield_bearing_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {},
    UpdateYieldBearingDenom {
        yield_bearing_denom: String,
    },
    Withdraw {},
}

// used by receive cw20
#[cw_serde]
pub enum Cw20HookMsg {
    Withdraw {},
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
    pub underlying_amount: Uint128,
}

#[cw_serde]
pub struct TotalDepositResponse {
    pub underlying_amount: Uint128,
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub red_bank: Addr,
    pub underlying_denom: String,
    pub yield_bearing_denom: String,
}

#[cw_serde]
pub struct StateResponse {
    pub underlying_deposited: Uint128,
    pub exchange_rate: Decimal,
}

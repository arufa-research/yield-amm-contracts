use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Addr, Decimal};
use cw20::Cw20ReceiveMsg;

#[cw_serde]
pub struct InstantiateMsg {
    pub expiry_time: Uint128,
    pub underlying_asset: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Deposit pToken and yToken to withdraw ybToken
    Withdraw {},

    UpdateYieldBearingToken {
        yield_bearing_token: Addr,
    },
    UpdatePrincipleToken {
        principle_token: Addr,
    },
    UpdateYieldToken {
        yield_token: Addr,
    },

    // Cw20 token interaction
    Receive(Cw20ReceiveMsg),
}

// used by receive cw20
#[cw_serde]
pub enum Cw20HookMsg {
    // Deposit ybToken to get pToken and yToken
    Deposit {},
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
    pub yield_bearing_token: Addr,
    pub principle_token: Addr,
    pub yield_token: Addr,
    pub expiry_time: Uint128,
    pub underlying_asset: String,
}

#[cw_serde]
pub struct StateResponse {
    pub yb_deposited: Uint128,
    pub p_issued: Uint128,
    pub y_issued: Uint128,
    pub exchange_rate: Decimal,
}
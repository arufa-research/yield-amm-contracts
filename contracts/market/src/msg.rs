use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Uint128, Addr};
// use cosmwasm_std::{Uint128, Addr, Decimal};

#[cw_serde]
pub struct InstantiateMsg {
    pub red_bank: Addr,
    pub mars_adapter: Addr,
    pub splitter: Addr,
    pub underlying_denom: String,
    pub yield_bearing_denom: String,
    pub principle_denom: String,
    pub yield_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    SwapYbToY {},
    SwapYbToP {},
    SwapYToYb {},
    SwapPToYb {},
    SwapPtoY {},
    SwapYtoP {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Config returns the contract config as a json-encoded number
    #[returns(ConfigResponse)]
    Config {},
    // State returns the contract state as a json-encoded number
    #[returns(StateResponse)]
    State {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Addr,
    pub red_bank: Addr,
    pub mars_adapter: Addr,
    pub splitter: Addr,
    pub underlying_denom: String,
    pub yield_bearing_denom: String,
    pub principle_denom: String,
    pub yield_denom: String,
}

#[cw_serde]
pub struct StateResponse {
    pub yb_in_pool: Uint128,
    pub p_in_pool: Uint128,
    pub scaling_factor: u64,
}

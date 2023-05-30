use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
// use cosmwasm_std::{Addr, Uint128, Decimal};
use cw_storage_plus::Item;
// use mars_utils::{
//     error::ValidationError,
//     helpers::{decimal_param_le_one, decimal_param_lt_one},
// };

#[cw_serde]
pub struct Config {
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
pub struct State {
    pub yb_in_pool: Uint128,
    pub p_in_pool: Uint128,
    pub scaling_factor: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
// pub const OSMO_BALANCES: Map<&Addr, Uint128> = Map::new("osmo_balance");

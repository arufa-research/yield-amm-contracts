use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Decimal};
use cw_storage_plus::Item;
// use mars_utils::{
//     error::ValidationError,
//     helpers::{decimal_param_le_one, decimal_param_lt_one},
// };

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub red_bank: Addr,
}

#[cw_serde]
pub struct State {
    pub osmo_deposited: Uint128,
    pub exchange_rate: Decimal,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");

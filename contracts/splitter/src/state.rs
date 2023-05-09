use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Decimal};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub red_bank: Addr,
    pub yield_bearing_token: Option<Addr>,
    pub principle_token: Option<Addr>,
    pub yield_token: Option<Addr>,
    pub expiry_time: Uint128,
    pub underlying_asset: String,
}

#[cw_serde]
pub struct State {
    pub yb_deposited: Uint128,
    pub p_issued: Uint128,
    pub y_issued: Uint128,
    pub exchange_rate: Decimal,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
// pub const OSMO_BALANCES: Map<&Addr, Uint128> = Map::new("osmo_balance");

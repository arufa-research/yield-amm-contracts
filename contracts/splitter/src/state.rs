use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128, Decimal};
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub red_bank: Addr,
    pub mars_adapter: Addr,
    pub underlying_denom: String,
    pub yield_bearing_denom: String,
    pub principle_denom: String,
    pub yield_denom: String,
    pub epoch_period: u64,
    pub expiry_period: u64,
    pub rewards_contract: Option<Addr>,
}

#[cw_serde]
pub struct State {
    pub yb_deposited: Uint128,
    pub p_issued: Uint128,
    pub y_issued: Uint128,
    pub exchange_rate: Decimal,
    pub prev_exchange_rate: Decimal,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
// pub const OSMO_BALANCES: Map<&Addr, Uint128> = Map::new("osmo_balance");

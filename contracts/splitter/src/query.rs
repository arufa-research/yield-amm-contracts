use cosmwasm_std::{
    Deps, StdResult,
};

use crate::msg::{UserDepositResponse, TotalDepositResponse, ConfigResponse, StateResponse};
use crate::state::{STATE, CONFIG, State, Config};

// TODO: query the ybToken balance of user
// and multiply with exchangeRate
pub fn query_user_deposit(
    deps: Deps,
) -> StdResult<UserDepositResponse> {
    let state: State = STATE.load(deps.storage)?;
    Ok(UserDepositResponse { yb_deposited: state.yb_deposited })
}

pub fn query_total_deposit(
    deps: Deps,
) -> StdResult<TotalDepositResponse> {
    let state: State = STATE.load(deps.storage)?;
    Ok(TotalDepositResponse { yb_deposited: state.yb_deposited })
}

pub fn query_config(
    deps: Deps,
) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        red_bank: config.red_bank,
        yield_bearing_token: config.yield_bearing_token.unwrap(),
        principle_token: config.principle_token.unwrap(),
        yield_token: config.yield_token.unwrap(),
        expiry_time: config.expiry_time,
        underlying_asset: config.underlying_asset,
    })
}

pub fn query_state(
    deps: Deps,
) -> StdResult<StateResponse> {
    let state: State = STATE.load(deps.storage)?;
    Ok(StateResponse {
        yb_deposited: state.yb_deposited,
        p_issued: state.p_issued,
        y_issued: state.y_issued,
        exchange_rate: state.exchange_rate,
    })
}

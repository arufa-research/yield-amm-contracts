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
        mars_adapter: config.mars_adapter,
        underlying_denom: config.underlying_denom,
        yield_bearing_denom: config.yield_bearing_denom,
        principle_denom: config.principle_denom,
        yield_denom: config.yield_denom,
        expiry_period: config.expiry_period,
        epoch_period: config.epoch_period,
        rewards_contract: config.rewards_contract.unwrap(),
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

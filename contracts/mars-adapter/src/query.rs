use cosmwasm_std::{
    Deps, StdResult,
};

use crate::msg::{UserDepositResponse, TotalDepositResponse, ConfigResponse, StateResponse};
use crate::state::{STATE, CONFIG, State, Config};

pub fn query_user_deposit(
    deps: Deps,
) -> StdResult<UserDepositResponse> {
    let state: State = STATE.load(deps.storage)?;
    Ok(UserDepositResponse { osmo_amount: state.osmo_deposited })
}

pub fn query_total_deposit(
    deps: Deps,
) -> StdResult<TotalDepositResponse> {
    let state: State = STATE.load(deps.storage)?;
    Ok(TotalDepositResponse { osmo_amount: state.osmo_deposited })
}

pub fn query_config(
    deps: Deps,
) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        red_bank: config.red_bank,
    })
}

pub fn query_state(
    deps: Deps,
) -> StdResult<StateResponse> {
    let state: State = STATE.load(deps.storage)?;
    Ok(StateResponse {
        osmo_deposited: state.osmo_deposited,
        exchange_rate: state.exchange_rate,
    })
}

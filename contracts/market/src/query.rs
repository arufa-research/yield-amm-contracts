use cosmwasm_std::{
    Deps, StdResult,
};

use crate::msg::{ConfigResponse, StateResponse};
use crate::state::{STATE, CONFIG, State, Config};

pub fn query_config(
    deps: Deps,
) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        red_bank: config.red_bank,
        mars_adapter: config.mars_adapter,
        splitter: config.splitter,
        underlying_denom: config.underlying_denom,
        yield_bearing_denom: config.yield_bearing_denom,
        principle_denom: config.principle_denom,
        yield_denom: config.yield_denom,
    })
}

pub fn query_state(
    deps: Deps,
) -> StdResult<StateResponse> {
    let state: State = STATE.load(deps.storage)?;
    Ok(StateResponse {
        yb_in_pool: state.yb_in_pool,
        p_in_pool: state.p_in_pool,
        scaling_factor: state.scaling_factor,
    })
}

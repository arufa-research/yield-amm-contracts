use cosmwasm_std::{
    DepsMut,
    MessageInfo, Response, Uint128,
};

use crate::error::ContractError;
// use crate::state::{Config, State, set_config, set_state};

pub fn try_deposit(
    deps: DepsMut,
) -> Result<Response, ContractError> {

    // set_config(deps.storage).update(|mut state| -> Result<_, ContractError> {
    //     state.count += 1;
    //     Ok(state)
    // })?;

    deps.api.debug("osmo deposited successfully");
    Ok(Response::default())
}

pub fn try_withdraw(
    deps: DepsMut,
    _info: MessageInfo,
    _amount: Uint128,
) -> Result<Response, ContractError> {

    // set_config(deps.storage).update(|mut state| -> Result<_, ContractError> {
    //     if info.sender != state.owner {
    //         return Err(ContractError::Unauthorized {});
    //     }
    //     state.count = count;
    //     Ok(state)
    // })?;

    deps.api.debug("osmo withdrawn successfully");
    Ok(Response::default())
}

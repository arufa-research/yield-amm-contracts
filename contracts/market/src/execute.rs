use cosmwasm_std::{
    DepsMut, MessageInfo, Response, Uint128, CosmosMsg, Env,
};

// use cosmwasm_std::{
//     DepsMut, MessageInfo, Response, Uint128, CosmosMsg, Decimal, Coin,
//     WasmMsg, StdError, to_binary, WasmQuery, QueryRequest, Env, BankMsg, coins,
// };
// use osmosis_std::types::osmosis::tokenfactory::v1beta1::{
//     MsgMint, MsgBurn
// };

use crate::error::ContractError;
use crate::state::{CONFIG, Config, STATE, State};

pub fn try_swap_yb_to_y(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let contract_addr = env.contract.address.to_string();

    let config: Config = CONFIG.load(deps.storage)?;
    let mut state: State = STATE.load(deps.storage)?;

    let yield_bearing_denom = config.yield_bearing_denom.clone();

    // read amount of underlying_denom (OSMO) sent by user on deposit
    let mut amount_raw: Uint128 = Uint128::default();
    for coin in &info.funds {
        if coin.denom == config.underlying_denom {
            amount_raw = coin.amount
        }
    }

    deps.api.debug("osmo deposited successfully");
    Ok(
        Response::new()
        .add_messages(messages)
        .add_attribute("underlying_deposited", amount_raw.to_string())
    )
}

pub fn try_swap_yb_to_p(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let contract_addr = env.contract.address.to_string();

    deps.api.debug("osmo deposited successfully");
    Ok(Response::default())
}

pub fn try_swap_y_to_yb(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let contract_addr = env.contract.address.to_string();

    deps.api.debug("osmo deposited successfully");
    Ok(Response::default())
}

pub fn try_swap_p_to_yb(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let contract_addr = env.contract.address.to_string();

    deps.api.debug("osmo deposited successfully");
    Ok(Response::default())
}

pub fn try_swap_p_to_y(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let contract_addr = env.contract.address.to_string();

    deps.api.debug("osmo deposited successfully");
    Ok(Response::default())
}

pub fn try_swap_y_to_p(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let contract_addr = env.contract.address.to_string();

    deps.api.debug("osmo deposited successfully");
    Ok(Response::default())
}

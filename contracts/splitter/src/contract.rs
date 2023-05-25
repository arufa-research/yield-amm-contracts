use cosmwasm_std::{
    entry_point, to_binary, Binary, Env, Deps, DepsMut,
    MessageInfo, Response, StdError, StdResult, Uint128, WasmQuery, QueryRequest,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::execute::{
    try_update_yield_bearing_token, try_receive_cw20, try_withdraw,
    try_update_principle_token, try_update_yield_token
};
use crate::query::{query_user_deposit, query_total_deposit, query_config, query_state};
use crate::red_bank::{RedBankQueryMsg, MarketResponse};
use crate::state::{Config, State, CONFIG, STATE};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config = Config {
        owner: info.sender.clone(),
        red_bank: msg.red_bank.clone(),
        yield_bearing_token: None,
        principle_token: None,
        yield_token: None,
        expiry_time: msg.expiry_time.clone(),
        underlying_asset: msg.underlying_asset.clone(),
    };

    let market_msg = RedBankQueryMsg::Market {
        denom: msg.denom.clone(),
    };
    let market_query = WasmQuery::Smart {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&market_msg)?,
    };
    let market_data: MarketResponse = deps.querier.query(&QueryRequest::Wasm(
        market_query
    ))?;

    let state = State {
        yb_deposited: Uint128::from(0u128),
        p_issued: Uint128::from(0u128),
        y_issued: Uint128::from(0u128),
        exchange_rate:market_data.liquidity_index,
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    deps.api.debug(&format!("Contract was initialized by {}", info.sender));

    Ok(Response::default()
        .add_attribute("action", "init")
        .add_attribute("sender", info.sender.clone())
        .add_attribute("red_bank address", config.red_bank.clone())
        .add_attribute("underlying_asset", config.underlying_asset.clone())
    )
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Withdraw {} => try_withdraw(deps, info),

        ExecuteMsg::UpdateYieldBearingToken { yield_bearing_token } => 
            try_update_yield_bearing_token(deps, info, yield_bearing_token),
        ExecuteMsg::UpdatePrincipleToken { principle_token } => 
            try_update_principle_token(deps, info, principle_token),
        ExecuteMsg::UpdateYieldToken { yield_token } => 
            try_update_yield_token(deps, info, yield_token),

        ExecuteMsg::Receive(_msg) => try_receive_cw20(deps, env, info, _msg),
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserDeposit {} => to_binary(&query_user_deposit(deps)?),
        QueryMsg::TotalDeposit {} => to_binary(&query_total_deposit(deps)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
    }
}

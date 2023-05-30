use cosmwasm_std::{
    entry_point, to_binary, Binary, Env, Deps, DepsMut, QueryRequest,
    MessageInfo, Response, StdError, StdResult, CosmosMsg, WasmQuery, Uint128
};

use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

use crate::error::ContractError;
use crate::state::{Config, State, CONFIG, STATE};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::red_bank::{RedBankQueryMsg, MarketResponse};
use crate::execute::{try_deposit, try_update_yield_bearing_denom, try_withdraw};
use crate::query::{query_user_deposit, query_total_deposit, query_config, query_state};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config = Config {
        owner: info.sender.clone(),
        red_bank: msg.red_bank.clone(),
        underlying_denom: msg.underlying_denom.clone(),
        yield_bearing_denom: msg.yield_bearing_denom.clone(),
    };

    // create yield_bearing_denom
    // factory/contract_addr/yield_bearing_denom
    let msg_create_denom: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: config.yield_bearing_denom.clone(),
    }.into();
    let messages = vec![msg_create_denom];

    let market_msg = RedBankQueryMsg::Market {
        denom: config.underlying_denom.clone(),
    };
    let market_query = WasmQuery::Smart {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&market_msg)?,
    };
    let market_data: MarketResponse = deps.querier.query(&QueryRequest::Wasm(
        market_query
    ))?;

    let state = State {
        underlying_deposited: Uint128::from(0u128),
        exchange_rate: market_data.liquidity_index,
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    deps.api.debug(&format!("Contract was initialized by {}", info.sender));

    Ok(
        Response::default()
        .add_messages(messages)
        .add_attribute("action", "init")
        .add_attribute("sender", info.sender.clone())
        .add_attribute("underlying_denom", config.underlying_denom.clone())
        .add_attribute("yield_bearing_denom", config.yield_bearing_denom.clone())
        .add_attribute("red_bank", config.red_bank.clone())
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
        ExecuteMsg::Deposit {} => try_deposit(deps, env, info),
        ExecuteMsg::UpdateYieldBearingDenom { yield_bearing_denom } =>
            try_update_yield_bearing_denom(deps, info, yield_bearing_denom),

        ExecuteMsg::Withdraw {} => try_withdraw(deps, env, info),
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

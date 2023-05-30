use cosmwasm_std::{
    entry_point, to_binary, Binary, Env, Deps, DepsMut,
    MessageInfo, Response, StdError, StdResult, Uint128, WasmQuery, QueryRequest, CosmosMsg,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

use crate::error::ContractError;
use crate::state::{Config, State, CONFIG, STATE};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::red_bank::{RedBankQueryMsg, MarketResponse};
use crate::query::{query_user_deposit, query_total_deposit, query_config, query_state};
use crate::execute::{try_update_rewards_contract, try_withdraw, try_deposit, try_advance};

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
        mars_adapter: msg.mars_adapter.clone(),
        expiry_period: msg.expiry_period.clone(),
        epoch_period: msg.epoch_period.clone(),
        underlying_denom: msg.underlying_denom.clone(),
        yield_bearing_denom: msg.yield_bearing_denom.clone(),
        principle_denom: msg.principle_denom.clone(),
        yield_denom: msg.yield_denom.clone(),
        rewards_contract: None,
    };

    let mut messages: Vec<CosmosMsg> = vec![];

    // create principle_denom coin
    // factory/contract_addr/principle_denom
    let msg_create_denom_principle: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: config.principle_denom.clone(),
    }.into();
    messages.push(msg_create_denom_principle);

    // create yield_denom coin
    // factory/contract_addr/yield_denom
    let msg_create_denom_yield: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: config.yield_denom.clone(),
    }.into();
    messages.push(msg_create_denom_yield);

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
        yb_deposited: Uint128::from(0u128),
        p_issued: Uint128::from(0u128),
        y_issued: Uint128::from(0u128),
        exchange_rate: market_data.liquidity_index,
        prev_exchange_rate: market_data.liquidity_index, // same initially
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    deps.api.debug(&format!("Contract was initialized by {}", info.sender));

    Ok(
        Response::default()
        .add_messages(messages)
        .add_attribute("action", "init")
        .add_attribute("sender", info.sender.clone())
        .add_attribute("red_bank", config.red_bank.clone())
        .add_attribute("expiry_period", config.expiry_period.to_string())
        .add_attribute("epoch_period", config.epoch_period.to_string())
        .add_attribute("underlying_denom", config.underlying_denom.clone())
        .add_attribute("yield_bearing_denom", config.yield_bearing_denom.clone())
        .add_attribute("principle_denom", config.principle_denom.clone())
        .add_attribute("yield_denom", config.yield_denom.clone())
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
        ExecuteMsg::Withdraw {} => try_withdraw(deps, env, info),

        ExecuteMsg::Advance {} => try_advance(deps, env, info),

        ExecuteMsg::UpdateRewardsContract { rewards_contract } => 
            try_update_rewards_contract(deps, info, rewards_contract),
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

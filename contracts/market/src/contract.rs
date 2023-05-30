use cosmwasm_std::{
    entry_point, to_binary, Binary, Env, Deps, DepsMut,
    MessageInfo, Response, StdError, StdResult, CosmosMsg, Uint128
};

// use osmo_bindings::OsmosisQuery;
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap::v1beta1::{
    MsgCreateStableswapPool
};
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap::v1beta1::{PoolParams};

use crate::error::ContractError;
use crate::state::{Config, State, CONFIG, STATE};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::execute::{
    try_swap_yb_to_y, try_swap_yb_to_p, try_swap_y_to_yb,
    try_swap_p_to_yb, try_swap_p_to_y, try_swap_y_to_p
};
use crate::query::{query_config, query_state};

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
        splitter: msg.splitter.clone(),
        underlying_denom: msg.underlying_denom.clone(),
        yield_bearing_denom: msg.yield_bearing_denom.clone(),
        principle_denom: msg.principle_denom.clone(),
        yield_denom: msg.yield_denom.clone(),
    };

    let yield_bearing_denom = config.yield_bearing_denom.clone();
    let principle_denom = config.principle_denom.clone();

    let mars_adapter = config.mars_adapter.clone();
    let splitter = config.splitter.clone();

    // create yb_token/p_token swap pair
    // stableSwap with an initial scaling factor
    // factory/contract_addr/yield_bearing_denom
    let msg_create_pool: CosmosMsg = MsgCreateStableswapPool {
        sender: env.contract.address.to_string(),
        pool_params: Some(PoolParams {
            swap_fee: "1".into(),
            exit_fee: "0".into(),
        }),
        initial_pool_liquidity: vec![
            Coin {
                denom: format!("factory/{mars_adapter}/{yield_bearing_denom}"),
                amount: "1000000".into(),   // 1 ybToken
            }.into(),
            Coin {
                denom: format!("factory/{splitter}/{principle_denom}"),
                amount: "1000000".into(),   // 1 pToken
            }.into(),
        ],
        future_pool_governor: env.contract.address.to_string(),
        scaling_factors: vec![],
        scaling_factor_controller: env.contract.address.to_string(), // market contract will update the scaling factor
    }.into();
    let messages = vec![msg_create_pool];

    let state = State {
        yb_in_pool: Uint128::from(0u128),
        p_in_pool: Uint128::from(0u128),
        scaling_factor: 1,
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
        .add_attribute("mars_adapter", config.mars_adapter.clone())
        .add_attribute("splitter", config.splitter.clone())
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
        ExecuteMsg::SwapYbToY {} => try_swap_yb_to_y(deps, env, info),
        ExecuteMsg::SwapYbToP {} => try_swap_yb_to_p(deps, env, info),
        ExecuteMsg::SwapYToYb {} => try_swap_y_to_yb(deps, env, info),
        ExecuteMsg::SwapPToYb {} => try_swap_p_to_yb(deps, env, info),
        ExecuteMsg::SwapPtoY {} => try_swap_p_to_y(deps, env, info),
        ExecuteMsg::SwapYtoP {} => try_swap_y_to_p(deps, env, info),
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
    }
}

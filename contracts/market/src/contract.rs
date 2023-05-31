use cosmwasm_std::{
    entry_point, to_binary, Binary, Env, Deps, DepsMut, Reply,
    MessageInfo, Response, StdError, StdResult, Uint128, SubMsgResult, SubMsgResponse, CosmosMsg, SubMsg
};

// use osmo_bindings::OsmosisQuery;
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap::v1beta1::{
    MsgCreateStableswapPool, MsgCreateStableswapPoolResponse
};
use osmosis_std::types::osmosis::gamm::poolmodels::stableswap::v1beta1::PoolParams;

use crate::error::ContractError;
use crate::state::{Config, State, CONFIG, STATE};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::execute::{
    try_swap_yb_to_y, try_swap_yb_to_p, try_swap_y_to_yb,
    try_swap_p_to_yb, try_swap_p_to_y, try_swap_y_to_p,
};
use crate::query::{query_config, query_state};

const CREATE_POOL_REPLY_ID: u64 = 1;

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
            swap_fee: "1000".into(),
            exit_fee: "0".into(),
        }),
        initial_pool_liquidity: vec![
            Coin {
                denom: format!("factory/{splitter}/{principle_denom}"),
                amount: "100000".into(),   // 0.1 pToken
            }.into(),
            Coin {
                denom: format!("factory/{mars_adapter}/{yield_bearing_denom}"),
                amount: "100000".into(),   // 0.1 ybToken
            }.into(),
        ],
        future_pool_governor: env.contract.address.to_string(),
        scaling_factors: vec![100000, 100000],
        scaling_factor_controller: env.contract.address.to_string(), // market contract will update the scaling factor
    }.into();
    let messages = vec![SubMsg::reply_on_success(
        msg_create_pool,
        CREATE_POOL_REPLY_ID,
    )];

    let state = State {
        yb_in_pool: Uint128::from(0u128),
        p_in_pool: Uint128::from(0u128),
        scaling_factor: 1,  // ratio
        pool_id: 0,
    };

    CONFIG.save(deps.storage, &config)?;
    STATE.save(deps.storage, &state)?;

    deps.api.debug(&format!("Contract was initialized by {}", info.sender));

    Ok(
        Response::default()
        .add_submessages(messages)
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut,
    _env: Env,
    msg: Reply,
) -> Result<Response, ContractError> {
    if msg.id == CREATE_POOL_REPLY_ID {
        if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
            // This is only for response deserialization demonstration purpose.
            // `pool_id` can actually be retrieved from `pool_created` event.
            let res: MsgCreateStableswapPoolResponse = b.try_into().map_err(ContractError::Std)?;

            let mut state: State = STATE.load(deps.storage)?;
            state.pool_id = res.pool_id;
            STATE.save(deps.storage, &state)?;

            return Ok(Response::new().add_attribute("pool_id", format!("{}", res.pool_id)));
        }
    };

    Ok(Response::new())
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

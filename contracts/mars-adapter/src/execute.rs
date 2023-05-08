use cosmwasm_std::{
    DepsMut,
    MessageInfo, Response, Uint128, Addr, CosmosMsg, WasmMsg, StdError, to_binary, WasmQuery, QueryRequest, Decimal, Coin,
};
use cw20::Cw20ExecuteMsg;

use crate::error::ContractError;
use crate::state::{CONFIG, Config, STATE, State};
use crate::red_bank::{RedBankQueryMsg, MarketResponse, RedBankExecuteMsg};

pub fn try_deposit(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];

    // read amount of OSMO sent by user on deposit
    let mut amount_raw: Uint128 = Uint128::default();
    for coin in &info.funds {
        if coin.denom == "uosmo" {
            amount_raw = coin.amount
        }
    }

    let config: Config = CONFIG.load(deps.storage)?;
    let yield_bearing_token = config.yield_bearing_token.ok_or_else(|| {
        ContractError::Std(StdError::generic_err(
            "yield bearing token addr not registered".to_string(),
        ))
    })?.to_string();

    let mut state: State = STATE.load(deps.storage)?;
    
    let market_msg = RedBankQueryMsg::Market {
        denom: "uosmo".into(), // TODO: pick this from initMsg
    };
    let market_query = WasmQuery::Smart {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&market_msg)?,
    };
    let market_data: MarketResponse = deps.querier.query(&QueryRequest::Wasm(
        market_query
    ))?;

    // Update state (osmo_deposited, exchange_rate) first
    // To check: should this be updated after deposit has been made?
    state.osmo_deposited += amount_raw;
    state.exchange_rate = market_data.liquidity_index;
    STATE.save(deps.storage, &state)?;

    // Deposit user's OSMO into red bank
    let osmo_bank_deposit_msg = RedBankExecuteMsg::Deposit {
        on_behalf_of: None,
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&osmo_bank_deposit_msg)?,
        funds: vec![Coin {
            denom: "uosmo".to_string(),
            amount: amount_raw,
        }],
    }));

    // Calculate OSMOmars to mint = osmo_amount/exchange_rate
    let yeild_bearing_amount = 
        Decimal::from_ratio(amount_raw, Uint128::from(1u128))
            .checked_div(state.exchange_rate).unwrap().to_uint_floor();

    // Mint OSMOmars cw20 token to user
    let mint_msg = Cw20ExecuteMsg::Mint {
        recipient: info.sender.to_string(),
        amount: yeild_bearing_amount.into()
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: yield_bearing_token,
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    }));

    deps.api.debug("osmo deposited successfully");
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("osmo_deposited", amount_raw.to_string())
        .add_attribute("yield_bearing_minted", yeild_bearing_amount.to_string())
        .add_attribute("total_osmo_deposited", state.osmo_deposited.to_string())
        .add_attribute("exchange_rate", state.exchange_rate.to_string())
    )
}

pub fn try_withdraw(
    deps: DepsMut,
    _info: MessageInfo,
    _amount: Uint128,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];

    // TODO: Add Cw20 receive hook

    // Burn the ybToken CW20 token sent by user
    // Withdraw the osmo amount from red bank with user as receiver

    // Update state (osmo_deposited, exchange_rate)

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

pub fn try_update_yield_bearing_token(
    deps: DepsMut,
    _info: MessageInfo,
    yield_bearing_token: Addr,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    config.yield_bearing_token = Some(yield_bearing_token);

    CONFIG.save(deps.storage, &config)?;

    deps.api.debug("yield bearing token address updated successfully");
    Ok(Response::default())
}

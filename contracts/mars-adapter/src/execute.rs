use cosmwasm_std::{
    DepsMut, MessageInfo, Response, Uint128, CosmosMsg,
    WasmMsg, StdError, to_binary, WasmQuery, QueryRequest, Decimal, Coin, Addr, Env,
};

use crate::error::ContractError;
use crate::state::{CONFIG, Config, STATE, State};
use crate::red_bank::{RedBankQueryMsg, MarketResponse, RedBankExecuteMsg};
use crate::token_factory::TokenFactoryExecuteMsg;

pub fn try_deposit(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];

    let config: Config = CONFIG.load(deps.storage)?;
    let mut state: State = STATE.load(deps.storage)?;

    // read amount of underlying_denom (OSMO) sent by user on deposit
    let mut amount_raw: Uint128 = Uint128::default();
    for coin in &info.funds {
        if coin.denom == config.underlying_denom {
            amount_raw = coin.amount
        }
    }

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

    // Update state (underlying_deposited, exchange_rate) first
    // To check: should this be updated after deposit has been made?
    state.underlying_deposited += amount_raw;
    state.exchange_rate = market_data.liquidity_index;
    STATE.save(deps.storage, &state)?;

    // Deposit user's OSMO into red bank
    let underlying_bank_deposit_msg = RedBankExecuteMsg::Deposit {
        on_behalf_of: None,
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&underlying_bank_deposit_msg)?,
        funds: vec![Coin {
            denom: config.underlying_denom.clone(),
            amount: amount_raw,
        }],
    }));

    // Calculate OSMOmars to mint = osmo_amount/exchange_rate
    let yeild_bearing_amount = 
        Decimal::from_ratio(amount_raw, Uint128::from(1u128))
            .checked_div(market_data.liquidity_index).unwrap().to_uint_floor();

    // Mint OSMOmars native token to user
    let mint_msg = TokenFactoryExecuteMsg::Mint {
        to_address: info.sender.to_string(),
        amount: yeild_bearing_amount.into()
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.yield_bearing_token.into_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    }));

    deps.api.debug("osmo deposited successfully");
    Ok(
        Response::new()
        .add_messages(messages)
        .add_attribute("underlying_deposited", amount_raw.to_string())
        .add_attribute("yield_bearing_minted", yeild_bearing_amount.to_string())
        .add_attribute("total_underlying_deposited", state.underlying_deposited.to_string())
        .add_attribute("exchange_rate", state.exchange_rate.to_string())
    )
}

pub fn try_update_yield_bearing_denom(
    deps: DepsMut,
    info: MessageInfo,
    yield_bearing_denom: String,
    yield_bearing_token: Addr,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Std(StdError::generic_err(
            "Admin commands can only be ran from owner address",
        )));
    }

    config.yield_bearing_denom = yield_bearing_denom.clone();
    config.yield_bearing_token = yield_bearing_token.clone();
    CONFIG.save(deps.storage, &config)?;

    deps.api.debug("yield bearing denom address updated successfully");
    Ok(Response::default())
}

pub fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];

    // Calc amount of OSMO to withdraw against ybToken
    let config: Config = CONFIG.load(deps.storage)?;
    let mut state: State = STATE.load(deps.storage)?;

    // read amount of yield_bearing_denom (OSMOmars) sent by user on deposit
    let mut ybt_amount: Uint128 = Uint128::default();
    for coin in &info.funds {
        if coin.denom == config.yield_bearing_denom {
            ybt_amount = coin.amount
        }
    }

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

    let underlying_amount = 
        Decimal::from_ratio(ybt_amount, Uint128::from(1u128))
            .checked_mul(market_data.liquidity_index).unwrap().to_uint_floor();

    // Update state (underlying_deposited, exchange_rate) first
    // To check: should this be updated after deposit has been made?
    state.underlying_deposited = state.underlying_deposited.checked_sub(underlying_amount).unwrap();
    state.exchange_rate = market_data.liquidity_index;
    STATE.save(deps.storage, &state)?;

    // Burn the ybToken native token
    let burn_msg = TokenFactoryExecuteMsg::Burn {
        from_address: env.contract.address.into_string(),
        amount: ybt_amount.into(),
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.yield_bearing_token.into_string(),
        msg: to_binary(&burn_msg)?,
        funds: vec![],
    }));

    // Withdraw OSMO from mars with receiver as the sender
    let underlying_bank_withdraw_msg = RedBankExecuteMsg::Withdraw {
        denom: config.underlying_denom.to_string(),
        amount: Some(underlying_amount),
        recipient: Some(info.sender.to_string()),
    };
    messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.red_bank.to_string(),
        msg: to_binary(&underlying_bank_withdraw_msg)?,
        funds: vec![],
    }));

    deps.api.debug("OSMO withdrawn successfully");
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("underlying_withdrawn", underlying_amount.to_string())
        .add_attribute("yield_bearing_burned", ybt_amount.to_string())
        .add_attribute("total_underlying_deposited", state.underlying_deposited.to_string())
        .add_attribute("exchange_rate", state.exchange_rate.to_string())
    )
}

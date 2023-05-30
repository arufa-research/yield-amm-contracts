use cosmwasm_std::{
    DepsMut, MessageInfo, Response, Uint128, CosmosMsg, Env,
    StdError, to_binary, WasmQuery, QueryRequest, Decimal, Addr, BankMsg, coins,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgMint, MsgBurn};
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;

use crate::error::ContractError;
use crate::state::{CONFIG, Config, STATE, State};
use crate::red_bank::{RedBankQueryMsg, MarketResponse};

pub fn try_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let contract_addr = env.contract.address.to_string();
    
    let config: Config = CONFIG.load(deps.storage)?;
    
    let adapter_addr = config.mars_adapter.clone();
    let rewards_contract = config.rewards_contract.ok_or_else(|| {
        ContractError::Std(StdError::generic_err(
            "rewards_contract addr not registered".to_string(),
        ))
    })?.to_string();
    let yield_bearing_denom = config.yield_bearing_denom;
    let principle_denom = config.principle_denom;
    let yield_denom = config.yield_denom;

    // Fetch exchangeRate from red_bank
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

    // read user sent pToken and yToken amount
    let mut ptoken_amount: Uint128 = Uint128::default();
    let mut ytoken_amount: Uint128 = Uint128::default();
    for coin in &info.funds {
        if coin.denom == format!("factory/{contract_addr}/{principle_denom}") {
            ptoken_amount = coin.amount
        }
        if coin.denom == format!("factory/{contract_addr}/{yield_denom}") {
            ytoken_amount = coin.amount
        }
    }

    // if ptoken_amount != ytoken_amount or ptoken_amount == 0,
    // then raise error
    if ptoken_amount != ytoken_amount || ptoken_amount == Uint128::from(0u128) {
        return Err(ContractError::Std(StdError::generic_err(
            "pToken and yToken amount should be equal",
        )));
    }

    // calc amount of ybToken to send to user
    let ybt_amount = 
        Decimal::from_ratio(ptoken_amount, Uint128::from(1u128))
            .checked_div(market_data.liquidity_index).unwrap().to_uint_floor();

    // burn pToken and yToken
    let burn_ptoken_msg: CosmosMsg = MsgBurn {
        sender: contract_addr.clone(),
        amount: Some(OsmoCoin {
            denom: format!("factory/{contract_addr}/{principle_denom}"),
            amount: ptoken_amount.into(),
        }),
        burn_from_address: contract_addr.clone(),
    }
    .into();
    messages.push(burn_ptoken_msg);
    let burn_ytoken_msg: CosmosMsg = MsgBurn {
        sender: contract_addr.clone(),
        amount: Some(OsmoCoin {
            denom: format!("factory/{contract_addr}/{yield_denom}"),
            amount: ytoken_amount.into(),
        }),
        burn_from_address: contract_addr.clone(),
    }
    .into();
    messages.push(burn_ytoken_msg);

    // send ybToken to user
    let send_ybt_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(ybt_amount.u128(), format!("factory/{adapter_addr}/{yield_bearing_denom}")),
    };
    messages.push(cosmwasm_std::CosmosMsg::Bank(send_ybt_msg));

    // calc amount of ybToken to send to reward contract
    // send ybToken to reward contract

    let mut state: State = STATE.load(deps.storage)?;

    // Update state (yb_deposited, exchange_rate) first
    // To check: should this be updated after deposit has been made?
    state.exchange_rate = market_data.liquidity_index;

    state.yb_deposited += ybt_amount;
    state.prev_exchange_rate = market_data.liquidity_index;
    STATE.save(deps.storage, &state)?;

    deps.api.debug("ybToken withdrawn successfully");
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("ybToken withdrawn", ybt_amount.to_string())
        .add_attribute("pToken burned", ptoken_amount.to_string())
        .add_attribute("yToken burned", ytoken_amount.to_string())
        .add_attribute("exchange_rate", state.exchange_rate.to_string())
    )
}

pub fn try_advance(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    // calc amount of ybToken to send to reward contract
    // send ybToken to reward contract
    deps.api.debug("epoch moved successfully");
    Ok(Response::default())
}

pub fn try_deposit(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut messages: Vec<CosmosMsg> = vec![];
    let contract_addr = env.contract.address.to_string();
    
    let config: Config = CONFIG.load(deps.storage)?;
    
    let adapter_addr = config.mars_adapter.clone();
    let rewards_contract = config.rewards_contract.ok_or_else(|| {
        ContractError::Std(StdError::generic_err(
            "rewards_contract addr not registered".to_string(),
        ))
    })?.to_string();
    let yield_bearing_denom = config.yield_bearing_denom;
    let principle_denom = config.principle_denom;
    let yield_denom = config.yield_denom;

    // Fetch exchangeRate from red_bank
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

    // read amount of yield_bearing_denom (OSMOmars) sent by user
    let mut ybt_amount: Uint128 = Uint128::default();
    for coin in &info.funds {
        if coin.denom == format!("factory/{adapter_addr}/{yield_bearing_denom}") {
            ybt_amount = coin.amount
        }
    }

    // Calc amount of pToken and yToken to issue
    // pToken amont = (ybToken amount) * exchangeRate
    let ptoken_amount = 
        Decimal::from_ratio(ybt_amount, Uint128::from(1u128))
            .checked_mul(market_data.liquidity_index).unwrap().to_uint_floor();

    // yToken amont = (ybToken amount) * exchangeRate
    let ytoken_amount = 
        Decimal::from_ratio(ybt_amount, Uint128::from(1u128))
            .checked_mul(market_data.liquidity_index).unwrap().to_uint_floor();

    // Hold ybToken
    // Mint pToken and yToken to the user
    let mint_ptoken_msg: CosmosMsg = MsgMint {
        sender: contract_addr.clone(),
        amount: Some(OsmoCoin {
            denom: format!("factory/{contract_addr}/{principle_denom}"),
            amount: ptoken_amount.into(),
        }),
        mint_to_address: contract_addr.clone(),
    }
    .into();
    messages.push(mint_ptoken_msg);

    let mint_ytoken_msg: CosmosMsg = MsgMint {
        sender: contract_addr.clone(),
        amount: Some(OsmoCoin {
            denom: format!("factory/{contract_addr}/{yield_denom}"),
            amount: ytoken_amount.into(),
        }),
        mint_to_address: contract_addr.clone(),
    }
    .into();
    messages.push(mint_ytoken_msg);

    // send minted pToken and yToken to user
    let send_ptoken_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(ptoken_amount.u128(), format!("factory/{contract_addr}/{principle_denom}")),
    };
    messages.push(cosmwasm_std::CosmosMsg::Bank(send_ptoken_msg));
    let send_ytoken_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coins(ytoken_amount.u128(), format!("factory/{contract_addr}/{yield_denom}")),
    };
    messages.push(cosmwasm_std::CosmosMsg::Bank(send_ytoken_msg));

    let mut state: State = STATE.load(deps.storage)?;

    // Update state (yb_deposited, exchange_rate) first
    // To check: should this be updated after deposit has been made?
    state.exchange_rate = market_data.liquidity_index;

    // after updating exchange_rate and before updating prev_exchange_rate
    // calc the amount of yield generated using formula
    // (exchange_rate - prev_exchange_rate) * (ybt_deposited) / (prev_exchange_rate)
    // multiple exchange rates with 10**6 before using
    let rate_change = 
        (state.exchange_rate - state.prev_exchange_rate).checked_div(state.prev_exchange_rate).unwrap();

    // Some ybToken will go to rewards contract
    // Divide tokenAmount with exch_rate to get ybToken amount
    // TODO: cross check the calculation correctness
    let reward_amount =  Decimal::from_ratio(state.yb_deposited, Uint128::from(1u128))
        .checked_mul(rate_change).unwrap().checked_div(state.exchange_rate).unwrap().to_uint_floor();

    // TODO: check if amt > 0 before sending
    // // send reward_amount of ybToken to rewards_contract
    // let send_ybt_rewards = BankMsg::Send {
    //     to_address: rewards_contract.to_string(),
    //     amount: coins(reward_amount.u128(), format!("factory/{adapter_addr}/{yield_bearing_denom}")),
    // };
    // messages.push(cosmwasm_std::CosmosMsg::Bank(send_ybt_rewards));

    state.yb_deposited += ybt_amount;
    state.prev_exchange_rate = market_data.liquidity_index;
    STATE.save(deps.storage, &state)?;

    deps.api.debug("ybToken deposited successfully");
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("ybToken deposited", ybt_amount.to_string())
        .add_attribute("pToken minted", ptoken_amount.to_string())
        .add_attribute("yToken minted", ytoken_amount.to_string())
        .add_attribute("exchange_rate", state.exchange_rate.to_string())
    )
}

// pub fn try_calc_and_send_rewards(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
// )

pub fn try_update_rewards_contract(
    deps: DepsMut,
    info: MessageInfo,
    rewards_contract: Addr,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Std(StdError::generic_err(
            "Admin commands can only be ran from owner address",
        )));
    }

    config.rewards_contract = Some(rewards_contract);
    CONFIG.save(deps.storage, &config)?;

    deps.api.debug("rewards_contract address updated successfully");
    Ok(Response::default())
}

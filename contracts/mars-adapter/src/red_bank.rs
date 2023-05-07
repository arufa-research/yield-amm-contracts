use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Uint128};
// use cosmwasm_std::{Uint128, Addr, Decimal};

#[cw_serde]
#[derive(QueryResponses)]
pub enum RedBankQueryMsg {
    // #[returns(crate::red_bank::ConfigResponse)]
    // Config {},
    #[returns(MarketResponse)]
    Market {
        denom: String,
    },
    // #[returns(Vec<crate::red_bank::Market>)]
    // Markets {
    //     start_after: Option<String>,
    //     limit: Option<u32>,
    // },
}

#[cw_serde]
#[derive(Eq, Default)]
pub struct InterestRateModel {
    pub optimal_utilization_rate: Decimal,
    pub base: Decimal,
    pub slope_1: Decimal,
    pub slope_2: Decimal,
}

#[cw_serde]
pub struct MarketResponse {
    pub denom: String,
    pub max_loan_to_value: Decimal,
    pub liquidation_threshold: Decimal,
    pub liquidation_bonus: Decimal,
    pub reserve_factor: Decimal,
    pub interest_rate_model: InterestRateModel,
    pub borrow_index: Decimal,
    pub liquidity_index: Decimal,
    pub borrow_rate: Decimal,
    pub liquidity_rate: Decimal,
    pub indexes_last_updated: u64,
    pub collateral_total_scaled: Uint128,
    pub debt_total_scaled: Uint128,
    pub deposit_enabled: bool,
    pub borrow_enabled: bool,
    pub deposit_cap: Uint128,
}

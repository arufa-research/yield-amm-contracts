use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;

#[cw_serde]
pub enum TokenFactoryExecuteMsg {
    /// Mint token to address. Mint allowance is required and wiil be deducted after successful mint.
    Mint {
        to_address: String,
        amount: Uint128,
    },

    /// Burn token to address. Burn allowance is required and wiil be deducted after successful burn.
    Burn {
        from_address: String,
        amount: Uint128,
    },

    /// Block target address from sending/receiving token attached to this contract
    /// tokenfactory's beforesend listener must be set to this contract in order for it to work as intended.
    Blacklist {
        address: String,
        status: bool,
    },

    /// Block every token transfers of the token attached to this contract
    /// tokenfactory's beforesend listener must be set to this contract in order for it to work as intended.
    Freeze {
        status: bool,
    },
}

// #[cw_serde]
// #[derive(QueryResponses)]
// pub enum TokenFactoryQueryMsg {
//     /// IsFrozen returns if the entire token transfer functionality is frozen. Response: IsFrozenResponse
//     #[returns(IsFrozenResponse)]
//     IsFrozen {},
//     /// Denom returns the token denom that this contract is the admin for. Response: DenomResponse
//     #[returns(DenomResponse)]
//     Denom {},
//     /// Owner returns the owner of the contract. Response: OwnerResponse
//     #[returns(OwnerResponse)]
//     Owner {},
//     /// Allowance returns the allowance of the specified address. Response: AllowanceResponse
//     #[returns(AllowanceResponse)]
//     BurnAllowance { address: String },
//     /// Allowances Enumerates over all allownances. Response: Vec<AllowanceResponse>
//     #[returns(AllowancesResponse)]
//     BurnAllowances {
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//     /// Allowance returns the allowance of the specified user. Response: AllowanceResponse
//     #[returns(AllowanceResponse)]
//     MintAllowance { address: String },
//     /// Allowances Enumerates over all allownances. Response: AllowancesResponse
//     #[returns(AllowancesResponse)]
//     MintAllowances {
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//     /// IsBlacklisted returns wether the user is blacklisted or not. Response: StatusResponse
//     #[returns(StatusResponse)]
//     IsBlacklisted { address: String },
//     /// Blacklistees enumerates over all addresses on the blacklist. Response: BlacklisteesResponse
//     #[returns(BlacklisteesResponse)]
//     Blacklistees {
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//     /// IsBlacklister returns if the addres has blacklister privileges. Response: StatusResponse
//     #[returns(StatusResponse)]
//     IsBlacklister { address: String },
//     /// Blacklisters Enumerates over all the addresses with blacklister privileges. Response: BlacklisterAllowancesResponse
//     #[returns(BlacklisterAllowancesResponse)]
//     BlacklisterAllowances {
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
//     /// IsFreezer returns whether the address has freezer status. Response: StatusResponse
//     #[returns(StatusResponse)]
//     IsFreezer { address: String },
//     /// FreezerAllowances enumerates over all freezer addresses. Response: FreezerAllowancesResponse
//     #[returns(FreezerAllowancesResponse)]
//     FreezerAllowances {
//         start_after: Option<String>,
//         limit: Option<u32>,
//     },
// }

// // We define a custom struct for each query response
// #[cw_serde]
// pub struct IsFrozenResponse {
//     pub is_frozen: bool,
// }

// // We define a custom struct for each query response
// #[cw_serde]
// pub struct DenomResponse {
//     pub denom: String,
// }

// #[cw_serde]
// pub struct OwnerResponse {
//     pub address: String,
// }

// #[cw_serde]
// pub struct AllowanceResponse {
//     pub allowance: Uint128,
// }

// #[cw_serde]
// pub struct AllowanceInfo {
//     pub address: String,
//     pub allowance: Uint128,
// }

// #[cw_serde]
// pub struct AllowancesResponse {
//     pub allowances: Vec<AllowanceInfo>,
// }

// #[cw_serde]
// pub struct StatusResponse {
//     pub status: bool,
// }

// #[cw_serde]
// pub struct StatusInfo {
//     pub address: String,
//     pub status: bool,
// }

// #[cw_serde]
// pub struct BlacklisteesResponse {
//     pub blacklistees: Vec<StatusInfo>,
// }

// #[cw_serde]
// pub struct BlacklisterAllowancesResponse {
//     pub blacklisters: Vec<StatusInfo>,
// }

// #[cw_serde]
// pub struct FreezerAllowancesResponse {
//     pub freezers: Vec<StatusInfo>,
// }

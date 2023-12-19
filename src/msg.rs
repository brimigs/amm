use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub initial_funding_token1: Coin,
    pub initial_funding_token2: Coin,
    pub initial_lp_token_supply: Uint128,
    pub contract_config: Config,
}

#[cw_serde]
pub enum ExecuteMsg {
    Deposit {
        asset1: Coin,
        asset2: Coin,
    },
    Withdraw {
        amount_to_burn: Uint128,
    },
    Swap {
        offered_asset: Coin,
    },
}

#[cw_serde]
pub enum QueryMsg {
    GetReserves {},
    GetUserShare { user: Addr },
    GetConfig {},
}
#[cw_serde]
pub struct Config {
    /// The LP token address for the LP pair that corresponds to this pool
    pub lp_token_addr: String,
    /// The config for swap fee sharing
    pub fee_share: Decimal,
    /// Tolerance percentage for verifying deposit ratio
    pub tolerance_percentage: Decimal,
}
#[cw_serde]
pub struct ReservesResponse {
    pub asset1_reserve: Uint128,
    pub asset2_reserve: Uint128,
}
#[cw_serde]
pub struct UserShareResponse {
    pub user_share: Uint128,
}

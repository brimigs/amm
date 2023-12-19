use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};
use crate::msg::Config;

// The current total LP token supply
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_shares");

// Balances of liquidity shares per address
pub const SHARE_BALANCES: Map<&Addr, Uint128> = Map::new("share_balances");

// Current pool reserves for each asset
pub const POOL_RESERVES: Item<PoolReserves> = Item::new("pool_reserves");

pub const CONFIG: Item<Config> = Item::new("Config");

// Pool state to store the reserves using the Coin type
#[cw_serde]
pub struct PoolReserves {
    pub asset1: Coin,
    pub asset2: Coin,
}
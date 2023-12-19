use crate::msg::{Config, ReservesResponse, UserShareResponse};
use crate::state::{CONFIG, POOL_RESERVES, SHARE_BALANCES};
use cosmwasm_std::{Addr, Deps, StdResult};

pub fn query_reserves(deps: Deps) -> StdResult<ReservesResponse> {
    let pool_reserves = POOL_RESERVES.load(deps.storage)?;
    Ok(ReservesResponse {
        asset1_reserve: pool_reserves.asset1.amount,
        asset2_reserve: pool_reserves.asset2.amount,
    })
}

pub fn query_user_share(deps: Deps, user: Addr) -> StdResult<UserShareResponse> {
    let user_share = SHARE_BALANCES.load(deps.storage, &user.clone())?;
    Ok(UserShareResponse {
        user_share,
    })
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage)?;
    Ok(config)
}

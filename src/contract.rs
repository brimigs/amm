use cosmwasm_std::{Binary, DepsMut, Env, MessageInfo, Response, StdResult, Deps, to_json_binary, entry_point};
use cw2::set_contract_version;
use crate::error::ContractError;
use crate::execute::{deposit, swap, withdraw};
use crate::instantiate::set_up_contract;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{query_config, query_reserves, query_user_share};

const CONTRACT_NAME: &str = "xyk-amm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError>  {
    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), CONTRACT_VERSION)?;
    set_up_contract(deps, env, info, msg)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError>  {
    match msg {
        ExecuteMsg::Deposit {
            asset1,
            asset2,
        } => deposit(deps, env, info, asset1, asset2),

        ExecuteMsg::Withdraw {
            amount_to_burn,
        } => withdraw(deps, env, info, amount_to_burn),

        ExecuteMsg::Swap {
            offered_asset,
        } => swap(deps, env, info, offered_asset),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetReserves {} => to_json_binary(&query_reserves(deps)?),
        QueryMsg::GetUserShare { user } => to_json_binary(&query_user_share(deps, user)?),
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
    }
}
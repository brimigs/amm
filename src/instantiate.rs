use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use crate::error::ContractError;
use crate::msg::{Config, InstantiateMsg};
use crate::state::{CONFIG, POOL_RESERVES, PoolReserves, TOTAL_SUPPLY};

pub fn set_up_contract(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg
) -> Result<Response, ContractError> {
    // Ensure that the provided funding is not zero (see README for explanation on this)
    if msg.initial_funding_token1.amount.is_zero() || msg.initial_funding_token2.amount.is_zero() {
        return Err(ContractError::InvalidZeroAmount {});
    }

    // Validate LP token address and save to config
    let lp_token_address = deps.api.addr_validate(&msg.contract_config.lp_token_addr)?;
    let config = Config {
        lp_token_addr: lp_token_address.to_string(),
        fee_share: msg.contract_config.fee_share,
        tolerance_percentage: msg.contract_config.tolerance_percentage,
    };
    CONFIG.save(deps.storage, &config)?;

    // Set the initial pool reserves with the provided funding amounts
    let initial_reserves = PoolReserves {
        asset1: msg.initial_funding_token1,
        asset2: msg.initial_funding_token2,
    };
    POOL_RESERVES.save(deps.storage, &initial_reserves)?;

    // Initialize the total shares. For simplicity, total shares were issued before instantiating contract and the value is passed as a parameter.
    TOTAL_SUPPLY.save(deps.storage, &msg.initial_lp_token_supply)?;

    // Ensure that the correct funds are sent to match the initial pool funding
    if !info.funds.iter().any(|coin| coin.denom == initial_reserves.asset1.denom && coin.amount == initial_reserves.asset1.amount)
        || !info.funds.iter().any(|coin| coin.denom == initial_reserves.asset2.denom && coin.amount == initial_reserves.asset2.amount) {
        return Err(ContractError::InvalidInitialFunds {});
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("initial_funding_token1_denom", initial_reserves.asset1.denom)
        .add_attribute("initial_funding_token1_amount", initial_reserves.asset1.amount.to_string())
        .add_attribute("initial_funding_token2_denom", initial_reserves.asset2.denom)
        .add_attribute("initial_funding_token2_amount", initial_reserves.asset2.amount.to_string()))
}
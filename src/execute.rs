use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, Decimal, DepsMut, Env, MessageInfo, Response, to_binary, to_json_binary, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
use crate::error::ContractError;
use crate::msg::Config;
use crate::state::{CONFIG, POOL_RESERVES, SHARE_BALANCES, TOTAL_SUPPLY};

pub fn deposit(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    asset1: Coin,
    asset2: Coin,
) -> Result<Response, ContractError> {
    // Validate that exactly two assets are sent
    if info.funds.len() != 2 {
        return Err(ContractError::InvalidFunds {});
    }

    // Validate that the correct assets and amounts are sent
    let mut asset1_received = false;
    let mut asset2_received = false;

    for fund in &info.funds {
        if fund.denom == asset1.denom && fund.amount == asset1.amount {
            asset1_received = true;
        } else if fund.denom == asset2.denom && fund.amount == asset2.amount {
            asset2_received = true;
        }
    }

    if !asset1_received || !asset2_received {
        return Err(ContractError::InvalidFunds {});
    }

    // Load current pool state from storage
    let mut pool_reserves = POOL_RESERVES.load(deps.storage)?;

    // Load the contract config
    let config = CONFIG.load(deps.storage)?;

    // Load total supply of liquidity tokens
    let mut total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    // Calculate the ratio of the deposit: (deposit_x / deposit_y) = (reserve_x/reserve_y)
    let expected_deposit2 = (asset1.amount * pool_reserves.asset2.amount) / pool_reserves.asset1.amount;
    let expected_deposit1 = (asset2.amount * pool_reserves.asset1.amount) / pool_reserves.asset2.amount;

    // Calculate the minimum and maximum expected amounts for the assets based on a specified tolerance
    let min_expected_deposit2 = expected_deposit2 * (Decimal::one() - config.tolerance_percentage);
    let max_expected_deposit2 = expected_deposit2 * (Decimal::one() + config.tolerance_percentage);
    let min_expected_deposit1 = expected_deposit1 * (Decimal::one() - config.tolerance_percentage);
    let max_expected_deposit1 = expected_deposit1 * (Decimal::one() + config.tolerance_percentage);

    // Check if the deposited amount for the assets is within the tolerance range
    if asset1.amount < min_expected_deposit1 || asset1.amount > max_expected_deposit1 {
        return Err(ContractError::InvalidDepositRatio {});
    }
    if asset2.amount < min_expected_deposit2 || asset2.amount > max_expected_deposit2 {
        return Err(ContractError::InvalidDepositRatio {});
    }

    // Calculate the amount of liquidity tokens to mint: liquidity_tokens = (total_supply_of_liquidity_tokens * max(deposit_x / reserve_x, deposit_y / reserve_y))
    // Note: The above formula is broken down into several separate calculations below address potential overflow errors with rust

    // Calculate the liquidity tokens to mint using the smallest proportional deposit
    // To avoid integer division rounding down to zero, we use checked multiplication and division
    let asset1_ratio = asset1.amount.checked_mul(total_supply)?;
    let asset2_ratio = asset2.amount.checked_mul(total_supply)?;

    let lp_tokens_to_mint_asset1 = asset1_ratio / pool_reserves.asset1.amount;
    let lp_tokens_to_mint_asset2 = asset2_ratio / pool_reserves.asset2.amount;

    // Use the minimum of the two calculated mint amounts to ensure proportional addition of liquidity
    let lp_tokens_to_mint = std::cmp::min(lp_tokens_to_mint_asset1, lp_tokens_to_mint_asset2);

    // Check if the resulting LP tokens to mint is zero
    if lp_tokens_to_mint.is_zero() {
        // The deposit amounts are too small relative to the pool size
        return Err(ContractError::DepositTooSmall {});
    }

    // Mint LP tokens to the depositor's address
    mint_liquidity_tokens(&config, info.sender.clone(), lp_tokens_to_mint)?;

    // Update pool reserves in storage
    pool_reserves.asset1.amount += asset1.amount;
    pool_reserves.asset2.amount += asset2.amount;
    POOL_RESERVES.save(deps.storage, &pool_reserves)?;

    // Update total LP supply in storage
    total_supply += lp_tokens_to_mint;
    TOTAL_SUPPLY.save(deps.storage, &total_supply)?;

    // Add depositor's LP share to storage
    let mut depositors_shares = SHARE_BALANCES.may_load(deps.storage, &info.sender)?.unwrap_or(Uint128::zero());
    depositors_shares += lp_tokens_to_mint;
    SHARE_BALANCES.save(deps.storage, &info.sender, &depositors_shares)?;

    Ok(Response::new()
        .add_attribute("action", "deposit")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("LP_tokens_minted", lp_tokens_to_mint.to_string()))
}

pub fn withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount_to_burn: Uint128,
) -> Result<Response, ContractError> {
    // Load user's share amount
    let user_shares = SHARE_BALANCES.load(deps.storage, &info.sender)?;

    // Validate that the user hold enough shares to burn the requested amount
    if amount_to_burn > user_shares {
        return Err(ContractError::WithdrawError {});
    }

    // Load total supply of liquidity tokens
    let mut total_supply = TOTAL_SUPPLY.load(deps.storage)?;

    // Load current pool state from storage
    let mut pool_reserves = POOL_RESERVES.load(deps.storage)?;

    // Calculate the proportion of the total supply that the LP tokens represent
    let lp_token_share = Decimal::from_ratio(amount_to_burn, total_supply);

    // Calculate the amount of each asset to return to the LP
    let amount1 = lp_token_share * pool_reserves.asset1.amount;
    let amount2 = lp_token_share * pool_reserves.asset2.amount;

    // Update pool reserves in storage
    pool_reserves.asset1.amount -= amount1;
    pool_reserves.asset2.amount -= amount2;
    POOL_RESERVES.save(deps.storage, &pool_reserves)?;

    // Update total LP supply in storage
    total_supply -= amount_to_burn;
    TOTAL_SUPPLY.save(deps.storage, &total_supply)?;

    // Load the contract config
    let config = CONFIG.load(deps.storage)?;

    // Burn the LP tokens from the user's balance
    let burn_msg = Cw20ExecuteMsg::Burn { amount: amount_to_burn };
    let exec_burn_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.lp_token_addr.to_string(),
        msg: to_json_binary(&burn_msg)?,
        funds: vec![],
    });

    // Send the withdrawn assets to the user
    let send_x_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin { denom: pool_reserves.asset1.denom.to_string(), amount: amount1 }],
    };
    let send_y_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin { denom: pool_reserves.asset2.denom.to_string(), amount: amount2 }],
    };

    Ok(Response::new()
        .add_message(exec_burn_msg)
        .add_message(send_x_msg)
        .add_message(send_y_msg)
        .add_attribute("action", "withdraw_liquidity")
        .add_attribute("withdrawn_asset_x", amount1.to_string())
        .add_attribute("withdrawn_asset_y", amount2.to_string()))
}

// This swap function follows the constant product formula for an AMM (xy=K)
pub fn swap(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    offered_asset: Coin,
) -> Result<Response, ContractError> {

    // Load current pool state from storage
    let mut pool_reserves = POOL_RESERVES.load(deps.storage)?;

    // Load config
    let config = CONFIG.load(deps.storage)?;

    // Calculate the invariant k before the swap
    let k = pool_reserves.asset1.amount * pool_reserves.asset2.amount;

    let swap_fee = offered_asset.amount * config.fee_share;

    // Subtract fee from offered amount
    let new_coin_amount = offered_asset.amount - swap_fee;

    let output_amount: Uint128;
    let denom: String;

    if offered_asset.denom == pool_reserves.asset1.denom {
        pool_reserves.asset1.amount += new_coin_amount;

        // Calculate new_reserve2 such that new_reserve1 * new_reserve2 = k
        let new_reserve2 = Uint128::from(k/pool_reserves.asset1.amount);

        output_amount = pool_reserves.asset2.amount - new_reserve2;

        // Update pool for asset2
        pool_reserves.asset2.amount = new_reserve2;

        denom = pool_reserves.asset2.denom.clone();

    } else if offered_asset.denom == pool_reserves.asset2.denom {
        pool_reserves.asset2.amount += new_coin_amount;

        // Calculate new_reserve2 such that new_reserve1 * new_reserve2 = k
        let new_reserve1 = Uint128::from(k/pool_reserves.asset2.amount);

        output_amount = pool_reserves.asset1.amount - new_reserve1;

        // Update pool reserve in storage
        pool_reserves.asset1.amount = new_reserve1;

        denom = pool_reserves.asset1.denom.clone();

    } else {
        return Err(ContractError::InvalidFunds {});
    }

    // Create the message to send token B to the user
    let send_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin { denom, amount: output_amount }],
    };

    POOL_RESERVES.save(deps.storage, &pool_reserves)?;

    Ok(Response::new().add_message(CosmosMsg::Bank(send_msg)).add_attribute("action", "swap"))
}
pub fn mint_liquidity_tokens(
    config: &Config,
    recipient: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    let lp_token = config.lp_token_addr.clone();

    // Create the execution message for minting liquidity tokens
    let mint_msg = &Cw20ExecuteMsg::Mint {
        recipient: recipient.to_string(),
        amount,
    };

    let exec_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: lp_token,
        msg: to_binary(mint_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(exec_msg)
        .add_attribute("action", "mint_liquidity_tokens")
        .add_attribute("amount", amount.to_string()))
}
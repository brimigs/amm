use cosmwasm_std::{Addr, coin, Decimal, Uint128};
use cw_multi_test::{App, BankSudo, Contract, ContractWrapper, Executor, SudoMsg};
use amm_contract::msg::{Config, ExecuteMsg, InstantiateMsg, ReservesResponse, UserShareResponse};
use amm_contract::msg::QueryMsg::{GetConfig, GetReserves, GetUserShare};

#[test]
fn instantiate_success() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        amm_contract::contract::execute,
        amm_contract::contract::instantiate,
        amm_contract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));

    let owner = Addr::unchecked("owner");

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: owner.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    let addr = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            initial_funding_token1: coin(100000,"asset1"),
            initial_funding_token2: coin(100000,"asset2"),
            initial_lp_token_supply: Uint128::new(100),
            contract_config: Config {
                lp_token_addr: "lp_tokens".to_string(),
                fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
                tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            },
        },
        &[coin(100000,"asset1"), coin(100000,"asset2")],
        "mock-amm-contract",
        None,
    ).unwrap();

    let config: Config = app.wrap().query_wasm_smart(addr.clone(), &GetConfig {}).unwrap();
    let pool_reserves: ReservesResponse = app.wrap().query_wasm_smart(addr.clone(), &GetReserves {}).unwrap();

    assert_eq!(
        config,
        Config {
            lp_token_addr: "lp_tokens".to_string(),
            fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
        }
    );

    assert_eq!(
        pool_reserves,
        ReservesResponse {
            asset1_reserve: Uint128::new(100000),
            asset2_reserve: Uint128::new(100000),
        }
    );
}
#[test]
fn successful_deposit() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        amm_contract::contract::execute,
        amm_contract::contract::instantiate,
        amm_contract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));

    let owner = Addr::unchecked("owner");
    let depositor = Addr::unchecked("depositor");

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: owner.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: depositor.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    let addr = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            initial_funding_token1: coin(1000,"asset1"),
            initial_funding_token2: coin(1000,"asset2"),
            initial_lp_token_supply: Uint128::new(100),
            contract_config: Config {
                lp_token_addr: "lp_tokens".to_string(),
                fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
                tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            },
        },
        &[coin(1000,"asset1"), coin(1000,"asset2")],
        "mock-amm-contract",
        None,
    ).unwrap();

    app.execute_contract(
        depositor.clone(),
        addr.clone(),
        &ExecuteMsg::Deposit {
            asset1: coin(1000000,"asset1"),
            asset2: coin(1000000,"asset2"),
        },
        &[coin(1000000,"asset1"), coin(1000000,"asset2")]
    ).unwrap();

    let user_share: UserShareResponse = app.wrap().query_wasm_smart(addr.clone(), &GetUserShare { user: (depositor) }).unwrap();
    
    assert_eq!(
        user_share, 
        UserShareResponse {
            user_share: Uint128::new(100000),
        }
    );
}
#[test]
fn deposit_invalid_funds_error() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        amm_contract::contract::execute,
        amm_contract::contract::instantiate,
        amm_contract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));

    let owner = Addr::unchecked("owner");
    let depositor = Addr::unchecked("depositor");

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: owner.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: depositor.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    let addr = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            initial_funding_token1: coin(1000,"asset1"),
            initial_funding_token2: coin(1000,"asset2"),
            initial_lp_token_supply: Uint128::new(100),
            contract_config: Config {
                lp_token_addr: "lp_tokens".to_string(),
                fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
                tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            },
        },
        &[coin(1000,"asset1"), coin(1000,"asset2")],
        "mock-amm-contract",
        None,
    ).unwrap();

    app.execute_contract(
        depositor.clone(),
        addr.clone(),
        &ExecuteMsg::Deposit {
            asset1: coin(1000000,"asset1"),
            asset2: coin(1000000,"asset2"),
        },
        &[coin(1000000,"asset1")]
    ).unwrap_err();
}
#[test]
fn deposit_ratio_tolerance_exceeded() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        amm_contract::contract::execute,
        amm_contract::contract::instantiate,
        amm_contract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));

    let owner = Addr::unchecked("owner");
    let depositor = Addr::unchecked("depositor");

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: owner.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: depositor.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    let addr = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            initial_funding_token1: coin(1000,"asset1"),
            initial_funding_token2: coin(1000,"asset2"),
            initial_lp_token_supply: Uint128::new(100),
            contract_config: Config {
                lp_token_addr: "lp_tokens".to_string(),
                fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
                tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            },
        },
        &[coin(1000,"asset1"), coin(1000,"asset2")],
        "mock-amm-contract",
        None,
    ).unwrap();

    app.execute_contract(
        depositor.clone(),
        addr.clone(),
        &ExecuteMsg::Deposit {
            asset1: coin(1000000,"asset1"),
            asset2: coin(100,"asset2"),
        },
        &[coin(1000000,"asset1"), coin(100,"asset2")]
    ).unwrap_err();
}
#[test]
fn successful_deposit_more_complex_pool_ratio() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        amm_contract::contract::execute,
        amm_contract::contract::instantiate,
        amm_contract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));

    let owner = Addr::unchecked("owner");
    let depositor = Addr::unchecked("depositor");

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: owner.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: depositor.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    let addr = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            initial_funding_token1: coin(1500,"asset1"),
            initial_funding_token2: coin(3000,"asset2"),
            initial_lp_token_supply: Uint128::new(100),
            contract_config: Config {
                lp_token_addr: "lp_tokens".to_string(),
                fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
                tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            },
        },
        &[coin(1500,"asset1"), coin(3000,"asset2")],
        "mock-amm-contract",
        None,
    ).unwrap();

    app.execute_contract(
        depositor.clone(),
        addr.clone(),
        &ExecuteMsg::Deposit {
            asset1: coin(200,"asset1"),
            asset2: coin(400,"asset2"),
        },
        &[coin(200,"asset1"), coin(400,"asset2")]
    ).unwrap();

    let user_share: UserShareResponse = app.wrap().query_wasm_smart(addr.clone(), &GetUserShare { user: (depositor) }).unwrap();

    assert_eq!(
        user_share,
        UserShareResponse {
            user_share: Uint128::new(13),
        }
    )

    // This is calculated by:
    // liquidity_tokens = (total_supply_of_liquidity_tokens * max(deposit_x / reserve_x, deposit_y / reserve_y))
    // 100 * (200/1500) = 13
}
#[test]
fn attempting_to_withdraw_more_than_owned() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        amm_contract::contract::execute,
        amm_contract::contract::instantiate,
        amm_contract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));

    let owner = Addr::unchecked("owner");
    let depositor = Addr::unchecked("depositor");
    let lp_contract_addr = Addr::unchecked("lp_tokens");

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: owner.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: depositor.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: lp_contract_addr.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2"), coin(100000000,"lp_tokens")],
    }))
        .unwrap();

    let addr = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            initial_funding_token1: coin(1000,"asset1"),
            initial_funding_token2: coin(1000,"asset2"),
            initial_lp_token_supply: Uint128::new(100),
            contract_config: Config {
                lp_token_addr: lp_contract_addr.to_string(),
                fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
                tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            },
        },
        &[coin(1000,"asset1"), coin(1000,"asset2")],
        "mock-amm-contract",
        None,
    ).unwrap();

    app.execute_contract(
        depositor.clone(),
        addr.clone(),
        &ExecuteMsg::Deposit {
            asset1: coin(1000000,"asset1"),
            asset2: coin(1000000,"asset2"),
        },
        &[coin(1000000,"asset1"), coin(1000000,"asset2")]
    ).unwrap();

    app.execute_contract(
        depositor.clone(),
        addr.clone(),
        &ExecuteMsg::Withdraw {
            amount_to_burn: Uint128::new(10000000),
        },
        &[]
    ).unwrap_err();
}
#[test]
fn successful_swap() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        amm_contract::contract::execute,
        amm_contract::contract::instantiate,
        amm_contract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));

    let owner = Addr::unchecked("owner");
    let swapper = Addr::unchecked("depositor");

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: owner.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: swapper.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    let addr = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            initial_funding_token1: coin(100000,"asset1"),
            initial_funding_token2: coin(100000,"asset2"),
            initial_lp_token_supply: Uint128::new(100),
            contract_config: Config {
                lp_token_addr: "lp_tokens".to_string(),
                fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
                tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            },
        },
        &[coin(100000,"asset1"), coin(100000,"asset2")],
        "mock-amm-contract",
        None,
    ).unwrap();

    let pool_reserves: ReservesResponse = app.wrap().query_wasm_smart(addr.clone(), &GetReserves {}).unwrap();

    assert_eq!(
        pool_reserves,
        ReservesResponse {
            asset1_reserve: Uint128::new(100000),
            asset2_reserve: Uint128::new(100000),
        }
    );

    app.execute_contract(
        swapper.clone(),
        addr.clone(),
        &ExecuteMsg::Swap {
            offered_asset: coin(100, "asset1"),
        },
        &[coin(100,"asset1")]
    ).unwrap();

    let pool_reserves: ReservesResponse = app.wrap().query_wasm_smart(addr.clone(), &GetReserves {}).unwrap();

    // Assert pool ratio was accurately updated
    assert_eq!(
        pool_reserves,
        ReservesResponse {
            asset1_reserve: Uint128::new(100100),
            asset2_reserve: Uint128::new(99900),
        }
    );

}
#[test]
fn query_pool_data_and_contract_data() {
    let mut app = App::default();
    let code = ContractWrapper::new(
        amm_contract::contract::execute,
        amm_contract::contract::instantiate,
        amm_contract::contract::query,
    );
    let code_id = app.store_code(Box::new(code));

    let owner = Addr::unchecked("owner");
    let depositor = Addr::unchecked("depositor");

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: owner.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    app.sudo(SudoMsg::Bank(BankSudo::Mint {
        to_address: depositor.to_string(),
        amount: vec![coin(100000000,"asset1"), coin(100000000,"asset2")],
    }))
        .unwrap();

    let addr = app.instantiate_contract(
        code_id,
        Addr::unchecked("owner"),
        &InstantiateMsg {
            initial_funding_token1: coin(1000,"asset1"),
            initial_funding_token2: coin(1000,"asset2"),
            initial_lp_token_supply: Uint128::new(100),
            contract_config: Config {
                lp_token_addr: "lp_tokens".to_string(),
                fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
                tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            },
        },
        &[coin(1000,"asset1"), coin(1000,"asset2")],
        "mock-amm-contract",
        None,
    ).unwrap();

    app.execute_contract(
        depositor.clone(),
        addr.clone(),
        &ExecuteMsg::Deposit {
            asset1: coin(1000000,"asset1"),
            asset2: coin(1000000,"asset2"),
        },
        &[coin(1000000,"asset1"), coin(1000000,"asset2")]
    ).unwrap();

    let user_share: UserShareResponse = app.wrap().query_wasm_smart(addr.clone(), &GetUserShare { user: (depositor) }).unwrap();
    let pool_reserves: ReservesResponse = app.wrap().query_wasm_smart(addr.clone(), &GetReserves {}).unwrap();
    let config: Config = app.wrap().query_wasm_smart(addr.clone(), &GetConfig {}).unwrap();

    assert_eq!(
        user_share,
        UserShareResponse {
            user_share: Uint128::new(100000),
        }
    );

    assert_eq!(
        pool_reserves,
        ReservesResponse {
            asset1_reserve: Uint128::new(1001000),
            asset2_reserve: Uint128::new(1001000),
        }
    );

    assert_eq!(
        config,
        Config {
            lp_token_addr: "lp_tokens".to_string(),
            fee_share: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
            tolerance_percentage: Decimal::from_ratio(Uint128::new(3), Uint128::new(1000)),
        }
    );
}
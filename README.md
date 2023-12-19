# Automated Market Maker (AMM) Smart Contract

## Overview
This Automated Market Maker (AMM) Smart Contract provides decentralized exchange functionality allowing users to swap between two different assets, provide liquidity to the liquidity pool, withdraw liquidity, and query the pool reserve and individual shares. It is designed to run on a CosmWasm-compatible blockchain. The AMM uses a constant product formula (x * y = k) to maintain market liquidity and determine prices.

## Instantiate 
When instantiating the contract, the following parameters are needed:
- LP Token Address: contract address for the LP tokens 
- Fee share percentage: The swap fees for the pool 
- Deposit ratio tolerance percentage: The tolerance when calculating the correct deposit ratio for the XYK pool 
You also have the option to contribute to the pool during instantiation: 
- Initial Funding token1
- Initial Funding token 2

NOTE: This contract was created under the assumption that the liquidity pool was already created and funded and the LP token was already minted beforehand.
If this isn't done beforehand, a few changes will need to be made to the contract. 

If the pool is not properly funded before deploying this contract, a few things need to be considered and addressed: 
- No Initial Trading: If the pool starts with no liquidity, no trading can occur until liquidity is provided. That means users cannot swap tokens until at least one liquidity provider adds funds to the pool.
- Incentive to Provide Liquidity: There must be an incentive mechanism in place to encourage users to provide liquidity. Without initial liquidity, the first providers take on the most risk, and typically, protocols offer them higher rewards.
- Price Impact: The first liquidity provider sets the initial price of the tokens in the pool. This can have a significant impact on the market, especially if the provided liquidity is not balanced.
- Slippage: With very low liquidity, trades will have high slippage, making it costly for traders until the pool grows larger.
The contract would need to be updated to handle this situation, and it would be recommended to implement a liquidity bootstrapping mechanism.

## Execution
- Deposit: Provide liquidity to the AMM pool by depositing a pair of assets.
- Withdraw: Remove liquidity from the AMM pool by burning your liquidity tokens.
- Swap: Swap one asset for another within the AMM pool.

## Queries 
- GetReserves: The current reserves of the AMM pool.
- GetUserShare: The share of a specific user in the AMM pool.
- GetConfig: The contract's configuration.

## Testing 
Tests are written with cw-multi-test 
```shell
cargo build 
cargo test 
```

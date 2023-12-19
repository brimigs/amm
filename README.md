# Automated Market Maker (AMM) Smart Contract

## Overview
This Automated Market Maker (AMM) Smart Contract provides decentralized exchange functionality allowing users to swap between two different token types and provide liquidity to token pairs. It is designed to run on a CosmWasm-compatible blockchain. The AMM uses a constant product formula (x * y = k) to maintain market liquidity and determine prices.

# Features
Token Swapping: Users can swap between two types of tokens at rates determined by the current liquidity pool's state.
Liquidity Provision: Users can add liquidity to the pool and receive liquidity provider (LP) tokens in return. These LP tokens represent their share of the pool.
Liquidity Removal: LP token holders can remove their share of liquidity from the pool and receive a portion of the underlying assets in return.
Fee Collection: A fee is applied to swaps which is then distributed to liquidity providers, proportional to their share of the pool.
Slippage Protection: Trades are protected by slippage tolerance levels to prevent significant unexpected price movements.

# Functionality
## Swapping Tokens
To swap tokens, users submit a swap transaction specifying the amount and type of token they wish to swap and the token type they wish to receive. The contract calculates the swap price based on the current pool reserves while ensuring the constant product formula remains intact and that the slippage does not exceed the tolerated amount. 
## Providing Liquidity
To provide liquidity, users deposit a pair of tokens into the pool. The contract mints LP tokens to the user based on the current share price of the pool's liquidity. The deposited tokens are then added to the pool's reserves.
## Withdrawing Liquidity
Liquidity providers can burn their LP tokens to remove their share of liquidity from the pool. The contract returns an amount of each underlying token proportional to the LP tokens burned.

# Contract Explanation 
## Instantiate 
When instantiating the contract, the initial Pool Reserves, Fee Percentage, and Total Shares are set.
- Pool Reserves: Set the initial reserves for both tokens in the liquidity pool to zero. This means that the pool is empty at the start, and liquidity providers will need to deposit tokens to create liquidity.
- Fee Percentage: If your contract takes a fee for swaps, you'll need to set the initial fee percentage. This should be a value that's reasonable to compensate liquidity providers for the risk of impermanent loss without being so high as to discourage traders from using the AMM.
- Total Shares: Set the total number of liquidity shares to zero since there are no liquidity providers at the start.

NOTE: Currently the contract is set to only allow instantiation if initial liquidity is provided to the pool. 
If you want to update the contract to allow for a pool to be deployed with no initial liquidity, two topics need to be considered:
- No Initial Trading: If the pool starts with no liquidity, no trading can occur until liquidity is provided. That means users cannot swap tokens until at least one liquidity provider adds funds to the pool.
- Incentive to Provide Liquidity: There must be an incentive mechanism in place to encourage users to provide liquidity. Without initial liquidity, the first providers take on the most risk, and typically, protocols offer them higher rewards.
- Price Impact: The first liquidity provider sets the initial price of the tokens in the pool. This can have a significant impact on the market, especially if the provided liquidity is not balanced.
- Slippage: With very low liquidity, trades will have high slippage, making it costly for traders until the pool grows larger.
The contract would need to be updated to handle this situation, and it would be recommended to implement a liquidity bootstrapping mechanism. 

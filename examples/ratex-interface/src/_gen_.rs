macro_rules! gen_crate_docs {
    () => {
        concat!(" ", "Ratex Contracts", " v", "0.1.0",
        " program interface generated from Anchor IDL.")
    };
}
pub(crate) use gen_crate_docs;
pub use anchor_interface::prelude::*;
pub mod instruction {
    #[allow(unused_imports)]
    use super::types::*;
    #[allow(unused_imports)]
    use super::state::*;
    #[derive(Debug)]
    pub enum RatexContractsInstruction {
        /// Add Keeper
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[writable]` state
        AddKeeper { new_keeper: ::solana_program::pubkey::Pubkey },
        /// Add Lp Shares
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` token vault base
        /// 2. `[writable]` token vault quote
        /// 3. `[writable]` tick array lower
        /// 4. `[writable]` tick array upper
        /// 5. `[writable]` yield market
        /// 6. `[writable]` token owner account base
        /// 7. `[writable]` token owner account quote
        /// 8. `[]` token mint base
        /// 9. `[]` token mint quote
        /// 10. `[writable]` margin market
        /// 11. `[writable]` margin market vault
        /// 12. `[]` oracle
        /// 13. `[writable]` user token account
        /// 14. `[]` token program
        /// 15. `[]` system program
        /// 16. `[writable]` lp
        /// 17. `[signer, writable]` authority
        AddLpShares {
            amount: i64,
            margin_index: u32,
            market_index: u32,
            lower_rate: u64,
            upper_rate: u64,
        },
        /// Admin Add Lp Shares
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` token vault base
        /// 2. `[writable]` token vault quote
        /// 3. `[writable]` tick array lower
        /// 4. `[writable]` tick array upper
        /// 5. `[writable]` yield market
        /// 6. `[writable]` token owner account base
        /// 7. `[writable]` token owner account quote
        /// 8. `[]` token mint base
        /// 9. `[]` token mint quote
        /// 10. `[writable]` margin market
        /// 11. `[writable]` margin market vault
        /// 12. `[]` oracle
        /// 13. `[writable]` user token account
        /// 14. `[]` token program
        /// 15. `[]` system program
        /// 16. `[writable]` lp
        /// 17. `[signer, writable]` authority
        AdminAddLpShares {
            amount: i64,
            margin_index: u32,
            market_index: u32,
            lower_rate: u64,
            upper_rate: u64,
        },
        /// Admin Add Margin
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        /// 4. `[writable]` margin market
        /// 5. `[writable]` user token account
        /// 6. `[writable]` margin market vault
        /// 7. `[]` token program
        AdminAddMargin { amount: i64 },
        /// Admin Transfer Margin
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[]` margin market
        /// 3. `[writable]` user token account
        /// 4. `[writable]` margin market vault
        /// 5. `[]` token program
        AdminTransferMargin { amount: u64 },
        /// Begin Vault Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` state
        /// 1. `[signer]` authority
        /// 2. `[writable]` in margin market vault
        /// 3. `[writable]` out margin market vault
        /// 4. `[writable]` in margin market
        /// 5. `[writable]` out margin market
        /// 6. `[writable]` yield market
        /// 7. `[]` oracle
        /// 8. `[writable]` in user token account
        /// 9. `[writable]` out user token account
        /// 10. `[]` token program
        /// 11. `[]` instructions
        BeginVaultSwap { amount: u64, other_amount_threshold: u64, is_exact_in: bool },
        /// Calculate Earn Invest
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` earn vault
        /// 1. `[writable]` state
        /// 2. `[signer, writable]` authority
        /// 3. `[]` token program
        /// 4. `[writable]` yield market
        /// 5. `[writable]` margin market
        /// 6. `[writable]` margin market vault
        /// 7. `[]` oracle
        /// 8. `[writable]` observation state
        /// 9. `[writable]` token owner account base
        /// 10. `[writable]` token vault base
        /// 11. `[writable]` token owner account quote
        /// 12. `[writable]` token vault quote
        /// 13. `[writable]` pt mint
        /// 14. `[]` system program
        CalculateEarnInvest { margin_amount: u64 },
        /// Calculate Implied Rate
        ///
        CalculateImpliedRate { maturity: u64, sqrt_price_x64: u128 },
        /// Calculate Lp Remove Max Ratio
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` yield market
        /// 2. `[]` observation state
        /// 3. `[writable]` lp
        /// 4. `[writable]` token vault base
        /// 5. `[writable]` token vault quote
        /// 6. `[writable]` tick array lower
        /// 7. `[writable]` tick array upper
        /// 8. `[writable]` token owner account base
        /// 9. `[writable]` token owner account quote
        /// 10. `[]` token program
        CalculateLpRemoveMaxRatio { sqrt_price_limit: u128 },
        /// Calculate Lp Sloss
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[]` lp
        CalculateLpSloss { rm_liquidity_percent: u64 },
        /// Calculate Lp Value
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[]` lp
        CalculateLpValue,
        /// Calculate Margin Value
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` user
        CalculateMarginValue,
        /// Calculate Position Value
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` state
        /// 1. `[]` user
        CalculatePositionValue,
        /// Calculate Pt Price
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[]` earn vault
        /// 2. `[]` oracle
        CalculatePtPrice,
        /// Calculate Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        CalculateSwap {
            amount: u64,
            a_to_b: bool,
            amount_specified_is_input: bool,
            sqrt_price_limit: u128,
            skip_standardize: bool,
        },
        /// Calculate Swap V2
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[]` observation state
        CalculateSwapV2 {
            amount: u64,
            a_to_b: bool,
            amount_specified_is_input: bool,
            sqrt_price_limit: u128,
            skip_standardize: bool,
        },
        /// Calculate Tick Index
        ///
        CalculateTickIndex {
            maturity: u64,
            implied_rate: u64,
            tick_spacing: i32,
            is_lower: bool,
        },
        /// Calculate Trader Pnl
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[]` user
        CalculateTraderPnl,
        /// Cancel Isolated Order
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` margin market vault
        /// 2. `[]` token program
        /// 3. `[signer]` authority
        /// 4. `[]` system program
        CancelIsolatedOrder { order_id: u32 },
        /// Cancel Order
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` user
        /// 2. `[signer]` authority
        CancelOrder { order_id: u32 },
        /// Claim Insurance
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        /// 4. `[writable]` margin market
        /// 5. `[writable]` token owner account
        /// 6. `[writable]` token vault margin
        /// 7. `[]` token program
        ClaimInsurance,
        /// Claim Keeper Fee
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` authority
        /// 1. `[writable]` state
        ClaimKeeperFee,
        /// Claim Yield
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` user
        /// 2. `[signer]` authority
        /// 3. `[writable]` margin market
        /// 4. `[writable]` margin market vault
        /// 5. `[writable]` user token account
        /// 6. `[]` token program
        ClaimYield { market_index: u32, amount: i64 },
        /// Collect Earn Fee
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` earn vault
        /// 1. `[writable]` yield market
        /// 2. `[writable]` margin market
        /// 3. `[writable]` margin market vault
        /// 4. `[writable]` user token account
        /// 5. `[]` oracle
        /// 6. `[signer]` admin
        /// 7. `[writable]` state
        /// 8. `[]` token program
        CollectEarnFee,
        /// Collect Fees
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[]` margin market
        /// 2. `[]` state
        /// 3. `[signer]` authority
        /// 4. `[]` oracle
        /// 5. `[writable]` lp
        /// 6. `[writable]` token owner account
        /// 7. `[writable]` token vault margin
        /// 8. `[]` token program
        CollectFees,
        /// Collect Protocol Fees
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` state
        /// 1. `[]` margin market
        /// 2. `[]` ammpools config
        /// 3. `[signer]` collect protocol fees authority
        /// 4. `[writable]` yield market
        /// 5. `[]` oracle
        /// 6. `[writable]` token vault margin
        /// 7. `[writable]` token destination
        /// 8. `[]` token program
        CollectProtocolFees,
        /// Delete Lp
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` lp
        /// 1. `[writable]` user stats
        /// 2. `[writable]` state
        /// 3. `[signer, writable]` payer
        /// 4. `[writable]` authority
        DeleteLp,
        /// Delete Tick Array
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[signer, writable]` authority
        /// 2. `[]` state
        /// 3. `[writable]` tick array
        DeleteTickArray,
        /// Delete User
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` user
        /// 1. `[writable]` user stats
        /// 2. `[writable]` state
        /// 3. `[signer, writable]` payer
        /// 4. `[writable]` authority
        DeleteUser,
        /// Deposit
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` user
        /// 1. `[signer, writable]` authority
        /// 2. `[writable]` state
        /// 3. `[writable]` margin market
        /// 4. `[writable]` margin market vault
        /// 5. `[writable]` user token account
        /// 6. `[]` token program
        Deposit { amount: i64 },
        /// Earn Invest
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` earn vault
        /// 1. `[writable]` state
        /// 2. `[signer, writable]` authority
        /// 3. `[]` token program
        /// 4. `[writable]` fee vault
        /// 5. `[writable]` yield market
        /// 6. `[writable]` margin market
        /// 7. `[writable]` margin market vault
        /// 8. `[writable]` user token account
        /// 9. `[writable]` user fee account
        /// 10. `[]` oracle
        /// 11. `[writable]` observation state
        /// 12. `[writable]` token owner account base
        /// 13. `[writable]` token vault base
        /// 14. `[writable]` token owner account quote
        /// 15. `[writable]` token vault quote
        /// 16. `[writable]` pt token account
        /// 17. `[writable]` pt mint
        /// 18. `[]` associated token program
        /// 19. `[]` system program
        EarnInvest { amount: u64 },
        /// Earn Redeem
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` earn vault
        /// 1. `[writable]` state
        /// 2. `[signer, writable]` authority
        /// 3. `[]` token program
        /// 4. `[writable]` fee vault
        /// 5. `[writable]` yield market
        /// 6. `[writable]` margin market
        /// 7. `[writable]` margin market vault
        /// 8. `[writable]` user token account
        /// 9. `[writable]` user fee account
        /// 10. `[]` oracle
        /// 11. `[writable]` observation state
        /// 12. `[writable]` token owner account base
        /// 13. `[writable]` token vault base
        /// 14. `[writable]` token owner account quote
        /// 15. `[writable]` token vault quote
        /// 16. `[writable]` pt token account
        /// 17. `[writable]` pt mint
        /// 18. `[]` associated token program
        /// 19. `[]` system program
        EarnRedeem { amount: u64, sqrt_price_limit: u128 },
        /// End Vault Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` state
        /// 1. `[signer]` authority
        /// 2. `[writable]` in margin market vault
        /// 3. `[writable]` out margin market vault
        /// 4. `[writable]` in margin market
        /// 5. `[writable]` out margin market
        /// 6. `[writable]` yield market
        /// 7. `[]` oracle
        /// 8. `[writable]` in user token account
        /// 9. `[writable]` out user token account
        /// 10. `[]` token program
        /// 11. `[]` instructions
        EndVaultSwap { amount: u64, other_amount_threshold: u64, is_exact_in: bool },
        /// Epoch Update Add
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` authority
        /// 1. `[writable]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        /// 4. `[writable]` token vault base
        /// 5. `[writable]` token vault quote
        /// 6. `[writable]` token owner account base
        /// 7. `[writable]` token owner account quote
        /// 8. `[writable]` margin market
        /// 9. `[writable]` margin market vault
        /// 10. `[]` margin market mint
        /// 11. `[writable]` user token account
        /// 12. `[]` token program
        /// 13. `[]` associated token program
        /// 14. `[]` system program
        EpochUpdateAdd { market_index: u32, is_expired: bool },
        /// Epoch Update Begin
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        EpochUpdateBegin { is_expired: bool },
        /// Epoch Update Change Price
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        EpochUpdateChangePrice { is_expired: bool },
        /// Epoch Update End
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        /// 4. `[]` token program
        /// 5. `[writable]` margin market
        /// 6. `[writable]` margin market vault
        /// 7. `[]` margin market mint
        /// 8. `[writable]` user token account
        /// 9. `[]` associated token program
        /// 10. `[]` system program
        EpochUpdateEnd { is_expired: bool },
        /// Epoch Update Expiry Apply
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        EpochUpdateExpiryApply,
        /// Epoch Update Expiry Check
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        EpochUpdateExpiryCheck,
        /// Epoch Update Remove
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` authority
        /// 1. `[writable]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        /// 4. `[writable]` token vault base
        /// 5. `[writable]` token vault quote
        /// 6. `[writable]` token owner account base
        /// 7. `[writable]` token owner account quote
        /// 8. `[writable]` margin market
        /// 9. `[writable]` margin market vault
        /// 10. `[]` margin market mint
        /// 11. `[writable]` user token account
        /// 12. `[]` token program
        /// 13. `[]` associated token program
        /// 14. `[]` system program
        EpochUpdateRemove { market_index: u32, is_expired: bool },
        /// Fill Order
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` margin market vault
        /// 2. `[signer]` authority
        /// 3. `[]` token program
        /// 4. `[writable]` yield market
        /// 5. `[writable]` token owner account base
        /// 6. `[writable]` token vault base
        /// 7. `[writable]` token owner account quote
        /// 8. `[writable]` token vault quote
        /// 9. `[]` system program
        FillOrder { order_id: u32 },
        /// Get Amm Twap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[]` observation
        GetAmmTwap { seconds_ago: u32 },
        /// Initialize
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        /// 2. `[]` system program
        /// 3. `[]` token program
        Initialize { margin_index_start: u32, market_index_start: u32, keeper_fee: u64 },
        #[doc = concat!(
            " ",
            "Initializes a AmmpoolsConfig account that hosts info & authorities"
        )]
        #[doc = concat!(" ", "required to govern a set of Ammpools.")]
        #[doc = concat!(" ", "")]
        #[doc = concat!(" ", "### Parameters")]
        #[doc = concat!(
            " ",
            "- `fee_authority` - Authority authorized to initialize fee-tiers and set customs fees."
        )]
        #[doc = concat!(
            " ",
            "- `collect_protocol_fees_authority` - Authority authorized to collect protocol fees."
        )]
        #[doc = concat!(
            " ",
            "- `reward_emissions_super_authority` - Authority authorized to set reward authorities in pools."
        )]
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` config
        /// 1. `[signer, writable]` admin
        /// 2. `[]` state
        /// 3. `[]` system program
        InitializeConfig {
            fee_authority: ::solana_program::pubkey::Pubkey,
            collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
            reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
            default_protocol_fee_rate: u16,
        },
        /// Initialize Earn Vault
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` earn vault
        /// 1. `[writable]` margin market
        /// 2. `[writable]` yield market
        /// 3. `[writable]` pt mint
        /// 4. `[writable]` mint metadata
        /// 5. `[writable]` state
        /// 6. `[signer, writable]` admin
        /// 7. `[]` rent
        /// 8. `[]` token program
        /// 9. `[]` token metadata program
        /// 10. `[]` system program
        InitializeEarnVault { user_ratio: u64 },
        #[doc = concat!(
            " ",
            "Initializes a fee_tier account usable by Ammpools in a AmmpoolConfig space."
        )]
        #[doc = concat!(" ", "")]
        #[doc = concat!(" ", "### Authority")]
        #[doc = concat!(" ", "- \"fee_authority\" - Set authority in the AmmpoolConfig")]
        #[doc = concat!(" ", "")]
        #[doc = concat!(" ", "### Parameters")]
        #[doc = concat!(
            " ",
            "- `tick_spacing` - The tick-spacing that this fee-tier suggests the default_fee_rate for."
        )]
        #[doc = concat!(
            " ",
            "- `default_fee_rate` - The default fee rate that a pool will use if the pool uses this"
        )]
        #[doc = concat!(" ", "fee tier during initialization.")]
        #[doc = concat!(" ", "")]
        #[doc = concat!(" ", "#### Special Errors")]
        #[doc = concat!(
            " ",
            "- `FeeRateMaxExceeded` - If the provided default_fee_rate exceeds MAX_FEE_RATE."
        )]
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` config
        /// 1. `[writable]` fee tier
        /// 2. `[signer, writable]` funder
        /// 3. `[signer]` fee authority
        /// 4. `[]` system program
        InitializeFeeTier { tick_spacing: u16, default_fee_rate: u16 },
        /// Initialize Lp
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` lp
        /// 1. `[writable]` user stats
        /// 2. `[writable]` state
        /// 3. `[signer]` authority
        /// 4. `[signer, writable]` payer
        /// 5. `[]` rent
        /// 6. `[]` system program
        InitializeLp { sub_account_id: u16 },
        /// Initialize Margin Market
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` margin market
        /// 2. `[]` margin market mint
        /// 3. `[writable]` margin market vault
        /// 4. `[writable]` state
        /// 5. `[]` system program
        /// 6. `[]` token program
        InitializeMarginMarket { name: [u8; 32usize] },
        /// Initialize Oracle
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` oracle
        /// 2. `[]` state
        /// 3. `[]` system program
        InitializeOracle {
            name: [u8; 32usize],
            market_rate: u64,
            rate: u64,
            last_rate: u64,
            epoch_start_timestamp: i64,
            decimals: u32,
        },
        #[doc = concat!(
            " ",
            "Initializes a tick_array account to represent a tick-range in a Ammpool."
        )]
        #[doc = concat!(" ", "")]
        #[doc = concat!(" ", "### Parameters")]
        #[doc = concat!(
            " ",
            "- `start_tick_index` - The starting tick index for this tick-array."
        )]
        #[doc = concat!(
            " ",
            "Has to be a multiple of TickArray size & the tick spacing of this pool."
        )]
        #[doc = concat!(" ", "")]
        #[doc = concat!(" ", "#### Special Errors")]
        #[doc = concat!(
            " ",
            "- `InvalidStartTick` - if the provided start tick is out of bounds or is not a multiple of"
        )]
        #[doc = concat!(" ", "TICK_ARRAY_SIZE * tick spacing.")]
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[signer, writable]` funder
        /// 2. `[writable]` tick array
        /// 3. `[]` system program
        InitializeTickArray { start_tick_index: i32 },
        /// Initialize User
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` user
        /// 1. `[writable]` user stats
        /// 2. `[writable]` state
        /// 3. `[signer]` authority
        /// 4. `[signer, writable]` payer
        /// 5. `[]` rent
        /// 6. `[]` system program
        InitializeUser { sub_account_id: u16, is_isolated: bool },
        /// Initialize User Stats
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` user stats
        /// 1. `[writable]` state
        /// 2. `[signer]` authority
        /// 3. `[signer, writable]` payer
        /// 4. `[]` rent
        /// 5. `[]` system program
        InitializeUserStats,
        /// Initialize Yield Market
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` margin market
        /// 4. `[]` lp margin market
        /// 5. `[]` oracle
        /// 6. `[writable]` base asset mint
        /// 7. `[writable]` quote asset mint
        /// 8. `[writable]` base asset vault
        /// 9. `[writable]` quote asset vault
        /// 10. `[writable]` token vault base
        /// 11. `[writable]` token vault quote
        /// 12. `[]` token program
        /// 13. `[]` ammpools config
        /// 14. `[]` fee tier
        /// 15. `[writable]` observation state
        /// 16. `[]` rent
        /// 17. `[]` system program
        InitializeYieldMarket {
            tick_spacing: u16,
            sqrt_price: u128,
            order_step_size: u64,
            min_order_size: u64,
            min_liquidation_size: u64,
            start_ts: i64,
            expire_ts: i64,
            active_ratio_coef: u64,
            margin_type: MarginType,
            lp_margin_type: MarginType,
            min_lp_amount: u64,
            lower_rate_bound: u64,
            upper_rate_bound: u64,
            bound_percentage: u64,
            market_type: MarketType,
            name: [u8; 32usize],
        },
        /// Initialize Yield Market Token Account A
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        /// 2. `[writable]` base asset mint
        /// 3. `[writable]` quote asset mint
        /// 4. `[writable]` base asset vault
        /// 5. `[]` token program
        /// 6. `[]` rent
        /// 7. `[]` system program
        InitializeYieldMarketTokenAccountA {
            yield_market: ::solana_program::pubkey::Pubkey,
        },
        /// Initialize Yield Market Token Account Aa
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        /// 2. `[writable]` base asset mint
        /// 3. `[writable]` quote asset mint
        /// 4. `[writable]` quote asset vault
        /// 5. `[]` token program
        /// 6. `[]` rent
        /// 7. `[]` system program
        InitializeYieldMarketTokenAccountAa {
            yield_market: ::solana_program::pubkey::Pubkey,
        },
        /// Initialize Yield Market Token Account B
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        /// 2. `[writable]` base asset mint
        /// 3. `[writable]` quote asset mint
        /// 4. `[writable]` token vault base
        /// 5. `[]` token program
        /// 6. `[]` rent
        /// 7. `[]` system program
        InitializeYieldMarketTokenAccountB {
            yield_market: ::solana_program::pubkey::Pubkey,
        },
        /// Initialize Yield Market Token Account Bb
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        /// 2. `[writable]` base asset mint
        /// 3. `[writable]` quote asset mint
        /// 4. `[writable]` token vault quote
        /// 5. `[]` token program
        /// 6. `[]` rent
        /// 7. `[]` system program
        InitializeYieldMarketTokenAccountBb {
            yield_market: ::solana_program::pubkey::Pubkey,
        },
        /// Liquidate
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` user
        /// 1. `[writable]` state
        /// 2. `[signer]` authority
        Liquidate,
        /// Liquidate Insurance
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[signer]` authority
        /// 2. `[writable]` yield market
        /// 3. `[writable]` margin market
        /// 4. `[writable]` margin market vault
        /// 5. `[]` oracle
        /// 6. `[]` observation
        /// 7. `[]` token program
        /// 8. `[]` system program
        LiquidateInsurance { market_index: u32, adl_finish: bool },
        /// Liquidate Lp
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` lp
        /// 1. `[writable]` state
        /// 2. `[writable]` token vault base
        /// 3. `[writable]` token vault quote
        /// 4. `[writable]` tick array lower
        /// 5. `[writable]` tick array upper
        /// 6. `[writable]` yield market
        /// 7. `[writable]` token owner account base
        /// 8. `[writable]` token owner account quote
        /// 9. `[]` token mint base
        /// 10. `[]` token mint quote
        /// 11. `[]` oracle
        /// 12. `[writable]` observation state
        /// 13. `[]` token program
        /// 14. `[signer]` authority
        LiquidateLp,
        /// Load Observation State
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        LoadObservationState,
        /// Multi Sig Deposit
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` user
        /// 1. `[signer, writable]` authority
        /// 2. `[signer, writable]` admin
        /// 3. `[writable]` state
        /// 4. `[writable]` margin market
        /// 5. `[writable]` margin market vault
        /// 6. `[writable]` user token account
        /// 7. `[]` token program
        MultiSigDeposit { amount: i64 },
        /// Observe
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` yield market
        /// 1. `[]` observation
        Observe { seconds_agos: Vec<u32> },
        /// Place Order
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` user
        /// 2. `[signer]` authority
        /// 3. `[]` system program
        PlaceOrder { params: OrderParams },
        /// Remove Keeper
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[writable]` state
        RemoveKeeper { remove_keeper: ::solana_program::pubkey::Pubkey },
        /// Remove Lp Shares
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[signer, writable]` authority
        /// 2. `[writable]` lp
        /// 3. `[writable]` token vault base
        /// 4. `[writable]` token vault quote
        /// 5. `[writable]` tick array lower
        /// 6. `[writable]` tick array upper
        /// 7. `[writable]` yield market
        /// 8. `[writable]` token owner account base
        /// 9. `[writable]` token owner account quote
        /// 10. `[]` token mint base
        /// 11. `[]` token mint quote
        /// 12. `[writable]` margin market
        /// 13. `[writable]` margin market vault
        /// 14. `[]` oracle
        /// 15. `[writable]` user token account
        /// 16. `[writable]` observation state
        /// 17. `[]` token program
        /// 18. `[]` system program
        RemoveLpShares { rm_liquidity_percent: u64, sqrt_price_limit: u128 },
        /// Rollback Oracle
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` oracle
        /// 2. `[]` state
        RollbackOracle {
            market_rate: u64,
            rate: u64,
            last_rate: u64,
            epoch_start_timestamp: i64,
            last_epoch_start_timestamp: i64,
        },
        /// Set Collateral Ratio
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        SetCollateralRatio {
            collateral_ratio_initial: i64,
            collateral_ratio_maintenance: i64,
            collateral_ratio_initial_pre_expiry: i64,
        },
        /// Set Keeper Fee
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        SetKeeperFee { keeper_fee_per_tx: u64 },
        /// Set Twap Duration
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` state
        SetTwapDuration { twap_duration: u32 },
        /// Settle Expiry User
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` authority
        /// 1. `[writable]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        SettleExpiryUser,
        /// Transfer Base Token
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` token vault base
        /// 3. `[writable]` token owner account base
        /// 4. `[]` token mint base
        /// 5. `[]` token program
        TransferBaseToken { delta_a: u64 },
        /// Transfer Quote Token
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` token vault quote
        /// 3. `[writable]` token owner account quote
        /// 4. `[]` token mint quote
        /// 5. `[]` token program
        TransferQuoteToken { delta_b: u64 },
        /// Update Fees And Rewards
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` yield market
        /// 1. `[signer]` authority
        /// 2. `[writable]` lp
        /// 3. `[]` tick array lower
        /// 4. `[]` tick array upper
        UpdateFeesAndRewards,
        /// Update Oracle
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` admin
        /// 1. `[writable]` oracle
        /// 2. `[]` state
        UpdateOracle {
            market_rate: u64,
            rate: u64,
            last_rate: u64,
            epoch_start_timestamp: i64,
        },
        /// Update Spot Yield Market Collateral Ratio
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateSpotYieldMarketCollateralRatio,
        /// Update Tick Liquidity
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[]` yield market
        /// 3. `[writable]` tick array
        UpdateTickLiquidity { tick_index: i32, new_liquidity: u128 },
        /// Update User Position
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` user
        UpdateUserPosition { base_asset_amount: i64, quote_asset_amount: i64 },
        /// Update Yield Market
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarket { number_of_active_lps: u64 },
        /// Update Yield Market Active Ratio Coef
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketActiveRatioCoef { active_ratio_coef: u64 },
        /// Update Yield Market Collateral Ratio Initial Pre Expiry
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketCollateralRatioInitialPreExpiry {
            collateral_ratio_initial_pre_expiry: i64,
        },
        /// Update Yield Market Collateral Ratio Maintenance
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketCollateralRatioMaintenance {
            collateral_ratio_maintenance: i64,
        },
        /// Update Yield Market Expire Total Pos Quote Amount
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketExpireTotalPosQuoteAmount {
            expire_total_pos_quote_amount: i64,
        },
        /// Update Yield Market Expire Ts
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketExpireTs { expire_ts: i64 },
        /// Update Yield Market Fee Rate
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketFeeRate { fee_rate: u16 },
        /// Update Yield Market Insurance
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketInsurance { base_asset_amount: i64, quote_asset_amount: i64 },
        /// Update Yield Market Keeper Fee
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketKeeperFee { keeper_fee: i64 },
        /// Update Yield Market Liq Fee Rate
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketLiqFeeRate { liq_fee_rate: i64 },
        /// Update Yield Market Lower Upper Rate Bound
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketLowerUpperRateBound {
            lower_rate_bound: u64,
            upper_rate_bound: u64,
        },
        /// Update Yield Market Lp Accounts Processed
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketLpAccountsProcessed { lp_accounts_processed: u64 },
        /// Update Yield Market Margin Decimals And Lp Margin Decimals
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketMarginDecimalsAndLpMarginDecimals {
            margin_decimals: u8,
            lp_margin_decimals: u8,
        },
        /// Update Yield Market Min Lp Amount
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketMinLpAmount { min_lp_amount: u64 },
        /// Update Yield Market Min Order Size
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketMinOrderSize { min_order_size: u64 },
        /// Update Yield Market Net Base Amount
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketNetBaseAmount { net_base_amount: i64 },
        /// Update Yield Market Net Quote Amount
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketNetQuoteAmount { net_quote_amount: i64 },
        /// Update Yield Market Number Of Active Users
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketNumberOfActiveUsers { number_of_active_users: u64 },
        /// Update Yield Market Number Of Processed Users
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketNumberOfProcessedUsers { number_of_processed_users: u64 },
        /// Update Yield Market Oracle
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketOracle,
        /// Update Yield Market Order Step Size
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketOrderStepSize { order_step_size: u64 },
        /// Update Yield Market Pool Liquidity
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketPoolLiquidity { liquidity: u128 },
        /// Update Yield Market Pt Data
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` mint metadata
        /// 3. `[]` token metadata program
        UpdateYieldMarketPtData {
            name: [u8; 32usize],
            symbol: [u8; 10usize],
            uri: [u8; 200usize],
        },
        /// Update Yield Market Social Loss
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketSocialLoss { base_asset_amount: i64, quote_asset_amount: i64 },
        /// Update Yield Market Start Ts
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketStartTs { start_ts: i64 },
        /// Update Yield Market Status
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketStatus { status: MarketStatus },
        /// Update Yield Market Tick Index
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` authority
        /// 1. `[writable]` state
        /// 2. `[writable]` yield market
        UpdateYieldMarketTickIndex { tick_lower_index: i32, tick_upper_index: i32 },
        /// Update Yield Market Total Reserve Quote And Base
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` admin
        /// 1. `[]` state
        /// 2. `[writable]` yield market
        /// 3. `[]` oracle
        UpdateYieldMarketTotalReserveQuoteAndBase {
            total_reserve_quote_amount: i64,
            total_reserve_base_amount: i64,
        },
        /// Vault Transfer
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` yield market
        /// 1. `[writable]` earn vault
        /// 2. `[writable]` lp margin market
        /// 3. `[writable]` other margin market
        /// 4. `[writable]` lp vault
        /// 5. `[writable]` other vault
        /// 6. `[signer, writable]` authority
        /// 7. `[]` state
        /// 8. `[]` oracle
        /// 9. `[]` token program
        VaultTransfer { from_lp_amount: i64, is_earn: bool },
        /// Withdraw
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` state
        /// 1. `[writable]` user
        /// 2. `[signer]` authority
        /// 3. `[writable]` margin market
        /// 4. `[writable]` margin market vault
        /// 5. `[writable]` user token account
        /// 6. `[]` token program
        Withdraw { amount: i64 },
    }
    impl RatexContractsInstruction {
        pub fn discriminator(&self) -> &'static [u8; 8] {
            match self {
                Self::AddKeeper { .. } => {
                    &[73u8, 181u8, 232u8, 2u8, 99u8, 47u8, 150u8, 179u8]
                }
                Self::AddLpShares { .. } => {
                    &[210u8, 205u8, 121u8, 221u8, 202u8, 167u8, 59u8, 191u8]
                }
                Self::AdminAddLpShares { .. } => {
                    &[5u8, 100u8, 225u8, 180u8, 252u8, 120u8, 238u8, 254u8]
                }
                Self::AdminAddMargin { .. } => {
                    &[198u8, 233u8, 89u8, 6u8, 152u8, 177u8, 250u8, 99u8]
                }
                Self::AdminTransferMargin { .. } => {
                    &[36u8, 42u8, 110u8, 72u8, 201u8, 72u8, 162u8, 44u8]
                }
                Self::BeginVaultSwap { .. } => {
                    &[23u8, 90u8, 16u8, 187u8, 221u8, 166u8, 33u8, 66u8]
                }
                Self::CalculateEarnInvest { .. } => {
                    &[179u8, 69u8, 22u8, 166u8, 5u8, 227u8, 175u8, 62u8]
                }
                Self::CalculateImpliedRate { .. } => {
                    &[28u8, 11u8, 110u8, 76u8, 206u8, 246u8, 115u8, 187u8]
                }
                Self::CalculateLpRemoveMaxRatio { .. } => {
                    &[179u8, 139u8, 95u8, 231u8, 232u8, 247u8, 22u8, 214u8]
                }
                Self::CalculateLpSloss { .. } => {
                    &[252u8, 122u8, 146u8, 236u8, 64u8, 109u8, 89u8, 97u8]
                }
                Self::CalculateLpValue => {
                    &[196u8, 205u8, 9u8, 32u8, 168u8, 164u8, 116u8, 101u8]
                }
                Self::CalculateMarginValue => {
                    &[124u8, 69u8, 9u8, 16u8, 66u8, 212u8, 29u8, 66u8]
                }
                Self::CalculatePositionValue => {
                    &[94u8, 181u8, 155u8, 245u8, 115u8, 194u8, 78u8, 236u8]
                }
                Self::CalculatePtPrice => {
                    &[244u8, 35u8, 167u8, 21u8, 93u8, 179u8, 102u8, 235u8]
                }
                Self::CalculateSwap { .. } => {
                    &[187u8, 165u8, 113u8, 222u8, 104u8, 2u8, 79u8, 219u8]
                }
                Self::CalculateSwapV2 { .. } => {
                    &[237u8, 14u8, 233u8, 63u8, 73u8, 144u8, 161u8, 163u8]
                }
                Self::CalculateTickIndex { .. } => {
                    &[10u8, 28u8, 137u8, 105u8, 211u8, 252u8, 38u8, 203u8]
                }
                Self::CalculateTraderPnl => {
                    &[172u8, 2u8, 209u8, 230u8, 49u8, 245u8, 183u8, 199u8]
                }
                Self::CancelIsolatedOrder { .. } => {
                    &[204u8, 114u8, 199u8, 244u8, 169u8, 90u8, 175u8, 233u8]
                }
                Self::CancelOrder { .. } => {
                    &[95u8, 129u8, 237u8, 240u8, 8u8, 49u8, 223u8, 132u8]
                }
                Self::ClaimInsurance => {
                    &[96u8, 254u8, 157u8, 145u8, 19u8, 96u8, 95u8, 55u8]
                }
                Self::ClaimKeeperFee => {
                    &[233u8, 182u8, 238u8, 12u8, 107u8, 123u8, 30u8, 161u8]
                }
                Self::ClaimYield { .. } => {
                    &[49u8, 74u8, 111u8, 7u8, 186u8, 22u8, 61u8, 165u8]
                }
                Self::CollectEarnFee => {
                    &[136u8, 35u8, 138u8, 164u8, 87u8, 2u8, 169u8, 213u8]
                }
                Self::CollectFees => {
                    &[164u8, 152u8, 207u8, 99u8, 30u8, 186u8, 19u8, 182u8]
                }
                Self::CollectProtocolFees => {
                    &[22u8, 67u8, 23u8, 98u8, 150u8, 178u8, 70u8, 220u8]
                }
                Self::DeleteLp => &[135u8, 33u8, 154u8, 68u8, 253u8, 179u8, 43u8, 87u8],
                Self::DeleteTickArray => {
                    &[173u8, 196u8, 196u8, 224u8, 43u8, 132u8, 3u8, 8u8]
                }
                Self::DeleteUser => {
                    &[186u8, 85u8, 17u8, 249u8, 219u8, 231u8, 98u8, 251u8]
                }
                Self::Deposit { .. } => {
                    &[242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8]
                }
                Self::EarnInvest { .. } => {
                    &[110u8, 63u8, 157u8, 138u8, 202u8, 152u8, 33u8, 172u8]
                }
                Self::EarnRedeem { .. } => {
                    &[93u8, 162u8, 58u8, 1u8, 75u8, 18u8, 212u8, 66u8]
                }
                Self::EndVaultSwap { .. } => {
                    &[116u8, 254u8, 32u8, 14u8, 145u8, 222u8, 49u8, 2u8]
                }
                Self::EpochUpdateAdd { .. } => {
                    &[245u8, 55u8, 200u8, 120u8, 79u8, 247u8, 146u8, 137u8]
                }
                Self::EpochUpdateBegin { .. } => {
                    &[91u8, 166u8, 232u8, 37u8, 88u8, 175u8, 78u8, 243u8]
                }
                Self::EpochUpdateChangePrice { .. } => {
                    &[89u8, 16u8, 55u8, 172u8, 43u8, 74u8, 200u8, 11u8]
                }
                Self::EpochUpdateEnd { .. } => {
                    &[164u8, 158u8, 251u8, 170u8, 210u8, 146u8, 160u8, 208u8]
                }
                Self::EpochUpdateExpiryApply => {
                    &[44u8, 61u8, 49u8, 72u8, 123u8, 218u8, 79u8, 6u8]
                }
                Self::EpochUpdateExpiryCheck => {
                    &[99u8, 200u8, 237u8, 96u8, 245u8, 113u8, 213u8, 36u8]
                }
                Self::EpochUpdateRemove { .. } => {
                    &[116u8, 253u8, 6u8, 150u8, 75u8, 48u8, 176u8, 168u8]
                }
                Self::FillOrder { .. } => {
                    &[232u8, 122u8, 115u8, 25u8, 199u8, 143u8, 136u8, 162u8]
                }
                Self::GetAmmTwap { .. } => {
                    &[93u8, 161u8, 159u8, 51u8, 126u8, 140u8, 113u8, 177u8]
                }
                Self::Initialize { .. } => {
                    &[175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8]
                }
                Self::InitializeConfig { .. } => {
                    &[208u8, 127u8, 21u8, 1u8, 194u8, 190u8, 196u8, 70u8]
                }
                Self::InitializeEarnVault { .. } => {
                    &[250u8, 227u8, 213u8, 86u8, 49u8, 121u8, 53u8, 231u8]
                }
                Self::InitializeFeeTier { .. } => {
                    &[183u8, 74u8, 156u8, 160u8, 112u8, 2u8, 42u8, 30u8]
                }
                Self::InitializeLp { .. } => {
                    &[110u8, 252u8, 116u8, 251u8, 81u8, 191u8, 57u8, 96u8]
                }
                Self::InitializeMarginMarket { .. } => {
                    &[188u8, 92u8, 253u8, 198u8, 89u8, 179u8, 165u8, 83u8]
                }
                Self::InitializeOracle { .. } => {
                    &[144u8, 223u8, 131u8, 120u8, 196u8, 253u8, 181u8, 99u8]
                }
                Self::InitializeTickArray { .. } => {
                    &[11u8, 188u8, 193u8, 214u8, 141u8, 91u8, 149u8, 184u8]
                }
                Self::InitializeUser { .. } => {
                    &[111u8, 17u8, 185u8, 250u8, 60u8, 122u8, 38u8, 254u8]
                }
                Self::InitializeUserStats => {
                    &[254u8, 243u8, 72u8, 98u8, 251u8, 130u8, 168u8, 213u8]
                }
                Self::InitializeYieldMarket { .. } => {
                    &[108u8, 101u8, 35u8, 38u8, 98u8, 214u8, 231u8, 231u8]
                }
                Self::InitializeYieldMarketTokenAccountA { .. } => {
                    &[202u8, 175u8, 124u8, 235u8, 234u8, 83u8, 219u8, 218u8]
                }
                Self::InitializeYieldMarketTokenAccountAa { .. } => {
                    &[247u8, 19u8, 79u8, 46u8, 98u8, 38u8, 117u8, 103u8]
                }
                Self::InitializeYieldMarketTokenAccountB { .. } => {
                    &[87u8, 55u8, 154u8, 192u8, 99u8, 4u8, 14u8, 237u8]
                }
                Self::InitializeYieldMarketTokenAccountBb { .. } => {
                    &[216u8, 5u8, 249u8, 180u8, 174u8, 97u8, 76u8, 66u8]
                }
                Self::Liquidate => &[223u8, 179u8, 226u8, 125u8, 48u8, 46u8, 39u8, 74u8],
                Self::LiquidateInsurance { .. } => {
                    &[125u8, 56u8, 6u8, 102u8, 255u8, 77u8, 1u8, 47u8]
                }
                Self::LiquidateLp => &[184u8, 134u8, 42u8, 9u8, 9u8, 99u8, 65u8, 25u8],
                Self::LoadObservationState => {
                    &[189u8, 246u8, 203u8, 10u8, 242u8, 132u8, 96u8, 156u8]
                }
                Self::MultiSigDeposit { .. } => {
                    &[243u8, 249u8, 76u8, 39u8, 66u8, 11u8, 190u8, 229u8]
                }
                Self::Observe { .. } => {
                    &[204u8, 78u8, 178u8, 115u8, 194u8, 147u8, 65u8, 74u8]
                }
                Self::PlaceOrder { .. } => {
                    &[51u8, 194u8, 155u8, 175u8, 109u8, 130u8, 96u8, 106u8]
                }
                Self::RemoveKeeper { .. } => {
                    &[193u8, 167u8, 169u8, 215u8, 44u8, 36u8, 88u8, 247u8]
                }
                Self::RemoveLpShares { .. } => {
                    &[76u8, 225u8, 62u8, 212u8, 158u8, 149u8, 209u8, 84u8]
                }
                Self::RollbackOracle { .. } => {
                    &[229u8, 81u8, 205u8, 92u8, 162u8, 102u8, 13u8, 135u8]
                }
                Self::SetCollateralRatio { .. } => {
                    &[154u8, 202u8, 184u8, 203u8, 41u8, 180u8, 3u8, 3u8]
                }
                Self::SetKeeperFee { .. } => {
                    &[70u8, 15u8, 64u8, 136u8, 169u8, 67u8, 173u8, 168u8]
                }
                Self::SetTwapDuration { .. } => {
                    &[27u8, 9u8, 151u8, 243u8, 72u8, 166u8, 22u8, 226u8]
                }
                Self::SettleExpiryUser => {
                    &[20u8, 52u8, 112u8, 83u8, 224u8, 135u8, 171u8, 189u8]
                }
                Self::TransferBaseToken { .. } => {
                    &[143u8, 16u8, 107u8, 107u8, 245u8, 66u8, 255u8, 68u8]
                }
                Self::TransferQuoteToken { .. } => {
                    &[193u8, 247u8, 26u8, 0u8, 0u8, 46u8, 73u8, 80u8]
                }
                Self::UpdateFeesAndRewards => {
                    &[154u8, 230u8, 250u8, 13u8, 236u8, 209u8, 75u8, 223u8]
                }
                Self::UpdateOracle { .. } => {
                    &[112u8, 41u8, 209u8, 18u8, 248u8, 226u8, 252u8, 188u8]
                }
                Self::UpdateSpotYieldMarketCollateralRatio => {
                    &[207u8, 41u8, 41u8, 101u8, 217u8, 208u8, 246u8, 12u8]
                }
                Self::UpdateTickLiquidity { .. } => {
                    &[48u8, 5u8, 202u8, 102u8, 110u8, 60u8, 133u8, 49u8]
                }
                Self::UpdateUserPosition { .. } => {
                    &[55u8, 141u8, 157u8, 156u8, 105u8, 153u8, 183u8, 153u8]
                }
                Self::UpdateYieldMarket { .. } => {
                    &[104u8, 236u8, 82u8, 238u8, 56u8, 49u8, 132u8, 152u8]
                }
                Self::UpdateYieldMarketActiveRatioCoef { .. } => {
                    &[26u8, 248u8, 248u8, 245u8, 225u8, 104u8, 225u8, 118u8]
                }
                Self::UpdateYieldMarketCollateralRatioInitialPreExpiry { .. } => {
                    &[245u8, 255u8, 224u8, 236u8, 15u8, 173u8, 92u8, 250u8]
                }
                Self::UpdateYieldMarketCollateralRatioMaintenance { .. } => {
                    &[166u8, 76u8, 158u8, 8u8, 206u8, 18u8, 235u8, 199u8]
                }
                Self::UpdateYieldMarketExpireTotalPosQuoteAmount { .. } => {
                    &[120u8, 213u8, 112u8, 169u8, 145u8, 83u8, 143u8, 67u8]
                }
                Self::UpdateYieldMarketExpireTs { .. } => {
                    &[34u8, 253u8, 131u8, 199u8, 30u8, 185u8, 230u8, 184u8]
                }
                Self::UpdateYieldMarketFeeRate { .. } => {
                    &[24u8, 214u8, 193u8, 47u8, 93u8, 42u8, 23u8, 218u8]
                }
                Self::UpdateYieldMarketInsurance { .. } => {
                    &[140u8, 16u8, 164u8, 75u8, 14u8, 201u8, 179u8, 20u8]
                }
                Self::UpdateYieldMarketKeeperFee { .. } => {
                    &[60u8, 162u8, 17u8, 133u8, 29u8, 167u8, 164u8, 78u8]
                }
                Self::UpdateYieldMarketLiqFeeRate { .. } => {
                    &[12u8, 249u8, 151u8, 236u8, 81u8, 119u8, 111u8, 212u8]
                }
                Self::UpdateYieldMarketLowerUpperRateBound { .. } => {
                    &[97u8, 124u8, 168u8, 170u8, 119u8, 123u8, 216u8, 223u8]
                }
                Self::UpdateYieldMarketLpAccountsProcessed { .. } => {
                    &[107u8, 251u8, 36u8, 15u8, 100u8, 110u8, 246u8, 36u8]
                }
                Self::UpdateYieldMarketMarginDecimalsAndLpMarginDecimals { .. } => {
                    &[252u8, 173u8, 129u8, 34u8, 100u8, 195u8, 50u8, 248u8]
                }
                Self::UpdateYieldMarketMinLpAmount { .. } => {
                    &[53u8, 233u8, 252u8, 220u8, 209u8, 173u8, 5u8, 243u8]
                }
                Self::UpdateYieldMarketMinOrderSize { .. } => {
                    &[232u8, 8u8, 157u8, 233u8, 67u8, 254u8, 41u8, 69u8]
                }
                Self::UpdateYieldMarketNetBaseAmount { .. } => {
                    &[25u8, 67u8, 64u8, 221u8, 78u8, 132u8, 229u8, 152u8]
                }
                Self::UpdateYieldMarketNetQuoteAmount { .. } => {
                    &[124u8, 18u8, 201u8, 112u8, 55u8, 134u8, 176u8, 116u8]
                }
                Self::UpdateYieldMarketNumberOfActiveUsers { .. } => {
                    &[62u8, 9u8, 100u8, 169u8, 25u8, 112u8, 78u8, 130u8]
                }
                Self::UpdateYieldMarketNumberOfProcessedUsers { .. } => {
                    &[116u8, 170u8, 60u8, 63u8, 31u8, 207u8, 180u8, 242u8]
                }
                Self::UpdateYieldMarketOracle => {
                    &[184u8, 214u8, 121u8, 145u8, 119u8, 111u8, 171u8, 186u8]
                }
                Self::UpdateYieldMarketOrderStepSize { .. } => {
                    &[176u8, 26u8, 197u8, 18u8, 43u8, 13u8, 98u8, 4u8]
                }
                Self::UpdateYieldMarketPoolLiquidity { .. } => {
                    &[131u8, 180u8, 171u8, 119u8, 73u8, 129u8, 11u8, 199u8]
                }
                Self::UpdateYieldMarketPtData { .. } => {
                    &[131u8, 82u8, 138u8, 231u8, 215u8, 139u8, 53u8, 136u8]
                }
                Self::UpdateYieldMarketSocialLoss { .. } => {
                    &[188u8, 137u8, 141u8, 51u8, 72u8, 241u8, 194u8, 43u8]
                }
                Self::UpdateYieldMarketStartTs { .. } => {
                    &[64u8, 30u8, 124u8, 204u8, 254u8, 161u8, 237u8, 169u8]
                }
                Self::UpdateYieldMarketStatus { .. } => {
                    &[177u8, 3u8, 6u8, 192u8, 64u8, 227u8, 170u8, 23u8]
                }
                Self::UpdateYieldMarketTickIndex { .. } => {
                    &[253u8, 45u8, 187u8, 126u8, 152u8, 129u8, 67u8, 102u8]
                }
                Self::UpdateYieldMarketTotalReserveQuoteAndBase { .. } => {
                    &[139u8, 225u8, 156u8, 224u8, 232u8, 210u8, 173u8, 61u8]
                }
                Self::VaultTransfer { .. } => {
                    &[211u8, 125u8, 3u8, 105u8, 45u8, 33u8, 227u8, 214u8]
                }
                Self::Withdraw { .. } => {
                    &[183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8]
                }
            }
        }
        pub fn pack(self) -> Vec<u8> {
            let mut out = Vec::new();
            out.extend(self.discriminator());
            let data = ::borsh::to_vec(&self).unwrap();
            out.extend(data);
            out
        }
        pub fn unpack(data: &[u8]) -> ::std::io::Result<Self> {
            use ::borsh::BorshDeserialize;
            let (discriminator, mut ix_data) = data.split_at(8);
            Ok(
                match discriminator {
                    [73u8, 181u8, 232u8, 2u8, 99u8, 47u8, 150u8, 179u8] => {
                        AddKeeperDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [210u8, 205u8, 121u8, 221u8, 202u8, 167u8, 59u8, 191u8] => {
                        AddLpSharesDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [5u8, 100u8, 225u8, 180u8, 252u8, 120u8, 238u8, 254u8] => {
                        AdminAddLpSharesDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [198u8, 233u8, 89u8, 6u8, 152u8, 177u8, 250u8, 99u8] => {
                        AdminAddMarginDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [36u8, 42u8, 110u8, 72u8, 201u8, 72u8, 162u8, 44u8] => {
                        AdminTransferMarginDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [23u8, 90u8, 16u8, 187u8, 221u8, 166u8, 33u8, 66u8] => {
                        BeginVaultSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [179u8, 69u8, 22u8, 166u8, 5u8, 227u8, 175u8, 62u8] => {
                        CalculateEarnInvestDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [28u8, 11u8, 110u8, 76u8, 206u8, 246u8, 115u8, 187u8] => {
                        CalculateImpliedRateDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [179u8, 139u8, 95u8, 231u8, 232u8, 247u8, 22u8, 214u8] => {
                        CalculateLpRemoveMaxRatioDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [252u8, 122u8, 146u8, 236u8, 64u8, 109u8, 89u8, 97u8] => {
                        CalculateLpSlossDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [196u8, 205u8, 9u8, 32u8, 168u8, 164u8, 116u8, 101u8] => {
                        CalculateLpValueDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [124u8, 69u8, 9u8, 16u8, 66u8, 212u8, 29u8, 66u8] => {
                        CalculateMarginValueDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [94u8, 181u8, 155u8, 245u8, 115u8, 194u8, 78u8, 236u8] => {
                        CalculatePositionValueDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [244u8, 35u8, 167u8, 21u8, 93u8, 179u8, 102u8, 235u8] => {
                        CalculatePtPriceDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [187u8, 165u8, 113u8, 222u8, 104u8, 2u8, 79u8, 219u8] => {
                        CalculateSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [237u8, 14u8, 233u8, 63u8, 73u8, 144u8, 161u8, 163u8] => {
                        CalculateSwapV2Deserializer::deserialize(&mut ix_data)?.into()
                    }
                    [10u8, 28u8, 137u8, 105u8, 211u8, 252u8, 38u8, 203u8] => {
                        CalculateTickIndexDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [172u8, 2u8, 209u8, 230u8, 49u8, 245u8, 183u8, 199u8] => {
                        CalculateTraderPnlDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [204u8, 114u8, 199u8, 244u8, 169u8, 90u8, 175u8, 233u8] => {
                        CancelIsolatedOrderDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [95u8, 129u8, 237u8, 240u8, 8u8, 49u8, 223u8, 132u8] => {
                        CancelOrderDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [96u8, 254u8, 157u8, 145u8, 19u8, 96u8, 95u8, 55u8] => {
                        ClaimInsuranceDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [233u8, 182u8, 238u8, 12u8, 107u8, 123u8, 30u8, 161u8] => {
                        ClaimKeeperFeeDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [49u8, 74u8, 111u8, 7u8, 186u8, 22u8, 61u8, 165u8] => {
                        ClaimYieldDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [136u8, 35u8, 138u8, 164u8, 87u8, 2u8, 169u8, 213u8] => {
                        CollectEarnFeeDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [164u8, 152u8, 207u8, 99u8, 30u8, 186u8, 19u8, 182u8] => {
                        CollectFeesDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [22u8, 67u8, 23u8, 98u8, 150u8, 178u8, 70u8, 220u8] => {
                        CollectProtocolFeesDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [135u8, 33u8, 154u8, 68u8, 253u8, 179u8, 43u8, 87u8] => {
                        DeleteLpDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [173u8, 196u8, 196u8, 224u8, 43u8, 132u8, 3u8, 8u8] => {
                        DeleteTickArrayDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [186u8, 85u8, 17u8, 249u8, 219u8, 231u8, 98u8, 251u8] => {
                        DeleteUserDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                        DepositDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [110u8, 63u8, 157u8, 138u8, 202u8, 152u8, 33u8, 172u8] => {
                        EarnInvestDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [93u8, 162u8, 58u8, 1u8, 75u8, 18u8, 212u8, 66u8] => {
                        EarnRedeemDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [116u8, 254u8, 32u8, 14u8, 145u8, 222u8, 49u8, 2u8] => {
                        EndVaultSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [245u8, 55u8, 200u8, 120u8, 79u8, 247u8, 146u8, 137u8] => {
                        EpochUpdateAddDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [91u8, 166u8, 232u8, 37u8, 88u8, 175u8, 78u8, 243u8] => {
                        EpochUpdateBeginDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [89u8, 16u8, 55u8, 172u8, 43u8, 74u8, 200u8, 11u8] => {
                        EpochUpdateChangePriceDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [164u8, 158u8, 251u8, 170u8, 210u8, 146u8, 160u8, 208u8] => {
                        EpochUpdateEndDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [44u8, 61u8, 49u8, 72u8, 123u8, 218u8, 79u8, 6u8] => {
                        EpochUpdateExpiryApplyDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [99u8, 200u8, 237u8, 96u8, 245u8, 113u8, 213u8, 36u8] => {
                        EpochUpdateExpiryCheckDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [116u8, 253u8, 6u8, 150u8, 75u8, 48u8, 176u8, 168u8] => {
                        EpochUpdateRemoveDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [232u8, 122u8, 115u8, 25u8, 199u8, 143u8, 136u8, 162u8] => {
                        FillOrderDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [93u8, 161u8, 159u8, 51u8, 126u8, 140u8, 113u8, 177u8] => {
                        GetAmmTwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                        InitializeDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [208u8, 127u8, 21u8, 1u8, 194u8, 190u8, 196u8, 70u8] => {
                        InitializeConfigDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [250u8, 227u8, 213u8, 86u8, 49u8, 121u8, 53u8, 231u8] => {
                        InitializeEarnVaultDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [183u8, 74u8, 156u8, 160u8, 112u8, 2u8, 42u8, 30u8] => {
                        InitializeFeeTierDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [110u8, 252u8, 116u8, 251u8, 81u8, 191u8, 57u8, 96u8] => {
                        InitializeLpDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [188u8, 92u8, 253u8, 198u8, 89u8, 179u8, 165u8, 83u8] => {
                        InitializeMarginMarketDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [144u8, 223u8, 131u8, 120u8, 196u8, 253u8, 181u8, 99u8] => {
                        InitializeOracleDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [11u8, 188u8, 193u8, 214u8, 141u8, 91u8, 149u8, 184u8] => {
                        InitializeTickArrayDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [111u8, 17u8, 185u8, 250u8, 60u8, 122u8, 38u8, 254u8] => {
                        InitializeUserDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [254u8, 243u8, 72u8, 98u8, 251u8, 130u8, 168u8, 213u8] => {
                        InitializeUserStatsDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [108u8, 101u8, 35u8, 38u8, 98u8, 214u8, 231u8, 231u8] => {
                        InitializeYieldMarketDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [202u8, 175u8, 124u8, 235u8, 234u8, 83u8, 219u8, 218u8] => {
                        InitializeYieldMarketTokenAccountADeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [247u8, 19u8, 79u8, 46u8, 98u8, 38u8, 117u8, 103u8] => {
                        InitializeYieldMarketTokenAccountAaDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [87u8, 55u8, 154u8, 192u8, 99u8, 4u8, 14u8, 237u8] => {
                        InitializeYieldMarketTokenAccountBDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [216u8, 5u8, 249u8, 180u8, 174u8, 97u8, 76u8, 66u8] => {
                        InitializeYieldMarketTokenAccountBbDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [223u8, 179u8, 226u8, 125u8, 48u8, 46u8, 39u8, 74u8] => {
                        LiquidateDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [125u8, 56u8, 6u8, 102u8, 255u8, 77u8, 1u8, 47u8] => {
                        LiquidateInsuranceDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [184u8, 134u8, 42u8, 9u8, 9u8, 99u8, 65u8, 25u8] => {
                        LiquidateLpDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [189u8, 246u8, 203u8, 10u8, 242u8, 132u8, 96u8, 156u8] => {
                        LoadObservationStateDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [243u8, 249u8, 76u8, 39u8, 66u8, 11u8, 190u8, 229u8] => {
                        MultiSigDepositDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [204u8, 78u8, 178u8, 115u8, 194u8, 147u8, 65u8, 74u8] => {
                        ObserveDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [51u8, 194u8, 155u8, 175u8, 109u8, 130u8, 96u8, 106u8] => {
                        PlaceOrderDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [193u8, 167u8, 169u8, 215u8, 44u8, 36u8, 88u8, 247u8] => {
                        RemoveKeeperDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [76u8, 225u8, 62u8, 212u8, 158u8, 149u8, 209u8, 84u8] => {
                        RemoveLpSharesDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [229u8, 81u8, 205u8, 92u8, 162u8, 102u8, 13u8, 135u8] => {
                        RollbackOracleDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [154u8, 202u8, 184u8, 203u8, 41u8, 180u8, 3u8, 3u8] => {
                        SetCollateralRatioDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [70u8, 15u8, 64u8, 136u8, 169u8, 67u8, 173u8, 168u8] => {
                        SetKeeperFeeDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [27u8, 9u8, 151u8, 243u8, 72u8, 166u8, 22u8, 226u8] => {
                        SetTwapDurationDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [20u8, 52u8, 112u8, 83u8, 224u8, 135u8, 171u8, 189u8] => {
                        SettleExpiryUserDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [143u8, 16u8, 107u8, 107u8, 245u8, 66u8, 255u8, 68u8] => {
                        TransferBaseTokenDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [193u8, 247u8, 26u8, 0u8, 0u8, 46u8, 73u8, 80u8] => {
                        TransferQuoteTokenDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [154u8, 230u8, 250u8, 13u8, 236u8, 209u8, 75u8, 223u8] => {
                        UpdateFeesAndRewardsDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [112u8, 41u8, 209u8, 18u8, 248u8, 226u8, 252u8, 188u8] => {
                        UpdateOracleDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [207u8, 41u8, 41u8, 101u8, 217u8, 208u8, 246u8, 12u8] => {
                        UpdateSpotYieldMarketCollateralRatioDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [48u8, 5u8, 202u8, 102u8, 110u8, 60u8, 133u8, 49u8] => {
                        UpdateTickLiquidityDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [55u8, 141u8, 157u8, 156u8, 105u8, 153u8, 183u8, 153u8] => {
                        UpdateUserPositionDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [104u8, 236u8, 82u8, 238u8, 56u8, 49u8, 132u8, 152u8] => {
                        UpdateYieldMarketDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [26u8, 248u8, 248u8, 245u8, 225u8, 104u8, 225u8, 118u8] => {
                        UpdateYieldMarketActiveRatioCoefDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [245u8, 255u8, 224u8, 236u8, 15u8, 173u8, 92u8, 250u8] => {
                        UpdateYieldMarketCollateralRatioInitialPreExpiryDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [166u8, 76u8, 158u8, 8u8, 206u8, 18u8, 235u8, 199u8] => {
                        UpdateYieldMarketCollateralRatioMaintenanceDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [120u8, 213u8, 112u8, 169u8, 145u8, 83u8, 143u8, 67u8] => {
                        UpdateYieldMarketExpireTotalPosQuoteAmountDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [34u8, 253u8, 131u8, 199u8, 30u8, 185u8, 230u8, 184u8] => {
                        UpdateYieldMarketExpireTsDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [24u8, 214u8, 193u8, 47u8, 93u8, 42u8, 23u8, 218u8] => {
                        UpdateYieldMarketFeeRateDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [140u8, 16u8, 164u8, 75u8, 14u8, 201u8, 179u8, 20u8] => {
                        UpdateYieldMarketInsuranceDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [60u8, 162u8, 17u8, 133u8, 29u8, 167u8, 164u8, 78u8] => {
                        UpdateYieldMarketKeeperFeeDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [12u8, 249u8, 151u8, 236u8, 81u8, 119u8, 111u8, 212u8] => {
                        UpdateYieldMarketLiqFeeRateDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [97u8, 124u8, 168u8, 170u8, 119u8, 123u8, 216u8, 223u8] => {
                        UpdateYieldMarketLowerUpperRateBoundDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [107u8, 251u8, 36u8, 15u8, 100u8, 110u8, 246u8, 36u8] => {
                        UpdateYieldMarketLpAccountsProcessedDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [252u8, 173u8, 129u8, 34u8, 100u8, 195u8, 50u8, 248u8] => {
                        UpdateYieldMarketMarginDecimalsAndLpMarginDecimalsDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [53u8, 233u8, 252u8, 220u8, 209u8, 173u8, 5u8, 243u8] => {
                        UpdateYieldMarketMinLpAmountDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [232u8, 8u8, 157u8, 233u8, 67u8, 254u8, 41u8, 69u8] => {
                        UpdateYieldMarketMinOrderSizeDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [25u8, 67u8, 64u8, 221u8, 78u8, 132u8, 229u8, 152u8] => {
                        UpdateYieldMarketNetBaseAmountDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [124u8, 18u8, 201u8, 112u8, 55u8, 134u8, 176u8, 116u8] => {
                        UpdateYieldMarketNetQuoteAmountDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [62u8, 9u8, 100u8, 169u8, 25u8, 112u8, 78u8, 130u8] => {
                        UpdateYieldMarketNumberOfActiveUsersDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [116u8, 170u8, 60u8, 63u8, 31u8, 207u8, 180u8, 242u8] => {
                        UpdateYieldMarketNumberOfProcessedUsersDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [184u8, 214u8, 121u8, 145u8, 119u8, 111u8, 171u8, 186u8] => {
                        UpdateYieldMarketOracleDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [176u8, 26u8, 197u8, 18u8, 43u8, 13u8, 98u8, 4u8] => {
                        UpdateYieldMarketOrderStepSizeDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [131u8, 180u8, 171u8, 119u8, 73u8, 129u8, 11u8, 199u8] => {
                        UpdateYieldMarketPoolLiquidityDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [131u8, 82u8, 138u8, 231u8, 215u8, 139u8, 53u8, 136u8] => {
                        UpdateYieldMarketPtDataDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [188u8, 137u8, 141u8, 51u8, 72u8, 241u8, 194u8, 43u8] => {
                        UpdateYieldMarketSocialLossDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [64u8, 30u8, 124u8, 204u8, 254u8, 161u8, 237u8, 169u8] => {
                        UpdateYieldMarketStartTsDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [177u8, 3u8, 6u8, 192u8, 64u8, 227u8, 170u8, 23u8] => {
                        UpdateYieldMarketStatusDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [253u8, 45u8, 187u8, 126u8, 152u8, 129u8, 67u8, 102u8] => {
                        UpdateYieldMarketTickIndexDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [139u8, 225u8, 156u8, 224u8, 232u8, 210u8, 173u8, 61u8] => {
                        UpdateYieldMarketTotalReserveQuoteAndBaseDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [211u8, 125u8, 3u8, 105u8, 45u8, 33u8, 227u8, 214u8] => {
                        VaultTransferDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                        WithdrawDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    _ => {
                        return Err(
                            std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                "invalid discriminator",
                            ),
                        );
                    }
                },
            )
        }
    }
    impl ::borsh::BorshSerialize for RatexContractsInstruction {
        fn serialize<W: ::borsh::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), ::borsh::io::Error> {
            match self {
                Self::AddKeeper { new_keeper } => {
                    ::borsh::BorshSerialize::serialize(new_keeper, writer)?;
                }
                Self::AddLpShares {
                    amount,
                    margin_index,
                    market_index,
                    lower_rate,
                    upper_rate,
                } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                    ::borsh::BorshSerialize::serialize(margin_index, writer)?;
                    ::borsh::BorshSerialize::serialize(market_index, writer)?;
                    ::borsh::BorshSerialize::serialize(lower_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(upper_rate, writer)?;
                }
                Self::AdminAddLpShares {
                    amount,
                    margin_index,
                    market_index,
                    lower_rate,
                    upper_rate,
                } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                    ::borsh::BorshSerialize::serialize(margin_index, writer)?;
                    ::borsh::BorshSerialize::serialize(market_index, writer)?;
                    ::borsh::BorshSerialize::serialize(lower_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(upper_rate, writer)?;
                }
                Self::AdminAddMargin { amount } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                }
                Self::AdminTransferMargin { amount } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                }
                Self::BeginVaultSwap { amount, other_amount_threshold, is_exact_in } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                    ::borsh::BorshSerialize::serialize(other_amount_threshold, writer)?;
                    ::borsh::BorshSerialize::serialize(is_exact_in, writer)?;
                }
                Self::CalculateEarnInvest { margin_amount } => {
                    ::borsh::BorshSerialize::serialize(margin_amount, writer)?;
                }
                Self::CalculateImpliedRate { maturity, sqrt_price_x64 } => {
                    ::borsh::BorshSerialize::serialize(maturity, writer)?;
                    ::borsh::BorshSerialize::serialize(sqrt_price_x64, writer)?;
                }
                Self::CalculateLpRemoveMaxRatio { sqrt_price_limit } => {
                    ::borsh::BorshSerialize::serialize(sqrt_price_limit, writer)?;
                }
                Self::CalculateLpSloss { rm_liquidity_percent } => {
                    ::borsh::BorshSerialize::serialize(rm_liquidity_percent, writer)?;
                }
                Self::CalculateLpValue => {}
                Self::CalculateMarginValue => {}
                Self::CalculatePositionValue => {}
                Self::CalculatePtPrice => {}
                Self::CalculateSwap {
                    amount,
                    a_to_b,
                    amount_specified_is_input,
                    sqrt_price_limit,
                    skip_standardize,
                } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                    ::borsh::BorshSerialize::serialize(a_to_b, writer)?;
                    ::borsh::BorshSerialize::serialize(
                        amount_specified_is_input,
                        writer,
                    )?;
                    ::borsh::BorshSerialize::serialize(sqrt_price_limit, writer)?;
                    ::borsh::BorshSerialize::serialize(skip_standardize, writer)?;
                }
                Self::CalculateSwapV2 {
                    amount,
                    a_to_b,
                    amount_specified_is_input,
                    sqrt_price_limit,
                    skip_standardize,
                } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                    ::borsh::BorshSerialize::serialize(a_to_b, writer)?;
                    ::borsh::BorshSerialize::serialize(
                        amount_specified_is_input,
                        writer,
                    )?;
                    ::borsh::BorshSerialize::serialize(sqrt_price_limit, writer)?;
                    ::borsh::BorshSerialize::serialize(skip_standardize, writer)?;
                }
                Self::CalculateTickIndex {
                    maturity,
                    implied_rate,
                    tick_spacing,
                    is_lower,
                } => {
                    ::borsh::BorshSerialize::serialize(maturity, writer)?;
                    ::borsh::BorshSerialize::serialize(implied_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(tick_spacing, writer)?;
                    ::borsh::BorshSerialize::serialize(is_lower, writer)?;
                }
                Self::CalculateTraderPnl => {}
                Self::CancelIsolatedOrder { order_id } => {
                    ::borsh::BorshSerialize::serialize(order_id, writer)?;
                }
                Self::CancelOrder { order_id } => {
                    ::borsh::BorshSerialize::serialize(order_id, writer)?;
                }
                Self::ClaimInsurance => {}
                Self::ClaimKeeperFee => {}
                Self::ClaimYield { market_index, amount } => {
                    ::borsh::BorshSerialize::serialize(market_index, writer)?;
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                }
                Self::CollectEarnFee => {}
                Self::CollectFees => {}
                Self::CollectProtocolFees => {}
                Self::DeleteLp => {}
                Self::DeleteTickArray => {}
                Self::DeleteUser => {}
                Self::Deposit { amount } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                }
                Self::EarnInvest { amount } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                }
                Self::EarnRedeem { amount, sqrt_price_limit } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                    ::borsh::BorshSerialize::serialize(sqrt_price_limit, writer)?;
                }
                Self::EndVaultSwap { amount, other_amount_threshold, is_exact_in } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                    ::borsh::BorshSerialize::serialize(other_amount_threshold, writer)?;
                    ::borsh::BorshSerialize::serialize(is_exact_in, writer)?;
                }
                Self::EpochUpdateAdd { market_index, is_expired } => {
                    ::borsh::BorshSerialize::serialize(market_index, writer)?;
                    ::borsh::BorshSerialize::serialize(is_expired, writer)?;
                }
                Self::EpochUpdateBegin { is_expired } => {
                    ::borsh::BorshSerialize::serialize(is_expired, writer)?;
                }
                Self::EpochUpdateChangePrice { is_expired } => {
                    ::borsh::BorshSerialize::serialize(is_expired, writer)?;
                }
                Self::EpochUpdateEnd { is_expired } => {
                    ::borsh::BorshSerialize::serialize(is_expired, writer)?;
                }
                Self::EpochUpdateExpiryApply => {}
                Self::EpochUpdateExpiryCheck => {}
                Self::EpochUpdateRemove { market_index, is_expired } => {
                    ::borsh::BorshSerialize::serialize(market_index, writer)?;
                    ::borsh::BorshSerialize::serialize(is_expired, writer)?;
                }
                Self::FillOrder { order_id } => {
                    ::borsh::BorshSerialize::serialize(order_id, writer)?;
                }
                Self::GetAmmTwap { seconds_ago } => {
                    ::borsh::BorshSerialize::serialize(seconds_ago, writer)?;
                }
                Self::Initialize {
                    margin_index_start,
                    market_index_start,
                    keeper_fee,
                } => {
                    ::borsh::BorshSerialize::serialize(margin_index_start, writer)?;
                    ::borsh::BorshSerialize::serialize(market_index_start, writer)?;
                    ::borsh::BorshSerialize::serialize(keeper_fee, writer)?;
                }
                Self::InitializeConfig {
                    fee_authority,
                    collect_protocol_fees_authority,
                    reward_emissions_super_authority,
                    default_protocol_fee_rate,
                } => {
                    ::borsh::BorshSerialize::serialize(fee_authority, writer)?;
                    ::borsh::BorshSerialize::serialize(
                        collect_protocol_fees_authority,
                        writer,
                    )?;
                    ::borsh::BorshSerialize::serialize(
                        reward_emissions_super_authority,
                        writer,
                    )?;
                    ::borsh::BorshSerialize::serialize(
                        default_protocol_fee_rate,
                        writer,
                    )?;
                }
                Self::InitializeEarnVault { user_ratio } => {
                    ::borsh::BorshSerialize::serialize(user_ratio, writer)?;
                }
                Self::InitializeFeeTier { tick_spacing, default_fee_rate } => {
                    ::borsh::BorshSerialize::serialize(tick_spacing, writer)?;
                    ::borsh::BorshSerialize::serialize(default_fee_rate, writer)?;
                }
                Self::InitializeLp { sub_account_id } => {
                    ::borsh::BorshSerialize::serialize(sub_account_id, writer)?;
                }
                Self::InitializeMarginMarket { name } => {
                    ::borsh::BorshSerialize::serialize(name, writer)?;
                }
                Self::InitializeOracle {
                    name,
                    market_rate,
                    rate,
                    last_rate,
                    epoch_start_timestamp,
                    decimals,
                } => {
                    ::borsh::BorshSerialize::serialize(name, writer)?;
                    ::borsh::BorshSerialize::serialize(market_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(rate, writer)?;
                    ::borsh::BorshSerialize::serialize(last_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(epoch_start_timestamp, writer)?;
                    ::borsh::BorshSerialize::serialize(decimals, writer)?;
                }
                Self::InitializeTickArray { start_tick_index } => {
                    ::borsh::BorshSerialize::serialize(start_tick_index, writer)?;
                }
                Self::InitializeUser { sub_account_id, is_isolated } => {
                    ::borsh::BorshSerialize::serialize(sub_account_id, writer)?;
                    ::borsh::BorshSerialize::serialize(is_isolated, writer)?;
                }
                Self::InitializeUserStats => {}
                Self::InitializeYieldMarket {
                    tick_spacing,
                    sqrt_price,
                    order_step_size,
                    min_order_size,
                    min_liquidation_size,
                    start_ts,
                    expire_ts,
                    active_ratio_coef,
                    margin_type,
                    lp_margin_type,
                    min_lp_amount,
                    lower_rate_bound,
                    upper_rate_bound,
                    bound_percentage,
                    market_type,
                    name,
                } => {
                    ::borsh::BorshSerialize::serialize(tick_spacing, writer)?;
                    ::borsh::BorshSerialize::serialize(sqrt_price, writer)?;
                    ::borsh::BorshSerialize::serialize(order_step_size, writer)?;
                    ::borsh::BorshSerialize::serialize(min_order_size, writer)?;
                    ::borsh::BorshSerialize::serialize(min_liquidation_size, writer)?;
                    ::borsh::BorshSerialize::serialize(start_ts, writer)?;
                    ::borsh::BorshSerialize::serialize(expire_ts, writer)?;
                    ::borsh::BorshSerialize::serialize(active_ratio_coef, writer)?;
                    ::borsh::BorshSerialize::serialize(margin_type, writer)?;
                    ::borsh::BorshSerialize::serialize(lp_margin_type, writer)?;
                    ::borsh::BorshSerialize::serialize(min_lp_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(lower_rate_bound, writer)?;
                    ::borsh::BorshSerialize::serialize(upper_rate_bound, writer)?;
                    ::borsh::BorshSerialize::serialize(bound_percentage, writer)?;
                    ::borsh::BorshSerialize::serialize(market_type, writer)?;
                    ::borsh::BorshSerialize::serialize(name, writer)?;
                }
                Self::InitializeYieldMarketTokenAccountA { yield_market } => {
                    ::borsh::BorshSerialize::serialize(yield_market, writer)?;
                }
                Self::InitializeYieldMarketTokenAccountAa { yield_market } => {
                    ::borsh::BorshSerialize::serialize(yield_market, writer)?;
                }
                Self::InitializeYieldMarketTokenAccountB { yield_market } => {
                    ::borsh::BorshSerialize::serialize(yield_market, writer)?;
                }
                Self::InitializeYieldMarketTokenAccountBb { yield_market } => {
                    ::borsh::BorshSerialize::serialize(yield_market, writer)?;
                }
                Self::Liquidate => {}
                Self::LiquidateInsurance { market_index, adl_finish } => {
                    ::borsh::BorshSerialize::serialize(market_index, writer)?;
                    ::borsh::BorshSerialize::serialize(adl_finish, writer)?;
                }
                Self::LiquidateLp => {}
                Self::LoadObservationState => {}
                Self::MultiSigDeposit { amount } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                }
                Self::Observe { seconds_agos } => {
                    ::borsh::BorshSerialize::serialize(seconds_agos, writer)?;
                }
                Self::PlaceOrder { params } => {
                    ::borsh::BorshSerialize::serialize(params, writer)?;
                }
                Self::RemoveKeeper { remove_keeper } => {
                    ::borsh::BorshSerialize::serialize(remove_keeper, writer)?;
                }
                Self::RemoveLpShares { rm_liquidity_percent, sqrt_price_limit } => {
                    ::borsh::BorshSerialize::serialize(rm_liquidity_percent, writer)?;
                    ::borsh::BorshSerialize::serialize(sqrt_price_limit, writer)?;
                }
                Self::RollbackOracle {
                    market_rate,
                    rate,
                    last_rate,
                    epoch_start_timestamp,
                    last_epoch_start_timestamp,
                } => {
                    ::borsh::BorshSerialize::serialize(market_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(rate, writer)?;
                    ::borsh::BorshSerialize::serialize(last_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(epoch_start_timestamp, writer)?;
                    ::borsh::BorshSerialize::serialize(
                        last_epoch_start_timestamp,
                        writer,
                    )?;
                }
                Self::SetCollateralRatio {
                    collateral_ratio_initial,
                    collateral_ratio_maintenance,
                    collateral_ratio_initial_pre_expiry,
                } => {
                    ::borsh::BorshSerialize::serialize(
                        collateral_ratio_initial,
                        writer,
                    )?;
                    ::borsh::BorshSerialize::serialize(
                        collateral_ratio_maintenance,
                        writer,
                    )?;
                    ::borsh::BorshSerialize::serialize(
                        collateral_ratio_initial_pre_expiry,
                        writer,
                    )?;
                }
                Self::SetKeeperFee { keeper_fee_per_tx } => {
                    ::borsh::BorshSerialize::serialize(keeper_fee_per_tx, writer)?;
                }
                Self::SetTwapDuration { twap_duration } => {
                    ::borsh::BorshSerialize::serialize(twap_duration, writer)?;
                }
                Self::SettleExpiryUser => {}
                Self::TransferBaseToken { delta_a } => {
                    ::borsh::BorshSerialize::serialize(delta_a, writer)?;
                }
                Self::TransferQuoteToken { delta_b } => {
                    ::borsh::BorshSerialize::serialize(delta_b, writer)?;
                }
                Self::UpdateFeesAndRewards => {}
                Self::UpdateOracle {
                    market_rate,
                    rate,
                    last_rate,
                    epoch_start_timestamp,
                } => {
                    ::borsh::BorshSerialize::serialize(market_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(rate, writer)?;
                    ::borsh::BorshSerialize::serialize(last_rate, writer)?;
                    ::borsh::BorshSerialize::serialize(epoch_start_timestamp, writer)?;
                }
                Self::UpdateSpotYieldMarketCollateralRatio => {}
                Self::UpdateTickLiquidity { tick_index, new_liquidity } => {
                    ::borsh::BorshSerialize::serialize(tick_index, writer)?;
                    ::borsh::BorshSerialize::serialize(new_liquidity, writer)?;
                }
                Self::UpdateUserPosition { base_asset_amount, quote_asset_amount } => {
                    ::borsh::BorshSerialize::serialize(base_asset_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(quote_asset_amount, writer)?;
                }
                Self::UpdateYieldMarket { number_of_active_lps } => {
                    ::borsh::BorshSerialize::serialize(number_of_active_lps, writer)?;
                }
                Self::UpdateYieldMarketActiveRatioCoef { active_ratio_coef } => {
                    ::borsh::BorshSerialize::serialize(active_ratio_coef, writer)?;
                }
                Self::UpdateYieldMarketCollateralRatioInitialPreExpiry {
                    collateral_ratio_initial_pre_expiry,
                } => {
                    ::borsh::BorshSerialize::serialize(
                        collateral_ratio_initial_pre_expiry,
                        writer,
                    )?;
                }
                Self::UpdateYieldMarketCollateralRatioMaintenance {
                    collateral_ratio_maintenance,
                } => {
                    ::borsh::BorshSerialize::serialize(
                        collateral_ratio_maintenance,
                        writer,
                    )?;
                }
                Self::UpdateYieldMarketExpireTotalPosQuoteAmount {
                    expire_total_pos_quote_amount,
                } => {
                    ::borsh::BorshSerialize::serialize(
                        expire_total_pos_quote_amount,
                        writer,
                    )?;
                }
                Self::UpdateYieldMarketExpireTs { expire_ts } => {
                    ::borsh::BorshSerialize::serialize(expire_ts, writer)?;
                }
                Self::UpdateYieldMarketFeeRate { fee_rate } => {
                    ::borsh::BorshSerialize::serialize(fee_rate, writer)?;
                }
                Self::UpdateYieldMarketInsurance {
                    base_asset_amount,
                    quote_asset_amount,
                } => {
                    ::borsh::BorshSerialize::serialize(base_asset_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(quote_asset_amount, writer)?;
                }
                Self::UpdateYieldMarketKeeperFee { keeper_fee } => {
                    ::borsh::BorshSerialize::serialize(keeper_fee, writer)?;
                }
                Self::UpdateYieldMarketLiqFeeRate { liq_fee_rate } => {
                    ::borsh::BorshSerialize::serialize(liq_fee_rate, writer)?;
                }
                Self::UpdateYieldMarketLowerUpperRateBound {
                    lower_rate_bound,
                    upper_rate_bound,
                } => {
                    ::borsh::BorshSerialize::serialize(lower_rate_bound, writer)?;
                    ::borsh::BorshSerialize::serialize(upper_rate_bound, writer)?;
                }
                Self::UpdateYieldMarketLpAccountsProcessed { lp_accounts_processed } => {
                    ::borsh::BorshSerialize::serialize(lp_accounts_processed, writer)?;
                }
                Self::UpdateYieldMarketMarginDecimalsAndLpMarginDecimals {
                    margin_decimals,
                    lp_margin_decimals,
                } => {
                    ::borsh::BorshSerialize::serialize(margin_decimals, writer)?;
                    ::borsh::BorshSerialize::serialize(lp_margin_decimals, writer)?;
                }
                Self::UpdateYieldMarketMinLpAmount { min_lp_amount } => {
                    ::borsh::BorshSerialize::serialize(min_lp_amount, writer)?;
                }
                Self::UpdateYieldMarketMinOrderSize { min_order_size } => {
                    ::borsh::BorshSerialize::serialize(min_order_size, writer)?;
                }
                Self::UpdateYieldMarketNetBaseAmount { net_base_amount } => {
                    ::borsh::BorshSerialize::serialize(net_base_amount, writer)?;
                }
                Self::UpdateYieldMarketNetQuoteAmount { net_quote_amount } => {
                    ::borsh::BorshSerialize::serialize(net_quote_amount, writer)?;
                }
                Self::UpdateYieldMarketNumberOfActiveUsers {
                    number_of_active_users,
                } => {
                    ::borsh::BorshSerialize::serialize(number_of_active_users, writer)?;
                }
                Self::UpdateYieldMarketNumberOfProcessedUsers {
                    number_of_processed_users,
                } => {
                    ::borsh::BorshSerialize::serialize(
                        number_of_processed_users,
                        writer,
                    )?;
                }
                Self::UpdateYieldMarketOracle => {}
                Self::UpdateYieldMarketOrderStepSize { order_step_size } => {
                    ::borsh::BorshSerialize::serialize(order_step_size, writer)?;
                }
                Self::UpdateYieldMarketPoolLiquidity { liquidity } => {
                    ::borsh::BorshSerialize::serialize(liquidity, writer)?;
                }
                Self::UpdateYieldMarketPtData { name, symbol, uri } => {
                    ::borsh::BorshSerialize::serialize(name, writer)?;
                    ::borsh::BorshSerialize::serialize(symbol, writer)?;
                    ::borsh::BorshSerialize::serialize(uri, writer)?;
                }
                Self::UpdateYieldMarketSocialLoss {
                    base_asset_amount,
                    quote_asset_amount,
                } => {
                    ::borsh::BorshSerialize::serialize(base_asset_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(quote_asset_amount, writer)?;
                }
                Self::UpdateYieldMarketStartTs { start_ts } => {
                    ::borsh::BorshSerialize::serialize(start_ts, writer)?;
                }
                Self::UpdateYieldMarketStatus { status } => {
                    ::borsh::BorshSerialize::serialize(status, writer)?;
                }
                Self::UpdateYieldMarketTickIndex {
                    tick_lower_index,
                    tick_upper_index,
                } => {
                    ::borsh::BorshSerialize::serialize(tick_lower_index, writer)?;
                    ::borsh::BorshSerialize::serialize(tick_upper_index, writer)?;
                }
                Self::UpdateYieldMarketTotalReserveQuoteAndBase {
                    total_reserve_quote_amount,
                    total_reserve_base_amount,
                } => {
                    ::borsh::BorshSerialize::serialize(
                        total_reserve_quote_amount,
                        writer,
                    )?;
                    ::borsh::BorshSerialize::serialize(
                        total_reserve_base_amount,
                        writer,
                    )?;
                }
                Self::VaultTransfer { from_lp_amount, is_earn } => {
                    ::borsh::BorshSerialize::serialize(from_lp_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(is_earn, writer)?;
                }
                Self::Withdraw { amount } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                }
            }
            Ok(())
        }
    }
    struct AddKeeperDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for AddKeeperDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::AddKeeper {
                    new_keeper: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<AddKeeperDeserializer> for RatexContractsInstruction {
        fn from(helper: AddKeeperDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct AddLpSharesDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for AddLpSharesDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::AddLpShares {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    margin_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    market_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    lower_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    upper_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<AddLpSharesDeserializer> for RatexContractsInstruction {
        fn from(helper: AddLpSharesDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct AdminAddLpSharesDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for AdminAddLpSharesDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::AdminAddLpShares {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    margin_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    market_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    lower_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    upper_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<AdminAddLpSharesDeserializer> for RatexContractsInstruction {
        fn from(helper: AdminAddLpSharesDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct AdminAddMarginDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for AdminAddMarginDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::AdminAddMargin {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<AdminAddMarginDeserializer> for RatexContractsInstruction {
        fn from(helper: AdminAddMarginDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct AdminTransferMarginDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for AdminTransferMarginDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::AdminTransferMargin {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<AdminTransferMarginDeserializer> for RatexContractsInstruction {
        fn from(helper: AdminTransferMarginDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct BeginVaultSwapDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for BeginVaultSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::BeginVaultSwap {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    other_amount_threshold: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    is_exact_in: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<BeginVaultSwapDeserializer> for RatexContractsInstruction {
        fn from(helper: BeginVaultSwapDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateEarnInvestDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateEarnInvestDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateEarnInvest {
                    margin_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<CalculateEarnInvestDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculateEarnInvestDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateImpliedRateDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateImpliedRateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateImpliedRate {
                    maturity: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    sqrt_price_x64: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<CalculateImpliedRateDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculateImpliedRateDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateLpRemoveMaxRatioDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateLpRemoveMaxRatioDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateLpRemoveMaxRatio {
                    sqrt_price_limit: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<CalculateLpRemoveMaxRatioDeserializer> for RatexContractsInstruction {
        fn from(
            helper: CalculateLpRemoveMaxRatioDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateLpSlossDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateLpSlossDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateLpSloss {
                    rm_liquidity_percent: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<CalculateLpSlossDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculateLpSlossDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateLpValueDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateLpValueDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateLpValue {
                }),
            )
        }
    }
    impl From<CalculateLpValueDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculateLpValueDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateMarginValueDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateMarginValueDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateMarginValue {
                }),
            )
        }
    }
    impl From<CalculateMarginValueDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculateMarginValueDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculatePositionValueDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculatePositionValueDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculatePositionValue {
                }),
            )
        }
    }
    impl From<CalculatePositionValueDeserializer> for RatexContractsInstruction {
        fn from(
            helper: CalculatePositionValueDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculatePtPriceDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculatePtPriceDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculatePtPrice {
                }),
            )
        }
    }
    impl From<CalculatePtPriceDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculatePtPriceDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateSwapDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateSwap {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    a_to_b: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    amount_specified_is_input: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    sqrt_price_limit: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    skip_standardize: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<CalculateSwapDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculateSwapDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateSwapV2Deserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateSwapV2Deserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateSwapV2 {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    a_to_b: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    amount_specified_is_input: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    sqrt_price_limit: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    skip_standardize: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<CalculateSwapV2Deserializer> for RatexContractsInstruction {
        fn from(helper: CalculateSwapV2Deserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateTickIndexDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateTickIndexDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateTickIndex {
                    maturity: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    implied_rate: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    tick_spacing: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    is_lower: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<CalculateTickIndexDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculateTickIndexDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CalculateTraderPnlDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CalculateTraderPnlDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CalculateTraderPnl {
                }),
            )
        }
    }
    impl From<CalculateTraderPnlDeserializer> for RatexContractsInstruction {
        fn from(helper: CalculateTraderPnlDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CancelIsolatedOrderDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CancelIsolatedOrderDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CancelIsolatedOrder {
                    order_id: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<CancelIsolatedOrderDeserializer> for RatexContractsInstruction {
        fn from(helper: CancelIsolatedOrderDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CancelOrderDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CancelOrderDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CancelOrder {
                    order_id: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<CancelOrderDeserializer> for RatexContractsInstruction {
        fn from(helper: CancelOrderDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct ClaimInsuranceDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for ClaimInsuranceDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::ClaimInsurance {
                }),
            )
        }
    }
    impl From<ClaimInsuranceDeserializer> for RatexContractsInstruction {
        fn from(helper: ClaimInsuranceDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct ClaimKeeperFeeDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for ClaimKeeperFeeDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::ClaimKeeperFee {
                }),
            )
        }
    }
    impl From<ClaimKeeperFeeDeserializer> for RatexContractsInstruction {
        fn from(helper: ClaimKeeperFeeDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct ClaimYieldDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for ClaimYieldDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::ClaimYield {
                    market_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<ClaimYieldDeserializer> for RatexContractsInstruction {
        fn from(helper: ClaimYieldDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CollectEarnFeeDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CollectEarnFeeDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CollectEarnFee {
                }),
            )
        }
    }
    impl From<CollectEarnFeeDeserializer> for RatexContractsInstruction {
        fn from(helper: CollectEarnFeeDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CollectFeesDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CollectFeesDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CollectFees {
                }),
            )
        }
    }
    impl From<CollectFeesDeserializer> for RatexContractsInstruction {
        fn from(helper: CollectFeesDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct CollectProtocolFeesDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for CollectProtocolFeesDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::CollectProtocolFees {
                }),
            )
        }
    }
    impl From<CollectProtocolFeesDeserializer> for RatexContractsInstruction {
        fn from(helper: CollectProtocolFeesDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct DeleteLpDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for DeleteLpDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::DeleteLp {
                }),
            )
        }
    }
    impl From<DeleteLpDeserializer> for RatexContractsInstruction {
        fn from(helper: DeleteLpDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct DeleteTickArrayDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for DeleteTickArrayDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::DeleteTickArray {
                }),
            )
        }
    }
    impl From<DeleteTickArrayDeserializer> for RatexContractsInstruction {
        fn from(helper: DeleteTickArrayDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct DeleteUserDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for DeleteUserDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::DeleteUser {
                }),
            )
        }
    }
    impl From<DeleteUserDeserializer> for RatexContractsInstruction {
        fn from(helper: DeleteUserDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct DepositDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for DepositDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::Deposit {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<DepositDeserializer> for RatexContractsInstruction {
        fn from(helper: DepositDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EarnInvestDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EarnInvestDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EarnInvest {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<EarnInvestDeserializer> for RatexContractsInstruction {
        fn from(helper: EarnInvestDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EarnRedeemDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EarnRedeemDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EarnRedeem {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    sqrt_price_limit: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<EarnRedeemDeserializer> for RatexContractsInstruction {
        fn from(helper: EarnRedeemDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EndVaultSwapDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EndVaultSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EndVaultSwap {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    other_amount_threshold: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    is_exact_in: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<EndVaultSwapDeserializer> for RatexContractsInstruction {
        fn from(helper: EndVaultSwapDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EpochUpdateAddDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EpochUpdateAddDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EpochUpdateAdd {
                    market_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    is_expired: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<EpochUpdateAddDeserializer> for RatexContractsInstruction {
        fn from(helper: EpochUpdateAddDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EpochUpdateBeginDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EpochUpdateBeginDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EpochUpdateBegin {
                    is_expired: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<EpochUpdateBeginDeserializer> for RatexContractsInstruction {
        fn from(helper: EpochUpdateBeginDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EpochUpdateChangePriceDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EpochUpdateChangePriceDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EpochUpdateChangePrice {
                    is_expired: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<EpochUpdateChangePriceDeserializer> for RatexContractsInstruction {
        fn from(
            helper: EpochUpdateChangePriceDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EpochUpdateEndDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EpochUpdateEndDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EpochUpdateEnd {
                    is_expired: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<EpochUpdateEndDeserializer> for RatexContractsInstruction {
        fn from(helper: EpochUpdateEndDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EpochUpdateExpiryApplyDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EpochUpdateExpiryApplyDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EpochUpdateExpiryApply {
                }),
            )
        }
    }
    impl From<EpochUpdateExpiryApplyDeserializer> for RatexContractsInstruction {
        fn from(
            helper: EpochUpdateExpiryApplyDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EpochUpdateExpiryCheckDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EpochUpdateExpiryCheckDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EpochUpdateExpiryCheck {
                }),
            )
        }
    }
    impl From<EpochUpdateExpiryCheckDeserializer> for RatexContractsInstruction {
        fn from(
            helper: EpochUpdateExpiryCheckDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct EpochUpdateRemoveDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for EpochUpdateRemoveDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::EpochUpdateRemove {
                    market_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    is_expired: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<EpochUpdateRemoveDeserializer> for RatexContractsInstruction {
        fn from(helper: EpochUpdateRemoveDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct FillOrderDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for FillOrderDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::FillOrder {
                    order_id: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<FillOrderDeserializer> for RatexContractsInstruction {
        fn from(helper: FillOrderDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct GetAmmTwapDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for GetAmmTwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::GetAmmTwap {
                    seconds_ago: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<GetAmmTwapDeserializer> for RatexContractsInstruction {
        fn from(helper: GetAmmTwapDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::Initialize {
                    margin_index_start: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    market_index_start: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    keeper_fee: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeConfigDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeConfigDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeConfig {
                    fee_authority: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    collect_protocol_fees_authority: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    reward_emissions_super_authority: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    default_protocol_fee_rate: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<InitializeConfigDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeConfigDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeEarnVaultDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeEarnVaultDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeEarnVault {
                    user_ratio: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeEarnVaultDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeEarnVaultDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeFeeTierDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeFeeTierDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeFeeTier {
                    tick_spacing: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    default_fee_rate: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<InitializeFeeTierDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeFeeTierDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeLpDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeLpDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeLp {
                    sub_account_id: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<InitializeLpDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeLpDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeMarginMarketDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeMarginMarketDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeMarginMarket {
                    name: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeMarginMarketDeserializer> for RatexContractsInstruction {
        fn from(
            helper: InitializeMarginMarketDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeOracleDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeOracleDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeOracle {
                    name: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    market_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    last_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    epoch_start_timestamp: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    decimals: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeOracleDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeOracleDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeTickArrayDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeTickArrayDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeTickArray {
                    start_tick_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<InitializeTickArrayDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeTickArrayDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeUserDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeUserDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeUser {
                    sub_account_id: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    is_isolated: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeUserDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeUserDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeUserStatsDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeUserStatsDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeUserStats {
                }),
            )
        }
    }
    impl From<InitializeUserStatsDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeUserStatsDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeYieldMarketDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeYieldMarketDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeYieldMarket {
                    tick_spacing: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    sqrt_price: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    order_step_size: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    min_order_size: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    min_liquidation_size: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    start_ts: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    expire_ts: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    active_ratio_coef: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    margin_type: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    lp_margin_type: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    min_lp_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    lower_rate_bound: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    upper_rate_bound: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    bound_percentage: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    market_type: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    name: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeYieldMarketDeserializer> for RatexContractsInstruction {
        fn from(helper: InitializeYieldMarketDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeYieldMarketTokenAccountADeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize
    for InitializeYieldMarketTokenAccountADeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeYieldMarketTokenAccountA {
                    yield_market: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeYieldMarketTokenAccountADeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: InitializeYieldMarketTokenAccountADeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeYieldMarketTokenAccountAaDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize
    for InitializeYieldMarketTokenAccountAaDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeYieldMarketTokenAccountAa {
                    yield_market: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeYieldMarketTokenAccountAaDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: InitializeYieldMarketTokenAccountAaDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeYieldMarketTokenAccountBDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize
    for InitializeYieldMarketTokenAccountBDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeYieldMarketTokenAccountB {
                    yield_market: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeYieldMarketTokenAccountBDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: InitializeYieldMarketTokenAccountBDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct InitializeYieldMarketTokenAccountBbDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize
    for InitializeYieldMarketTokenAccountBbDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::InitializeYieldMarketTokenAccountBb {
                    yield_market: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeYieldMarketTokenAccountBbDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: InitializeYieldMarketTokenAccountBbDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct LiquidateDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for LiquidateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::Liquidate {
                }),
            )
        }
    }
    impl From<LiquidateDeserializer> for RatexContractsInstruction {
        fn from(helper: LiquidateDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct LiquidateInsuranceDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for LiquidateInsuranceDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::LiquidateInsurance {
                    market_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    adl_finish: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<LiquidateInsuranceDeserializer> for RatexContractsInstruction {
        fn from(helper: LiquidateInsuranceDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct LiquidateLpDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for LiquidateLpDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::LiquidateLp {
                }),
            )
        }
    }
    impl From<LiquidateLpDeserializer> for RatexContractsInstruction {
        fn from(helper: LiquidateLpDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct LoadObservationStateDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for LoadObservationStateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::LoadObservationState {
                }),
            )
        }
    }
    impl From<LoadObservationStateDeserializer> for RatexContractsInstruction {
        fn from(helper: LoadObservationStateDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct MultiSigDepositDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for MultiSigDepositDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::MultiSigDeposit {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<MultiSigDepositDeserializer> for RatexContractsInstruction {
        fn from(helper: MultiSigDepositDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct ObserveDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for ObserveDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::Observe {
                    seconds_agos: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<ObserveDeserializer> for RatexContractsInstruction {
        fn from(helper: ObserveDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct PlaceOrderDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for PlaceOrderDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::PlaceOrder {
                    params: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<PlaceOrderDeserializer> for RatexContractsInstruction {
        fn from(helper: PlaceOrderDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct RemoveKeeperDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for RemoveKeeperDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::RemoveKeeper {
                    remove_keeper: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<RemoveKeeperDeserializer> for RatexContractsInstruction {
        fn from(helper: RemoveKeeperDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct RemoveLpSharesDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for RemoveLpSharesDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::RemoveLpShares {
                    rm_liquidity_percent: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    sqrt_price_limit: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<RemoveLpSharesDeserializer> for RatexContractsInstruction {
        fn from(helper: RemoveLpSharesDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct RollbackOracleDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for RollbackOracleDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::RollbackOracle {
                    market_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    last_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    epoch_start_timestamp: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    last_epoch_start_timestamp: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<RollbackOracleDeserializer> for RatexContractsInstruction {
        fn from(helper: RollbackOracleDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct SetCollateralRatioDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for SetCollateralRatioDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::SetCollateralRatio {
                    collateral_ratio_initial: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    collateral_ratio_maintenance: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    collateral_ratio_initial_pre_expiry: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<SetCollateralRatioDeserializer> for RatexContractsInstruction {
        fn from(helper: SetCollateralRatioDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct SetKeeperFeeDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for SetKeeperFeeDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::SetKeeperFee {
                    keeper_fee_per_tx: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<SetKeeperFeeDeserializer> for RatexContractsInstruction {
        fn from(helper: SetKeeperFeeDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct SetTwapDurationDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for SetTwapDurationDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::SetTwapDuration {
                    twap_duration: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<SetTwapDurationDeserializer> for RatexContractsInstruction {
        fn from(helper: SetTwapDurationDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct SettleExpiryUserDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for SettleExpiryUserDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::SettleExpiryUser {
                }),
            )
        }
    }
    impl From<SettleExpiryUserDeserializer> for RatexContractsInstruction {
        fn from(helper: SettleExpiryUserDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct TransferBaseTokenDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for TransferBaseTokenDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::TransferBaseToken {
                    delta_a: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<TransferBaseTokenDeserializer> for RatexContractsInstruction {
        fn from(helper: TransferBaseTokenDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct TransferQuoteTokenDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for TransferQuoteTokenDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::TransferQuoteToken {
                    delta_b: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<TransferQuoteTokenDeserializer> for RatexContractsInstruction {
        fn from(helper: TransferQuoteTokenDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateFeesAndRewardsDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateFeesAndRewardsDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateFeesAndRewards {
                }),
            )
        }
    }
    impl From<UpdateFeesAndRewardsDeserializer> for RatexContractsInstruction {
        fn from(helper: UpdateFeesAndRewardsDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateOracleDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateOracleDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateOracle {
                    market_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    last_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    epoch_start_timestamp: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateOracleDeserializer> for RatexContractsInstruction {
        fn from(helper: UpdateOracleDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateSpotYieldMarketCollateralRatioDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize
    for UpdateSpotYieldMarketCollateralRatioDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateSpotYieldMarketCollateralRatio {
                }),
            )
        }
    }
    impl From<UpdateSpotYieldMarketCollateralRatioDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateSpotYieldMarketCollateralRatioDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateTickLiquidityDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateTickLiquidityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateTickLiquidity {
                    tick_index: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    new_liquidity: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateTickLiquidityDeserializer> for RatexContractsInstruction {
        fn from(helper: UpdateTickLiquidityDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateUserPositionDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateUserPositionDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateUserPosition {
                    base_asset_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    quote_asset_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateUserPositionDeserializer> for RatexContractsInstruction {
        fn from(helper: UpdateUserPositionDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarket {
                    number_of_active_lps: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketDeserializer> for RatexContractsInstruction {
        fn from(helper: UpdateYieldMarketDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketActiveRatioCoefDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketActiveRatioCoefDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketActiveRatioCoef {
                    active_ratio_coef: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketActiveRatioCoefDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketActiveRatioCoefDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketCollateralRatioInitialPreExpiryDeserializer(
        RatexContractsInstruction,
    );
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketCollateralRatioInitialPreExpiryDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketCollateralRatioInitialPreExpiry {
                    collateral_ratio_initial_pre_expiry: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketCollateralRatioInitialPreExpiryDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketCollateralRatioInitialPreExpiryDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketCollateralRatioMaintenanceDeserializer(
        RatexContractsInstruction,
    );
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketCollateralRatioMaintenanceDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketCollateralRatioMaintenance {
                    collateral_ratio_maintenance: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketCollateralRatioMaintenanceDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketCollateralRatioMaintenanceDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketExpireTotalPosQuoteAmountDeserializer(
        RatexContractsInstruction,
    );
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketExpireTotalPosQuoteAmountDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketExpireTotalPosQuoteAmount {
                    expire_total_pos_quote_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketExpireTotalPosQuoteAmountDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketExpireTotalPosQuoteAmountDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketExpireTsDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketExpireTsDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketExpireTs {
                    expire_ts: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketExpireTsDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketExpireTsDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketFeeRateDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketFeeRateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketFeeRate {
                    fee_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketFeeRateDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketFeeRateDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketInsuranceDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketInsuranceDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketInsurance {
                    base_asset_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    quote_asset_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketInsuranceDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketInsuranceDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketKeeperFeeDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketKeeperFeeDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketKeeperFee {
                    keeper_fee: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketKeeperFeeDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketKeeperFeeDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketLiqFeeRateDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketLiqFeeRateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketLiqFeeRate {
                    liq_fee_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketLiqFeeRateDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketLiqFeeRateDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketLowerUpperRateBoundDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketLowerUpperRateBoundDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketLowerUpperRateBound {
                    lower_rate_bound: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    upper_rate_bound: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketLowerUpperRateBoundDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketLowerUpperRateBoundDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketLpAccountsProcessedDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketLpAccountsProcessedDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketLpAccountsProcessed {
                    lp_accounts_processed: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketLpAccountsProcessedDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketLpAccountsProcessedDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketMarginDecimalsAndLpMarginDecimalsDeserializer(
        RatexContractsInstruction,
    );
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketMarginDecimalsAndLpMarginDecimalsDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketMarginDecimalsAndLpMarginDecimals {
                    margin_decimals: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    lp_margin_decimals: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketMarginDecimalsAndLpMarginDecimalsDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketMarginDecimalsAndLpMarginDecimalsDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketMinLpAmountDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketMinLpAmountDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketMinLpAmount {
                    min_lp_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketMinLpAmountDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketMinLpAmountDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketMinOrderSizeDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketMinOrderSizeDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketMinOrderSize {
                    min_order_size: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketMinOrderSizeDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketMinOrderSizeDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketNetBaseAmountDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketNetBaseAmountDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketNetBaseAmount {
                    net_base_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketNetBaseAmountDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketNetBaseAmountDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketNetQuoteAmountDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketNetQuoteAmountDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketNetQuoteAmount {
                    net_quote_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketNetQuoteAmountDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketNetQuoteAmountDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketNumberOfActiveUsersDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketNumberOfActiveUsersDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketNumberOfActiveUsers {
                    number_of_active_users: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketNumberOfActiveUsersDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketNumberOfActiveUsersDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketNumberOfProcessedUsersDeserializer(
        RatexContractsInstruction,
    );
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketNumberOfProcessedUsersDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketNumberOfProcessedUsers {
                    number_of_processed_users: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketNumberOfProcessedUsersDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketNumberOfProcessedUsersDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketOracleDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketOracleDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketOracle {
                }),
            )
        }
    }
    impl From<UpdateYieldMarketOracleDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketOracleDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketOrderStepSizeDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketOrderStepSizeDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketOrderStepSize {
                    order_step_size: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketOrderStepSizeDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketOrderStepSizeDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketPoolLiquidityDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketPoolLiquidityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketPoolLiquidity {
                    liquidity: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketPoolLiquidityDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketPoolLiquidityDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketPtDataDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketPtDataDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketPtData {
                    name: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    symbol: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    uri: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketPtDataDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketPtDataDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketSocialLossDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketSocialLossDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketSocialLoss {
                    base_asset_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    quote_asset_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketSocialLossDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketSocialLossDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketStartTsDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketStartTsDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketStartTs {
                    start_ts: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketStartTsDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketStartTsDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketStatusDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketStatusDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketStatus {
                    status: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketStatusDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketStatusDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketTickIndexDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateYieldMarketTickIndexDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketTickIndex {
                    tick_lower_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    tick_upper_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketTickIndexDeserializer> for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketTickIndexDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct UpdateYieldMarketTotalReserveQuoteAndBaseDeserializer(
        RatexContractsInstruction,
    );
    impl ::borsh::de::BorshDeserialize
    for UpdateYieldMarketTotalReserveQuoteAndBaseDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::UpdateYieldMarketTotalReserveQuoteAndBase {
                    total_reserve_quote_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    total_reserve_base_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<UpdateYieldMarketTotalReserveQuoteAndBaseDeserializer>
    for RatexContractsInstruction {
        fn from(
            helper: UpdateYieldMarketTotalReserveQuoteAndBaseDeserializer,
        ) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct VaultTransferDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for VaultTransferDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::VaultTransfer {
                    from_lp_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    is_earn: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<VaultTransferDeserializer> for RatexContractsInstruction {
        fn from(helper: VaultTransferDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    struct WithdrawDeserializer(RatexContractsInstruction);
    impl ::borsh::de::BorshDeserialize for WithdrawDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(RatexContractsInstruction::Withdraw {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<WithdrawDeserializer> for RatexContractsInstruction {
        fn from(helper: WithdrawDeserializer) -> RatexContractsInstruction {
            helper.0
        }
    }
    #[derive(Debug)]
    pub struct AddKeeper {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub new_keeper: ::solana_program::pubkey::Pubkey,
    }
    impl AddKeeper {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, admin, state, trailing_accounts, new_keeper } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::AddKeeper {
                new_keeper,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct AddKeeperAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl AddKeeperAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for AddKeeperAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct AddLpShares {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_mint_base: ::solana_program::pubkey::Pubkey,
        pub token_mint_quote: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: i64,
        pub margin_index: u32,
        pub market_index: u32,
        pub lower_rate: u64,
        pub upper_rate: u64,
    }
    impl AddLpShares {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                token_vault_base,
                token_vault_quote,
                tick_array_lower,
                tick_array_upper,
                yield_market,
                token_owner_account_base,
                token_owner_account_quote,
                token_mint_base,
                token_mint_quote,
                margin_market,
                margin_market_vault,
                oracle,
                user_token_account,
                token_program,
                system_program,
                lp,
                authority,
                trailing_accounts,
                amount,
                margin_index,
                market_index,
                lower_rate,
                upper_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_lower,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_upper,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_base,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_quote,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false), ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::AddLpShares {
                amount,
                margin_index,
                market_index,
                lower_rate,
                upper_rate,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct AddLpSharesAccountIndexes {
        pub state: usize,
        pub token_vault_base: usize,
        pub token_vault_quote: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub yield_market: usize,
        pub token_owner_account_base: usize,
        pub token_owner_account_quote: usize,
        pub token_mint_base: usize,
        pub token_mint_quote: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub oracle: usize,
        pub user_token_account: usize,
        pub token_program: usize,
        pub system_program: usize,
        pub lp: usize,
        pub authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl AddLpSharesAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const TOKEN_VAULT_BASE: usize = 1usize;
        pub const TOKEN_VAULT_QUOTE: usize = 2usize;
        pub const TICK_ARRAY_LOWER: usize = 3usize;
        pub const TICK_ARRAY_UPPER: usize = 4usize;
        pub const YIELD_MARKET: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 6usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 7usize;
        pub const TOKEN_MINT_BASE: usize = 8usize;
        pub const TOKEN_MINT_QUOTE: usize = 9usize;
        pub const MARGIN_MARKET: usize = 10usize;
        pub const MARGIN_MARKET_VAULT: usize = 11usize;
        pub const ORACLE: usize = 12usize;
        pub const USER_TOKEN_ACCOUNT: usize = 13usize;
        pub const TOKEN_PROGRAM: usize = 14usize;
        pub const SYSTEM_PROGRAM: usize = 15usize;
        pub const LP: usize = 16usize;
        pub const AUTHORITY: usize = 17usize;
    }
    impl<'a> TryFrom<&'a [u8]> for AddLpSharesAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            1usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            2usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            3usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            4usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            5usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            6usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            7usize,
                        ),
                    )?,
                token_mint_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_base),
                            8usize,
                        ),
                    )?,
                token_mint_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_quote),
                            9usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            10usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            11usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            12usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            13usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            14usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            15usize,
                        ),
                    )?,
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            16usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            17usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct AdminAddLpShares {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_mint_base: ::solana_program::pubkey::Pubkey,
        pub token_mint_quote: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: i64,
        pub margin_index: u32,
        pub market_index: u32,
        pub lower_rate: u64,
        pub upper_rate: u64,
    }
    impl AdminAddLpShares {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                token_vault_base,
                token_vault_quote,
                tick_array_lower,
                tick_array_upper,
                yield_market,
                token_owner_account_base,
                token_owner_account_quote,
                token_mint_base,
                token_mint_quote,
                margin_market,
                margin_market_vault,
                oracle,
                user_token_account,
                token_program,
                system_program,
                lp,
                authority,
                trailing_accounts,
                amount,
                margin_index,
                market_index,
                lower_rate,
                upper_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_lower,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_upper,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_base,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_quote,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false), ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::AdminAddLpShares {
                amount,
                margin_index,
                market_index,
                lower_rate,
                upper_rate,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct AdminAddLpSharesAccountIndexes {
        pub state: usize,
        pub token_vault_base: usize,
        pub token_vault_quote: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub yield_market: usize,
        pub token_owner_account_base: usize,
        pub token_owner_account_quote: usize,
        pub token_mint_base: usize,
        pub token_mint_quote: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub oracle: usize,
        pub user_token_account: usize,
        pub token_program: usize,
        pub system_program: usize,
        pub lp: usize,
        pub authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl AdminAddLpSharesAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const TOKEN_VAULT_BASE: usize = 1usize;
        pub const TOKEN_VAULT_QUOTE: usize = 2usize;
        pub const TICK_ARRAY_LOWER: usize = 3usize;
        pub const TICK_ARRAY_UPPER: usize = 4usize;
        pub const YIELD_MARKET: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 6usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 7usize;
        pub const TOKEN_MINT_BASE: usize = 8usize;
        pub const TOKEN_MINT_QUOTE: usize = 9usize;
        pub const MARGIN_MARKET: usize = 10usize;
        pub const MARGIN_MARKET_VAULT: usize = 11usize;
        pub const ORACLE: usize = 12usize;
        pub const USER_TOKEN_ACCOUNT: usize = 13usize;
        pub const TOKEN_PROGRAM: usize = 14usize;
        pub const SYSTEM_PROGRAM: usize = 15usize;
        pub const LP: usize = 16usize;
        pub const AUTHORITY: usize = 17usize;
    }
    impl<'a> TryFrom<&'a [u8]> for AdminAddLpSharesAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            1usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            2usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            3usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            4usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            5usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            6usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            7usize,
                        ),
                    )?,
                token_mint_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_base),
                            8usize,
                        ),
                    )?,
                token_mint_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_quote),
                            9usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            10usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            11usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            12usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            13usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            14usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            15usize,
                        ),
                    )?,
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            16usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            17usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct AdminAddMargin {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: i64,
    }
    impl AdminAddMargin {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                margin_market,
                user_token_account,
                margin_market_vault,
                token_program,
                trailing_accounts,
                amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
                ::solana_program::instruction::AccountMeta::new(margin_market, false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::AdminAddMargin {
                amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct AdminAddMarginAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub margin_market: usize,
        pub user_token_account: usize,
        pub margin_market_vault: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl AdminAddMarginAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
        pub const MARGIN_MARKET: usize = 4usize;
        pub const USER_TOKEN_ACCOUNT: usize = 5usize;
        pub const MARGIN_MARKET_VAULT: usize = 6usize;
        pub const TOKEN_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for AdminAddMarginAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            4usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            5usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            6usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            7usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct AdminTransferMargin {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: u64,
    }
    impl AdminTransferMargin {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                margin_market,
                user_token_account,
                margin_market_vault,
                token_program,
                trailing_accounts,
                amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::AdminTransferMargin {
                amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct AdminTransferMarginAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub margin_market: usize,
        pub user_token_account: usize,
        pub margin_market_vault: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl AdminTransferMarginAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const MARGIN_MARKET: usize = 2usize;
        pub const USER_TOKEN_ACCOUNT: usize = 3usize;
        pub const MARGIN_MARKET_VAULT: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
    }
    impl<'a> TryFrom<&'a [u8]> for AdminTransferMarginAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            2usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            3usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            4usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            5usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct BeginVaultSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub in_margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub out_margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub in_margin_market: ::solana_program::pubkey::Pubkey,
        pub out_margin_market: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub in_user_token_account: ::solana_program::pubkey::Pubkey,
        pub out_user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub instructions: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: u64,
        pub other_amount_threshold: u64,
        pub is_exact_in: bool,
    }
    impl BeginVaultSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                authority,
                in_margin_market_vault,
                out_margin_market_vault,
                in_margin_market,
                out_margin_market,
                yield_market,
                oracle,
                in_user_token_account,
                out_user_token_account,
                token_program,
                instructions,
                trailing_accounts,
                amount,
                other_amount_threshold,
                is_exact_in,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
                ::solana_program::instruction::AccountMeta::new(in_margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(out_margin_market_vault,
                false), ::solana_program::instruction::AccountMeta::new(in_margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(out_margin_market,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(in_user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(out_user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(instructions,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::BeginVaultSwap {
                amount,
                other_amount_threshold,
                is_exact_in,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct BeginVaultSwapAccountIndexes {
        pub state: usize,
        pub authority: usize,
        pub in_margin_market_vault: usize,
        pub out_margin_market_vault: usize,
        pub in_margin_market: usize,
        pub out_margin_market: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub in_user_token_account: usize,
        pub out_user_token_account: usize,
        pub token_program: usize,
        pub instructions: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl BeginVaultSwapAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const IN_MARGIN_MARKET_VAULT: usize = 2usize;
        pub const OUT_MARGIN_MARKET_VAULT: usize = 3usize;
        pub const IN_MARGIN_MARKET: usize = 4usize;
        pub const OUT_MARGIN_MARKET: usize = 5usize;
        pub const YIELD_MARKET: usize = 6usize;
        pub const ORACLE: usize = 7usize;
        pub const IN_USER_TOKEN_ACCOUNT: usize = 8usize;
        pub const OUT_USER_TOKEN_ACCOUNT: usize = 9usize;
        pub const TOKEN_PROGRAM: usize = 10usize;
        pub const INSTRUCTIONS: usize = 11usize;
    }
    impl<'a> TryFrom<&'a [u8]> for BeginVaultSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            1usize,
                        ),
                    )?,
                in_margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(in_margin_market_vault),
                            2usize,
                        ),
                    )?,
                out_margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(out_margin_market_vault),
                            3usize,
                        ),
                    )?,
                in_margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(in_margin_market),
                            4usize,
                        ),
                    )?,
                out_margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(out_margin_market),
                            5usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            6usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            7usize,
                        ),
                    )?,
                in_user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(in_user_token_account),
                            8usize,
                        ),
                    )?,
                out_user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(out_user_token_account),
                            9usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            10usize,
                        ),
                    )?,
                instructions: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(instructions),
                            11usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateEarnInvest {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub pt_mint: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub margin_amount: u64,
    }
    impl CalculateEarnInvest {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                earn_vault,
                state,
                authority,
                token_program,
                yield_market,
                margin_market,
                margin_market_vault,
                oracle,
                observation_state,
                token_owner_account_base,
                token_vault_base,
                token_owner_account_quote,
                token_vault_quote,
                pt_mint,
                system_program,
                trailing_accounts,
                margin_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(earn_vault, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(observation_state,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false), ::solana_program::instruction::AccountMeta::new(pt_mint, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateEarnInvest {
                margin_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateEarnInvestAccountIndexes {
        pub earn_vault: usize,
        pub state: usize,
        pub authority: usize,
        pub token_program: usize,
        pub yield_market: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub oracle: usize,
        pub observation_state: usize,
        pub token_owner_account_base: usize,
        pub token_vault_base: usize,
        pub token_owner_account_quote: usize,
        pub token_vault_quote: usize,
        pub pt_mint: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateEarnInvestAccountIndexes {
        pub const EARN_VAULT: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
        pub const TOKEN_PROGRAM: usize = 3usize;
        pub const YIELD_MARKET: usize = 4usize;
        pub const MARGIN_MARKET: usize = 5usize;
        pub const MARGIN_MARKET_VAULT: usize = 6usize;
        pub const ORACLE: usize = 7usize;
        pub const OBSERVATION_STATE: usize = 8usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 9usize;
        pub const TOKEN_VAULT_BASE: usize = 10usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 11usize;
        pub const TOKEN_VAULT_QUOTE: usize = 12usize;
        pub const PT_MINT: usize = 13usize;
        pub const SYSTEM_PROGRAM: usize = 14usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculateEarnInvestAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                earn_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(earn_vault),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            3usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            4usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            5usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            6usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            7usize,
                        ),
                    )?,
                observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation_state),
                            8usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            9usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            10usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            11usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            12usize,
                        ),
                    )?,
                pt_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pt_mint),
                            13usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            14usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateImpliedRate {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub maturity: u64,
        pub sqrt_price_x64: u128,
    }
    impl CalculateImpliedRate {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, trailing_accounts, maturity, sqrt_price_x64 } = self;
            let mut accounts = vec![];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateImpliedRate {
                maturity,
                sqrt_price_x64,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateImpliedRateAccountIndexes {
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateImpliedRateAccountIndexes {}
    impl<'a> TryFrom<&'a [u8]> for CalculateImpliedRateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateLpRemoveMaxRatio {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub sqrt_price_limit: u128,
    }
    impl CalculateLpRemoveMaxRatio {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                yield_market,
                observation_state,
                lp,
                token_vault_base,
                token_vault_quote,
                tick_array_lower,
                tick_array_upper,
                token_owner_account_base,
                token_owner_account_quote,
                token_program,
                trailing_accounts,
                sqrt_price_limit,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(observation_state,
                false), ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_lower,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_upper,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateLpRemoveMaxRatio {
                sqrt_price_limit,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateLpRemoveMaxRatioAccountIndexes {
        pub state: usize,
        pub yield_market: usize,
        pub observation_state: usize,
        pub lp: usize,
        pub token_vault_base: usize,
        pub token_vault_quote: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub token_owner_account_base: usize,
        pub token_owner_account_quote: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateLpRemoveMaxRatioAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const YIELD_MARKET: usize = 1usize;
        pub const OBSERVATION_STATE: usize = 2usize;
        pub const LP: usize = 3usize;
        pub const TOKEN_VAULT_BASE: usize = 4usize;
        pub const TOKEN_VAULT_QUOTE: usize = 5usize;
        pub const TICK_ARRAY_LOWER: usize = 6usize;
        pub const TICK_ARRAY_UPPER: usize = 7usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 8usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 9usize;
        pub const TOKEN_PROGRAM: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculateLpRemoveMaxRatioAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            1usize,
                        ),
                    )?,
                observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation_state),
                            2usize,
                        ),
                    )?,
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            3usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            4usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            5usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            6usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            7usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            8usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            9usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            10usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateLpSloss {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub rm_liquidity_percent: u64,
    }
    impl CalculateLpSloss {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                lp,
                trailing_accounts,
                rm_liquidity_percent,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false), ::solana_program::instruction::AccountMeta::new_readonly(lp,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateLpSloss {
                rm_liquidity_percent,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateLpSlossAccountIndexes {
        pub yield_market: usize,
        pub lp: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateLpSlossAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const LP: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculateLpSlossAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateLpValue {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CalculateLpValue {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, yield_market, lp, trailing_accounts } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false), ::solana_program::instruction::AccountMeta::new_readonly(lp,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateLpValue {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateLpValueAccountIndexes {
        pub yield_market: usize,
        pub lp: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateLpValueAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const LP: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculateLpValueAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateMarginValue {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CalculateMarginValue {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, user, trailing_accounts } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(user, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateMarginValue {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateMarginValueAccountIndexes {
        pub user: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateMarginValueAccountIndexes {
        pub const USER: usize = 0usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculateMarginValueAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            0usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculatePositionValue {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CalculatePositionValue {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, state, user, trailing_accounts } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(user, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculatePositionValue {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculatePositionValueAccountIndexes {
        pub state: usize,
        pub user: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculatePositionValueAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const USER: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculatePositionValueAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculatePtPrice {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CalculatePtPrice {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                earn_vault,
                oracle,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(earn_vault,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculatePtPrice {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculatePtPriceAccountIndexes {
        pub yield_market: usize,
        pub earn_vault: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculatePtPriceAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const EARN_VAULT: usize = 1usize;
        pub const ORACLE: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculatePtPriceAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                earn_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(earn_vault),
                            1usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: u64,
        pub a_to_b: bool,
        pub amount_specified_is_input: bool,
        pub sqrt_price_limit: u128,
        pub skip_standardize: bool,
    }
    impl CalculateSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                trailing_accounts,
                amount,
                a_to_b,
                amount_specified_is_input,
                sqrt_price_limit,
                skip_standardize,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateSwap {
                amount,
                a_to_b,
                amount_specified_is_input,
                sqrt_price_limit,
                skip_standardize,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateSwapAccountIndexes {
        pub yield_market: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateSwapAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculateSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateSwapV2 {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: u64,
        pub a_to_b: bool,
        pub amount_specified_is_input: bool,
        pub sqrt_price_limit: u128,
        pub skip_standardize: bool,
    }
    impl CalculateSwapV2 {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                observation_state,
                trailing_accounts,
                amount,
                a_to_b,
                amount_specified_is_input,
                sqrt_price_limit,
                skip_standardize,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(observation_state,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateSwapV2 {
                amount,
                a_to_b,
                amount_specified_is_input,
                sqrt_price_limit,
                skip_standardize,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateSwapV2AccountIndexes {
        pub yield_market: usize,
        pub observation_state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateSwapV2AccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const OBSERVATION_STATE: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculateSwapV2AccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation_state),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateTickIndex {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub maturity: u64,
        pub implied_rate: u64,
        pub tick_spacing: i32,
        pub is_lower: bool,
    }
    impl CalculateTickIndex {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                trailing_accounts,
                maturity,
                implied_rate,
                tick_spacing,
                is_lower,
            } = self;
            let mut accounts = vec![];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateTickIndex {
                maturity,
                implied_rate,
                tick_spacing,
                is_lower,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateTickIndexAccountIndexes {
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateTickIndexAccountIndexes {}
    impl<'a> TryFrom<&'a [u8]> for CalculateTickIndexAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CalculateTraderPnl {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CalculateTraderPnl {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, yield_market, user, trailing_accounts } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false), ::solana_program::instruction::AccountMeta::new_readonly(user,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CalculateTraderPnl {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CalculateTraderPnlAccountIndexes {
        pub yield_market: usize,
        pub user: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CalculateTraderPnlAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const USER: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CalculateTraderPnlAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CancelIsolatedOrder {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub order_id: u32,
    }
    impl CancelIsolatedOrder {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                margin_market_vault,
                token_program,
                authority,
                system_program,
                trailing_accounts,
                order_id,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CancelIsolatedOrder {
                order_id,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CancelIsolatedOrderAccountIndexes {
        pub state: usize,
        pub margin_market_vault: usize,
        pub token_program: usize,
        pub authority: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CancelIsolatedOrderAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const MARGIN_MARKET_VAULT: usize = 1usize;
        pub const TOKEN_PROGRAM: usize = 2usize;
        pub const AUTHORITY: usize = 3usize;
        pub const SYSTEM_PROGRAM: usize = 4usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CancelIsolatedOrderAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            1usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            2usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            3usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            4usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CancelOrder {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub order_id: u32,
    }
    impl CancelOrder {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                user,
                authority,
                trailing_accounts,
                order_id,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CancelOrder {
                order_id,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CancelOrderAccountIndexes {
        pub state: usize,
        pub user: usize,
        pub authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CancelOrderAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const USER: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CancelOrderAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct ClaimInsurance {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub token_owner_account: ::solana_program::pubkey::Pubkey,
        pub token_vault_margin: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl ClaimInsurance {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                margin_market,
                token_owner_account,
                token_vault_margin,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
                ::solana_program::instruction::AccountMeta::new(margin_market, false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account,
                false),
                ::solana_program::instruction::AccountMeta::new(token_vault_margin,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::ClaimInsurance {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct ClaimInsuranceAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub margin_market: usize,
        pub token_owner_account: usize,
        pub token_vault_margin: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl ClaimInsuranceAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
        pub const MARGIN_MARKET: usize = 4usize;
        pub const TOKEN_OWNER_ACCOUNT: usize = 5usize;
        pub const TOKEN_VAULT_MARGIN: usize = 6usize;
        pub const TOKEN_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for ClaimInsuranceAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            4usize,
                        ),
                    )?,
                token_owner_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account),
                            5usize,
                        ),
                    )?,
                token_vault_margin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_margin),
                            6usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            7usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct ClaimKeeperFee {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl ClaimKeeperFee {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, authority, state, trailing_accounts } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::ClaimKeeperFee {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct ClaimKeeperFeeAccountIndexes {
        pub authority: usize,
        pub state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl ClaimKeeperFeeAccountIndexes {
        pub const AUTHORITY: usize = 0usize;
        pub const STATE: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for ClaimKeeperFeeAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct ClaimYield {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub market_index: u32,
        pub amount: i64,
    }
    impl ClaimYield {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                user,
                authority,
                margin_market,
                margin_market_vault,
                user_token_account,
                token_program,
                trailing_accounts,
                market_index,
                amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::ClaimYield {
                market_index,
                amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct ClaimYieldAccountIndexes {
        pub state: usize,
        pub user: usize,
        pub authority: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub user_token_account: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl ClaimYieldAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const USER: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
        pub const MARGIN_MARKET: usize = 3usize;
        pub const MARGIN_MARKET_VAULT: usize = 4usize;
        pub const USER_TOKEN_ACCOUNT: usize = 5usize;
        pub const TOKEN_PROGRAM: usize = 6usize;
    }
    impl<'a> TryFrom<&'a [u8]> for ClaimYieldAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            3usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            4usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            5usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            6usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CollectEarnFee {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CollectEarnFee {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                earn_vault,
                yield_market,
                margin_market,
                margin_market_vault,
                user_token_account,
                oracle,
                admin,
                state,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(earn_vault, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new(margin_market, false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false), ::solana_program::instruction::AccountMeta::new_readonly(admin,
                true), ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CollectEarnFee {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CollectEarnFeeAccountIndexes {
        pub earn_vault: usize,
        pub yield_market: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub user_token_account: usize,
        pub oracle: usize,
        pub admin: usize,
        pub state: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CollectEarnFeeAccountIndexes {
        pub const EARN_VAULT: usize = 0usize;
        pub const YIELD_MARKET: usize = 1usize;
        pub const MARGIN_MARKET: usize = 2usize;
        pub const MARGIN_MARKET_VAULT: usize = 3usize;
        pub const USER_TOKEN_ACCOUNT: usize = 4usize;
        pub const ORACLE: usize = 5usize;
        pub const ADMIN: usize = 6usize;
        pub const STATE: usize = 7usize;
        pub const TOKEN_PROGRAM: usize = 8usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CollectEarnFeeAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                earn_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(earn_vault),
                            0usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            1usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            2usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            3usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            4usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            5usize,
                        ),
                    )?,
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            6usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            7usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            8usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CollectFees {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub token_owner_account: ::solana_program::pubkey::Pubkey,
        pub token_vault_margin: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CollectFees {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                margin_market,
                state,
                authority,
                oracle,
                lp,
                token_owner_account,
                token_vault_margin,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(margin_market,
                false), ::solana_program::instruction::AccountMeta::new_readonly(state,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false), ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account,
                false),
                ::solana_program::instruction::AccountMeta::new(token_vault_margin,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CollectFees {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CollectFeesAccountIndexes {
        pub yield_market: usize,
        pub margin_market: usize,
        pub state: usize,
        pub authority: usize,
        pub oracle: usize,
        pub lp: usize,
        pub token_owner_account: usize,
        pub token_vault_margin: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CollectFeesAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const MARGIN_MARKET: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const AUTHORITY: usize = 3usize;
        pub const ORACLE: usize = 4usize;
        pub const LP: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT: usize = 6usize;
        pub const TOKEN_VAULT_MARGIN: usize = 7usize;
        pub const TOKEN_PROGRAM: usize = 8usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CollectFeesAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            3usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            4usize,
                        ),
                    )?,
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            5usize,
                        ),
                    )?,
                token_owner_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account),
                            6usize,
                        ),
                    )?,
                token_vault_margin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_margin),
                            7usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            8usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CollectProtocolFees {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub ammpools_config: ::solana_program::pubkey::Pubkey,
        pub collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub token_vault_margin: ::solana_program::pubkey::Pubkey,
        pub token_destination: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CollectProtocolFees {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                margin_market,
                ammpools_config,
                collect_protocol_fees_authority,
                yield_market,
                oracle,
                token_vault_margin,
                token_destination,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(ammpools_config,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(collect_protocol_fees_authority,
                true), ::solana_program::instruction::AccountMeta::new(yield_market,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(token_vault_margin,
                false),
                ::solana_program::instruction::AccountMeta::new(token_destination,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::CollectProtocolFees {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CollectProtocolFeesAccountIndexes {
        pub state: usize,
        pub margin_market: usize,
        pub ammpools_config: usize,
        pub collect_protocol_fees_authority: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub token_vault_margin: usize,
        pub token_destination: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CollectProtocolFeesAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const MARGIN_MARKET: usize = 1usize;
        pub const AMMPOOLS_CONFIG: usize = 2usize;
        pub const COLLECT_PROTOCOL_FEES_AUTHORITY: usize = 3usize;
        pub const YIELD_MARKET: usize = 4usize;
        pub const ORACLE: usize = 5usize;
        pub const TOKEN_VAULT_MARGIN: usize = 6usize;
        pub const TOKEN_DESTINATION: usize = 7usize;
        pub const TOKEN_PROGRAM: usize = 8usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CollectProtocolFeesAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            1usize,
                        ),
                    )?,
                ammpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(ammpools_config),
                            2usize,
                        ),
                    )?,
                collect_protocol_fees_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(collect_protocol_fees_authority),
                            3usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            4usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            5usize,
                        ),
                    )?,
                token_vault_margin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_margin),
                            6usize,
                        ),
                    )?,
                token_destination: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_destination),
                            7usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            8usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct DeleteLp {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub user_stats: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub payer: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl DeleteLp {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                lp,
                user_stats,
                state,
                payer,
                authority,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new(user_stats, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(payer, true),
                ::solana_program::instruction::AccountMeta::new(authority, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::DeleteLp {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct DeleteLpAccountIndexes {
        pub lp: usize,
        pub user_stats: usize,
        pub state: usize,
        pub payer: usize,
        pub authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl DeleteLpAccountIndexes {
        pub const LP: usize = 0usize;
        pub const USER_STATS: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const PAYER: usize = 3usize;
        pub const AUTHORITY: usize = 4usize;
    }
    impl<'a> TryFrom<&'a [u8]> for DeleteLpAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            0usize,
                        ),
                    )?,
                user_stats: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_stats),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                payer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(payer),
                            3usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            4usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct DeleteTickArray {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub tick_array: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl DeleteTickArray {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                authority,
                state,
                tick_array,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false), ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(tick_array, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::DeleteTickArray {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct DeleteTickArrayAccountIndexes {
        pub yield_market: usize,
        pub authority: usize,
        pub state: usize,
        pub tick_array: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl DeleteTickArrayAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const TICK_ARRAY: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for DeleteTickArrayAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                tick_array: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct DeleteUser {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub user_stats: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub payer: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl DeleteUser {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                user,
                user_stats,
                state,
                payer,
                authority,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new(user_stats, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(payer, true),
                ::solana_program::instruction::AccountMeta::new(authority, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::DeleteUser {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct DeleteUserAccountIndexes {
        pub user: usize,
        pub user_stats: usize,
        pub state: usize,
        pub payer: usize,
        pub authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl DeleteUserAccountIndexes {
        pub const USER: usize = 0usize;
        pub const USER_STATS: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const PAYER: usize = 3usize;
        pub const AUTHORITY: usize = 4usize;
    }
    impl<'a> TryFrom<&'a [u8]> for DeleteUserAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            0usize,
                        ),
                    )?,
                user_stats: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_stats),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                payer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(payer),
                            3usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            4usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct Deposit {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: i64,
    }
    impl Deposit {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                user,
                authority,
                state,
                margin_market,
                margin_market_vault,
                user_token_account,
                token_program,
                trailing_accounts,
                amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(margin_market, false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::Deposit {
                amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct DepositAccountIndexes {
        pub user: usize,
        pub authority: usize,
        pub state: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub user_token_account: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl DepositAccountIndexes {
        pub const USER: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const MARGIN_MARKET: usize = 3usize;
        pub const MARGIN_MARKET_VAULT: usize = 4usize;
        pub const USER_TOKEN_ACCOUNT: usize = 5usize;
        pub const TOKEN_PROGRAM: usize = 6usize;
    }
    impl<'a> TryFrom<&'a [u8]> for DepositAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            0usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            3usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            4usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            5usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            6usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EarnInvest {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub fee_vault: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub user_fee_account: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub pt_token_account: ::solana_program::pubkey::Pubkey,
        pub pt_mint: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: u64,
    }
    impl EarnInvest {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                earn_vault,
                state,
                authority,
                token_program,
                fee_vault,
                yield_market,
                margin_market,
                margin_market_vault,
                user_token_account,
                user_fee_account,
                oracle,
                observation_state,
                token_owner_account_base,
                token_vault_base,
                token_owner_account_quote,
                token_vault_quote,
                pt_token_account,
                pt_mint,
                associated_token_program,
                system_program,
                trailing_accounts,
                amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(earn_vault, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new(fee_vault,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false), ::solana_program::instruction::AccountMeta::new(user_fee_account,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(observation_state,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false), ::solana_program::instruction::AccountMeta::new(pt_token_account,
                false), ::solana_program::instruction::AccountMeta::new(pt_mint, false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EarnInvest {
                amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EarnInvestAccountIndexes {
        pub earn_vault: usize,
        pub state: usize,
        pub authority: usize,
        pub token_program: usize,
        pub fee_vault: usize,
        pub yield_market: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub user_token_account: usize,
        pub user_fee_account: usize,
        pub oracle: usize,
        pub observation_state: usize,
        pub token_owner_account_base: usize,
        pub token_vault_base: usize,
        pub token_owner_account_quote: usize,
        pub token_vault_quote: usize,
        pub pt_token_account: usize,
        pub pt_mint: usize,
        pub associated_token_program: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EarnInvestAccountIndexes {
        pub const EARN_VAULT: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
        pub const TOKEN_PROGRAM: usize = 3usize;
        pub const FEE_VAULT: usize = 4usize;
        pub const YIELD_MARKET: usize = 5usize;
        pub const MARGIN_MARKET: usize = 6usize;
        pub const MARGIN_MARKET_VAULT: usize = 7usize;
        pub const USER_TOKEN_ACCOUNT: usize = 8usize;
        pub const USER_FEE_ACCOUNT: usize = 9usize;
        pub const ORACLE: usize = 10usize;
        pub const OBSERVATION_STATE: usize = 11usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 12usize;
        pub const TOKEN_VAULT_BASE: usize = 13usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 14usize;
        pub const TOKEN_VAULT_QUOTE: usize = 15usize;
        pub const PT_TOKEN_ACCOUNT: usize = 16usize;
        pub const PT_MINT: usize = 17usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 18usize;
        pub const SYSTEM_PROGRAM: usize = 19usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EarnInvestAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                earn_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(earn_vault),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            3usize,
                        ),
                    )?,
                fee_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_vault),
                            4usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            5usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            6usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            7usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            8usize,
                        ),
                    )?,
                user_fee_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_fee_account),
                            9usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            10usize,
                        ),
                    )?,
                observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation_state),
                            11usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            12usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            13usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            14usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            15usize,
                        ),
                    )?,
                pt_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pt_token_account),
                            16usize,
                        ),
                    )?,
                pt_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pt_mint),
                            17usize,
                        ),
                    )?,
                associated_token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(associated_token_program),
                            18usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            19usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EarnRedeem {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub fee_vault: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub user_fee_account: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub pt_token_account: ::solana_program::pubkey::Pubkey,
        pub pt_mint: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: u64,
        pub sqrt_price_limit: u128,
    }
    impl EarnRedeem {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                earn_vault,
                state,
                authority,
                token_program,
                fee_vault,
                yield_market,
                margin_market,
                margin_market_vault,
                user_token_account,
                user_fee_account,
                oracle,
                observation_state,
                token_owner_account_base,
                token_vault_base,
                token_owner_account_quote,
                token_vault_quote,
                pt_token_account,
                pt_mint,
                associated_token_program,
                system_program,
                trailing_accounts,
                amount,
                sqrt_price_limit,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(earn_vault, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new(fee_vault,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false), ::solana_program::instruction::AccountMeta::new(user_fee_account,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(observation_state,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false), ::solana_program::instruction::AccountMeta::new(pt_token_account,
                false), ::solana_program::instruction::AccountMeta::new(pt_mint, false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EarnRedeem {
                amount,
                sqrt_price_limit,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EarnRedeemAccountIndexes {
        pub earn_vault: usize,
        pub state: usize,
        pub authority: usize,
        pub token_program: usize,
        pub fee_vault: usize,
        pub yield_market: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub user_token_account: usize,
        pub user_fee_account: usize,
        pub oracle: usize,
        pub observation_state: usize,
        pub token_owner_account_base: usize,
        pub token_vault_base: usize,
        pub token_owner_account_quote: usize,
        pub token_vault_quote: usize,
        pub pt_token_account: usize,
        pub pt_mint: usize,
        pub associated_token_program: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EarnRedeemAccountIndexes {
        pub const EARN_VAULT: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
        pub const TOKEN_PROGRAM: usize = 3usize;
        pub const FEE_VAULT: usize = 4usize;
        pub const YIELD_MARKET: usize = 5usize;
        pub const MARGIN_MARKET: usize = 6usize;
        pub const MARGIN_MARKET_VAULT: usize = 7usize;
        pub const USER_TOKEN_ACCOUNT: usize = 8usize;
        pub const USER_FEE_ACCOUNT: usize = 9usize;
        pub const ORACLE: usize = 10usize;
        pub const OBSERVATION_STATE: usize = 11usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 12usize;
        pub const TOKEN_VAULT_BASE: usize = 13usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 14usize;
        pub const TOKEN_VAULT_QUOTE: usize = 15usize;
        pub const PT_TOKEN_ACCOUNT: usize = 16usize;
        pub const PT_MINT: usize = 17usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 18usize;
        pub const SYSTEM_PROGRAM: usize = 19usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EarnRedeemAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                earn_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(earn_vault),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            3usize,
                        ),
                    )?,
                fee_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_vault),
                            4usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            5usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            6usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            7usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            8usize,
                        ),
                    )?,
                user_fee_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_fee_account),
                            9usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            10usize,
                        ),
                    )?,
                observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation_state),
                            11usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            12usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            13usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            14usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            15usize,
                        ),
                    )?,
                pt_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pt_token_account),
                            16usize,
                        ),
                    )?,
                pt_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pt_mint),
                            17usize,
                        ),
                    )?,
                associated_token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(associated_token_program),
                            18usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            19usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EndVaultSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub in_margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub out_margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub in_margin_market: ::solana_program::pubkey::Pubkey,
        pub out_margin_market: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub in_user_token_account: ::solana_program::pubkey::Pubkey,
        pub out_user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub instructions: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: u64,
        pub other_amount_threshold: u64,
        pub is_exact_in: bool,
    }
    impl EndVaultSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                authority,
                in_margin_market_vault,
                out_margin_market_vault,
                in_margin_market,
                out_margin_market,
                yield_market,
                oracle,
                in_user_token_account,
                out_user_token_account,
                token_program,
                instructions,
                trailing_accounts,
                amount,
                other_amount_threshold,
                is_exact_in,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
                ::solana_program::instruction::AccountMeta::new(in_margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(out_margin_market_vault,
                false), ::solana_program::instruction::AccountMeta::new(in_margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(out_margin_market,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(in_user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(out_user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(instructions,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EndVaultSwap {
                amount,
                other_amount_threshold,
                is_exact_in,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EndVaultSwapAccountIndexes {
        pub state: usize,
        pub authority: usize,
        pub in_margin_market_vault: usize,
        pub out_margin_market_vault: usize,
        pub in_margin_market: usize,
        pub out_margin_market: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub in_user_token_account: usize,
        pub out_user_token_account: usize,
        pub token_program: usize,
        pub instructions: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EndVaultSwapAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const IN_MARGIN_MARKET_VAULT: usize = 2usize;
        pub const OUT_MARGIN_MARKET_VAULT: usize = 3usize;
        pub const IN_MARGIN_MARKET: usize = 4usize;
        pub const OUT_MARGIN_MARKET: usize = 5usize;
        pub const YIELD_MARKET: usize = 6usize;
        pub const ORACLE: usize = 7usize;
        pub const IN_USER_TOKEN_ACCOUNT: usize = 8usize;
        pub const OUT_USER_TOKEN_ACCOUNT: usize = 9usize;
        pub const TOKEN_PROGRAM: usize = 10usize;
        pub const INSTRUCTIONS: usize = 11usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EndVaultSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            1usize,
                        ),
                    )?,
                in_margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(in_margin_market_vault),
                            2usize,
                        ),
                    )?,
                out_margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(out_margin_market_vault),
                            3usize,
                        ),
                    )?,
                in_margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(in_margin_market),
                            4usize,
                        ),
                    )?,
                out_margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(out_margin_market),
                            5usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            6usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            7usize,
                        ),
                    )?,
                in_user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(in_user_token_account),
                            8usize,
                        ),
                    )?,
                out_user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(out_user_token_account),
                            9usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            10usize,
                        ),
                    )?,
                instructions: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(instructions),
                            11usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateAdd {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub margin_market_mint: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub market_index: u32,
        pub is_expired: bool,
    }
    impl EpochUpdateAdd {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                authority,
                state,
                yield_market,
                oracle,
                token_vault_base,
                token_vault_quote,
                token_owner_account_base,
                token_owner_account_quote,
                margin_market,
                margin_market_vault,
                margin_market_mint,
                user_token_account,
                token_program,
                associated_token_program,
                system_program,
                trailing_accounts,
                market_index,
                is_expired,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(margin_market_mint,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EpochUpdateAdd {
                market_index,
                is_expired,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateAddAccountIndexes {
        pub authority: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub token_vault_base: usize,
        pub token_vault_quote: usize,
        pub token_owner_account_base: usize,
        pub token_owner_account_quote: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub margin_market_mint: usize,
        pub user_token_account: usize,
        pub token_program: usize,
        pub associated_token_program: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EpochUpdateAddAccountIndexes {
        pub const AUTHORITY: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
        pub const TOKEN_VAULT_BASE: usize = 4usize;
        pub const TOKEN_VAULT_QUOTE: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 6usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 7usize;
        pub const MARGIN_MARKET: usize = 8usize;
        pub const MARGIN_MARKET_VAULT: usize = 9usize;
        pub const MARGIN_MARKET_MINT: usize = 10usize;
        pub const USER_TOKEN_ACCOUNT: usize = 11usize;
        pub const TOKEN_PROGRAM: usize = 12usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 13usize;
        pub const SYSTEM_PROGRAM: usize = 14usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EpochUpdateAddAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            4usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            5usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            6usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            7usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            8usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            9usize,
                        ),
                    )?,
                margin_market_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_mint),
                            10usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            11usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            12usize,
                        ),
                    )?,
                associated_token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(associated_token_program),
                            13usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            14usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateBegin {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub is_expired: bool,
    }
    impl EpochUpdateBegin {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                is_expired,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EpochUpdateBegin {
                is_expired,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateBeginAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EpochUpdateBeginAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EpochUpdateBeginAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateChangePrice {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub is_expired: bool,
    }
    impl EpochUpdateChangePrice {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                is_expired,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EpochUpdateChangePrice {
                is_expired,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateChangePriceAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EpochUpdateChangePriceAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EpochUpdateChangePriceAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateEnd {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub margin_market_mint: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub is_expired: bool,
    }
    impl EpochUpdateEnd {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                token_program,
                margin_market,
                margin_market_vault,
                margin_market_mint,
                user_token_account,
                associated_token_program,
                system_program,
                trailing_accounts,
                is_expired,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(margin_market_mint,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EpochUpdateEnd {
                is_expired,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateEndAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub token_program: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub margin_market_mint: usize,
        pub user_token_account: usize,
        pub associated_token_program: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EpochUpdateEndAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
        pub const TOKEN_PROGRAM: usize = 4usize;
        pub const MARGIN_MARKET: usize = 5usize;
        pub const MARGIN_MARKET_VAULT: usize = 6usize;
        pub const MARGIN_MARKET_MINT: usize = 7usize;
        pub const USER_TOKEN_ACCOUNT: usize = 8usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 9usize;
        pub const SYSTEM_PROGRAM: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EpochUpdateEndAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            4usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            5usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            6usize,
                        ),
                    )?,
                margin_market_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_mint),
                            7usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            8usize,
                        ),
                    )?,
                associated_token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(associated_token_program),
                            9usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            10usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateExpiryApply {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl EpochUpdateExpiryApply {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EpochUpdateExpiryApply {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateExpiryApplyAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EpochUpdateExpiryApplyAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EpochUpdateExpiryApplyAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateExpiryCheck {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl EpochUpdateExpiryCheck {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EpochUpdateExpiryCheck {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateExpiryCheckAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EpochUpdateExpiryCheckAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EpochUpdateExpiryCheckAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateRemove {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub margin_market_mint: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub market_index: u32,
        pub is_expired: bool,
    }
    impl EpochUpdateRemove {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                authority,
                state,
                yield_market,
                oracle,
                token_vault_base,
                token_vault_quote,
                token_owner_account_base,
                token_owner_account_quote,
                margin_market,
                margin_market_vault,
                margin_market_mint,
                user_token_account,
                token_program,
                associated_token_program,
                system_program,
                trailing_accounts,
                market_index,
                is_expired,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(margin_market_mint,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::EpochUpdateRemove {
                market_index,
                is_expired,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct EpochUpdateRemoveAccountIndexes {
        pub authority: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub token_vault_base: usize,
        pub token_vault_quote: usize,
        pub token_owner_account_base: usize,
        pub token_owner_account_quote: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub margin_market_mint: usize,
        pub user_token_account: usize,
        pub token_program: usize,
        pub associated_token_program: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl EpochUpdateRemoveAccountIndexes {
        pub const AUTHORITY: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
        pub const TOKEN_VAULT_BASE: usize = 4usize;
        pub const TOKEN_VAULT_QUOTE: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 6usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 7usize;
        pub const MARGIN_MARKET: usize = 8usize;
        pub const MARGIN_MARKET_VAULT: usize = 9usize;
        pub const MARGIN_MARKET_MINT: usize = 10usize;
        pub const USER_TOKEN_ACCOUNT: usize = 11usize;
        pub const TOKEN_PROGRAM: usize = 12usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 13usize;
        pub const SYSTEM_PROGRAM: usize = 14usize;
    }
    impl<'a> TryFrom<&'a [u8]> for EpochUpdateRemoveAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            4usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            5usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            6usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            7usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            8usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            9usize,
                        ),
                    )?,
                margin_market_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_mint),
                            10usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            11usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            12usize,
                        ),
                    )?,
                associated_token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(associated_token_program),
                            13usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            14usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct FillOrder {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub order_id: u32,
    }
    impl FillOrder {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                margin_market_vault,
                authority,
                token_program,
                yield_market,
                token_owner_account_base,
                token_vault_base,
                token_owner_account_quote,
                token_vault_quote,
                system_program,
                trailing_accounts,
                order_id,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::FillOrder {
                order_id,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct FillOrderAccountIndexes {
        pub state: usize,
        pub margin_market_vault: usize,
        pub authority: usize,
        pub token_program: usize,
        pub yield_market: usize,
        pub token_owner_account_base: usize,
        pub token_vault_base: usize,
        pub token_owner_account_quote: usize,
        pub token_vault_quote: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl FillOrderAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const MARGIN_MARKET_VAULT: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
        pub const TOKEN_PROGRAM: usize = 3usize;
        pub const YIELD_MARKET: usize = 4usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 5usize;
        pub const TOKEN_VAULT_BASE: usize = 6usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 7usize;
        pub const TOKEN_VAULT_QUOTE: usize = 8usize;
        pub const SYSTEM_PROGRAM: usize = 9usize;
    }
    impl<'a> TryFrom<&'a [u8]> for FillOrderAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            3usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            4usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            5usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            6usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            7usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            8usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            9usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct GetAmmTwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub observation: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub seconds_ago: u32,
    }
    impl GetAmmTwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                observation,
                trailing_accounts,
                seconds_ago,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(observation,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::GetAmmTwap {
                seconds_ago,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct GetAmmTwapAccountIndexes {
        pub yield_market: usize,
        pub observation: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl GetAmmTwapAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const OBSERVATION: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for GetAmmTwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                observation: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct Initialize {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub margin_index_start: u32,
        pub market_index_start: u32,
        pub keeper_fee: u64,
    }
    impl Initialize {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                system_program,
                token_program,
                trailing_accounts,
                margin_index_start,
                market_index_start,
                keeper_fee,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::Initialize {
                margin_index_start,
                market_index_start,
                keeper_fee,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub system_program: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const SYSTEM_PROGRAM: usize = 2usize;
        pub const TOKEN_PROGRAM: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            2usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeConfig {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub config: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
        pub reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
        pub default_protocol_fee_rate: u16,
    }
    impl InitializeConfig {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                config,
                admin,
                state,
                system_program,
                trailing_accounts,
                fee_authority,
                collect_protocol_fees_authority,
                reward_emissions_super_authority,
                default_protocol_fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(config, false),
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeConfig {
                fee_authority,
                collect_protocol_fees_authority,
                reward_emissions_super_authority,
                default_protocol_fee_rate,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeConfigAccountIndexes {
        pub config: usize,
        pub admin: usize,
        pub state: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeConfigAccountIndexes {
        pub const CONFIG: usize = 0usize;
        pub const ADMIN: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const SYSTEM_PROGRAM: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeConfigAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(config),
                            0usize,
                        ),
                    )?,
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeEarnVault {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub pt_mint: ::solana_program::pubkey::Pubkey,
        pub mint_metadata: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub token_metadata_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub user_ratio: u64,
    }
    impl InitializeEarnVault {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                earn_vault,
                margin_market,
                yield_market,
                pt_mint,
                mint_metadata,
                state,
                admin,
                rent,
                token_program,
                token_metadata_program,
                system_program,
                trailing_accounts,
                user_ratio,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(earn_vault, false),
                ::solana_program::instruction::AccountMeta::new(margin_market, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new(pt_mint, false),
                ::solana_program::instruction::AccountMeta::new(mint_metadata, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(rent, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_metadata_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeEarnVault {
                user_ratio,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeEarnVaultAccountIndexes {
        pub earn_vault: usize,
        pub margin_market: usize,
        pub yield_market: usize,
        pub pt_mint: usize,
        pub mint_metadata: usize,
        pub state: usize,
        pub admin: usize,
        pub rent: usize,
        pub token_program: usize,
        pub token_metadata_program: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeEarnVaultAccountIndexes {
        pub const EARN_VAULT: usize = 0usize;
        pub const MARGIN_MARKET: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const PT_MINT: usize = 3usize;
        pub const MINT_METADATA: usize = 4usize;
        pub const STATE: usize = 5usize;
        pub const ADMIN: usize = 6usize;
        pub const RENT: usize = 7usize;
        pub const TOKEN_PROGRAM: usize = 8usize;
        pub const TOKEN_METADATA_PROGRAM: usize = 9usize;
        pub const SYSTEM_PROGRAM: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeEarnVaultAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                earn_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(earn_vault),
                            0usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                pt_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pt_mint),
                            3usize,
                        ),
                    )?,
                mint_metadata: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(mint_metadata),
                            4usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            5usize,
                        ),
                    )?,
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            6usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            7usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            8usize,
                        ),
                    )?,
                token_metadata_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_metadata_program),
                            9usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            10usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeFeeTier {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub config: ::solana_program::pubkey::Pubkey,
        pub fee_tier: ::solana_program::pubkey::Pubkey,
        pub funder: ::solana_program::pubkey::Pubkey,
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub tick_spacing: u16,
        pub default_fee_rate: u16,
    }
    impl InitializeFeeTier {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                config,
                fee_tier,
                funder,
                fee_authority,
                system_program,
                trailing_accounts,
                tick_spacing,
                default_fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(config, false),
                ::solana_program::instruction::AccountMeta::new(fee_tier, false),
                ::solana_program::instruction::AccountMeta::new(funder, true),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeFeeTier {
                tick_spacing,
                default_fee_rate,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeFeeTierAccountIndexes {
        pub config: usize,
        pub fee_tier: usize,
        pub funder: usize,
        pub fee_authority: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeFeeTierAccountIndexes {
        pub const CONFIG: usize = 0usize;
        pub const FEE_TIER: usize = 1usize;
        pub const FUNDER: usize = 2usize;
        pub const FEE_AUTHORITY: usize = 3usize;
        pub const SYSTEM_PROGRAM: usize = 4usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeFeeTierAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(config),
                            0usize,
                        ),
                    )?,
                fee_tier: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_tier),
                            1usize,
                        ),
                    )?,
                funder: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(funder),
                            2usize,
                        ),
                    )?,
                fee_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_authority),
                            3usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            4usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeLp {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub user_stats: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub payer: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub sub_account_id: u16,
    }
    impl InitializeLp {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                lp,
                user_stats,
                state,
                authority,
                payer,
                rent,
                system_program,
                trailing_accounts,
                sub_account_id,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new(user_stats, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new(payer, true),
                ::solana_program::instruction::AccountMeta::new_readonly(rent, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeLp {
                sub_account_id,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeLpAccountIndexes {
        pub lp: usize,
        pub user_stats: usize,
        pub state: usize,
        pub authority: usize,
        pub payer: usize,
        pub rent: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeLpAccountIndexes {
        pub const LP: usize = 0usize;
        pub const USER_STATS: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const AUTHORITY: usize = 3usize;
        pub const PAYER: usize = 4usize;
        pub const RENT: usize = 5usize;
        pub const SYSTEM_PROGRAM: usize = 6usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeLpAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            0usize,
                        ),
                    )?,
                user_stats: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_stats),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            3usize,
                        ),
                    )?,
                payer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(payer),
                            4usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            5usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            6usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeMarginMarket {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_mint: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub name: [u8; 32usize],
    }
    impl InitializeMarginMarket {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                margin_market,
                margin_market_mint,
                margin_market_vault,
                state,
                system_program,
                token_program,
                trailing_accounts,
                name,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(margin_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(margin_market_mint,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false), ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeMarginMarket {
                name,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeMarginMarketAccountIndexes {
        pub admin: usize,
        pub margin_market: usize,
        pub margin_market_mint: usize,
        pub margin_market_vault: usize,
        pub state: usize,
        pub system_program: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeMarginMarketAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const MARGIN_MARKET: usize = 1usize;
        pub const MARGIN_MARKET_MINT: usize = 2usize;
        pub const MARGIN_MARKET_VAULT: usize = 3usize;
        pub const STATE: usize = 4usize;
        pub const SYSTEM_PROGRAM: usize = 5usize;
        pub const TOKEN_PROGRAM: usize = 6usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeMarginMarketAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            1usize,
                        ),
                    )?,
                margin_market_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_mint),
                            2usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            3usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            4usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            5usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            6usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeOracle {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub name: [u8; 32usize],
        pub market_rate: u64,
        pub rate: u64,
        pub last_rate: u64,
        pub epoch_start_timestamp: i64,
        pub decimals: u32,
    }
    impl InitializeOracle {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                oracle,
                state,
                system_program,
                trailing_accounts,
                name,
                market_rate,
                rate,
                last_rate,
                epoch_start_timestamp,
                decimals,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(oracle, false),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeOracle {
                name,
                market_rate,
                rate,
                last_rate,
                epoch_start_timestamp,
                decimals,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeOracleAccountIndexes {
        pub admin: usize,
        pub oracle: usize,
        pub state: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeOracleAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const ORACLE: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const SYSTEM_PROGRAM: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeOracleAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeTickArray {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub funder: ::solana_program::pubkey::Pubkey,
        pub tick_array: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub start_tick_index: i32,
    }
    impl InitializeTickArray {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                funder,
                tick_array,
                system_program,
                trailing_accounts,
                start_tick_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false), ::solana_program::instruction::AccountMeta::new(funder, true),
                ::solana_program::instruction::AccountMeta::new(tick_array, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeTickArray {
                start_tick_index,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeTickArrayAccountIndexes {
        pub yield_market: usize,
        pub funder: usize,
        pub tick_array: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeTickArrayAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const FUNDER: usize = 1usize;
        pub const TICK_ARRAY: usize = 2usize;
        pub const SYSTEM_PROGRAM: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeTickArrayAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                funder: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(funder),
                            1usize,
                        ),
                    )?,
                tick_array: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array),
                            2usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeUser {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub user_stats: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub payer: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub sub_account_id: u16,
        pub is_isolated: bool,
    }
    impl InitializeUser {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                user,
                user_stats,
                state,
                authority,
                payer,
                rent,
                system_program,
                trailing_accounts,
                sub_account_id,
                is_isolated,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new(user_stats, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new(payer, true),
                ::solana_program::instruction::AccountMeta::new_readonly(rent, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeUser {
                sub_account_id,
                is_isolated,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeUserAccountIndexes {
        pub user: usize,
        pub user_stats: usize,
        pub state: usize,
        pub authority: usize,
        pub payer: usize,
        pub rent: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeUserAccountIndexes {
        pub const USER: usize = 0usize;
        pub const USER_STATS: usize = 1usize;
        pub const STATE: usize = 2usize;
        pub const AUTHORITY: usize = 3usize;
        pub const PAYER: usize = 4usize;
        pub const RENT: usize = 5usize;
        pub const SYSTEM_PROGRAM: usize = 6usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeUserAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            0usize,
                        ),
                    )?,
                user_stats: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_stats),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            3usize,
                        ),
                    )?,
                payer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(payer),
                            4usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            5usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            6usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeUserStats {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub user_stats: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub payer: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl InitializeUserStats {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                user_stats,
                state,
                authority,
                payer,
                rent,
                system_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(user_stats, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new(payer, true),
                ::solana_program::instruction::AccountMeta::new_readonly(rent, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeUserStats {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeUserStatsAccountIndexes {
        pub user_stats: usize,
        pub state: usize,
        pub authority: usize,
        pub payer: usize,
        pub rent: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeUserStatsAccountIndexes {
        pub const USER_STATS: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
        pub const PAYER: usize = 3usize;
        pub const RENT: usize = 4usize;
        pub const SYSTEM_PROGRAM: usize = 5usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeUserStatsAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                user_stats: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_stats),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                payer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(payer),
                            3usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            4usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            5usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarket {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub lp_margin_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub base_asset_mint: ::solana_program::pubkey::Pubkey,
        pub quote_asset_mint: ::solana_program::pubkey::Pubkey,
        pub base_asset_vault: ::solana_program::pubkey::Pubkey,
        pub quote_asset_vault: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub ammpools_config: ::solana_program::pubkey::Pubkey,
        pub fee_tier: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub tick_spacing: u16,
        pub sqrt_price: u128,
        pub order_step_size: u64,
        pub min_order_size: u64,
        pub min_liquidation_size: u64,
        pub start_ts: i64,
        pub expire_ts: i64,
        pub active_ratio_coef: u64,
        pub margin_type: MarginType,
        pub lp_margin_type: MarginType,
        pub min_lp_amount: u64,
        pub lower_rate_bound: u64,
        pub upper_rate_bound: u64,
        pub bound_percentage: u64,
        pub market_type: MarketType,
        pub name: [u8; 32usize],
    }
    impl InitializeYieldMarket {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                margin_market,
                lp_margin_market,
                oracle,
                base_asset_mint,
                quote_asset_mint,
                base_asset_vault,
                quote_asset_vault,
                token_vault_base,
                token_vault_quote,
                token_program,
                ammpools_config,
                fee_tier,
                observation_state,
                rent,
                system_program,
                trailing_accounts,
                tick_spacing,
                sqrt_price,
                order_step_size,
                min_order_size,
                min_liquidation_size,
                start_ts,
                expire_ts,
                active_ratio_coef,
                margin_type,
                lp_margin_type,
                min_lp_amount,
                lower_rate_bound,
                upper_rate_bound,
                bound_percentage,
                market_type,
                name,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(lp_margin_market,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false), ::solana_program::instruction::AccountMeta::new(base_asset_mint,
                false), ::solana_program::instruction::AccountMeta::new(quote_asset_mint,
                false), ::solana_program::instruction::AccountMeta::new(base_asset_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(quote_asset_vault,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(ammpools_config,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_tier,
                false),
                ::solana_program::instruction::AccountMeta::new(observation_state,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeYieldMarket {
                tick_spacing,
                sqrt_price,
                order_step_size,
                min_order_size,
                min_liquidation_size,
                start_ts,
                expire_ts,
                active_ratio_coef,
                margin_type,
                lp_margin_type,
                min_lp_amount,
                lower_rate_bound,
                upper_rate_bound,
                bound_percentage,
                market_type,
                name,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub margin_market: usize,
        pub lp_margin_market: usize,
        pub oracle: usize,
        pub base_asset_mint: usize,
        pub quote_asset_mint: usize,
        pub base_asset_vault: usize,
        pub quote_asset_vault: usize,
        pub token_vault_base: usize,
        pub token_vault_quote: usize,
        pub token_program: usize,
        pub ammpools_config: usize,
        pub fee_tier: usize,
        pub observation_state: usize,
        pub rent: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeYieldMarketAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const MARGIN_MARKET: usize = 3usize;
        pub const LP_MARGIN_MARKET: usize = 4usize;
        pub const ORACLE: usize = 5usize;
        pub const BASE_ASSET_MINT: usize = 6usize;
        pub const QUOTE_ASSET_MINT: usize = 7usize;
        pub const BASE_ASSET_VAULT: usize = 8usize;
        pub const QUOTE_ASSET_VAULT: usize = 9usize;
        pub const TOKEN_VAULT_BASE: usize = 10usize;
        pub const TOKEN_VAULT_QUOTE: usize = 11usize;
        pub const TOKEN_PROGRAM: usize = 12usize;
        pub const AMMPOOLS_CONFIG: usize = 13usize;
        pub const FEE_TIER: usize = 14usize;
        pub const OBSERVATION_STATE: usize = 15usize;
        pub const RENT: usize = 16usize;
        pub const SYSTEM_PROGRAM: usize = 17usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeYieldMarketAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            3usize,
                        ),
                    )?,
                lp_margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp_margin_market),
                            4usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            5usize,
                        ),
                    )?,
                base_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_asset_mint),
                            6usize,
                        ),
                    )?,
                quote_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_asset_mint),
                            7usize,
                        ),
                    )?,
                base_asset_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_asset_vault),
                            8usize,
                        ),
                    )?,
                quote_asset_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_asset_vault),
                            9usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            10usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            11usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            12usize,
                        ),
                    )?,
                ammpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(ammpools_config),
                            13usize,
                        ),
                    )?,
                fee_tier: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_tier),
                            14usize,
                        ),
                    )?,
                observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation_state),
                            15usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            16usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            17usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketTokenAccountA {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub base_asset_mint: ::solana_program::pubkey::Pubkey,
        pub quote_asset_mint: ::solana_program::pubkey::Pubkey,
        pub base_asset_vault: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub yield_market: ::solana_program::pubkey::Pubkey,
    }
    impl InitializeYieldMarketTokenAccountA {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                base_asset_mint,
                quote_asset_mint,
                base_asset_vault,
                token_program,
                rent,
                system_program,
                trailing_accounts,
                yield_market,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(base_asset_mint, false),
                ::solana_program::instruction::AccountMeta::new(quote_asset_mint, false),
                ::solana_program::instruction::AccountMeta::new(base_asset_vault, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeYieldMarketTokenAccountA {
                yield_market,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketTokenAccountAAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub base_asset_mint: usize,
        pub quote_asset_mint: usize,
        pub base_asset_vault: usize,
        pub token_program: usize,
        pub rent: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeYieldMarketTokenAccountAAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const BASE_ASSET_MINT: usize = 2usize;
        pub const QUOTE_ASSET_MINT: usize = 3usize;
        pub const BASE_ASSET_VAULT: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
        pub const RENT: usize = 6usize;
        pub const SYSTEM_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeYieldMarketTokenAccountAAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                base_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_asset_mint),
                            2usize,
                        ),
                    )?,
                quote_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_asset_mint),
                            3usize,
                        ),
                    )?,
                base_asset_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_asset_vault),
                            4usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            5usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            6usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            7usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketTokenAccountAa {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub base_asset_mint: ::solana_program::pubkey::Pubkey,
        pub quote_asset_mint: ::solana_program::pubkey::Pubkey,
        pub quote_asset_vault: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub yield_market: ::solana_program::pubkey::Pubkey,
    }
    impl InitializeYieldMarketTokenAccountAa {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                base_asset_mint,
                quote_asset_mint,
                quote_asset_vault,
                token_program,
                rent,
                system_program,
                trailing_accounts,
                yield_market,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(base_asset_mint, false),
                ::solana_program::instruction::AccountMeta::new(quote_asset_mint, false),
                ::solana_program::instruction::AccountMeta::new(quote_asset_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeYieldMarketTokenAccountAa {
                yield_market,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketTokenAccountAaAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub base_asset_mint: usize,
        pub quote_asset_mint: usize,
        pub quote_asset_vault: usize,
        pub token_program: usize,
        pub rent: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeYieldMarketTokenAccountAaAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const BASE_ASSET_MINT: usize = 2usize;
        pub const QUOTE_ASSET_MINT: usize = 3usize;
        pub const QUOTE_ASSET_VAULT: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
        pub const RENT: usize = 6usize;
        pub const SYSTEM_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeYieldMarketTokenAccountAaAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                base_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_asset_mint),
                            2usize,
                        ),
                    )?,
                quote_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_asset_mint),
                            3usize,
                        ),
                    )?,
                quote_asset_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_asset_vault),
                            4usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            5usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            6usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            7usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketTokenAccountB {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub base_asset_mint: ::solana_program::pubkey::Pubkey,
        pub quote_asset_mint: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub yield_market: ::solana_program::pubkey::Pubkey,
    }
    impl InitializeYieldMarketTokenAccountB {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                base_asset_mint,
                quote_asset_mint,
                token_vault_base,
                token_program,
                rent,
                system_program,
                trailing_accounts,
                yield_market,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(base_asset_mint, false),
                ::solana_program::instruction::AccountMeta::new(quote_asset_mint, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeYieldMarketTokenAccountB {
                yield_market,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketTokenAccountBAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub base_asset_mint: usize,
        pub quote_asset_mint: usize,
        pub token_vault_base: usize,
        pub token_program: usize,
        pub rent: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeYieldMarketTokenAccountBAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const BASE_ASSET_MINT: usize = 2usize;
        pub const QUOTE_ASSET_MINT: usize = 3usize;
        pub const TOKEN_VAULT_BASE: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
        pub const RENT: usize = 6usize;
        pub const SYSTEM_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeYieldMarketTokenAccountBAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                base_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_asset_mint),
                            2usize,
                        ),
                    )?,
                quote_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_asset_mint),
                            3usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            4usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            5usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            6usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            7usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketTokenAccountBb {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub base_asset_mint: ::solana_program::pubkey::Pubkey,
        pub quote_asset_mint: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub yield_market: ::solana_program::pubkey::Pubkey,
    }
    impl InitializeYieldMarketTokenAccountBb {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                base_asset_mint,
                quote_asset_mint,
                token_vault_quote,
                token_program,
                rent,
                system_program,
                trailing_accounts,
                yield_market,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(base_asset_mint, false),
                ::solana_program::instruction::AccountMeta::new(quote_asset_mint, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::InitializeYieldMarketTokenAccountBb {
                yield_market,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct InitializeYieldMarketTokenAccountBbAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub base_asset_mint: usize,
        pub quote_asset_mint: usize,
        pub token_vault_quote: usize,
        pub token_program: usize,
        pub rent: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeYieldMarketTokenAccountBbAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const BASE_ASSET_MINT: usize = 2usize;
        pub const QUOTE_ASSET_MINT: usize = 3usize;
        pub const TOKEN_VAULT_QUOTE: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
        pub const RENT: usize = 6usize;
        pub const SYSTEM_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeYieldMarketTokenAccountBbAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                base_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_asset_mint),
                            2usize,
                        ),
                    )?,
                quote_asset_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_asset_mint),
                            3usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            4usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            5usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            6usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            7usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct Liquidate {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl Liquidate {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, user, state, authority, trailing_accounts } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::Liquidate {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct LiquidateAccountIndexes {
        pub user: usize,
        pub state: usize,
        pub authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl LiquidateAccountIndexes {
        pub const USER: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for LiquidateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct LiquidateInsurance {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub observation: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub market_index: u32,
        pub adl_finish: bool,
    }
    impl LiquidateInsurance {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                authority,
                yield_market,
                margin_market,
                margin_market_vault,
                oracle,
                observation,
                token_program,
                system_program,
                trailing_accounts,
                market_index,
                adl_finish,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new(yield_market,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(observation,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::LiquidateInsurance {
                market_index,
                adl_finish,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct LiquidateInsuranceAccountIndexes {
        pub state: usize,
        pub authority: usize,
        pub yield_market: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub oracle: usize,
        pub observation: usize,
        pub token_program: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl LiquidateInsuranceAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const MARGIN_MARKET: usize = 3usize;
        pub const MARGIN_MARKET_VAULT: usize = 4usize;
        pub const ORACLE: usize = 5usize;
        pub const OBSERVATION: usize = 6usize;
        pub const TOKEN_PROGRAM: usize = 7usize;
        pub const SYSTEM_PROGRAM: usize = 8usize;
    }
    impl<'a> TryFrom<&'a [u8]> for LiquidateInsuranceAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            3usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            4usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            5usize,
                        ),
                    )?,
                observation: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation),
                            6usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            7usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            8usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct LiquidateLp {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_mint_base: ::solana_program::pubkey::Pubkey,
        pub token_mint_quote: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl LiquidateLp {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                lp,
                state,
                token_vault_base,
                token_vault_quote,
                tick_array_lower,
                tick_array_upper,
                yield_market,
                token_owner_account_base,
                token_owner_account_quote,
                token_mint_base,
                token_mint_quote,
                oracle,
                observation_state,
                token_program,
                authority,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_lower,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_upper,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_base,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_quote,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(observation_state,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::LiquidateLp {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct LiquidateLpAccountIndexes {
        pub lp: usize,
        pub state: usize,
        pub token_vault_base: usize,
        pub token_vault_quote: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub yield_market: usize,
        pub token_owner_account_base: usize,
        pub token_owner_account_quote: usize,
        pub token_mint_base: usize,
        pub token_mint_quote: usize,
        pub oracle: usize,
        pub observation_state: usize,
        pub token_program: usize,
        pub authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl LiquidateLpAccountIndexes {
        pub const LP: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const TOKEN_VAULT_BASE: usize = 2usize;
        pub const TOKEN_VAULT_QUOTE: usize = 3usize;
        pub const TICK_ARRAY_LOWER: usize = 4usize;
        pub const TICK_ARRAY_UPPER: usize = 5usize;
        pub const YIELD_MARKET: usize = 6usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 7usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 8usize;
        pub const TOKEN_MINT_BASE: usize = 9usize;
        pub const TOKEN_MINT_QUOTE: usize = 10usize;
        pub const ORACLE: usize = 11usize;
        pub const OBSERVATION_STATE: usize = 12usize;
        pub const TOKEN_PROGRAM: usize = 13usize;
        pub const AUTHORITY: usize = 14usize;
    }
    impl<'a> TryFrom<&'a [u8]> for LiquidateLpAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            2usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            3usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            4usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            5usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            6usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            7usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            8usize,
                        ),
                    )?,
                token_mint_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_base),
                            9usize,
                        ),
                    )?,
                token_mint_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_quote),
                            10usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            11usize,
                        ),
                    )?,
                observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation_state),
                            12usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            13usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            14usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct LoadObservationState {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl LoadObservationState {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, yield_market, trailing_accounts } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::LoadObservationState {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct LoadObservationStateAccountIndexes {
        pub yield_market: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl LoadObservationStateAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
    }
    impl<'a> TryFrom<&'a [u8]> for LoadObservationStateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct MultiSigDeposit {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: i64,
    }
    impl MultiSigDeposit {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                user,
                authority,
                admin,
                state,
                margin_market,
                margin_market_vault,
                user_token_account,
                token_program,
                trailing_accounts,
                amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(margin_market, false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::MultiSigDeposit {
                amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct MultiSigDepositAccountIndexes {
        pub user: usize,
        pub authority: usize,
        pub admin: usize,
        pub state: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub user_token_account: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl MultiSigDepositAccountIndexes {
        pub const USER: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const ADMIN: usize = 2usize;
        pub const STATE: usize = 3usize;
        pub const MARGIN_MARKET: usize = 4usize;
        pub const MARGIN_MARKET_VAULT: usize = 5usize;
        pub const USER_TOKEN_ACCOUNT: usize = 6usize;
        pub const TOKEN_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for MultiSigDepositAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            0usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            1usize,
                        ),
                    )?,
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            2usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            3usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            4usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            5usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            6usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            7usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct Observe {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub observation: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub seconds_agos: Vec<u32>,
    }
    impl Observe {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                observation,
                trailing_accounts,
                seconds_agos,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(observation,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::Observe {
                seconds_agos,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct ObserveAccountIndexes {
        pub yield_market: usize,
        pub observation: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl ObserveAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const OBSERVATION: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for ObserveAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                observation: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct PlaceOrder {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub params: OrderParams,
    }
    impl PlaceOrder {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                user,
                authority,
                system_program,
                trailing_accounts,
                params,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::PlaceOrder {
                params,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct PlaceOrderAccountIndexes {
        pub state: usize,
        pub user: usize,
        pub authority: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl PlaceOrderAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const USER: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
        pub const SYSTEM_PROGRAM: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for PlaceOrderAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct RemoveKeeper {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub remove_keeper: ::solana_program::pubkey::Pubkey,
    }
    impl RemoveKeeper {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, admin, state, trailing_accounts, remove_keeper } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::RemoveKeeper {
                remove_keeper,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct RemoveKeeperAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl RemoveKeeperAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for RemoveKeeperAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct RemoveLpShares {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_mint_base: ::solana_program::pubkey::Pubkey,
        pub token_mint_quote: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub rm_liquidity_percent: u64,
        pub sqrt_price_limit: u128,
    }
    impl RemoveLpShares {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                authority,
                lp,
                token_vault_base,
                token_vault_quote,
                tick_array_lower,
                tick_array_upper,
                yield_market,
                token_owner_account_base,
                token_owner_account_quote,
                token_mint_base,
                token_mint_quote,
                margin_market,
                margin_market_vault,
                oracle,
                user_token_account,
                observation_state,
                token_program,
                system_program,
                trailing_accounts,
                rm_liquidity_percent,
                sqrt_price_limit,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_lower,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_upper,
                false), ::solana_program::instruction::AccountMeta::new(yield_market,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_base,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_quote,
                false), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(observation_state,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::RemoveLpShares {
                rm_liquidity_percent,
                sqrt_price_limit,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct RemoveLpSharesAccountIndexes {
        pub state: usize,
        pub authority: usize,
        pub lp: usize,
        pub token_vault_base: usize,
        pub token_vault_quote: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub yield_market: usize,
        pub token_owner_account_base: usize,
        pub token_owner_account_quote: usize,
        pub token_mint_base: usize,
        pub token_mint_quote: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub oracle: usize,
        pub user_token_account: usize,
        pub observation_state: usize,
        pub token_program: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl RemoveLpSharesAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const LP: usize = 2usize;
        pub const TOKEN_VAULT_BASE: usize = 3usize;
        pub const TOKEN_VAULT_QUOTE: usize = 4usize;
        pub const TICK_ARRAY_LOWER: usize = 5usize;
        pub const TICK_ARRAY_UPPER: usize = 6usize;
        pub const YIELD_MARKET: usize = 7usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 8usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 9usize;
        pub const TOKEN_MINT_BASE: usize = 10usize;
        pub const TOKEN_MINT_QUOTE: usize = 11usize;
        pub const MARGIN_MARKET: usize = 12usize;
        pub const MARGIN_MARKET_VAULT: usize = 13usize;
        pub const ORACLE: usize = 14usize;
        pub const USER_TOKEN_ACCOUNT: usize = 15usize;
        pub const OBSERVATION_STATE: usize = 16usize;
        pub const TOKEN_PROGRAM: usize = 17usize;
        pub const SYSTEM_PROGRAM: usize = 18usize;
    }
    impl<'a> TryFrom<&'a [u8]> for RemoveLpSharesAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            1usize,
                        ),
                    )?,
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            2usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            3usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            4usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            5usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            6usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            7usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            8usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            9usize,
                        ),
                    )?,
                token_mint_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_base),
                            10usize,
                        ),
                    )?,
                token_mint_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_quote),
                            11usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            12usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            13usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            14usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            15usize,
                        ),
                    )?,
                observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(observation_state),
                            16usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            17usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            18usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct RollbackOracle {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub market_rate: u64,
        pub rate: u64,
        pub last_rate: u64,
        pub epoch_start_timestamp: i64,
        pub last_epoch_start_timestamp: i64,
    }
    impl RollbackOracle {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                oracle,
                state,
                trailing_accounts,
                market_rate,
                rate,
                last_rate,
                epoch_start_timestamp,
                last_epoch_start_timestamp,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(oracle, false),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::RollbackOracle {
                market_rate,
                rate,
                last_rate,
                epoch_start_timestamp,
                last_epoch_start_timestamp,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct RollbackOracleAccountIndexes {
        pub admin: usize,
        pub oracle: usize,
        pub state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl RollbackOracleAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const ORACLE: usize = 1usize;
        pub const STATE: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for RollbackOracleAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetCollateralRatio {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub collateral_ratio_initial: i64,
        pub collateral_ratio_maintenance: i64,
        pub collateral_ratio_initial_pre_expiry: i64,
    }
    impl SetCollateralRatio {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                trailing_accounts,
                collateral_ratio_initial,
                collateral_ratio_maintenance,
                collateral_ratio_initial_pre_expiry,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::SetCollateralRatio {
                collateral_ratio_initial,
                collateral_ratio_maintenance,
                collateral_ratio_initial_pre_expiry,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct SetCollateralRatioAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetCollateralRatioAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetCollateralRatioAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetKeeperFee {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub keeper_fee_per_tx: u64,
    }
    impl SetKeeperFee {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                trailing_accounts,
                keeper_fee_per_tx,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::SetKeeperFee {
                keeper_fee_per_tx,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct SetKeeperFeeAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetKeeperFeeAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetKeeperFeeAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetTwapDuration {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub twap_duration: u32,
    }
    impl SetTwapDuration {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self { program_id, admin, state, trailing_accounts, twap_duration } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::SetTwapDuration {
                twap_duration,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct SetTwapDurationAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetTwapDurationAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetTwapDurationAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SettleExpiryUser {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl SettleExpiryUser {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                authority,
                state,
                yield_market,
                oracle,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::SettleExpiryUser {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct SettleExpiryUserAccountIndexes {
        pub authority: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SettleExpiryUserAccountIndexes {
        pub const AUTHORITY: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SettleExpiryUserAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct TransferBaseToken {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_base: ::solana_program::pubkey::Pubkey,
        pub token_mint_base: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub delta_a: u64,
    }
    impl TransferBaseToken {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                token_vault_base,
                token_owner_account_base,
                token_mint_base,
                token_program,
                trailing_accounts,
                delta_a,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_base, false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_base,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_base,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::TransferBaseToken {
                delta_a,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct TransferBaseTokenAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub token_vault_base: usize,
        pub token_owner_account_base: usize,
        pub token_mint_base: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl TransferBaseTokenAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const TOKEN_VAULT_BASE: usize = 2usize;
        pub const TOKEN_OWNER_ACCOUNT_BASE: usize = 3usize;
        pub const TOKEN_MINT_BASE: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
    }
    impl<'a> TryFrom<&'a [u8]> for TransferBaseTokenAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                token_vault_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_base),
                            2usize,
                        ),
                    )?,
                token_owner_account_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_base),
                            3usize,
                        ),
                    )?,
                token_mint_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_base),
                            4usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            5usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct TransferQuoteToken {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_quote: ::solana_program::pubkey::Pubkey,
        pub token_mint_quote: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub delta_b: u64,
    }
    impl TransferQuoteToken {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                token_vault_quote,
                token_owner_account_quote,
                token_mint_quote,
                token_program,
                trailing_accounts,
                delta_b,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_quote,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::TransferQuoteToken {
                delta_b,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct TransferQuoteTokenAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub token_vault_quote: usize,
        pub token_owner_account_quote: usize,
        pub token_mint_quote: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl TransferQuoteTokenAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const TOKEN_VAULT_QUOTE: usize = 2usize;
        pub const TOKEN_OWNER_ACCOUNT_QUOTE: usize = 3usize;
        pub const TOKEN_MINT_QUOTE: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
    }
    impl<'a> TryFrom<&'a [u8]> for TransferQuoteTokenAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                token_vault_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_quote),
                            2usize,
                        ),
                    )?,
                token_owner_account_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_quote),
                            3usize,
                        ),
                    )?,
                token_mint_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_quote),
                            4usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            5usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateFeesAndRewards {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub lp: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl UpdateFeesAndRewards {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                authority,
                lp,
                tick_array_lower,
                tick_array_upper,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new(lp, false),
                ::solana_program::instruction::AccountMeta::new_readonly(tick_array_lower,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(tick_array_upper,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateFeesAndRewards {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateFeesAndRewardsAccountIndexes {
        pub yield_market: usize,
        pub authority: usize,
        pub lp: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateFeesAndRewardsAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const LP: usize = 2usize;
        pub const TICK_ARRAY_LOWER: usize = 3usize;
        pub const TICK_ARRAY_UPPER: usize = 4usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateFeesAndRewardsAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            1usize,
                        ),
                    )?,
                lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp),
                            2usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            3usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            4usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateOracle {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub market_rate: u64,
        pub rate: u64,
        pub last_rate: u64,
        pub epoch_start_timestamp: i64,
    }
    impl UpdateOracle {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                oracle,
                state,
                trailing_accounts,
                market_rate,
                rate,
                last_rate,
                epoch_start_timestamp,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(admin, true),
                ::solana_program::instruction::AccountMeta::new(oracle, false),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateOracle {
                market_rate,
                rate,
                last_rate,
                epoch_start_timestamp,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateOracleAccountIndexes {
        pub admin: usize,
        pub oracle: usize,
        pub state: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateOracleAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const ORACLE: usize = 1usize;
        pub const STATE: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateOracleAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            1usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateSpotYieldMarketCollateralRatio {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl UpdateSpotYieldMarketCollateralRatio {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateSpotYieldMarketCollateralRatio {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateSpotYieldMarketCollateralRatioAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateSpotYieldMarketCollateralRatioAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateSpotYieldMarketCollateralRatioAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateTickLiquidity {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub tick_array: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub tick_index: i32,
        pub new_liquidity: u128,
    }
    impl UpdateTickLiquidity {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                tick_array,
                trailing_accounts,
                tick_index,
                new_liquidity,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(yield_market,
                false), ::solana_program::instruction::AccountMeta::new(tick_array,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateTickLiquidity {
                tick_index,
                new_liquidity,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateTickLiquidityAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub tick_array: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateTickLiquidityAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const TICK_ARRAY: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateTickLiquidityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                tick_array: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateUserPosition {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub base_asset_amount: i64,
        pub quote_asset_amount: i64,
    }
    impl UpdateUserPosition {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                user,
                trailing_accounts,
                base_asset_amount,
                quote_asset_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(user, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateUserPosition {
                base_asset_amount,
                quote_asset_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateUserPositionAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub user: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateUserPositionAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const USER: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateUserPositionAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarket {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub number_of_active_lps: u64,
    }
    impl UpdateYieldMarket {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                number_of_active_lps,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarket {
                number_of_active_lps,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketActiveRatioCoef {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub active_ratio_coef: u64,
    }
    impl UpdateYieldMarketActiveRatioCoef {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                active_ratio_coef,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketActiveRatioCoef {
                active_ratio_coef,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketActiveRatioCoefAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketActiveRatioCoefAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketActiveRatioCoefAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketCollateralRatioInitialPreExpiry {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub collateral_ratio_initial_pre_expiry: i64,
    }
    impl UpdateYieldMarketCollateralRatioInitialPreExpiry {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                collateral_ratio_initial_pre_expiry,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketCollateralRatioInitialPreExpiry {
                collateral_ratio_initial_pre_expiry,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketCollateralRatioInitialPreExpiryAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketCollateralRatioInitialPreExpiryAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]>
    for UpdateYieldMarketCollateralRatioInitialPreExpiryAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketCollateralRatioMaintenance {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub collateral_ratio_maintenance: i64,
    }
    impl UpdateYieldMarketCollateralRatioMaintenance {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                collateral_ratio_maintenance,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketCollateralRatioMaintenance {
                collateral_ratio_maintenance,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketCollateralRatioMaintenanceAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketCollateralRatioMaintenanceAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]>
    for UpdateYieldMarketCollateralRatioMaintenanceAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketExpireTotalPosQuoteAmount {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub expire_total_pos_quote_amount: i64,
    }
    impl UpdateYieldMarketExpireTotalPosQuoteAmount {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                expire_total_pos_quote_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketExpireTotalPosQuoteAmount {
                expire_total_pos_quote_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketExpireTotalPosQuoteAmountAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketExpireTotalPosQuoteAmountAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]>
    for UpdateYieldMarketExpireTotalPosQuoteAmountAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketExpireTs {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub expire_ts: i64,
    }
    impl UpdateYieldMarketExpireTs {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                expire_ts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketExpireTs {
                expire_ts,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketExpireTsAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketExpireTsAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketExpireTsAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketFeeRate {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub fee_rate: u16,
    }
    impl UpdateYieldMarketFeeRate {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketFeeRate {
                fee_rate,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketFeeRateAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketFeeRateAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketFeeRateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketInsurance {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub base_asset_amount: i64,
        pub quote_asset_amount: i64,
    }
    impl UpdateYieldMarketInsurance {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                base_asset_amount,
                quote_asset_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketInsurance {
                base_asset_amount,
                quote_asset_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketInsuranceAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketInsuranceAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketInsuranceAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketKeeperFee {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub keeper_fee: i64,
    }
    impl UpdateYieldMarketKeeperFee {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                keeper_fee,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketKeeperFee {
                keeper_fee,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketKeeperFeeAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketKeeperFeeAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketKeeperFeeAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketLiqFeeRate {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub liq_fee_rate: i64,
    }
    impl UpdateYieldMarketLiqFeeRate {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                liq_fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketLiqFeeRate {
                liq_fee_rate,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketLiqFeeRateAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketLiqFeeRateAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketLiqFeeRateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketLowerUpperRateBound {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub lower_rate_bound: u64,
        pub upper_rate_bound: u64,
    }
    impl UpdateYieldMarketLowerUpperRateBound {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                lower_rate_bound,
                upper_rate_bound,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketLowerUpperRateBound {
                lower_rate_bound,
                upper_rate_bound,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketLowerUpperRateBoundAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketLowerUpperRateBoundAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketLowerUpperRateBoundAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketLpAccountsProcessed {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub lp_accounts_processed: u64,
    }
    impl UpdateYieldMarketLpAccountsProcessed {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                lp_accounts_processed,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketLpAccountsProcessed {
                lp_accounts_processed,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketLpAccountsProcessedAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketLpAccountsProcessedAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketLpAccountsProcessedAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketMarginDecimalsAndLpMarginDecimals {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub margin_decimals: u8,
        pub lp_margin_decimals: u8,
    }
    impl UpdateYieldMarketMarginDecimalsAndLpMarginDecimals {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                margin_decimals,
                lp_margin_decimals,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketMarginDecimalsAndLpMarginDecimals {
                margin_decimals,
                lp_margin_decimals,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketMarginDecimalsAndLpMarginDecimalsAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketMarginDecimalsAndLpMarginDecimalsAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]>
    for UpdateYieldMarketMarginDecimalsAndLpMarginDecimalsAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketMinLpAmount {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub min_lp_amount: u64,
    }
    impl UpdateYieldMarketMinLpAmount {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                min_lp_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketMinLpAmount {
                min_lp_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketMinLpAmountAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketMinLpAmountAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketMinLpAmountAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketMinOrderSize {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub min_order_size: u64,
    }
    impl UpdateYieldMarketMinOrderSize {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                min_order_size,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketMinOrderSize {
                min_order_size,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketMinOrderSizeAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketMinOrderSizeAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketMinOrderSizeAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketNetBaseAmount {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub net_base_amount: i64,
    }
    impl UpdateYieldMarketNetBaseAmount {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                net_base_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketNetBaseAmount {
                net_base_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketNetBaseAmountAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketNetBaseAmountAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketNetBaseAmountAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketNetQuoteAmount {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub net_quote_amount: i64,
    }
    impl UpdateYieldMarketNetQuoteAmount {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                net_quote_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketNetQuoteAmount {
                net_quote_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketNetQuoteAmountAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketNetQuoteAmountAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketNetQuoteAmountAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketNumberOfActiveUsers {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub number_of_active_users: u64,
    }
    impl UpdateYieldMarketNumberOfActiveUsers {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                number_of_active_users,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketNumberOfActiveUsers {
                number_of_active_users,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketNumberOfActiveUsersAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketNumberOfActiveUsersAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketNumberOfActiveUsersAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketNumberOfProcessedUsers {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub number_of_processed_users: u64,
    }
    impl UpdateYieldMarketNumberOfProcessedUsers {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                number_of_processed_users,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketNumberOfProcessedUsers {
                number_of_processed_users,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketNumberOfProcessedUsersAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketNumberOfProcessedUsersAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]>
    for UpdateYieldMarketNumberOfProcessedUsersAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketOracle {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl UpdateYieldMarketOracle {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketOracle {
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketOracleAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketOracleAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketOracleAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketOrderStepSize {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub order_step_size: u64,
    }
    impl UpdateYieldMarketOrderStepSize {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                order_step_size,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketOrderStepSize {
                order_step_size,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketOrderStepSizeAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketOrderStepSizeAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketOrderStepSizeAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketPoolLiquidity {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub liquidity: u128,
    }
    impl UpdateYieldMarketPoolLiquidity {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                liquidity,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketPoolLiquidity {
                liquidity,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketPoolLiquidityAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketPoolLiquidityAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketPoolLiquidityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketPtData {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub mint_metadata: ::solana_program::pubkey::Pubkey,
        pub token_metadata_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub name: [u8; 32usize],
        pub symbol: [u8; 10usize],
        pub uri: [u8; 200usize],
    }
    impl UpdateYieldMarketPtData {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                mint_metadata,
                token_metadata_program,
                trailing_accounts,
                name,
                symbol,
                uri,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(mint_metadata, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_metadata_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketPtData {
                name,
                symbol,
                uri,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketPtDataAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub mint_metadata: usize,
        pub token_metadata_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketPtDataAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const MINT_METADATA: usize = 2usize;
        pub const TOKEN_METADATA_PROGRAM: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketPtDataAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                mint_metadata: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(mint_metadata),
                            2usize,
                        ),
                    )?,
                token_metadata_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_metadata_program),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketSocialLoss {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub base_asset_amount: i64,
        pub quote_asset_amount: i64,
    }
    impl UpdateYieldMarketSocialLoss {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                base_asset_amount,
                quote_asset_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketSocialLoss {
                base_asset_amount,
                quote_asset_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketSocialLossAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketSocialLossAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketSocialLossAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketStartTs {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub start_ts: i64,
    }
    impl UpdateYieldMarketStartTs {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                start_ts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketStartTs {
                start_ts,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketStartTsAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketStartTsAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketStartTsAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketStatus {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub status: MarketStatus,
    }
    impl UpdateYieldMarketStatus {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                status,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketStatus {
                status,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketStatusAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketStatusAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketStatusAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketTickIndex {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
    }
    impl UpdateYieldMarketTickIndex {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                authority,
                state,
                yield_market,
                trailing_accounts,
                tick_lower_index,
                tick_upper_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketTickIndex {
                tick_lower_index,
                tick_upper_index,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketTickIndexAccountIndexes {
        pub authority: usize,
        pub state: usize,
        pub yield_market: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketTickIndexAccountIndexes {
        pub const AUTHORITY: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateYieldMarketTickIndexAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketTotalReserveQuoteAndBase {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub admin: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub total_reserve_quote_amount: i64,
        pub total_reserve_base_amount: i64,
    }
    impl UpdateYieldMarketTotalReserveQuoteAndBase {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                admin,
                state,
                yield_market,
                oracle,
                trailing_accounts,
                total_reserve_quote_amount,
                total_reserve_base_amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(admin, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::UpdateYieldMarketTotalReserveQuoteAndBase {
                total_reserve_quote_amount,
                total_reserve_base_amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct UpdateYieldMarketTotalReserveQuoteAndBaseAccountIndexes {
        pub admin: usize,
        pub state: usize,
        pub yield_market: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateYieldMarketTotalReserveQuoteAndBaseAccountIndexes {
        pub const ADMIN: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const YIELD_MARKET: usize = 2usize;
        pub const ORACLE: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]>
    for UpdateYieldMarketTotalReserveQuoteAndBaseAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                admin: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin),
                            0usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            1usize,
                        ),
                    )?,
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            2usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct VaultTransfer {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub lp_margin_market: ::solana_program::pubkey::Pubkey,
        pub other_margin_market: ::solana_program::pubkey::Pubkey,
        pub lp_vault: ::solana_program::pubkey::Pubkey,
        pub other_vault: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub from_lp_amount: i64,
        pub is_earn: bool,
    }
    impl VaultTransfer {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                yield_market,
                earn_vault,
                lp_margin_market,
                other_margin_market,
                lp_vault,
                other_vault,
                authority,
                state,
                oracle,
                token_program,
                trailing_accounts,
                from_lp_amount,
                is_earn,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(yield_market, false),
                ::solana_program::instruction::AccountMeta::new(earn_vault, false),
                ::solana_program::instruction::AccountMeta::new(lp_margin_market, false),
                ::solana_program::instruction::AccountMeta::new(other_margin_market,
                false), ::solana_program::instruction::AccountMeta::new(lp_vault, false),
                ::solana_program::instruction::AccountMeta::new(other_vault, false),
                ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new_readonly(state, false),
                ::solana_program::instruction::AccountMeta::new_readonly(oracle, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::VaultTransfer {
                from_lp_amount,
                is_earn,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct VaultTransferAccountIndexes {
        pub yield_market: usize,
        pub earn_vault: usize,
        pub lp_margin_market: usize,
        pub other_margin_market: usize,
        pub lp_vault: usize,
        pub other_vault: usize,
        pub authority: usize,
        pub state: usize,
        pub oracle: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl VaultTransferAccountIndexes {
        pub const YIELD_MARKET: usize = 0usize;
        pub const EARN_VAULT: usize = 1usize;
        pub const LP_MARGIN_MARKET: usize = 2usize;
        pub const OTHER_MARGIN_MARKET: usize = 3usize;
        pub const LP_VAULT: usize = 4usize;
        pub const OTHER_VAULT: usize = 5usize;
        pub const AUTHORITY: usize = 6usize;
        pub const STATE: usize = 7usize;
        pub const ORACLE: usize = 8usize;
        pub const TOKEN_PROGRAM: usize = 9usize;
    }
    impl<'a> TryFrom<&'a [u8]> for VaultTransferAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                yield_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(yield_market),
                            0usize,
                        ),
                    )?,
                earn_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(earn_vault),
                            1usize,
                        ),
                    )?,
                lp_margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp_margin_market),
                            2usize,
                        ),
                    )?,
                other_margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(other_margin_market),
                            3usize,
                        ),
                    )?,
                lp_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(lp_vault),
                            4usize,
                        ),
                    )?,
                other_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(other_vault),
                            5usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            6usize,
                        ),
                    )?,
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            7usize,
                        ),
                    )?,
                oracle: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(oracle),
                            8usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            9usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct Withdraw {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
        pub user_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: i64,
    }
    impl Withdraw {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                state,
                user,
                authority,
                margin_market,
                margin_market_vault,
                user_token_account,
                token_program,
                trailing_accounts,
                amount,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(user, false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true), ::solana_program::instruction::AccountMeta::new(margin_market,
                false),
                ::solana_program::instruction::AccountMeta::new(margin_market_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(user_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = RatexContractsInstruction::Withdraw {
                amount,
            }
                .pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct WithdrawAccountIndexes {
        pub state: usize,
        pub user: usize,
        pub authority: usize,
        pub margin_market: usize,
        pub margin_market_vault: usize,
        pub user_token_account: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl WithdrawAccountIndexes {
        pub const STATE: usize = 0usize;
        pub const USER: usize = 1usize;
        pub const AUTHORITY: usize = 2usize;
        pub const MARGIN_MARKET: usize = 3usize;
        pub const MARGIN_MARKET_VAULT: usize = 4usize;
        pub const USER_TOKEN_ACCOUNT: usize = 5usize;
        pub const TOKEN_PROGRAM: usize = 6usize;
    }
    impl<'a> TryFrom<&'a [u8]> for WithdrawAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(state),
                            0usize,
                        ),
                    )?,
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            1usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            2usize,
                        ),
                    )?,
                margin_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market),
                            3usize,
                        ),
                    )?,
                margin_market_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(margin_market_vault),
                            4usize,
                        ),
                    )?,
                user_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_token_account),
                            5usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            6usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
}
pub mod types {
    #[derive(Clone, Copy, Default)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Ammpool {
        pub ammpools_config: ::solana_program::pubkey::Pubkey,
        pub liquidity: u128,
        pub sqrt_price: u128,
        pub protocol_fee_owed_a: u64,
        pub protocol_fee_owed_b: u64,
        pub token_mint_base: ::solana_program::pubkey::Pubkey,
        pub token_vault_base: ::solana_program::pubkey::Pubkey,
        pub fee_growth_global_a: u128,
        pub token_mint_quote: ::solana_program::pubkey::Pubkey,
        pub token_vault_quote: ::solana_program::pubkey::Pubkey,
        pub fee_growth_global_b: u128,
        pub reward_last_updated_timestamp: u64,
        pub reward_infos: [AmmpoolRewardInfo; 3usize],
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub tick_current_index: i32,
        #[doc = concat!(
            " ",
            "the most-recently updated index of the observations array"
        )]
        pub observation_index: u16,
        pub observation_update_duration: u16,
        pub tick_spacing: u16,
        pub tick_spacing_seed: [u8; 2usize],
        pub fee_rate: u16,
        pub protocol_fee_rate: u16,
        pub ammpool_bump: [u8; 1usize],
        pub padding: [u8; 7usize],
    }
    unsafe impl ::bytemuck::Pod for Ammpool {}
    unsafe impl ::bytemuck::Zeroable for Ammpool {}
    #[doc = concat!(
        " ",
        "Stores the state relevant for tracking liquidity mining rewards at the `Ammpool` level."
    )]
    #[doc = concat!(
        " ",
        "These values are used in conjunction with `PositionRewardInfo`, `Tick.reward_growths_outside`,"
    )]
    #[doc = concat!(
        " ",
        "and `Ammpool.reward_last_updated_timestamp` to determine how many rewards are earned by open"
    )]
    #[doc = concat!(" ", "positions.")]
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct AmmpoolRewardInfo {
        #[doc = concat!(" ", "Reward token mint.")]
        pub mint: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "Reward vault token account.")]
        pub vault: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(
            " ",
            "Authority account that has permission to initialize the reward and set emissions."
        )]
        pub authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(
            " ",
            "Q64.64 number that indicates how many tokens per second are earned per unit of liquidity."
        )]
        pub emissions_per_second_x64: u128,
        #[doc = concat!(
            " ",
            "Q64.64 number that tracks the total tokens earned per unit of liquidity since the reward"
        )]
        #[doc = concat!(" ", "emissions were turned on.")]
        pub growth_global_x64: u128,
    }
    #[doc = concat!(" ", "cancel_yield_order event")]
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct CancelOrderRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(
            " ",
            "The id for the order. Each users has their own order id space"
        )]
        pub order_id: u32,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct ClaimYieldRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "margin token index")]
        pub margin_index: u32,
        pub market_index: u32,
        pub amount: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct CollectEarnProtocolFeeRecord {
        pub admin: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub fee: u64,
        pub fee_amount: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct CollectFeesRecord {
        pub market_index: u32,
        pub user: ::solana_program::pubkey::Pubkey,
        pub fee_a: u64,
        pub fee_b: u64,
        pub fee_amount: u64,
        pub tick_lower: i32,
        #[doc = concat!(" ", "the upper tick")]
        pub tick_upper: i32,
        #[doc = concat!(" ", "the lower rate")]
        pub rate_lower: u64,
        #[doc = concat!(" ", "the upper rate")]
        pub rate_upper: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct CollectProtocolFeesRecord {
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub fee_a: u64,
        pub fee_b: u64,
        pub fee_amount: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct DecreaseLiquidityEvent {
        pub position: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub liquidity_amount: u128,
        pub token_a: u64,
        pub token_b: u64,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct DeleteLpRecord {
        #[doc = concat!(" ", "unix_timestamp of action")]
        pub ts: i64,
        #[doc = concat!(" ", "owner of subaccount")]
        pub lp_authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount")]
        pub lp: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount id")]
        pub sub_account_id: u16,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct DeleteTickArrayEvent {
        pub authority: ::solana_program::pubkey::Pubkey,
        pub tick_array: ::solana_program::pubkey::Pubkey,
        pub start_tick_index: i32,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct DeleteUserOrdersRecord {
        #[doc = concat!(" ", "unix_timestamp of action")]
        pub ts: i64,
        #[doc = concat!(" ", "owner of subaccount")]
        pub user_authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount")]
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount id")]
        pub sub_account_id: u16,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct DeleteUserRecord {
        #[doc = concat!(" ", "unix_timestamp of action")]
        pub ts: i64,
        #[doc = concat!(" ", "owner of subaccount")]
        pub user_authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount")]
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount id")]
        pub sub_account_id: u16,
    }
    #[doc = concat!(" ", "deposit/withdraw event")]
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum DepositDirection {
        #[doc = concat!(" ", "deposit/withdraw event")]
        Deposit,
        #[doc = concat!(" ", "deposit/withdraw event")]
        Withdraw,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct DepositRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "deposit/withdraw")]
        pub direction: DepositDirection,
        #[doc = concat!(" ", "margin token index")]
        pub market_index: u32,
        #[doc = concat!(" ", "record id, Each users has their own order id space")]
        pub deposit_record_id: u64,
        #[doc = concat!(" ", "the deposited/withdrawn amount")]
        pub amount: i64,
        #[doc = concat!(" ", "total balance")]
        pub total_balance: i64,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum EarnDirection {
        Invest,
        Redeem,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct EarnRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub direction: EarnDirection,
        pub base_amount_filled: i64,
        #[doc = concat!(" ", "amount of traded vt")]
        pub quote_amount_filled: i64,
        #[doc = concat!(" ", "amount of held yt after trade")]
        pub base_amount_held: i64,
        #[doc = concat!(" ", "amount of held vt after trade")]
        pub quote_amount_held: i64,
        pub pt_amount: u64,
        pub pt_amount_held: u64,
        pub margin_amount: u64,
        pub fee_amount: u64,
        pub rate: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct EpochUpdateAddSocialLossRecord {
        pub lp: ::solana_program::pubkey::Pubkey,
        pub debt_covered: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct EpochUpdateBeginRecord {
        pub keeper: ::solana_program::pubkey::Pubkey,
        pub epoch_start_timestamp: i64,
        pub implied_rate: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct EpochUpdateChangePriceRecord {
        pub sqrt_price_new: u128,
        pub epoch_start_timestamp: i64,
        pub ttm: i64,
        pub yield_market: ::solana_program::pubkey::Pubkey,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct EpochUpdateEndRecord {
        pub epoch_update_end_ts: i64,
        pub total_quote_asset_amount: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct EpochUpdateExpiryRecord {
        pub user: ::solana_program::pubkey::Pubkey,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct EpochUpdateLpRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub market_index: u32,
        pub direction: LpDirection,
        pub delta_base_asset_amount: i64,
        pub delta_quote_asset_amount: i64,
        #[doc = concat!(" ", "the lower tick")]
        pub tick_lower: i32,
        #[doc = concat!(" ", "the upper tick")]
        pub tick_upper: i32,
        #[doc = concat!(" ", "the lower rate")]
        pub rate_lower: u64,
        #[doc = concat!(" ", "the upper rate")]
        pub rate_upper: u64,
        #[doc = concat!(" ", "liquidity amount")]
        pub liquidity_amount: u128,
        pub reserve_base_amount: i64,
        pub reserve_quote_amount: i64,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum EpochUpdateStatus {
        Begin,
        Remove,
        ChangePrice,
        Expiry,
        ExpiryApply,
        Add,
        End,
    }
    #[doc = concat!(" ", "fill_yield_order event")]
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct FillOrderRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(
            " ",
            "The id for the order. Each users has their own order id space"
        )]
        pub order_id: u32,
        #[doc = concat!(" ", "The keeper")]
        pub filler: Option<::solana_program::pubkey::Pubkey>,
        #[doc = concat!(" ", "market index")]
        pub market_index: u32,
        #[doc = concat!(" ", "amount of traded yt")]
        pub base_amount_filled: i64,
        #[doc = concat!(" ", "amount of traded vt")]
        pub quote_amount_filled: i64,
        #[doc = concat!(" ", "amount of held yt after trade")]
        pub base_amount_held: i64,
        #[doc = concat!(" ", "amount of held vt after trade")]
        pub quote_amount_held: i64,
        #[doc = concat!(" ", "the average trading price in quote token")]
        pub trade_price: i64,
        #[doc = concat!(" ", "transaction fees")]
        pub fee: i64,
        #[doc = concat!(" ", "current oracle rate")]
        pub rate: u64,
        #[doc = concat!(" ", "realized pnl")]
        pub realized_pnl: i64,
        #[doc = concat!(" ", "is close flag in order params")]
        pub order_is_close: bool,
        #[doc = concat!(" ", "user margin balance")]
        pub total_balance: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct IncreaseLiquidityEvent {
        pub position: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub liquidity_amount: u128,
        pub token_a: u64,
        pub token_b: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct InitializeConfigEvent {
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
        pub reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
        pub default_protocol_fee_rate: u16,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct InitializeFeeTierEvent {
        pub fee_tier: ::solana_program::pubkey::Pubkey,
        pub tick_spacing: u16,
        pub default_fee_rate: u16,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct InitializeMarginMarketRecord {
        pub market_index: u32,
        pub margin_market: ::solana_program::pubkey::Pubkey,
        pub margin_market_mint: ::solana_program::pubkey::Pubkey,
        pub margin_market_vault: ::solana_program::pubkey::Pubkey,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct InitializePositionEvent {
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub owner: ::solana_program::pubkey::Pubkey,
        pub lower_rate: u64,
        pub upper_rate: u64,
        pub lower_tick_index: i32,
        pub upper_tick_index: i32,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct InitializeTickArrayEvent {
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub start_tick_index: i32,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct InitializeYieldMarketRecord {
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub name: [u8; 32usize],
        pub pubkey: ::solana_program::pubkey::Pubkey,
        pub market_index: u32,
        pub margin_index: u32,
        pub lp_margin_index: u32,
        pub order_step_size: u64,
        pub min_order_size: u64,
        pub min_liquidation_size: u64,
        pub start_ts: i64,
        pub expire_ts: i64,
        pub active_ratio_coef: u64,
        pub min_lp_amount: u64,
        pub default_fee_rate: u16,
        pub tick_spacing: u16,
        pub initial_sqrt_price: u128,
        #[doc = concat!(" ", "yield_market")]
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub quote_asset_mint: ::solana_program::pubkey::Pubkey,
        pub base_asset_mint: ::solana_program::pubkey::Pubkey,
        pub quote_asset_vault: ::solana_program::pubkey::Pubkey,
        pub base_asset_vault: ::solana_program::pubkey::Pubkey,
        pub observation_state: ::solana_program::pubkey::Pubkey,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct InsuranceLiquidationRecord {
        pub ts: i64,
        #[doc = concat!(" ", "market index")]
        pub market_index: u32,
        #[doc = concat!(" ", "amount of traded yt")]
        pub base_amount_filled: i64,
        #[doc = concat!(" ", "amount of traded vt")]
        pub quote_amount_filled: i64,
        #[doc = concat!(" ", "amount of held yt after trade")]
        pub base_amount_held: i64,
        #[doc = concat!(" ", "amount of held vt after trade")]
        pub quote_amount_held: i64,
        #[doc = concat!(" ", "the average trading price in quote token")]
        pub trade_price: i64,
        #[doc = concat!(" ", "realized pnl")]
        pub realized_pnl: i64,
        #[doc = concat!(" ", "margin balance")]
        pub total_balance: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct InsuranceTransferPositionRecord {
        pub ts: i64,
        pub market_index: u32,
        pub base_amount_transfered: i64,
        pub quote_amount_transfered: i64,
        pub margin_transfered: i64,
        pub base_amount_held: i64,
        pub quote_amount_held: i64,
        pub total_balance: i64,
        pub rate: u64,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum LpDirection {
        AddLiquidity,
        RemoveLiquidity,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct LpRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "add/remove liquidity")]
        pub direction: LpDirection,
        #[doc = concat!(" ", "the margin index")]
        pub margin_index: u32,
        #[doc = concat!(" ", "the yield market index")]
        pub market_index: u32,
        #[doc = concat!(
            " ",
            "the change to amm pool, positive for adding liquidity, negative for removing liquidity"
        )]
        pub delta_base_asset_amount: i64,
        pub delta_quote_asset_amount: i64,
        #[doc = concat!(" ", "the lower tick")]
        pub tick_lower: i32,
        #[doc = concat!(" ", "the upper tick")]
        pub tick_upper: i32,
        #[doc = concat!(" ", "the lower rate")]
        pub rate_lower: u64,
        #[doc = concat!(" ", "the upper rate")]
        pub rate_upper: u64,
        #[doc = concat!(" ", "the deposited amount of margin mint")]
        pub margin_amount: i64,
        #[doc = concat!(" ", "minted/burned quote asset amount")]
        pub minted_quote_amount: i64,
        #[doc = concat!(" ", "liquidity amount")]
        pub liquidity_amount: u128,
        pub reserve_base_amount: i64,
        pub reserve_quote_amount: i64,
        #[doc = concat!(" ", "total_minted_quote_amount")]
        pub total_quote_asset_amount: i64,
        #[doc = concat!(" ", "the total deposited amount of the lp mint;")]
        pub total_margin_amount: i64,
        pub is_active: bool,
        pub social_loss_base_amount_filled: i64,
        pub social_loss_quote_amount_filled: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct LiquidationRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "market index")]
        pub market_index: u32,
        #[doc = concat!(" ", "amount of traded yt")]
        pub base_amount_filled: i64,
        #[doc = concat!(" ", "amount of traded vt")]
        pub quote_amount_filled: i64,
        #[doc = concat!(" ", "amount of held yt after trade")]
        pub base_amount_held: i64,
        #[doc = concat!(" ", "amount of held vt after trade")]
        pub quote_amount_held: i64,
        #[doc = concat!(" ", "the average trading price in quote token")]
        pub trade_price: i64,
        #[doc = concat!(" ", "transaction fees")]
        pub fee: i64,
        #[doc = concat!(" ", "current oracle rate")]
        pub rate: u64,
        #[doc = concat!(" ", "realized pnl")]
        pub realized_pnl: i64,
        #[doc = concat!(" ", "margin balance")]
        pub total_balance: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct LpRemoveMaxRatioRecord {
        pub ratio: i64,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum LpStatus {
        Active,
        Updating,
    }
    #[derive(Clone, Copy, Default)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MarginPosition {
        #[doc = concat!(" ", "precision: token mint precision")]
        pub balance: i64,
        #[doc = concat!(" ", "The market index of the corresponding spot market")]
        pub market_index: u32,
        pub decimals: u32,
        pub padding2: [u8; 32usize],
    }
    unsafe impl ::bytemuck::Pod for MarginPosition {}
    unsafe impl ::bytemuck::Zeroable for MarginPosition {}
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum MarginType {
        NonYieldBearing,
        YieldBearing,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct MarginValue {
        pub margin_asset_value: i64,
        pub margin_liability_value: i64,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum MarketStatus {
        Initialized,
        Active,
        Paused,
        ReduceOnly,
        Updating,
        Expired,
        Settlement,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum MarketType {
        Perp,
        Spot,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct NewEarnVaultRecord {
        #[doc = concat!(" ", "unix_timestamp of action")]
        pub ts: i64,
        pub pt_mint: ::solana_program::pubkey::Pubkey,
        pub earn_vault: ::solana_program::pubkey::Pubkey,
        pub margin_index: u32,
        pub market_index: u32,
        pub user_ratio: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct NewLpRecord {
        #[doc = concat!(" ", "unix_timestamp of action")]
        pub ts: i64,
        #[doc = concat!(" ", "owner of subaccount")]
        pub lp_authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount")]
        pub lp: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount id")]
        pub sub_account_id: u16,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct NewUserOrdersRecord {
        #[doc = concat!(" ", "unix_timestamp of action")]
        pub ts: i64,
        #[doc = concat!(" ", "owner of subaccount")]
        pub user_authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount")]
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount id")]
        pub sub_account_id: u16,
    }
    #[doc = concat!(" ", "user init event")]
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct NewUserRecord {
        #[doc = concat!(" ", "unix_timestamp of action")]
        pub ts: i64,
        #[doc = concat!(" ", "owner of subaccount")]
        pub user_authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount")]
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "subaccount id")]
        pub sub_account_id: u16,
        #[doc = concat!(" ", "is_isolated")]
        pub is_isolated: bool,
    }
    #[doc = concat!(" ", "The element of observations in ObservationState")]
    #[derive(Clone, Copy, Default)]
    #[cfg_attr(not(target_arch = "bpf"), derive(Debug))]
    #[repr(C)]
    #[repr(packed)]
    pub struct Observation {
        #[doc = concat!(" ", "The block timestamp of the observation")]
        pub block_timestamp: u32,
        #[doc = concat!(" ", "the price of the observation timestamp, Q64.64")]
        pub sqrt_price_x64: u128,
        #[doc = concat!(" ", "the cumulative of price during the duration time, Q64.64")]
        pub cumulative_time_price_x64: u128,
        #[doc = concat!(" ", "padding for feature update")]
        pub padding: u128,
    }
    unsafe impl ::bytemuck::Pod for Observation {}
    unsafe impl ::bytemuck::Zeroable for Observation {}
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Order {
        #[doc = concat!(" ", "The slot the order was placed")]
        pub slot: u64,
        #[doc = concat!(
            " ",
            "The limit price for the order (can be 0 for market orders)"
        )]
        #[doc = concat!(
            " ",
            "For orders with an auction, this price isn't used until the auction is complete"
        )]
        #[doc = concat!(" ", "precision: PRICE_PRECISION")]
        pub price_limit: u128,
        #[doc = concat!(" ", "The size of the order")]
        #[doc = concat!(" ", "precision for yields: BASE_PRECISION")]
        pub base_asset_amount: i64,
        pub base_asset_amount_filled: i64,
        pub quote_asset_amount_filled: i64,
        #[doc = concat!(" ", "The time when the order will expire")]
        pub expire_ts: i64,
        #[doc = concat!(
            " ",
            "The id for the order. Each users has their own order id space"
        )]
        pub order_index: u32,
        pub order_id: u32,
        pub isolated_margin_amount: u64,
        pub market_index: u32,
        #[doc = concat!(" ", "Whether the order is open or unused")]
        pub status: OrderStatus,
        #[doc = concat!(" ", "The type of order")]
        pub order_type: OrderType,
        pub is_close: u8,
        pub padding1: [u8; 1usize],
        pub padding2: [u8; 32usize],
    }
    unsafe impl ::bytemuck::Pod for Order {}
    unsafe impl ::bytemuck::Zeroable for Order {}
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct OrderParams {
        pub base_asset_amount: i64,
        pub price_limit: u128,
        pub expire_ts: i64,
        pub market_index: u32,
        pub order_type: OrderType,
        pub isolated_margin_amount: u64,
        pub is_close: bool,
    }
    #[doc = concat!(" ", "place_yield_order event")]
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct OrderRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "order struct")]
        pub order: Order,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum OrderStatus {
        Init,
        Open,
        Filled,
        Canceled,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum OrderType {
        Market,
        Limit,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Position {
        pub ammpool: ::solana_program::pubkey::Pubkey,
        pub liquidity: u128,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub lower_rate: u64,
        pub upper_rate: u64,
        pub fee_growth_checkpoint_a: u128,
        pub fee_owed_a: u64,
        pub fee_growth_checkpoint_b: u128,
        pub fee_owed_b: u64,
        pub reward_infos: [PositionRewardInfo; 3usize],
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct PositionRewardInfo {
        pub growth_inside_checkpoint: u128,
        pub amount_owed: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct PositionValue {
        pub position_asset_value: i64,
        pub position_liability_value: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct SettleUserRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "market index")]
        pub market_index: u32,
        #[doc = concat!(" ", "amount of traded yt")]
        pub base_amount_filled: i64,
        #[doc = concat!(" ", "amount of traded vt")]
        pub quote_amount_filled: i64,
        #[doc = concat!(" ", "amount of held yt after trade")]
        pub base_amount_held: i64,
        #[doc = concat!(" ", "amount of held vt after trade")]
        pub quote_amount_held: i64,
        #[doc = concat!(" ", "current oracle rate")]
        pub rate: u64,
        #[doc = concat!(" ", "realized pnl")]
        pub realized_pnl: i64,
        #[doc = concat!(" ", "margin balance")]
        pub total_balance: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct SocialLossChangeRecord {
        pub ts: i64,
        pub market_index: u32,
        pub base_amount_filled: i64,
        pub quote_amount_filled: i64,
        pub margin_filled: i64,
        pub base_amount_held: i64,
        pub quote_amount_held: i64,
        pub total_balance: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct SocialLossTransferPositionRecord {
        pub ts: i64,
        pub market_index: u32,
        pub base_amount_transfered: i64,
        pub quote_amount_transfered: i64,
        pub margin_transfered: i64,
        pub base_amount_held: i64,
        pub quote_amount_held: i64,
        pub total_balance: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct SwapEvent {
        pub yield_market: ::solana_program::pubkey::Pubkey,
        pub amount_a: i64,
        pub amount_b: i64,
        pub a_to_b: bool,
        pub sqrt_price_x64: u128,
        pub tick_current_index: i32,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct SwapResult {
        pub amount_base_swap: u64,
        pub amount_quote_swap: u64,
        pub sqrt_price_x64: u128,
    }
    #[derive(Clone, Copy, Default)]
    #[cfg_attr(not(target_arch = "bpf"), derive(Debug))]
    #[repr(C)]
    #[repr(packed)]
    pub struct Tick {
        pub initialized: u8,
        pub liquidity_net: i128,
        pub liquidity_gross: u128,
        pub fee_growth_outside_a: u128,
        pub fee_growth_outside_b: u128,
        pub reward_growths_outside: [u128; 3usize],
    }
    unsafe impl ::bytemuck::Pod for Tick {}
    unsafe impl ::bytemuck::Zeroable for Tick {}
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct TransferLpRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub market_index: u32,
        pub direction: LpDirection,
        pub delta_base_asset_amount: i64,
        pub delta_quote_asset_amount: i64,
        #[doc = concat!(" ", "the lower tick")]
        pub tick_lower: i32,
        #[doc = concat!(" ", "the upper tick")]
        pub tick_upper: i32,
        #[doc = concat!(" ", "the lower rate")]
        pub rate_lower: u64,
        #[doc = concat!(" ", "the upper rate")]
        pub rate_upper: u64,
        #[doc = concat!(" ", "liquidity amount")]
        pub liquidity_amount: u128,
        pub reserve_base_amount: i64,
        pub reserve_quote_amount: i64,
        pub is_active: bool,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct TransferPositionRecord {
        pub ts: i64,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub market_index: u32,
        pub base_amount_transfered: i64,
        pub quote_amount_transfered: i64,
        pub margin_transfered: i64,
        pub base_amount_held: i64,
        pub quote_amount_held: i64,
        pub total_balance: i64,
        pub trade_price: i64,
        pub fee: u64,
        pub rate: u64,
        pub realized_pnl: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct UpdateOracleRecord {
        pub ts: i64,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub rate: u64,
        pub market_rate: u64,
        pub last_rate: u64,
        pub epoch_start_timestamp: i64,
        pub last_epoch_start_timestamp: i64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct VaultSwapRecord {
        pub ts: i64,
        pub market_index: u32,
        pub in_margin_index: u32,
        pub out_margin_index: u32,
        pub amount_in: u64,
        pub amount_out: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct VaultTransferRecord {
        pub ts: i64,
        pub market_index: u32,
        pub lp_margin_index: u32,
        pub other_margin_index: u32,
        pub from_lp_amount: i64,
    }
    #[derive(Clone, Copy, Default)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct YieldPosition {
        #[doc = concat!(" ", "the size of the users yield position")]
        #[doc = concat!(" ", "precision: BASE_PRECISION")]
        pub base_asset_amount: i64,
        #[doc = concat!(
            " ",
            "Used to calculate the users pnl. Upon entry, is equal to base_asset_amount * avg entry price - fees"
        )]
        #[doc = concat!(
            " ",
            "Updated when the user open/closes position or settles pnl. Includes fees/funding"
        )]
        #[doc = concat!(" ", "precision: QUOTE_PRECISION")]
        pub quote_asset_amount: i64,
        #[doc = concat!(" ", "last cumlative rate")]
        pub last_rate: u64,
        #[doc = concat!(" ", "The market index for the yield market")]
        pub market_index: u32,
        pub padding1: [u8; 4usize],
        pub padding2: [u8; 32usize],
    }
    unsafe impl ::bytemuck::Pod for YieldPosition {}
    unsafe impl ::bytemuck::Zeroable for YieldPosition {}
}
pub mod state {
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct AmmpoolsConfig {
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
        pub reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
        pub default_protocol_fee_rate: u16,
        pub padding: [u8; 30usize],
    }
    impl ::anchor_interface::Account for AmmpoolsConfig {
        const DISCRIMINATOR: &'static [u8] = &[
            231u8,
            149u8,
            203u8,
            83u8,
            71u8,
            11u8,
            230u8,
            197u8,
        ];
    }
    impl ::anchor_interface::AccountSerialize for AmmpoolsConfig {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> std::io::Result<()> {
            use ::anchor_interface::Account;
            writer.write_all(Self::DISCRIMINATOR)?;
            ::borsh::BorshSerialize::serialize(self, writer)?;
            Ok(())
        }
    }
    impl ::anchor_interface::AccountDeserialize for AmmpoolsConfig {
        fn try_deserialize(data: &mut &[u8]) -> std::io::Result<Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            let t = ::borsh::BorshDeserialize::deserialize(&mut &data[8usize..])?;
            Ok(t)
        }
    }
    #[derive(Clone, Copy, Default)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct EarnVault {
        pub pt_mint: ::solana_program::pubkey::Pubkey,
        pub pt_total_supply: u64,
        #[doc = concat!(" ", "The user's collateral")]
        pub margin_position: MarginPosition,
        pub yield_position: YieldPosition,
        pub protocol_fee: u64,
        #[doc = concat!(" ", "The last slot a user was active. Used to")]
        pub last_active_slot: u64,
        pub margin_index: u32,
        pub market_index: u32,
        #[doc = concat!(" ", "user's yield share")]
        pub user_ratio: u64,
        #[doc = concat!(" ", "User is idle if they haven't interacted")]
        #[doc = concat!(" ", "Off-chain keeper bots can ignore users t")]
        pub idle: u8,
        pub padding1: [u8; 7usize],
        pub net_quote_amount_realized: i64,
    }
    unsafe impl ::bytemuck::Pod for EarnVault {}
    unsafe impl ::bytemuck::Zeroable for EarnVault {}
    impl ::anchor_interface::Account for EarnVault {
        const DISCRIMINATOR: &'static [u8] = &[
            50u8,
            118u8,
            31u8,
            24u8,
            200u8,
            163u8,
            174u8,
            156u8,
        ];
    }
    impl ::anchor_interface::PodAccount for EarnVault {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct FeeTier {
        pub ammpools_config: ::solana_program::pubkey::Pubkey,
        pub tick_spacing: u16,
        pub default_fee_rate: u16,
        pub padding: [u8; 4usize],
    }
    impl ::anchor_interface::Account for FeeTier {
        const DISCRIMINATOR: &'static [u8] = &[
            56u8,
            75u8,
            159u8,
            76u8,
            142u8,
            68u8,
            190u8,
            105u8,
        ];
    }
    impl ::anchor_interface::AccountSerialize for FeeTier {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> std::io::Result<()> {
            use ::anchor_interface::Account;
            writer.write_all(Self::DISCRIMINATOR)?;
            ::borsh::BorshSerialize::serialize(self, writer)?;
            Ok(())
        }
    }
    impl ::anchor_interface::AccountDeserialize for FeeTier {
        fn try_deserialize(data: &mut &[u8]) -> std::io::Result<Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            let t = ::borsh::BorshDeserialize::deserialize(&mut &data[8usize..])?;
            Ok(t)
        }
    }
    #[derive(Clone, Copy)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Lp {
        #[doc = concat!(" ", "The owner/authority of the account")]
        pub authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "The user's liquidity")]
        pub amm_position: Position,
        #[doc = concat!(" ", "The user's yield positions")]
        pub reserve_quote_amount: i64,
        pub reserve_base_amount: i64,
        #[doc = concat!(
            " ",
            "The last slot a user was active. Used to determine if a user is idle"
        )]
        pub last_active_slot: u64,
        #[doc = concat!(" ", "The sub account id for this user")]
        pub sub_account_id: u16,
        #[doc = concat!(
            " ",
            "User is idle if they haven't interacted with the protocol in 1 week and they have no orders, yield positions or borrows"
        )]
        #[doc = concat!(" ", "Off-chain keeper bots can ignore users that are idle")]
        pub idle: u8,
        pub state: LpStatus,
        pub padding1: [u8; 7usize],
        pub padding2: [u8; 72usize],
    }
    unsafe impl ::bytemuck::Pod for Lp {}
    unsafe impl ::bytemuck::Zeroable for Lp {}
    impl ::anchor_interface::Account for Lp {
        const DISCRIMINATOR: &'static [u8] = &[
            31u8,
            47u8,
            62u8,
            188u8,
            110u8,
            128u8,
            12u8,
            82u8,
        ];
    }
    impl ::anchor_interface::PodAccount for Lp {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
    #[derive(Clone, Copy)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct MarginMarket {
        #[doc = concat!(
            " ",
            "The address of the margin market. It is a pda of the market index"
        )]
        pub pubkey: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "The token mint of the margin")]
        pub mint: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "The vault used to store the market's deposits")]
        pub vault: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "The encoded display name for the market e.g. SOL")]
        pub name: [u8; 32usize],
        pub market_index: u32,
        pub padding1: [u8; 4usize],
        #[doc = concat!(
            " ",
            "The sum of the scaled balances for deposits across users and pool balances"
        )]
        #[doc = concat!(
            " ",
            "To convert to the deposit token amount, multiply by the cumulative deposit interest"
        )]
        #[doc = concat!(" ", "precision: SPOT_BALANCE_PRECISION")]
        pub deposit_balance: i64,
        #[doc = concat!(
            " ",
            "Every deposit has a deposit record id. This is the next id to use"
        )]
        pub next_deposit_record_id: u64,
        #[doc = concat!(
            " ",
            "For swaps, the amount of token loaned out in the begin_swap ix"
        )]
        #[doc = concat!(" ", "precision: token mint precision")]
        pub flash_loan_amount: u64,
        #[doc = concat!(
            " ",
            "For swaps, the amount in the users token account in the begin_swap ix"
        )]
        #[doc = concat!(
            " ",
            "Used to calculate how much of the token left the system in end_swap ix"
        )]
        #[doc = concat!(" ", "precision: token mint precision")]
        pub flash_loan_initial_token_amount: u64,
        #[doc = concat!(
            " ",
            "The market's token mint's decimals. To from decimals to a precision, 10^decimals"
        )]
        pub decimals: u32,
        pub status: MarketStatus,
        pub padding: [u8; 64usize],
    }
    unsafe impl ::bytemuck::Pod for MarginMarket {}
    unsafe impl ::bytemuck::Zeroable for MarginMarket {}
    impl ::anchor_interface::Account for MarginMarket {
        const DISCRIMINATOR: &'static [u8] = &[
            239u8,
            74u8,
            160u8,
            96u8,
            142u8,
            137u8,
            58u8,
            112u8,
        ];
    }
    impl ::anchor_interface::PodAccount for MarginMarket {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
    #[derive(Clone, Copy)]
    #[cfg_attr(not(target_arch = "bpf"), derive(Debug))]
    #[repr(C)]
    #[repr(packed)]
    pub struct ObservationState {
        #[doc = concat!(" ", "Whether the ObservationState is initialized")]
        pub initialized: u8,
        pub market_index: u32,
        #[doc = concat!(" ", "observation array")]
        pub observations: [Observation; 1000usize],
        #[doc = concat!(" ", "padding for feature update")]
        pub padding: [u128; 5usize],
    }
    unsafe impl ::bytemuck::Pod for ObservationState {}
    unsafe impl ::bytemuck::Zeroable for ObservationState {}
    impl ::anchor_interface::Account for ObservationState {
        const DISCRIMINATOR: &'static [u8] = &[
            122u8,
            174u8,
            197u8,
            53u8,
            129u8,
            9u8,
            165u8,
            132u8,
        ];
    }
    impl ::anchor_interface::PodAccount for ObservationState {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
    #[derive(Clone, Copy, Default)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct Oracle {
        pub admin: ::solana_program::pubkey::Pubkey,
        pub name: [u8; 32usize],
        pub last_rate: u64,
        pub rate: u64,
        pub market_rate: u64,
        pub ts: i64,
        pub decimals: u32,
        pub padding: [u8; 4usize],
        pub epoch_start_timestamp: i64,
        pub last_epoch_start_timestamp: i64,
        pub padding1: [u8; 32usize],
    }
    unsafe impl ::bytemuck::Pod for Oracle {}
    unsafe impl ::bytemuck::Zeroable for Oracle {}
    impl ::anchor_interface::Account for Oracle {
        const DISCRIMINATOR: &'static [u8] = &[
            139u8,
            194u8,
            131u8,
            179u8,
            140u8,
            179u8,
            229u8,
            244u8,
        ];
    }
    impl ::anchor_interface::PodAccount for Oracle {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct State {
        pub admin: ::solana_program::pubkey::Pubkey,
        pub signer: ::solana_program::pubkey::Pubkey,
        pub number_of_authorities: u64,
        pub number_of_sub_accounts: u64,
        pub collateral_ratio_initial: i64,
        pub collateral_ratio_maintenance: i64,
        pub collateral_ratio_initial_pre_expiry: i64,
        pub number_of_yield_markets: u32,
        pub number_of_margin_markets: u32,
        pub signer_nonce: u8,
        pub twap_duration: u32,
        pub margin_index_start: u32,
        pub market_index_start: u32,
        pub keepers: [::solana_program::pubkey::Pubkey; 20usize],
        pub keeper_fee_per_tx: u64,
        pub keeper_fee: u64,
        pub padding0: [u8; 4usize],
        pub padding1: [u8; 32usize],
        pub padding2: [u8; 32usize],
    }
    impl ::anchor_interface::Account for State {
        const DISCRIMINATOR: &'static [u8] = &[
            216u8,
            146u8,
            107u8,
            94u8,
            104u8,
            75u8,
            182u8,
            177u8,
        ];
    }
    impl ::anchor_interface::AccountSerialize for State {
        fn try_serialize<W: std::io::Write>(
            &self,
            writer: &mut W,
        ) -> std::io::Result<()> {
            use ::anchor_interface::Account;
            writer.write_all(Self::DISCRIMINATOR)?;
            ::borsh::BorshSerialize::serialize(self, writer)?;
            Ok(())
        }
    }
    impl ::anchor_interface::AccountDeserialize for State {
        fn try_deserialize(data: &mut &[u8]) -> std::io::Result<Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            let t = ::borsh::BorshDeserialize::deserialize(&mut &data[8usize..])?;
            Ok(t)
        }
    }
    #[derive(Clone, Copy)]
    #[cfg_attr(not(target_arch = "bpf"), derive(Debug))]
    #[repr(C)]
    #[repr(packed)]
    pub struct TickArray {
        pub start_tick_index: i32,
        pub ticks: [Tick; 88usize],
        pub ammpool: ::solana_program::pubkey::Pubkey,
    }
    unsafe impl ::bytemuck::Pod for TickArray {}
    unsafe impl ::bytemuck::Zeroable for TickArray {}
    impl ::anchor_interface::Account for TickArray {
        const DISCRIMINATOR: &'static [u8] = &[
            69u8,
            97u8,
            189u8,
            190u8,
            110u8,
            7u8,
            66u8,
            187u8,
        ];
    }
    impl ::anchor_interface::PodAccount for TickArray {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
    #[derive(Clone, Copy)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct User {
        #[doc = concat!(" ", "The owner/authority of the account")]
        pub authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "The user's collateral")]
        pub margin_positions: [MarginPosition; 2usize],
        #[doc = concat!(" ", "The user's liquidity")]
        pub orders: [Order; 32usize],
        #[doc = concat!(" ", "The user's yield positions")]
        pub yield_positions: [YieldPosition; 8usize],
        #[doc = concat!(
            " ",
            "The last slot a user was active. Used to determine if a user is idle"
        )]
        pub last_active_slot: u64,
        pub last_order_id: u32,
        #[doc = concat!(" ", "The sub account id for this user")]
        pub sub_account_id: u16,
        #[doc = concat!(
            " ",
            "User is idle if they haven't interacted with the protocol in 1 week and they have no orders, yield positions or borrows"
        )]
        #[doc = concat!(" ", "Off-chain keeper bots can ignore users that are idle")]
        pub idle: u8,
        #[doc = concat!(" ", "Whether or not the subaccount has been liquidated")]
        pub padding2: u8,
        #[doc = concat!(" ", "isolated / cross margin flag")]
        pub is_isolated: u8,
        pub is_expiry_on: u8,
        pub padding1: [u8; 6usize],
    }
    unsafe impl ::bytemuck::Pod for User {}
    unsafe impl ::bytemuck::Zeroable for User {}
    impl ::anchor_interface::Account for User {
        const DISCRIMINATOR: &'static [u8] = &[
            159u8,
            117u8,
            95u8,
            227u8,
            239u8,
            151u8,
            58u8,
            236u8,
        ];
    }
    impl ::anchor_interface::PodAccount for User {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
    #[derive(Clone, Copy)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct UserStats {
        #[doc = concat!(" ", "The authority for all of a users sub accounts")]
        pub authority: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "The address that referred this user")]
        pub referrer: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "The current number of sub accounts")]
        pub number_of_sub_accounts: u16,
        #[doc = concat!(
            " ",
            "The number of sub accounts created. Can be greater than the number of sub accounts if user"
        )]
        #[doc = concat!(" ", "has deleted sub accountsget_margin_position_index")]
        pub number_of_sub_accounts_created: u16,
        pub padding: [u8; 52usize],
    }
    unsafe impl ::bytemuck::Pod for UserStats {}
    unsafe impl ::bytemuck::Zeroable for UserStats {}
    impl ::anchor_interface::Account for UserStats {
        const DISCRIMINATOR: &'static [u8] = &[
            176u8,
            223u8,
            136u8,
            27u8,
            122u8,
            79u8,
            32u8,
            227u8,
        ];
    }
    impl ::anchor_interface::PodAccount for UserStats {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
    #[derive(Clone, Copy)]
    #[derive(Debug)]
    #[repr(C)]
    pub struct YieldMarket {
        #[doc = concat!(
            " ",
            "The yield market's address. It is a pda of the market index"
        )]
        pub pubkey: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "the quote asset oracle")]
        pub oracle: ::solana_program::pubkey::Pubkey,
        #[doc = concat!(" ", "Encoded display name for the yield market e.g. MSOL-2406")]
        pub name: [u8; 32usize],
        pub quote_asset_vault: ::solana_program::pubkey::Pubkey,
        pub base_asset_vault: ::solana_program::pubkey::Pubkey,
        pub pool: Ammpool,
        #[doc = concat!(" ", "start time")]
        pub start_ts: i64,
        #[doc = concat!(" ", "expiration time")]
        pub expire_ts: i64,
        pub order_step_size: u64,
        pub min_order_size: u64,
        pub min_lp_amount: u64,
        pub min_liquidation_size: u64,
        pub market_index: u32,
        pub margin_index: u32,
        pub lp_margin_index: u32,
        pub margin_type: MarginType,
        pub lp_margin_type: MarginType,
        pub margin_decimals: u8,
        pub lp_margin_decimals: u8,
        pub collateral_ratio_initial: i64,
        pub collateral_ratio_initial_pre_expiry: i64,
        pub collateral_ratio_maintenance: i64,
        pub active_ratio_coef: u64,
        #[doc = concat!(" ", "the max open interest")]
        pub max_open_interest: u64,
        #[doc = concat!(" ", "current open interest")]
        pub open_interest: u64,
        pub number_of_active_users: u64,
        pub number_of_active_lps: u64,
        pub status: MarketStatus,
        pub market_type: MarketType,
        pub padding2: [u8; 6usize],
        pub net_quote_amount: i64,
        pub net_base_amount: i64,
        pub last_rate: u64,
        #[doc = concat!(" ", "lp stats")]
        #[doc = concat!(" ", "the quote asset amount minted by lps")]
        pub total_quote_asset_amount: i64,
        #[doc = concat!(" ", "the total deposited amount of the lp mint;")]
        pub total_margin_amount: i64,
        #[doc = concat!(" ", "net realized quote of all traders")]
        pub net_quote_amount_realized: i64,
        pub social_loss_margin_position: MarginPosition,
        pub social_loss_yield_position: YieldPosition,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        #[doc = concat!(" ", "insurance")]
        pub insurance_margin_position: MarginPosition,
        pub insurance_yield_position: YieldPosition,
        #[doc = concat!(" ", "epoch update")]
        pub keeper_fee: i64,
        pub lp_accounts_processed: u64,
        pub implied_rate: u64,
        pub lp_quote_amount: i64,
        pub lp_base_amount: i64,
        pub number_of_processed_users: u64,
        #[doc = concat!(" ", "epoch update expire")]
        pub expire_update_ts: i64,
        pub expire_total_debt: i64,
        pub expire_total_margin: i64,
        pub expire_total_pos_quote_amount: i64,
        pub expire_total_debt_covered: i64,
        pub total_reserve_base_amount: i64,
        pub liq_fee_rate: i64,
        pub protocol_fee: i64,
        pub epoch_update_status: EpochUpdateStatus,
        pub padding3: [u8; 7usize],
        pub earn_net_quote_amount_realized: i64,
        pub total_sloss_quote_quota: i64,
        pub epoch_update_end_ts: i64,
        pub total_reserve_quote_amount: i64,
        pub sloss_base_amount_filled: i64,
        pub sloss_quote_amount_filled: i64,
        pub lower_rate_bound: u64,
        pub upper_rate_bound: u64,
        pub sqrt_price_lower_bound: u128,
        pub sqrt_price_upper_bound: u128,
        pub bound_percentage: u64,
        pub padding4: [u8; 248usize],
        pub padding: [u8; 2usize],
    }
    unsafe impl ::bytemuck::Pod for YieldMarket {}
    unsafe impl ::bytemuck::Zeroable for YieldMarket {}
    impl ::anchor_interface::Account for YieldMarket {
        const DISCRIMINATOR: &'static [u8] = &[
            140u8,
            119u8,
            210u8,
            6u8,
            237u8,
            21u8,
            244u8,
            227u8,
        ];
    }
    impl ::anchor_interface::PodAccount for YieldMarket {
        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            data[..8usize].copy_from_slice(Self::DISCRIMINATOR);
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &mut data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes_mut(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
            use ::anchor_interface::Account;
            if data.len() < 8usize || &data[..8usize] != Self::DISCRIMINATOR {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`", &
                            data[..8usize.min(data.len())], Self::DISCRIMINATOR,
                        ),
                    ),
                );
            }
            if data.len() < 8usize + std::mem::size_of::<Self>() {
                return Err(
                    ::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "not enough data: got `{}`, expected `{}`", data.len(),
                            8usize + std::mem::size_of:: < Self > (),
                        ),
                    ),
                );
            }
            let payload = &data[8usize..8usize + std::mem::size_of::<Self>()];
            ::bytemuck::try_from_bytes(payload)
                .map_err(|err| {
                    ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                })
        }
    }
}
pub mod error {
    #[allow(unused_imports)]
    use super::types::*;
    use ::num_derive::FromPrimitive;
    use ::thiserror::Error;
    use ::solana_program::{decode_error::DecodeError, program_error::ProgramError};
    #[derive(Error, Clone, Copy, Debug, FromPrimitive, PartialEq, Eq)]
    #[repr(u32)]
    pub enum RatexContractsError {
        #[error("Invalid Margin Market Authority")]
        InvalidMarginMarketAuthority = 6000u32,
        #[error("Clearing house not insurance fund authority")]
        InvalidInsuranceFundAuthority = 6001u32,
        #[error("Insufficient deposit")]
        InsufficientDeposit = 6002u32,
        #[error("Insufficient collateral")]
        InsufficientCollateral = 6003u32,
        #[error("Insufficient collateral in vault")]
        InsufficientCollateralInVault = 6004u32,
        #[error("Order Size Too Small")]
        OrderSizeTooSmall = 6005u32,
        #[error("Math Error")]
        MathError = 6006u32,
        #[error("Conversion to u128/u64 failed with an overflow or underflow")]
        BnConversionError = 6007u32,
        #[error("Unable To Load Oracles")]
        UnableToLoadOracle = 6008u32,
        #[error("Invalid oracle rate")]
        InvalidRate = 6009u32,
        #[error("Exchange is paused")]
        ExchangePaused = 6010u32,
        #[error("User Has No Position In Market")]
        UserHasNoPositionInMarket = 6011u32,
        #[error("Yield market is paused")]
        YieldMarketPaused = 6012u32,
        #[error("Yield market not paused")]
        YieldMarketNotPaused = 6013u32,
        #[error("Insurance fund and position is empty")]
        InsuranceEmpty = 6014u32,
        #[error("Yield market not expired")]
        YieldMarketNotExpired = 6015u32,
        #[error("Yield market not in epoch update")]
        YieldMarketNotEpochUpdating = 6016u32,
        #[error("YieldMarketNotFound")]
        YieldMarketNotFound = 6017u32,
        #[error("InvalidMarketAccount")]
        InvalidMarketAccount = 6018u32,
        #[error("MarketIndexMismatch")]
        MarketIndexMismatch = 6019u32,
        #[error("RateMismatch")]
        RateMismatch = 6020u32,
        #[error("MarketWrongMutability")]
        MarketWrongMutability = 6021u32,
        #[error("CouldNotLoadMarketData")]
        CouldNotLoadMarketData = 6022u32,
        #[error("UnableToLoadYieldMarketAccount")]
        UnableToLoadYieldMarketAccount = 6023u32,
        #[error("Processed lp accounts is not zero")]
        ProcessedLpNotZero = 6024u32,
        #[error("Processed lp and user accounts is not zero")]
        ProcessedLpAndUserNotZero = 6025u32,
        #[error("Margin market is paused")]
        MarginMarketPaused = 6026u32,
        #[error("Referrer not found")]
        ReferrerNotFound = 6027u32,
        #[error("ReferrerNotFound")]
        ReferrerStatsNotFound = 6028u32,
        #[error("ReferrerMustBeWritable")]
        ReferrerMustBeWritable = 6029u32,
        #[error("ReferrerMustBeWritable")]
        ReferrerStatsMustBeWritable = 6030u32,
        #[error("ReferrerAndReferrerStatsAuthorityUnequal")]
        ReferrerAndReferrerStatsAuthorityUnequal = 6031u32,
        #[error("InvalidReferrer")]
        InvalidReferrer = 6032u32,
        #[error("InvalidOracle")]
        InvalidOracle = 6033u32,
        #[error("OracleNotFound")]
        OracleNotFound = 6034u32,
        #[error("Unable to load AccountLoader")]
        UnableToLoadAccountLoader = 6035u32,
        #[error("Can not delete user that still has collateral")]
        CantDeleteUserWithCollateral = 6036u32,
        #[error("Casting Failure")]
        CastingFailure = 6037u32,
        #[error("InvalidOrder")]
        InvalidOrder = 6038u32,
        #[error("CouldNotFindMarginPosition")]
        CouldNotFindMarginPosition = 6039u32,
        #[error("NoMarginPositionAvailable")]
        NoMarginPositionAvailable = 6040u32,
        #[error("NoYieldPositionAvailable")]
        NoYieldPositionAvailable = 6041u32,
        #[error("InvalidMarginMarketInitialization")]
        InvalidMarginMarketInitialization = 6042u32,
        #[error("CouldNotLoadMarginMarketData")]
        CouldNotLoadMarginMarketData = 6043u32,
        #[error("MarginMarketNotFound")]
        MarginMarketNotFound = 6044u32,
        #[error("InvalidMarginMarketAccount")]
        InvalidMarginMarketAccount = 6045u32,
        #[error("UnableToLoadMarginMarketAccount")]
        UnableToLoadMarginMarketAccount = 6046u32,
        #[error("MarginMarketWrongMutability")]
        MarginMarketWrongMutability = 6047u32,
        #[error("UserIsBeingLiquidated")]
        UserIsBeingLiquidated = 6048u32,
        #[error("UserNotBeingLiquidated")]
        UserNotBeingLiquidated = 6049u32,
        #[error("DefaultError")]
        DefaultError = 6050u32,
        #[error("User Cant Be Deleted")]
        UserCantBeDeleted = 6051u32,
        #[error("Lp Cant Be Deleted")]
        LpCantBeDeleted = 6052u32,
        #[error("Max Open Interest")]
        MaxOpenInterest = 6053u32,
        #[error("YieldMarketNotInSettlement")]
        YieldMarketNotInSettlement = 6054u32,
        #[error("YieldMarketNotInReduceOnly")]
        YieldMarketNotInReduceOnly = 6055u32,
        #[error("YieldMarketSettlementBufferNotReached")]
        YieldMarketSettlementBufferNotReached = 6056u32,
        #[error("YieldMarketSettlementUserHasOpenOrders")]
        YieldMarketSettlementUserHasOpenOrders = 6057u32,
        #[error("YieldMarketSettlementUserHasActiveLP")]
        YieldMarketSettlementUserHasActiveLp = 6058u32,
        #[error("Market Being Initialized")]
        MarketBeingInitialized = 6059u32,
        #[error("Invalid Sub Account Id")]
        InvalidUserSubAccountId = 6060u32,
        #[error("Failed Unwrap")]
        FailedUnwrap = 6061u32,
        #[error("CouldNotLoadUserData")]
        CouldNotLoadUserData = 6062u32,
        #[error("InvalidUserAccount")]
        InvalidUserAccount = 6063u32,
        #[error("CouldNotLoadUserData")]
        CouldNotLoadUserStatsData = 6064u32,
        #[error("UserWrongMutability")]
        UserStatsWrongMutability = 6065u32,
        #[error("InvalidUserStatsAccount")]
        InvalidUserStatsAccount = 6066u32,
        #[error("UnableToLoadUserAccount")]
        UnableToLoadUserAccount = 6067u32,
        #[error("InvalidJupSwap")]
        InvalidJupSwap = 6068u32,
        #[error("Unable to get twap price")]
        UnableToGetTwapPrice = 6069u32,
        #[error("Invalid adl liquidation")]
        InvalidAdlLiquidation = 6070u32,
        #[error("Cant self Adl")]
        CantSelfAdl = 6071u32,
        #[error("Unauthorized user or keeper")]
        UnauthorizedUserOrKeeper = 6072u32,
        #[error("User is not isolated")]
        UserNotIsolated = 6073u32,
        #[error("liquidity range is not allowed")]
        InvalidLiquidityRange = 6074u32,
        #[error("Invalid oracle account")]
        InvalidOracleAccount = 6075u32,
        #[error("Unable to load oracle account")]
        UnableToLoadOracleAccount = 6076u32,
        #[error("could not load oracle data")]
        CouldNotLoadOracleData = 6077u32,
        #[error("wrong oracle mutablility")]
        OracleWrongMutability = 6078u32,
        #[error("The keeper already exists in the list.")]
        KeeperAlreadyExists = 6079u32,
        #[error("The keepers list is full.")]
        KeepersListFull = 6080u32,
        #[error("The keeper was not found in the list.")]
        KeeperNotFound = 6081u32,
        #[error("Max open interest exceeded")]
        MaxOpenInterestExceeded = 6082u32,
        #[error("Max number of orders taken")]
        MaxNumberOfOrders = 6083u32,
        #[error("Invalid order step size")]
        InvalidOrderStepSize = 6084u32,
        #[error("Order expired")]
        OrderExpired = 6085u32,
        #[error("Order not open")]
        OrderNotOpen = 6086u32,
        #[error("Order does not exist")]
        OrderDoesNotExist = 6087u32,
        #[error("Only trader")]
        OnlyTrader = 6088u32,
        #[error("Only LP")]
        OnlyLp = 6089u32,
        #[error("Invalid Withdraw")]
        InvalidWithdraw = 6090u32,
        #[error("Invalid Liquidate")]
        InvalidLiquidate = 6091u32,
        #[error("Meet maintenance collateral requirement")]
        MeetMaintenanceCollateralRequirement = 6092u32,
        #[error("Fail to meet maintenance collateral requirement")]
        FailToMeetMaintenanceCollateralRequirement = 6093u32,
        #[error("Fail to meet initial collateral requirement")]
        FailToMeetInitialCollateralRequirement = 6094u32,
        #[error("Insurance fund meet maintenance collateral requirement")]
        InsuranceMeetMaintenanceCollateralRequirement = 6095u32,
        #[error("Negative margin balance")]
        NegativeMarginBalance = 6096u32,
        #[error("Invalid start tick index provided.")]
        InvalidStartTick = 6097u32,
        #[error("Tick-array already exists in this ammpool")]
        TickArrayExistInPool = 6098u32,
        #[error("Attempt to search for a tick-array failed")]
        TickArrayIndexOutofBounds = 6099u32,
        #[error("Tick-spacing is not supported")]
        InvalidTickSpacing = 6100u32,
        #[error("Position is not empty It cannot be closed")]
        ClosePositionNotEmpty = 6101u32,
        #[error("Unable to divide by zero")]
        DivideByZero = 6102u32,
        #[error("Unable to cast number into BigInt")]
        NumberCastError = 6103u32,
        #[error("Unable to down cast number")]
        NumberDownCastError = 6104u32,
        #[error("Tick not found within tick array")]
        TickNotFound = 6105u32,
        #[error("Provided tick index is either out of bounds or uninitializable")]
        InvalidTickIndex = 6106u32,
        #[error("Provided sqrt price out of bounds")]
        SqrtPriceOutOfBounds = 6107u32,
        #[error("Liquidity amount must be greater than zero")]
        LiquidityZero = 6108u32,
        #[error("Liquidity amount must be less than i64::MAX")]
        LiquidityTooHigh = 6109u32,
        #[error("Liquidity overflow")]
        LiquidityOverflow = 6110u32,
        #[error("Liquidity underflow")]
        LiquidityUnderflow = 6111u32,
        #[error("Tick liquidity net underflowed or overflowed")]
        LiquidityNetError = 6112u32,
        #[error("Exceeded token max")]
        TokenMaxExceeded = 6113u32,
        #[error("Did not meet token min")]
        TokenMinSubceeded = 6114u32,
        #[error("Position token account has a missing or invalid delegate")]
        MissingOrInvalidDelegate = 6115u32,
        #[error("Position token amount must be 1")]
        InvalidPositionTokenAmount = 6116u32,
        #[error("Timestamp should be convertible from i64 to u64")]
        InvalidTimestampConversion = 6117u32,
        #[error("Timestamp should be greater than the last updated timestamp")]
        InvalidTimestamp = 6118u32,
        #[error("Invalid tick array sequence provided for instruction.")]
        InvalidTickArraySequence = 6119u32,
        #[error("Token Mint in wrong order")]
        InvalidTokenMintOrder = 6120u32,
        #[error("Reward not initialized")]
        RewardNotInitialized = 6121u32,
        #[error("Invalid reward index")]
        InvalidRewardIndex = 6122u32,
        #[error(
            "Reward vault requires amount to support emissions for at least one day"
        )]
        RewardVaultAmountInsufficient = 6123u32,
        #[error("Exceeded max fee rate")]
        FeeRateMaxExceeded = 6124u32,
        #[error("Exceeded max protocol fee rate")]
        ProtocolFeeRateMaxExceeded = 6125u32,
        #[error("Multiplication with shift right overflow")]
        MultiplicationShiftRightOverflow = 6126u32,
        #[error("Muldiv overflow")]
        MulDivOverflow = 6127u32,
        #[error("Invalid div_u256 input")]
        MulDivInvalidInput = 6128u32,
        #[error("Multiplication overflow")]
        MultiplicationOverflow = 6129u32,
        #[error("Provided SqrtPriceLimit not in the same direction as the swap")]
        InvalidSqrtPriceLimitDirection = 6130u32,
        #[error("Invalid sqrt price in the swap")]
        InvalidSqrtPrice = 6131u32,
        #[error("There are no tradable amount to swap.")]
        ZeroTradableAmount = 6132u32,
        #[error("Amount out below minimum threshold")]
        AmountOutBelowMinimum = 6133u32,
        #[error("Amount in above maximum threshold")]
        AmountInAboveMaximum = 6134u32,
        #[error("Invalid index for tick array sequence")]
        TickArraySequenceInvalidIndex = 6135u32,
        #[error("InvalidAmmDetected")]
        InvalidAmmDetected = 6136u32,
        #[error("Amount calculated overflows")]
        AmountCalcOverflow = 6137u32,
        #[error("Amount remaining overflows")]
        AmountRemainingOverflow = 6138u32,
        #[error("Invalid intermediary mint")]
        InvalidIntermediaryMint = 6139u32,
        #[error("Duplicate two hop pool")]
        DuplicateTwoHopPool = 6140u32,
        #[error("Bundle index is out of bounds")]
        InvalidBundleIndex = 6141u32,
        #[error("Position has already been opened")]
        BundledPositionAlreadyOpened = 6142u32,
        #[error("Position has already been closed")]
        BundledPositionAlreadyClosed = 6143u32,
        #[error("Unable To Observation state")]
        UnableToLoadObservationStateAccount = 6144u32,
        #[error("Observation state not found")]
        ObservationStateNotFound = 6145u32,
        #[error("InvalidImpliedRate")]
        InvalidImpliedRate = 6146u32,
        #[error("InvalidRmLiquidityPercent")]
        InvalidRmLiquidityPercent = 6147u32,
        #[error("InvalidLpAccountsProcessed")]
        InvalidLpAccountsProcessed = 6148u32,
        #[error("InvalidUserAccountsProcessed")]
        InvalidUserAccountsProcessed = 6149u32,
        #[error("LpIsInactive")]
        LpIsInactive = 6150u32,
        #[error("InvalidYieldMarket")]
        InvalidYieldMarket = 6151u32,
        #[error("LpAmountTooLow")]
        LpAmountTooLow = 6152u32,
        #[error("LpIsEmpty")]
        LpIsEmpty = 6153u32,
        #[error("InvalidLpState")]
        InvalidLpState = 6154u32,
        #[error("WrongMakerDirection")]
        WrongMakerDirection = 6155u32,
        #[error("InsufficientSwapAmount")]
        InsufficientSwapAmount = 6156u32,
        #[error("InvalidEpochStartTimestamp")]
        InvalidEpochStartTimestamp = 6157u32,
        #[error("InvalidEarnPrice")]
        InvalidEarnPrice = 6158u32,
        #[error("Market expired")]
        MarketExpired = 6159u32,
        #[error("Market not expired")]
        MarketNotExpired = 6160u32,
        #[error("Invalid epoch state transition")]
        InvalidEpochStateTransition = 6161u32,
        #[error("negative balance in spot market")]
        NegativeBalanceInSpotMarketType = 6162u32,
        #[error("spot market type cant liquidate")]
        SpotMarketTypeCantLiquidate = 6163u32,
        #[error("mint mismatch")]
        MintMismatch = 6164u32,
        #[error("wrong margin market")]
        WrongMarginMarket = 6165u32,
        #[error("invalid realized amount")]
        InvalidRealizedAmount = 6166u32,
        #[error("invalid amm pool")]
        InvalidAmmpool = 6167u32,
        #[error("rate should not be less than current rate")]
        LessThanCurrentRate = 6168u32,
        #[error("more than one position for isolated user")]
        InvalidIsolatedUser = 6169u32,
        #[error("Cannot liquidate within TWAP duration after epoch update")]
        LiquidationDuringTwap = 6170u32,
        #[error("duplicate user")]
        DuplicateUser = 6171u32,
        #[error("deciaml not match")]
        DecimalNotMatch = 6172u32,
        #[error("non positive maturity")]
        NonPositiveMaturity = 6173u32,
        #[error("opposite open not supported")]
        OppositeOpenNotAllowed = 6174u32,
        #[error("invalid user authority")]
        InvalidUserAuthority = 6175u32,
        #[error("wrong user token account")]
        WrongUserTokenAccount = 6176u32,
        #[error("Liquidity must be zero")]
        LiquidityMustBeZero = 6177u32,
        #[error("Residual amount must full close")]
        ResidualAmountMustFullClose = 6178u32,
        #[error("Yield market not spot market")]
        NotSpotMarket = 6179u32,
        #[error("Insufficient fee")]
        InsufficientFee = 6180u32,
        #[error("Lower rate below bound")]
        LowerRateBelowBound = 6181u32,
        #[error("Upper rate above bound")]
        UpperRateAboveBound = 6182u32,
        #[error("Lower rate above upper rate")]
        LowerRateAboveUpperRate = 6183u32,
    }
    impl DecodeError<RatexContractsError> for RatexContractsError {
        fn type_of() -> &'static str {
            "RatexContractsError"
        }
    }
    impl From<RatexContractsError> for ProgramError {
        fn from(err: RatexContractsError) -> Self {
            Self::Custom(err as u32)
        }
    }
}

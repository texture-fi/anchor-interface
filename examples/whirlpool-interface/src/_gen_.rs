macro_rules! gen_crate_docs {
    () => {
        concat!(" ", "Whirlpool", " v", "0.1.0",
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
    pub enum WhirlpoolInstruction {
        /// Initialize Config
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` config
        /// 1. `[signer, writable]` funder
        /// 2. `[]` system program
        InitializeConfig {
            fee_authority: ::solana_program::pubkey::Pubkey,
            collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
            reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
            default_protocol_fee_rate: u16,
        },
        /// Initialize Pool
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpools config
        /// 1. `[]` token mint a
        /// 2. `[]` token mint b
        /// 3. `[signer, writable]` funder
        /// 4. `[writable]` whirlpool
        /// 5. `[signer, writable]` token vault a
        /// 6. `[signer, writable]` token vault b
        /// 7. `[]` fee tier
        /// 8. `[]` token program
        /// 9. `[]` system program
        /// 10. `[]` rent
        InitializePool {
            bumps: WhirlpoolBumps,
            tick_spacing: u16,
            initial_sqrt_price: u128,
        },
        /// Initialize Tick Array
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpool
        /// 1. `[signer, writable]` funder
        /// 2. `[writable]` tick array
        /// 3. `[]` system program
        InitializeTickArray { start_tick_index: i32 },
        /// Initialize Fee Tier
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` config
        /// 1. `[writable]` fee tier
        /// 2. `[signer, writable]` funder
        /// 3. `[signer]` fee authority
        /// 4. `[]` system program
        InitializeFeeTier { tick_spacing: u16, default_fee_rate: u16 },
        /// Initialize Reward
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` reward authority
        /// 1. `[signer, writable]` funder
        /// 2. `[writable]` whirlpool
        /// 3. `[]` reward mint
        /// 4. `[signer, writable]` reward vault
        /// 5. `[]` token program
        /// 6. `[]` system program
        /// 7. `[]` rent
        InitializeReward { reward_index: u8 },
        /// Set Reward Emissions
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpool
        /// 1. `[signer]` reward authority
        /// 2. `[]` reward vault
        SetRewardEmissions { reward_index: u8, emissions_per_second_x64: u128 },
        /// Open Position
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` funder
        /// 1. `[]` owner
        /// 2. `[writable]` position
        /// 3. `[signer, writable]` position mint
        /// 4. `[writable]` position token account
        /// 5. `[]` whirlpool
        /// 6. `[]` token program
        /// 7. `[]` system program
        /// 8. `[]` rent
        /// 9. `[]` associated token program
        OpenPosition {
            bumps: OpenPositionBumps,
            tick_lower_index: i32,
            tick_upper_index: i32,
        },
        /// Open Position With Metadata
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer, writable]` funder
        /// 1. `[]` owner
        /// 2. `[writable]` position
        /// 3. `[signer, writable]` position mint
        /// 4. `[writable]` position metadata account
        /// 5. `[writable]` position token account
        /// 6. `[]` whirlpool
        /// 7. `[]` token program
        /// 8. `[]` system program
        /// 9. `[]` rent
        /// 10. `[]` associated token program
        /// 11. `[]` metadata program
        /// 12. `[]` metadata update auth
        OpenPositionWithMetadata {
            bumps: OpenPositionWithMetadataBumps,
            tick_lower_index: i32,
            tick_upper_index: i32,
        },
        /// Increase Liquidity
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpool
        /// 1. `[]` token program
        /// 2. `[signer]` position authority
        /// 3. `[writable]` position
        /// 4. `[]` position token account
        /// 5. `[writable]` token owner account a
        /// 6. `[writable]` token owner account b
        /// 7. `[writable]` token vault a
        /// 8. `[writable]` token vault b
        /// 9. `[writable]` tick array lower
        /// 10. `[writable]` tick array upper
        IncreaseLiquidity { liquidity_amount: u128, token_max_a: u64, token_max_b: u64 },
        /// Decrease Liquidity
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpool
        /// 1. `[]` token program
        /// 2. `[signer]` position authority
        /// 3. `[writable]` position
        /// 4. `[]` position token account
        /// 5. `[writable]` token owner account a
        /// 6. `[writable]` token owner account b
        /// 7. `[writable]` token vault a
        /// 8. `[writable]` token vault b
        /// 9. `[writable]` tick array lower
        /// 10. `[writable]` tick array upper
        DecreaseLiquidity { liquidity_amount: u128, token_min_a: u64, token_min_b: u64 },
        /// Update Fees And Rewards
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpool
        /// 1. `[writable]` position
        /// 2. `[]` tick array lower
        /// 3. `[]` tick array upper
        UpdateFeesAndRewards,
        /// Collect Fees
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpool
        /// 1. `[signer]` position authority
        /// 2. `[writable]` position
        /// 3. `[]` position token account
        /// 4. `[writable]` token owner account a
        /// 5. `[writable]` token vault a
        /// 6. `[writable]` token owner account b
        /// 7. `[writable]` token vault b
        /// 8. `[]` token program
        CollectFees,
        /// Collect Reward
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpool
        /// 1. `[signer]` position authority
        /// 2. `[writable]` position
        /// 3. `[]` position token account
        /// 4. `[writable]` reward owner account
        /// 5. `[writable]` reward vault
        /// 6. `[]` token program
        CollectReward { reward_index: u8 },
        /// Collect Protocol Fees
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpools config
        /// 1. `[writable]` whirlpool
        /// 2. `[signer]` collect protocol fees authority
        /// 3. `[writable]` token vault a
        /// 4. `[writable]` token vault b
        /// 5. `[writable]` token destination a
        /// 6. `[writable]` token destination b
        /// 7. `[]` token program
        CollectProtocolFees,
        /// Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` token program
        /// 1. `[signer]` token authority
        /// 2. `[writable]` whirlpool
        /// 3. `[writable]` token owner account a
        /// 4. `[writable]` token vault a
        /// 5. `[writable]` token owner account b
        /// 6. `[writable]` token vault b
        /// 7. `[writable]` tick array0
        /// 8. `[writable]` tick array1
        /// 9. `[writable]` tick array2
        /// 10. `[]` oracle
        Swap {
            amount: u64,
            other_amount_threshold: u64,
            sqrt_price_limit: u128,
            amount_specified_is_input: bool,
            a_to_b: bool,
        },
        /// Close Position
        ///
        /// Accounts expected by this instruction:
        /// 0. `[signer]` position authority
        /// 1. `[writable]` receiver
        /// 2. `[writable]` position
        /// 3. `[writable]` position mint
        /// 4. `[writable]` position token account
        /// 5. `[]` token program
        ClosePosition,
        /// Set Default Fee Rate
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpools config
        /// 1. `[writable]` fee tier
        /// 2. `[signer]` fee authority
        SetDefaultFeeRate { default_fee_rate: u16 },
        /// Set Default Protocol Fee Rate
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpools config
        /// 1. `[signer]` fee authority
        SetDefaultProtocolFeeRate { default_protocol_fee_rate: u16 },
        /// Set Fee Rate
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpools config
        /// 1. `[writable]` whirlpool
        /// 2. `[signer]` fee authority
        SetFeeRate { fee_rate: u16 },
        /// Set Protocol Fee Rate
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpools config
        /// 1. `[writable]` whirlpool
        /// 2. `[signer]` fee authority
        SetProtocolFeeRate { protocol_fee_rate: u16 },
        /// Set Fee Authority
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpools config
        /// 1. `[signer]` fee authority
        /// 2. `[]` new fee authority
        SetFeeAuthority,
        /// Set Collect Protocol Fees Authority
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpools config
        /// 1. `[signer]` collect protocol fees authority
        /// 2. `[]` new collect protocol fees authority
        SetCollectProtocolFeesAuthority,
        /// Set Reward Authority
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpool
        /// 1. `[signer]` reward authority
        /// 2. `[]` new reward authority
        SetRewardAuthority { reward_index: u8 },
        /// Set Reward Authority By Super Authority
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` whirlpools config
        /// 1. `[writable]` whirlpool
        /// 2. `[signer]` reward emissions super authority
        /// 3. `[]` new reward authority
        SetRewardAuthorityBySuperAuthority { reward_index: u8 },
        /// Set Reward Emissions Super Authority
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` whirlpools config
        /// 1. `[signer]` reward emissions super authority
        /// 2. `[]` new reward emissions super authority
        SetRewardEmissionsSuperAuthority,
    }
    impl WhirlpoolInstruction {
        pub fn discriminator(&self) -> &'static [u8; 8] {
            match self {
                Self::InitializeConfig { .. } => {
                    &[208u8, 127u8, 21u8, 1u8, 194u8, 190u8, 196u8, 70u8]
                }
                Self::InitializePool { .. } => {
                    &[95u8, 180u8, 10u8, 172u8, 84u8, 174u8, 232u8, 40u8]
                }
                Self::InitializeTickArray { .. } => {
                    &[11u8, 188u8, 193u8, 214u8, 141u8, 91u8, 149u8, 184u8]
                }
                Self::InitializeFeeTier { .. } => {
                    &[183u8, 74u8, 156u8, 160u8, 112u8, 2u8, 42u8, 30u8]
                }
                Self::InitializeReward { .. } => {
                    &[95u8, 135u8, 192u8, 196u8, 242u8, 129u8, 230u8, 68u8]
                }
                Self::SetRewardEmissions { .. } => {
                    &[13u8, 197u8, 86u8, 168u8, 109u8, 176u8, 27u8, 244u8]
                }
                Self::OpenPosition { .. } => {
                    &[135u8, 128u8, 47u8, 77u8, 15u8, 152u8, 240u8, 49u8]
                }
                Self::OpenPositionWithMetadata { .. } => {
                    &[242u8, 29u8, 134u8, 48u8, 58u8, 110u8, 14u8, 60u8]
                }
                Self::IncreaseLiquidity { .. } => {
                    &[46u8, 156u8, 243u8, 118u8, 13u8, 205u8, 251u8, 178u8]
                }
                Self::DecreaseLiquidity { .. } => {
                    &[160u8, 38u8, 208u8, 111u8, 104u8, 91u8, 44u8, 1u8]
                }
                Self::UpdateFeesAndRewards => {
                    &[154u8, 230u8, 250u8, 13u8, 236u8, 209u8, 75u8, 223u8]
                }
                Self::CollectFees => {
                    &[164u8, 152u8, 207u8, 99u8, 30u8, 186u8, 19u8, 182u8]
                }
                Self::CollectReward { .. } => {
                    &[70u8, 5u8, 132u8, 87u8, 86u8, 235u8, 177u8, 34u8]
                }
                Self::CollectProtocolFees => {
                    &[22u8, 67u8, 23u8, 98u8, 150u8, 178u8, 70u8, 220u8]
                }
                Self::Swap { .. } => {
                    &[248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8]
                }
                Self::ClosePosition => &[123u8, 134u8, 81u8, 0u8, 49u8, 68u8, 98u8, 98u8],
                Self::SetDefaultFeeRate { .. } => {
                    &[118u8, 215u8, 214u8, 157u8, 182u8, 229u8, 208u8, 228u8]
                }
                Self::SetDefaultProtocolFeeRate { .. } => {
                    &[107u8, 205u8, 249u8, 226u8, 151u8, 35u8, 86u8, 0u8]
                }
                Self::SetFeeRate { .. } => {
                    &[53u8, 243u8, 137u8, 65u8, 8u8, 140u8, 158u8, 6u8]
                }
                Self::SetProtocolFeeRate { .. } => {
                    &[95u8, 7u8, 4u8, 50u8, 154u8, 79u8, 156u8, 131u8]
                }
                Self::SetFeeAuthority => {
                    &[31u8, 1u8, 50u8, 87u8, 237u8, 101u8, 97u8, 132u8]
                }
                Self::SetCollectProtocolFeesAuthority => {
                    &[34u8, 150u8, 93u8, 244u8, 139u8, 225u8, 233u8, 67u8]
                }
                Self::SetRewardAuthority { .. } => {
                    &[34u8, 39u8, 183u8, 252u8, 83u8, 28u8, 85u8, 127u8]
                }
                Self::SetRewardAuthorityBySuperAuthority { .. } => {
                    &[240u8, 154u8, 201u8, 198u8, 148u8, 93u8, 56u8, 25u8]
                }
                Self::SetRewardEmissionsSuperAuthority => {
                    &[207u8, 5u8, 200u8, 209u8, 122u8, 56u8, 82u8, 183u8]
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
                    [208u8, 127u8, 21u8, 1u8, 194u8, 190u8, 196u8, 70u8] => {
                        InitializeConfigDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [95u8, 180u8, 10u8, 172u8, 84u8, 174u8, 232u8, 40u8] => {
                        InitializePoolDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [11u8, 188u8, 193u8, 214u8, 141u8, 91u8, 149u8, 184u8] => {
                        InitializeTickArrayDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [183u8, 74u8, 156u8, 160u8, 112u8, 2u8, 42u8, 30u8] => {
                        InitializeFeeTierDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [95u8, 135u8, 192u8, 196u8, 242u8, 129u8, 230u8, 68u8] => {
                        InitializeRewardDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [13u8, 197u8, 86u8, 168u8, 109u8, 176u8, 27u8, 244u8] => {
                        SetRewardEmissionsDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [135u8, 128u8, 47u8, 77u8, 15u8, 152u8, 240u8, 49u8] => {
                        OpenPositionDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [242u8, 29u8, 134u8, 48u8, 58u8, 110u8, 14u8, 60u8] => {
                        OpenPositionWithMetadataDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [46u8, 156u8, 243u8, 118u8, 13u8, 205u8, 251u8, 178u8] => {
                        IncreaseLiquidityDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [160u8, 38u8, 208u8, 111u8, 104u8, 91u8, 44u8, 1u8] => {
                        DecreaseLiquidityDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [154u8, 230u8, 250u8, 13u8, 236u8, 209u8, 75u8, 223u8] => {
                        UpdateFeesAndRewardsDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [164u8, 152u8, 207u8, 99u8, 30u8, 186u8, 19u8, 182u8] => {
                        CollectFeesDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [70u8, 5u8, 132u8, 87u8, 86u8, 235u8, 177u8, 34u8] => {
                        CollectRewardDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [22u8, 67u8, 23u8, 98u8, 150u8, 178u8, 70u8, 220u8] => {
                        CollectProtocolFeesDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [248u8, 198u8, 158u8, 145u8, 225u8, 117u8, 135u8, 200u8] => {
                        SwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [123u8, 134u8, 81u8, 0u8, 49u8, 68u8, 98u8, 98u8] => {
                        ClosePositionDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [118u8, 215u8, 214u8, 157u8, 182u8, 229u8, 208u8, 228u8] => {
                        SetDefaultFeeRateDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [107u8, 205u8, 249u8, 226u8, 151u8, 35u8, 86u8, 0u8] => {
                        SetDefaultProtocolFeeRateDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [53u8, 243u8, 137u8, 65u8, 8u8, 140u8, 158u8, 6u8] => {
                        SetFeeRateDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [95u8, 7u8, 4u8, 50u8, 154u8, 79u8, 156u8, 131u8] => {
                        SetProtocolFeeRateDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [31u8, 1u8, 50u8, 87u8, 237u8, 101u8, 97u8, 132u8] => {
                        SetFeeAuthorityDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [34u8, 150u8, 93u8, 244u8, 139u8, 225u8, 233u8, 67u8] => {
                        SetCollectProtocolFeesAuthorityDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [34u8, 39u8, 183u8, 252u8, 83u8, 28u8, 85u8, 127u8] => {
                        SetRewardAuthorityDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [240u8, 154u8, 201u8, 198u8, 148u8, 93u8, 56u8, 25u8] => {
                        SetRewardAuthorityBySuperAuthorityDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
                    }
                    [207u8, 5u8, 200u8, 209u8, 122u8, 56u8, 82u8, 183u8] => {
                        SetRewardEmissionsSuperAuthorityDeserializer::deserialize(
                                &mut ix_data,
                            )?
                            .into()
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
    impl ::borsh::BorshSerialize for WhirlpoolInstruction {
        fn serialize<W: ::borsh::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), ::borsh::io::Error> {
            match self {
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
                Self::InitializePool { bumps, tick_spacing, initial_sqrt_price } => {
                    ::borsh::BorshSerialize::serialize(bumps, writer)?;
                    ::borsh::BorshSerialize::serialize(tick_spacing, writer)?;
                    ::borsh::BorshSerialize::serialize(initial_sqrt_price, writer)?;
                }
                Self::InitializeTickArray { start_tick_index } => {
                    ::borsh::BorshSerialize::serialize(start_tick_index, writer)?;
                }
                Self::InitializeFeeTier { tick_spacing, default_fee_rate } => {
                    ::borsh::BorshSerialize::serialize(tick_spacing, writer)?;
                    ::borsh::BorshSerialize::serialize(default_fee_rate, writer)?;
                }
                Self::InitializeReward { reward_index } => {
                    ::borsh::BorshSerialize::serialize(reward_index, writer)?;
                }
                Self::SetRewardEmissions { reward_index, emissions_per_second_x64 } => {
                    ::borsh::BorshSerialize::serialize(reward_index, writer)?;
                    ::borsh::BorshSerialize::serialize(
                        emissions_per_second_x64,
                        writer,
                    )?;
                }
                Self::OpenPosition { bumps, tick_lower_index, tick_upper_index } => {
                    ::borsh::BorshSerialize::serialize(bumps, writer)?;
                    ::borsh::BorshSerialize::serialize(tick_lower_index, writer)?;
                    ::borsh::BorshSerialize::serialize(tick_upper_index, writer)?;
                }
                Self::OpenPositionWithMetadata {
                    bumps,
                    tick_lower_index,
                    tick_upper_index,
                } => {
                    ::borsh::BorshSerialize::serialize(bumps, writer)?;
                    ::borsh::BorshSerialize::serialize(tick_lower_index, writer)?;
                    ::borsh::BorshSerialize::serialize(tick_upper_index, writer)?;
                }
                Self::IncreaseLiquidity {
                    liquidity_amount,
                    token_max_a,
                    token_max_b,
                } => {
                    ::borsh::BorshSerialize::serialize(liquidity_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(token_max_a, writer)?;
                    ::borsh::BorshSerialize::serialize(token_max_b, writer)?;
                }
                Self::DecreaseLiquidity {
                    liquidity_amount,
                    token_min_a,
                    token_min_b,
                } => {
                    ::borsh::BorshSerialize::serialize(liquidity_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(token_min_a, writer)?;
                    ::borsh::BorshSerialize::serialize(token_min_b, writer)?;
                }
                Self::UpdateFeesAndRewards => {}
                Self::CollectFees => {}
                Self::CollectReward { reward_index } => {
                    ::borsh::BorshSerialize::serialize(reward_index, writer)?;
                }
                Self::CollectProtocolFees => {}
                Self::Swap {
                    amount,
                    other_amount_threshold,
                    sqrt_price_limit,
                    amount_specified_is_input,
                    a_to_b,
                } => {
                    ::borsh::BorshSerialize::serialize(amount, writer)?;
                    ::borsh::BorshSerialize::serialize(other_amount_threshold, writer)?;
                    ::borsh::BorshSerialize::serialize(sqrt_price_limit, writer)?;
                    ::borsh::BorshSerialize::serialize(
                        amount_specified_is_input,
                        writer,
                    )?;
                    ::borsh::BorshSerialize::serialize(a_to_b, writer)?;
                }
                Self::ClosePosition => {}
                Self::SetDefaultFeeRate { default_fee_rate } => {
                    ::borsh::BorshSerialize::serialize(default_fee_rate, writer)?;
                }
                Self::SetDefaultProtocolFeeRate { default_protocol_fee_rate } => {
                    ::borsh::BorshSerialize::serialize(
                        default_protocol_fee_rate,
                        writer,
                    )?;
                }
                Self::SetFeeRate { fee_rate } => {
                    ::borsh::BorshSerialize::serialize(fee_rate, writer)?;
                }
                Self::SetProtocolFeeRate { protocol_fee_rate } => {
                    ::borsh::BorshSerialize::serialize(protocol_fee_rate, writer)?;
                }
                Self::SetFeeAuthority => {}
                Self::SetCollectProtocolFeesAuthority => {}
                Self::SetRewardAuthority { reward_index } => {
                    ::borsh::BorshSerialize::serialize(reward_index, writer)?;
                }
                Self::SetRewardAuthorityBySuperAuthority { reward_index } => {
                    ::borsh::BorshSerialize::serialize(reward_index, writer)?;
                }
                Self::SetRewardEmissionsSuperAuthority => {}
            }
            Ok(())
        }
    }
    struct InitializeConfigDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeConfigDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::InitializeConfig {
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
    impl From<InitializeConfigDeserializer> for WhirlpoolInstruction {
        fn from(helper: InitializeConfigDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct InitializePoolDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for InitializePoolDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::InitializePool {
                    bumps: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    tick_spacing: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    initial_sqrt_price: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<InitializePoolDeserializer> for WhirlpoolInstruction {
        fn from(helper: InitializePoolDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct InitializeTickArrayDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeTickArrayDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::InitializeTickArray {
                    start_tick_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<InitializeTickArrayDeserializer> for WhirlpoolInstruction {
        fn from(helper: InitializeTickArrayDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct InitializeFeeTierDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeFeeTierDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::InitializeFeeTier {
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
    impl From<InitializeFeeTierDeserializer> for WhirlpoolInstruction {
        fn from(helper: InitializeFeeTierDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct InitializeRewardDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for InitializeRewardDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::InitializeReward {
                    reward_index: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<InitializeRewardDeserializer> for WhirlpoolInstruction {
        fn from(helper: InitializeRewardDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetRewardEmissionsDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetRewardEmissionsDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetRewardEmissions {
                    reward_index: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    emissions_per_second_x64: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<SetRewardEmissionsDeserializer> for WhirlpoolInstruction {
        fn from(helper: SetRewardEmissionsDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct OpenPositionDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for OpenPositionDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::OpenPosition {
                    bumps: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
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
    impl From<OpenPositionDeserializer> for WhirlpoolInstruction {
        fn from(helper: OpenPositionDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct OpenPositionWithMetadataDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for OpenPositionWithMetadataDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::OpenPositionWithMetadata {
                    bumps: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
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
    impl From<OpenPositionWithMetadataDeserializer> for WhirlpoolInstruction {
        fn from(helper: OpenPositionWithMetadataDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct IncreaseLiquidityDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for IncreaseLiquidityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::IncreaseLiquidity {
                    liquidity_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    token_max_a: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    token_max_b: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<IncreaseLiquidityDeserializer> for WhirlpoolInstruction {
        fn from(helper: IncreaseLiquidityDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct DecreaseLiquidityDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for DecreaseLiquidityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::DecreaseLiquidity {
                    liquidity_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    token_min_a: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    token_min_b: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<DecreaseLiquidityDeserializer> for WhirlpoolInstruction {
        fn from(helper: DecreaseLiquidityDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct UpdateFeesAndRewardsDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for UpdateFeesAndRewardsDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::UpdateFeesAndRewards {
                }),
            )
        }
    }
    impl From<UpdateFeesAndRewardsDeserializer> for WhirlpoolInstruction {
        fn from(helper: UpdateFeesAndRewardsDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct CollectFeesDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for CollectFeesDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::CollectFees {
                }),
            )
        }
    }
    impl From<CollectFeesDeserializer> for WhirlpoolInstruction {
        fn from(helper: CollectFeesDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct CollectRewardDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for CollectRewardDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::CollectReward {
                    reward_index: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<CollectRewardDeserializer> for WhirlpoolInstruction {
        fn from(helper: CollectRewardDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct CollectProtocolFeesDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for CollectProtocolFeesDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::CollectProtocolFees {
                }),
            )
        }
    }
    impl From<CollectProtocolFeesDeserializer> for WhirlpoolInstruction {
        fn from(helper: CollectProtocolFeesDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SwapDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::Swap {
                    amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    other_amount_threshold: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    sqrt_price_limit: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    amount_specified_is_input: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    a_to_b: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<SwapDeserializer> for WhirlpoolInstruction {
        fn from(helper: SwapDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct ClosePositionDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for ClosePositionDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::ClosePosition {
                }),
            )
        }
    }
    impl From<ClosePositionDeserializer> for WhirlpoolInstruction {
        fn from(helper: ClosePositionDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetDefaultFeeRateDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetDefaultFeeRateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetDefaultFeeRate {
                    default_fee_rate: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<SetDefaultFeeRateDeserializer> for WhirlpoolInstruction {
        fn from(helper: SetDefaultFeeRateDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetDefaultProtocolFeeRateDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetDefaultProtocolFeeRateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetDefaultProtocolFeeRate {
                    default_protocol_fee_rate: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<SetDefaultProtocolFeeRateDeserializer> for WhirlpoolInstruction {
        fn from(helper: SetDefaultProtocolFeeRateDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetFeeRateDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetFeeRateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetFeeRate {
                    fee_rate: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<SetFeeRateDeserializer> for WhirlpoolInstruction {
        fn from(helper: SetFeeRateDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetProtocolFeeRateDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetProtocolFeeRateDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetProtocolFeeRate {
                    protocol_fee_rate: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<SetProtocolFeeRateDeserializer> for WhirlpoolInstruction {
        fn from(helper: SetProtocolFeeRateDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetFeeAuthorityDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetFeeAuthorityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetFeeAuthority {
                }),
            )
        }
    }
    impl From<SetFeeAuthorityDeserializer> for WhirlpoolInstruction {
        fn from(helper: SetFeeAuthorityDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetCollectProtocolFeesAuthorityDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetCollectProtocolFeesAuthorityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetCollectProtocolFeesAuthority {
                }),
            )
        }
    }
    impl From<SetCollectProtocolFeesAuthorityDeserializer> for WhirlpoolInstruction {
        fn from(
            helper: SetCollectProtocolFeesAuthorityDeserializer,
        ) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetRewardAuthorityDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetRewardAuthorityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetRewardAuthority {
                    reward_index: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<SetRewardAuthorityDeserializer> for WhirlpoolInstruction {
        fn from(helper: SetRewardAuthorityDeserializer) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetRewardAuthorityBySuperAuthorityDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize
    for SetRewardAuthorityBySuperAuthorityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetRewardAuthorityBySuperAuthority {
                    reward_index: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                }),
            )
        }
    }
    impl From<SetRewardAuthorityBySuperAuthorityDeserializer> for WhirlpoolInstruction {
        fn from(
            helper: SetRewardAuthorityBySuperAuthorityDeserializer,
        ) -> WhirlpoolInstruction {
            helper.0
        }
    }
    struct SetRewardEmissionsSuperAuthorityDeserializer(WhirlpoolInstruction);
    impl ::borsh::de::BorshDeserialize for SetRewardEmissionsSuperAuthorityDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(WhirlpoolInstruction::SetRewardEmissionsSuperAuthority {
                }),
            )
        }
    }
    impl From<SetRewardEmissionsSuperAuthorityDeserializer> for WhirlpoolInstruction {
        fn from(
            helper: SetRewardEmissionsSuperAuthorityDeserializer,
        ) -> WhirlpoolInstruction {
            helper.0
        }
    }
    #[derive(Debug)]
    pub struct InitializeConfig {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub config: ::solana_program::pubkey::Pubkey,
        pub funder: ::solana_program::pubkey::Pubkey,
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
                funder,
                system_program,
                trailing_accounts,
                fee_authority,
                collect_protocol_fees_authority,
                reward_emissions_super_authority,
                default_protocol_fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(config, true),
                ::solana_program::instruction::AccountMeta::new(funder, true),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::InitializeConfig {
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
        pub funder: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeConfigAccountIndexes {
        pub const CONFIG: usize = 0usize;
        pub const FUNDER: usize = 1usize;
        pub const SYSTEM_PROGRAM: usize = 2usize;
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
                funder: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(funder),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializePool {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub token_mint_a: ::solana_program::pubkey::Pubkey,
        pub token_mint_b: ::solana_program::pubkey::Pubkey,
        pub funder: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub token_vault_a: ::solana_program::pubkey::Pubkey,
        pub token_vault_b: ::solana_program::pubkey::Pubkey,
        pub fee_tier: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub bumps: WhirlpoolBumps,
        pub tick_spacing: u16,
        pub initial_sqrt_price: u128,
    }
    impl InitializePool {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                token_mint_a,
                token_mint_b,
                funder,
                whirlpool,
                token_vault_a,
                token_vault_b,
                fee_tier,
                token_program,
                system_program,
                rent,
                trailing_accounts,
                bumps,
                tick_spacing,
                initial_sqrt_price,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpools_config,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_a,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_mint_b,
                false), ::solana_program::instruction::AccountMeta::new(funder, true),
                ::solana_program::instruction::AccountMeta::new(whirlpool, false),
                ::solana_program::instruction::AccountMeta::new(token_vault_a, true),
                ::solana_program::instruction::AccountMeta::new(token_vault_b, true),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_tier,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::InitializePool {
                bumps,
                tick_spacing,
                initial_sqrt_price,
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
    pub struct InitializePoolAccountIndexes {
        pub whirlpools_config: usize,
        pub token_mint_a: usize,
        pub token_mint_b: usize,
        pub funder: usize,
        pub whirlpool: usize,
        pub token_vault_a: usize,
        pub token_vault_b: usize,
        pub fee_tier: usize,
        pub token_program: usize,
        pub system_program: usize,
        pub rent: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializePoolAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const TOKEN_MINT_A: usize = 1usize;
        pub const TOKEN_MINT_B: usize = 2usize;
        pub const FUNDER: usize = 3usize;
        pub const WHIRLPOOL: usize = 4usize;
        pub const TOKEN_VAULT_A: usize = 5usize;
        pub const TOKEN_VAULT_B: usize = 6usize;
        pub const FEE_TIER: usize = 7usize;
        pub const TOKEN_PROGRAM: usize = 8usize;
        pub const SYSTEM_PROGRAM: usize = 9usize;
        pub const RENT: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializePoolAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                token_mint_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_a),
                            1usize,
                        ),
                    )?,
                token_mint_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_mint_b),
                            2usize,
                        ),
                    )?,
                funder: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(funder),
                            3usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            4usize,
                        ),
                    )?,
                token_vault_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_a),
                            5usize,
                        ),
                    )?,
                token_vault_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_b),
                            6usize,
                        ),
                    )?,
                fee_tier: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_tier),
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
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            9usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            10usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InitializeTickArray {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
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
                whirlpool,
                funder,
                tick_array,
                system_program,
                trailing_accounts,
                start_tick_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpool,
                false), ::solana_program::instruction::AccountMeta::new(funder, true),
                ::solana_program::instruction::AccountMeta::new(tick_array, false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::InitializeTickArray {
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
        pub whirlpool: usize,
        pub funder: usize,
        pub tick_array: usize,
        pub system_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeTickArrayAccountIndexes {
        pub const WHIRLPOOL: usize = 0usize;
        pub const FUNDER: usize = 1usize;
        pub const TICK_ARRAY: usize = 2usize;
        pub const SYSTEM_PROGRAM: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeTickArrayAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
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
            let data = WhirlpoolInstruction::InitializeFeeTier {
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
    pub struct InitializeReward {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub reward_authority: ::solana_program::pubkey::Pubkey,
        pub funder: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub reward_mint: ::solana_program::pubkey::Pubkey,
        pub reward_vault: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub reward_index: u8,
    }
    impl InitializeReward {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                reward_authority,
                funder,
                whirlpool,
                reward_mint,
                reward_vault,
                token_program,
                system_program,
                rent,
                trailing_accounts,
                reward_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(reward_authority,
                true), ::solana_program::instruction::AccountMeta::new(funder, true),
                ::solana_program::instruction::AccountMeta::new(whirlpool, false),
                ::solana_program::instruction::AccountMeta::new_readonly(reward_mint,
                false), ::solana_program::instruction::AccountMeta::new(reward_vault,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::InitializeReward {
                reward_index,
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
    pub struct InitializeRewardAccountIndexes {
        pub reward_authority: usize,
        pub funder: usize,
        pub whirlpool: usize,
        pub reward_mint: usize,
        pub reward_vault: usize,
        pub token_program: usize,
        pub system_program: usize,
        pub rent: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InitializeRewardAccountIndexes {
        pub const REWARD_AUTHORITY: usize = 0usize;
        pub const FUNDER: usize = 1usize;
        pub const WHIRLPOOL: usize = 2usize;
        pub const REWARD_MINT: usize = 3usize;
        pub const REWARD_VAULT: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
        pub const SYSTEM_PROGRAM: usize = 6usize;
        pub const RENT: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InitializeRewardAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                reward_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_authority),
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
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            2usize,
                        ),
                    )?,
                reward_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_mint),
                            3usize,
                        ),
                    )?,
                reward_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_vault),
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
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetRewardEmissions {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub reward_authority: ::solana_program::pubkey::Pubkey,
        pub reward_vault: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub reward_index: u8,
        pub emissions_per_second_x64: u128,
    }
    impl SetRewardEmissions {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpool,
                reward_authority,
                reward_vault,
                trailing_accounts,
                reward_index,
                emissions_per_second_x64,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpool, false),
                ::solana_program::instruction::AccountMeta::new_readonly(reward_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(reward_vault,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetRewardEmissions {
                reward_index,
                emissions_per_second_x64,
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
    pub struct SetRewardEmissionsAccountIndexes {
        pub whirlpool: usize,
        pub reward_authority: usize,
        pub reward_vault: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetRewardEmissionsAccountIndexes {
        pub const WHIRLPOOL: usize = 0usize;
        pub const REWARD_AUTHORITY: usize = 1usize;
        pub const REWARD_VAULT: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetRewardEmissionsAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            0usize,
                        ),
                    )?,
                reward_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_authority),
                            1usize,
                        ),
                    )?,
                reward_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_vault),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct OpenPosition {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub funder: ::solana_program::pubkey::Pubkey,
        pub owner: ::solana_program::pubkey::Pubkey,
        pub position: ::solana_program::pubkey::Pubkey,
        pub position_mint: ::solana_program::pubkey::Pubkey,
        pub position_token_account: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub bumps: OpenPositionBumps,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
    }
    impl OpenPosition {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                funder,
                owner,
                position,
                position_mint,
                position_token_account,
                whirlpool,
                token_program,
                system_program,
                rent,
                associated_token_program,
                trailing_accounts,
                bumps,
                tick_lower_index,
                tick_upper_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(funder, true),
                ::solana_program::instruction::AccountMeta::new_readonly(owner, false),
                ::solana_program::instruction::AccountMeta::new(position, false),
                ::solana_program::instruction::AccountMeta::new(position_mint, true),
                ::solana_program::instruction::AccountMeta::new(position_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::OpenPosition {
                bumps,
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
    pub struct OpenPositionAccountIndexes {
        pub funder: usize,
        pub owner: usize,
        pub position: usize,
        pub position_mint: usize,
        pub position_token_account: usize,
        pub whirlpool: usize,
        pub token_program: usize,
        pub system_program: usize,
        pub rent: usize,
        pub associated_token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl OpenPositionAccountIndexes {
        pub const FUNDER: usize = 0usize;
        pub const OWNER: usize = 1usize;
        pub const POSITION: usize = 2usize;
        pub const POSITION_MINT: usize = 3usize;
        pub const POSITION_TOKEN_ACCOUNT: usize = 4usize;
        pub const WHIRLPOOL: usize = 5usize;
        pub const TOKEN_PROGRAM: usize = 6usize;
        pub const SYSTEM_PROGRAM: usize = 7usize;
        pub const RENT: usize = 8usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 9usize;
    }
    impl<'a> TryFrom<&'a [u8]> for OpenPositionAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                funder: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(funder),
                            0usize,
                        ),
                    )?,
                owner: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(owner),
                            1usize,
                        ),
                    )?,
                position: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position),
                            2usize,
                        ),
                    )?,
                position_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_mint),
                            3usize,
                        ),
                    )?,
                position_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_token_account),
                            4usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
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
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
                            7usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct OpenPositionWithMetadata {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub funder: ::solana_program::pubkey::Pubkey,
        pub owner: ::solana_program::pubkey::Pubkey,
        pub position: ::solana_program::pubkey::Pubkey,
        pub position_mint: ::solana_program::pubkey::Pubkey,
        pub position_metadata_account: ::solana_program::pubkey::Pubkey,
        pub position_token_account: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub metadata_program: ::solana_program::pubkey::Pubkey,
        pub metadata_update_auth: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub bumps: OpenPositionWithMetadataBumps,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
    }
    impl OpenPositionWithMetadata {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                funder,
                owner,
                position,
                position_mint,
                position_metadata_account,
                position_token_account,
                whirlpool,
                token_program,
                system_program,
                rent,
                associated_token_program,
                metadata_program,
                metadata_update_auth,
                trailing_accounts,
                bumps,
                tick_lower_index,
                tick_upper_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(funder, true),
                ::solana_program::instruction::AccountMeta::new_readonly(owner, false),
                ::solana_program::instruction::AccountMeta::new(position, false),
                ::solana_program::instruction::AccountMeta::new(position_mint, true),
                ::solana_program::instruction::AccountMeta::new(position_metadata_account,
                false),
                ::solana_program::instruction::AccountMeta::new(position_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(metadata_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(metadata_update_auth,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::OpenPositionWithMetadata {
                bumps,
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
    pub struct OpenPositionWithMetadataAccountIndexes {
        pub funder: usize,
        pub owner: usize,
        pub position: usize,
        pub position_mint: usize,
        pub position_metadata_account: usize,
        pub position_token_account: usize,
        pub whirlpool: usize,
        pub token_program: usize,
        pub system_program: usize,
        pub rent: usize,
        pub associated_token_program: usize,
        pub metadata_program: usize,
        pub metadata_update_auth: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl OpenPositionWithMetadataAccountIndexes {
        pub const FUNDER: usize = 0usize;
        pub const OWNER: usize = 1usize;
        pub const POSITION: usize = 2usize;
        pub const POSITION_MINT: usize = 3usize;
        pub const POSITION_METADATA_ACCOUNT: usize = 4usize;
        pub const POSITION_TOKEN_ACCOUNT: usize = 5usize;
        pub const WHIRLPOOL: usize = 6usize;
        pub const TOKEN_PROGRAM: usize = 7usize;
        pub const SYSTEM_PROGRAM: usize = 8usize;
        pub const RENT: usize = 9usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 10usize;
        pub const METADATA_PROGRAM: usize = 11usize;
        pub const METADATA_UPDATE_AUTH: usize = 12usize;
    }
    impl<'a> TryFrom<&'a [u8]> for OpenPositionWithMetadataAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                funder: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(funder),
                            0usize,
                        ),
                    )?,
                owner: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(owner),
                            1usize,
                        ),
                    )?,
                position: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position),
                            2usize,
                        ),
                    )?,
                position_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_mint),
                            3usize,
                        ),
                    )?,
                position_metadata_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_metadata_account),
                            4usize,
                        ),
                    )?,
                position_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_token_account),
                            5usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
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
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            9usize,
                        ),
                    )?,
                associated_token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(associated_token_program),
                            10usize,
                        ),
                    )?,
                metadata_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(metadata_program),
                            11usize,
                        ),
                    )?,
                metadata_update_auth: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(metadata_update_auth),
                            12usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct IncreaseLiquidity {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub position_authority: ::solana_program::pubkey::Pubkey,
        pub position: ::solana_program::pubkey::Pubkey,
        pub position_token_account: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_a: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_b: ::solana_program::pubkey::Pubkey,
        pub token_vault_a: ::solana_program::pubkey::Pubkey,
        pub token_vault_b: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub liquidity_amount: u128,
        pub token_max_a: u64,
        pub token_max_b: u64,
    }
    impl IncreaseLiquidity {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpool,
                token_program,
                position_authority,
                position,
                position_token_account,
                token_owner_account_a,
                token_owner_account_b,
                token_vault_a,
                token_vault_b,
                tick_array_lower,
                tick_array_upper,
                trailing_accounts,
                liquidity_amount,
                token_max_a,
                token_max_b,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpool, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(position_authority,
                true), ::solana_program::instruction::AccountMeta::new(position, false),
                ::solana_program::instruction::AccountMeta::new_readonly(position_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_a,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_b,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_a,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_b,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_lower,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_upper,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::IncreaseLiquidity {
                liquidity_amount,
                token_max_a,
                token_max_b,
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
    pub struct IncreaseLiquidityAccountIndexes {
        pub whirlpool: usize,
        pub token_program: usize,
        pub position_authority: usize,
        pub position: usize,
        pub position_token_account: usize,
        pub token_owner_account_a: usize,
        pub token_owner_account_b: usize,
        pub token_vault_a: usize,
        pub token_vault_b: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl IncreaseLiquidityAccountIndexes {
        pub const WHIRLPOOL: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const POSITION_AUTHORITY: usize = 2usize;
        pub const POSITION: usize = 3usize;
        pub const POSITION_TOKEN_ACCOUNT: usize = 4usize;
        pub const TOKEN_OWNER_ACCOUNT_A: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_B: usize = 6usize;
        pub const TOKEN_VAULT_A: usize = 7usize;
        pub const TOKEN_VAULT_B: usize = 8usize;
        pub const TICK_ARRAY_LOWER: usize = 9usize;
        pub const TICK_ARRAY_UPPER: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for IncreaseLiquidityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            0usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            1usize,
                        ),
                    )?,
                position_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_authority),
                            2usize,
                        ),
                    )?,
                position: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position),
                            3usize,
                        ),
                    )?,
                position_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_token_account),
                            4usize,
                        ),
                    )?,
                token_owner_account_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_a),
                            5usize,
                        ),
                    )?,
                token_owner_account_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_b),
                            6usize,
                        ),
                    )?,
                token_vault_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_a),
                            7usize,
                        ),
                    )?,
                token_vault_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_b),
                            8usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            9usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            10usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct DecreaseLiquidity {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub position_authority: ::solana_program::pubkey::Pubkey,
        pub position: ::solana_program::pubkey::Pubkey,
        pub position_token_account: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_a: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_b: ::solana_program::pubkey::Pubkey,
        pub token_vault_a: ::solana_program::pubkey::Pubkey,
        pub token_vault_b: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub liquidity_amount: u128,
        pub token_min_a: u64,
        pub token_min_b: u64,
    }
    impl DecreaseLiquidity {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpool,
                token_program,
                position_authority,
                position,
                position_token_account,
                token_owner_account_a,
                token_owner_account_b,
                token_vault_a,
                token_vault_b,
                tick_array_lower,
                tick_array_upper,
                trailing_accounts,
                liquidity_amount,
                token_min_a,
                token_min_b,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpool, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(position_authority,
                true), ::solana_program::instruction::AccountMeta::new(position, false),
                ::solana_program::instruction::AccountMeta::new_readonly(position_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_a,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_b,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_a,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_b,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_lower,
                false), ::solana_program::instruction::AccountMeta::new(tick_array_upper,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::DecreaseLiquidity {
                liquidity_amount,
                token_min_a,
                token_min_b,
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
    pub struct DecreaseLiquidityAccountIndexes {
        pub whirlpool: usize,
        pub token_program: usize,
        pub position_authority: usize,
        pub position: usize,
        pub position_token_account: usize,
        pub token_owner_account_a: usize,
        pub token_owner_account_b: usize,
        pub token_vault_a: usize,
        pub token_vault_b: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl DecreaseLiquidityAccountIndexes {
        pub const WHIRLPOOL: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const POSITION_AUTHORITY: usize = 2usize;
        pub const POSITION: usize = 3usize;
        pub const POSITION_TOKEN_ACCOUNT: usize = 4usize;
        pub const TOKEN_OWNER_ACCOUNT_A: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_B: usize = 6usize;
        pub const TOKEN_VAULT_A: usize = 7usize;
        pub const TOKEN_VAULT_B: usize = 8usize;
        pub const TICK_ARRAY_LOWER: usize = 9usize;
        pub const TICK_ARRAY_UPPER: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for DecreaseLiquidityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            0usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            1usize,
                        ),
                    )?,
                position_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_authority),
                            2usize,
                        ),
                    )?,
                position: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position),
                            3usize,
                        ),
                    )?,
                position_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_token_account),
                            4usize,
                        ),
                    )?,
                token_owner_account_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_a),
                            5usize,
                        ),
                    )?,
                token_owner_account_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_b),
                            6usize,
                        ),
                    )?,
                token_vault_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_a),
                            7usize,
                        ),
                    )?,
                token_vault_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_b),
                            8usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            9usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            10usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct UpdateFeesAndRewards {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub position: ::solana_program::pubkey::Pubkey,
        pub tick_array_lower: ::solana_program::pubkey::Pubkey,
        pub tick_array_upper: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl UpdateFeesAndRewards {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpool,
                position,
                tick_array_lower,
                tick_array_upper,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpool, false),
                ::solana_program::instruction::AccountMeta::new(position, false),
                ::solana_program::instruction::AccountMeta::new_readonly(tick_array_lower,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(tick_array_upper,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::UpdateFeesAndRewards {
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
        pub whirlpool: usize,
        pub position: usize,
        pub tick_array_lower: usize,
        pub tick_array_upper: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl UpdateFeesAndRewardsAccountIndexes {
        pub const WHIRLPOOL: usize = 0usize;
        pub const POSITION: usize = 1usize;
        pub const TICK_ARRAY_LOWER: usize = 2usize;
        pub const TICK_ARRAY_UPPER: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for UpdateFeesAndRewardsAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            0usize,
                        ),
                    )?,
                position: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position),
                            1usize,
                        ),
                    )?,
                tick_array_lower: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_lower),
                            2usize,
                        ),
                    )?,
                tick_array_upper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array_upper),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CollectFees {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub position_authority: ::solana_program::pubkey::Pubkey,
        pub position: ::solana_program::pubkey::Pubkey,
        pub position_token_account: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_a: ::solana_program::pubkey::Pubkey,
        pub token_vault_a: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_b: ::solana_program::pubkey::Pubkey,
        pub token_vault_b: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CollectFees {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpool,
                position_authority,
                position,
                position_token_account,
                token_owner_account_a,
                token_vault_a,
                token_owner_account_b,
                token_vault_b,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(position_authority,
                true), ::solana_program::instruction::AccountMeta::new(position, false),
                ::solana_program::instruction::AccountMeta::new_readonly(position_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_a,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_a,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_b,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_b,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::CollectFees {
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
        pub whirlpool: usize,
        pub position_authority: usize,
        pub position: usize,
        pub position_token_account: usize,
        pub token_owner_account_a: usize,
        pub token_vault_a: usize,
        pub token_owner_account_b: usize,
        pub token_vault_b: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CollectFeesAccountIndexes {
        pub const WHIRLPOOL: usize = 0usize;
        pub const POSITION_AUTHORITY: usize = 1usize;
        pub const POSITION: usize = 2usize;
        pub const POSITION_TOKEN_ACCOUNT: usize = 3usize;
        pub const TOKEN_OWNER_ACCOUNT_A: usize = 4usize;
        pub const TOKEN_VAULT_A: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_B: usize = 6usize;
        pub const TOKEN_VAULT_B: usize = 7usize;
        pub const TOKEN_PROGRAM: usize = 8usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CollectFeesAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            0usize,
                        ),
                    )?,
                position_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_authority),
                            1usize,
                        ),
                    )?,
                position: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position),
                            2usize,
                        ),
                    )?,
                position_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_token_account),
                            3usize,
                        ),
                    )?,
                token_owner_account_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_a),
                            4usize,
                        ),
                    )?,
                token_vault_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_a),
                            5usize,
                        ),
                    )?,
                token_owner_account_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_b),
                            6usize,
                        ),
                    )?,
                token_vault_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_b),
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
    pub struct CollectReward {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub position_authority: ::solana_program::pubkey::Pubkey,
        pub position: ::solana_program::pubkey::Pubkey,
        pub position_token_account: ::solana_program::pubkey::Pubkey,
        pub reward_owner_account: ::solana_program::pubkey::Pubkey,
        pub reward_vault: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub reward_index: u8,
    }
    impl CollectReward {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpool,
                position_authority,
                position,
                position_token_account,
                reward_owner_account,
                reward_vault,
                token_program,
                trailing_accounts,
                reward_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(position_authority,
                true), ::solana_program::instruction::AccountMeta::new(position, false),
                ::solana_program::instruction::AccountMeta::new_readonly(position_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(reward_owner_account,
                false), ::solana_program::instruction::AccountMeta::new(reward_vault,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::CollectReward {
                reward_index,
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
    pub struct CollectRewardAccountIndexes {
        pub whirlpool: usize,
        pub position_authority: usize,
        pub position: usize,
        pub position_token_account: usize,
        pub reward_owner_account: usize,
        pub reward_vault: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CollectRewardAccountIndexes {
        pub const WHIRLPOOL: usize = 0usize;
        pub const POSITION_AUTHORITY: usize = 1usize;
        pub const POSITION: usize = 2usize;
        pub const POSITION_TOKEN_ACCOUNT: usize = 3usize;
        pub const REWARD_OWNER_ACCOUNT: usize = 4usize;
        pub const REWARD_VAULT: usize = 5usize;
        pub const TOKEN_PROGRAM: usize = 6usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CollectRewardAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            0usize,
                        ),
                    )?,
                position_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_authority),
                            1usize,
                        ),
                    )?,
                position: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position),
                            2usize,
                        ),
                    )?,
                position_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_token_account),
                            3usize,
                        ),
                    )?,
                reward_owner_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_owner_account),
                            4usize,
                        ),
                    )?,
                reward_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_vault),
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
    pub struct CollectProtocolFees {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
        pub token_vault_a: ::solana_program::pubkey::Pubkey,
        pub token_vault_b: ::solana_program::pubkey::Pubkey,
        pub token_destination_a: ::solana_program::pubkey::Pubkey,
        pub token_destination_b: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CollectProtocolFees {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                whirlpool,
                collect_protocol_fees_authority,
                token_vault_a,
                token_vault_b,
                token_destination_a,
                token_destination_b,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpools_config,
                false), ::solana_program::instruction::AccountMeta::new(whirlpool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(collect_protocol_fees_authority,
                true), ::solana_program::instruction::AccountMeta::new(token_vault_a,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_b,
                false),
                ::solana_program::instruction::AccountMeta::new(token_destination_a,
                false),
                ::solana_program::instruction::AccountMeta::new(token_destination_b,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::CollectProtocolFees {
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
        pub whirlpools_config: usize,
        pub whirlpool: usize,
        pub collect_protocol_fees_authority: usize,
        pub token_vault_a: usize,
        pub token_vault_b: usize,
        pub token_destination_a: usize,
        pub token_destination_b: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CollectProtocolFeesAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const WHIRLPOOL: usize = 1usize;
        pub const COLLECT_PROTOCOL_FEES_AUTHORITY: usize = 2usize;
        pub const TOKEN_VAULT_A: usize = 3usize;
        pub const TOKEN_VAULT_B: usize = 4usize;
        pub const TOKEN_DESTINATION_A: usize = 5usize;
        pub const TOKEN_DESTINATION_B: usize = 6usize;
        pub const TOKEN_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CollectProtocolFeesAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            1usize,
                        ),
                    )?,
                collect_protocol_fees_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(collect_protocol_fees_authority),
                            2usize,
                        ),
                    )?,
                token_vault_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_a),
                            3usize,
                        ),
                    )?,
                token_vault_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_b),
                            4usize,
                        ),
                    )?,
                token_destination_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_destination_a),
                            5usize,
                        ),
                    )?,
                token_destination_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_destination_b),
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
    pub struct Swap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub token_authority: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_a: ::solana_program::pubkey::Pubkey,
        pub token_vault_a: ::solana_program::pubkey::Pubkey,
        pub token_owner_account_b: ::solana_program::pubkey::Pubkey,
        pub token_vault_b: ::solana_program::pubkey::Pubkey,
        pub tick_array0: ::solana_program::pubkey::Pubkey,
        pub tick_array1: ::solana_program::pubkey::Pubkey,
        pub tick_array2: ::solana_program::pubkey::Pubkey,
        pub oracle: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub amount: u64,
        pub other_amount_threshold: u64,
        pub sqrt_price_limit: u128,
        pub amount_specified_is_input: bool,
        pub a_to_b: bool,
    }
    impl Swap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                token_program,
                token_authority,
                whirlpool,
                token_owner_account_a,
                token_vault_a,
                token_owner_account_b,
                token_vault_b,
                tick_array0,
                tick_array1,
                tick_array2,
                oracle,
                trailing_accounts,
                amount,
                other_amount_threshold,
                sqrt_price_limit,
                amount_specified_is_input,
                a_to_b,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_authority,
                true), ::solana_program::instruction::AccountMeta::new(whirlpool, false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_a,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_a,
                false),
                ::solana_program::instruction::AccountMeta::new(token_owner_account_b,
                false), ::solana_program::instruction::AccountMeta::new(token_vault_b,
                false), ::solana_program::instruction::AccountMeta::new(tick_array0,
                false), ::solana_program::instruction::AccountMeta::new(tick_array1,
                false), ::solana_program::instruction::AccountMeta::new(tick_array2,
                false), ::solana_program::instruction::AccountMeta::new_readonly(oracle,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::Swap {
                amount,
                other_amount_threshold,
                sqrt_price_limit,
                amount_specified_is_input,
                a_to_b,
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
    pub struct SwapAccountIndexes {
        pub token_program: usize,
        pub token_authority: usize,
        pub whirlpool: usize,
        pub token_owner_account_a: usize,
        pub token_vault_a: usize,
        pub token_owner_account_b: usize,
        pub token_vault_b: usize,
        pub tick_array0: usize,
        pub tick_array1: usize,
        pub tick_array2: usize,
        pub oracle: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SwapAccountIndexes {
        pub const TOKEN_PROGRAM: usize = 0usize;
        pub const TOKEN_AUTHORITY: usize = 1usize;
        pub const WHIRLPOOL: usize = 2usize;
        pub const TOKEN_OWNER_ACCOUNT_A: usize = 3usize;
        pub const TOKEN_VAULT_A: usize = 4usize;
        pub const TOKEN_OWNER_ACCOUNT_B: usize = 5usize;
        pub const TOKEN_VAULT_B: usize = 6usize;
        pub const TICK_ARRAY0: usize = 7usize;
        pub const TICK_ARRAY1: usize = 8usize;
        pub const TICK_ARRAY2: usize = 9usize;
        pub const ORACLE: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            0usize,
                        ),
                    )?,
                token_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_authority),
                            1usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            2usize,
                        ),
                    )?,
                token_owner_account_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_a),
                            3usize,
                        ),
                    )?,
                token_vault_a: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_a),
                            4usize,
                        ),
                    )?,
                token_owner_account_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_owner_account_b),
                            5usize,
                        ),
                    )?,
                token_vault_b: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_vault_b),
                            6usize,
                        ),
                    )?,
                tick_array0: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array0),
                            7usize,
                        ),
                    )?,
                tick_array1: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array1),
                            8usize,
                        ),
                    )?,
                tick_array2: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array2),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct ClosePosition {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub position_authority: ::solana_program::pubkey::Pubkey,
        pub receiver: ::solana_program::pubkey::Pubkey,
        pub position: ::solana_program::pubkey::Pubkey,
        pub position_mint: ::solana_program::pubkey::Pubkey,
        pub position_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl ClosePosition {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                position_authority,
                receiver,
                position,
                position_mint,
                position_token_account,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(position_authority,
                true), ::solana_program::instruction::AccountMeta::new(receiver, false),
                ::solana_program::instruction::AccountMeta::new(position, false),
                ::solana_program::instruction::AccountMeta::new(position_mint, false),
                ::solana_program::instruction::AccountMeta::new(position_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::ClosePosition {
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
    pub struct ClosePositionAccountIndexes {
        pub position_authority: usize,
        pub receiver: usize,
        pub position: usize,
        pub position_mint: usize,
        pub position_token_account: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl ClosePositionAccountIndexes {
        pub const POSITION_AUTHORITY: usize = 0usize;
        pub const RECEIVER: usize = 1usize;
        pub const POSITION: usize = 2usize;
        pub const POSITION_MINT: usize = 3usize;
        pub const POSITION_TOKEN_ACCOUNT: usize = 4usize;
        pub const TOKEN_PROGRAM: usize = 5usize;
    }
    impl<'a> TryFrom<&'a [u8]> for ClosePositionAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                position_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_authority),
                            0usize,
                        ),
                    )?,
                receiver: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(receiver),
                            1usize,
                        ),
                    )?,
                position: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position),
                            2usize,
                        ),
                    )?,
                position_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_mint),
                            3usize,
                        ),
                    )?,
                position_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(position_token_account),
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
    pub struct SetDefaultFeeRate {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub fee_tier: ::solana_program::pubkey::Pubkey,
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub default_fee_rate: u16,
    }
    impl SetDefaultFeeRate {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                fee_tier,
                fee_authority,
                trailing_accounts,
                default_fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpools_config,
                false), ::solana_program::instruction::AccountMeta::new(fee_tier, false),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_authority,
                true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetDefaultFeeRate {
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
    pub struct SetDefaultFeeRateAccountIndexes {
        pub whirlpools_config: usize,
        pub fee_tier: usize,
        pub fee_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetDefaultFeeRateAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const FEE_TIER: usize = 1usize;
        pub const FEE_AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetDefaultFeeRateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
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
                fee_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetDefaultProtocolFeeRate {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub default_protocol_fee_rate: u16,
    }
    impl SetDefaultProtocolFeeRate {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                fee_authority,
                trailing_accounts,
                default_protocol_fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpools_config,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_authority,
                true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetDefaultProtocolFeeRate {
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
    pub struct SetDefaultProtocolFeeRateAccountIndexes {
        pub whirlpools_config: usize,
        pub fee_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetDefaultProtocolFeeRateAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const FEE_AUTHORITY: usize = 1usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetDefaultProtocolFeeRateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                fee_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_authority),
                            1usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetFeeRate {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub fee_rate: u16,
    }
    impl SetFeeRate {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                whirlpool,
                fee_authority,
                trailing_accounts,
                fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpools_config,
                false), ::solana_program::instruction::AccountMeta::new(whirlpool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_authority,
                true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetFeeRate {
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
    pub struct SetFeeRateAccountIndexes {
        pub whirlpools_config: usize,
        pub whirlpool: usize,
        pub fee_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetFeeRateAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const WHIRLPOOL: usize = 1usize;
        pub const FEE_AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetFeeRateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            1usize,
                        ),
                    )?,
                fee_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetProtocolFeeRate {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub protocol_fee_rate: u16,
    }
    impl SetProtocolFeeRate {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                whirlpool,
                fee_authority,
                trailing_accounts,
                protocol_fee_rate,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpools_config,
                false), ::solana_program::instruction::AccountMeta::new(whirlpool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_authority,
                true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetProtocolFeeRate {
                protocol_fee_rate,
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
    pub struct SetProtocolFeeRateAccountIndexes {
        pub whirlpools_config: usize,
        pub whirlpool: usize,
        pub fee_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetProtocolFeeRateAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const WHIRLPOOL: usize = 1usize;
        pub const FEE_AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetProtocolFeeRateAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            1usize,
                        ),
                    )?,
                fee_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetFeeAuthority {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub new_fee_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl SetFeeAuthority {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                fee_authority,
                new_fee_authority,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpools_config,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(new_fee_authority,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetFeeAuthority {
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
    pub struct SetFeeAuthorityAccountIndexes {
        pub whirlpools_config: usize,
        pub fee_authority: usize,
        pub new_fee_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetFeeAuthorityAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const FEE_AUTHORITY: usize = 1usize;
        pub const NEW_FEE_AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetFeeAuthorityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                fee_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_authority),
                            1usize,
                        ),
                    )?,
                new_fee_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(new_fee_authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetCollectProtocolFeesAuthority {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
        pub new_collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl SetCollectProtocolFeesAuthority {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                collect_protocol_fees_authority,
                new_collect_protocol_fees_authority,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpools_config,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(collect_protocol_fees_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(new_collect_protocol_fees_authority,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetCollectProtocolFeesAuthority {
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
    pub struct SetCollectProtocolFeesAuthorityAccountIndexes {
        pub whirlpools_config: usize,
        pub collect_protocol_fees_authority: usize,
        pub new_collect_protocol_fees_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetCollectProtocolFeesAuthorityAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const COLLECT_PROTOCOL_FEES_AUTHORITY: usize = 1usize;
        pub const NEW_COLLECT_PROTOCOL_FEES_AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetCollectProtocolFeesAuthorityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                collect_protocol_fees_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(collect_protocol_fees_authority),
                            1usize,
                        ),
                    )?,
                new_collect_protocol_fees_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(new_collect_protocol_fees_authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetRewardAuthority {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub reward_authority: ::solana_program::pubkey::Pubkey,
        pub new_reward_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub reward_index: u8,
    }
    impl SetRewardAuthority {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpool,
                reward_authority,
                new_reward_authority,
                trailing_accounts,
                reward_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpool, false),
                ::solana_program::instruction::AccountMeta::new_readonly(reward_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(new_reward_authority,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetRewardAuthority {
                reward_index,
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
    pub struct SetRewardAuthorityAccountIndexes {
        pub whirlpool: usize,
        pub reward_authority: usize,
        pub new_reward_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetRewardAuthorityAccountIndexes {
        pub const WHIRLPOOL: usize = 0usize;
        pub const REWARD_AUTHORITY: usize = 1usize;
        pub const NEW_REWARD_AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetRewardAuthorityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            0usize,
                        ),
                    )?,
                reward_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_authority),
                            1usize,
                        ),
                    )?,
                new_reward_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(new_reward_authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetRewardAuthorityBySuperAuthority {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
        pub new_reward_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub reward_index: u8,
    }
    impl SetRewardAuthorityBySuperAuthority {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                whirlpool,
                reward_emissions_super_authority,
                new_reward_authority,
                trailing_accounts,
                reward_index,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(whirlpools_config,
                false), ::solana_program::instruction::AccountMeta::new(whirlpool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(reward_emissions_super_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(new_reward_authority,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetRewardAuthorityBySuperAuthority {
                reward_index,
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
    pub struct SetRewardAuthorityBySuperAuthorityAccountIndexes {
        pub whirlpools_config: usize,
        pub whirlpool: usize,
        pub reward_emissions_super_authority: usize,
        pub new_reward_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetRewardAuthorityBySuperAuthorityAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const WHIRLPOOL: usize = 1usize;
        pub const REWARD_EMISSIONS_SUPER_AUTHORITY: usize = 2usize;
        pub const NEW_REWARD_AUTHORITY: usize = 3usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetRewardAuthorityBySuperAuthorityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
                            1usize,
                        ),
                    )?,
                reward_emissions_super_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_emissions_super_authority),
                            2usize,
                        ),
                    )?,
                new_reward_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(new_reward_authority),
                            3usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SetRewardEmissionsSuperAuthority {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
        pub new_reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl SetRewardEmissionsSuperAuthority {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                whirlpools_config,
                reward_emissions_super_authority,
                new_reward_emissions_super_authority,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(whirlpools_config,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(reward_emissions_super_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(new_reward_emissions_super_authority,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = WhirlpoolInstruction::SetRewardEmissionsSuperAuthority {
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
    pub struct SetRewardEmissionsSuperAuthorityAccountIndexes {
        pub whirlpools_config: usize,
        pub reward_emissions_super_authority: usize,
        pub new_reward_emissions_super_authority: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SetRewardEmissionsSuperAuthorityAccountIndexes {
        pub const WHIRLPOOLS_CONFIG: usize = 0usize;
        pub const REWARD_EMISSIONS_SUPER_AUTHORITY: usize = 1usize;
        pub const NEW_REWARD_EMISSIONS_SUPER_AUTHORITY: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SetRewardEmissionsSuperAuthorityAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                whirlpools_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpools_config),
                            0usize,
                        ),
                    )?,
                reward_emissions_super_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reward_emissions_super_authority),
                            1usize,
                        ),
                    )?,
                new_reward_emissions_super_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(new_reward_emissions_super_authority),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
}
pub mod types {
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct OpenPositionBumps {
        pub position_bump: u8,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct OpenPositionWithMetadataBumps {
        pub position_bump: u8,
        pub metadata_bump: u8,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct PositionRewardInfo {
        pub growth_inside_checkpoint: u128,
        pub amount_owed: u64,
    }
    #[derive(Clone, Copy, Default, ::bytemuck::Pod, ::bytemuck::Zeroable)]
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
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct WhirlpoolRewardInfo {
        pub mint: ::solana_program::pubkey::Pubkey,
        pub vault: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub emissions_per_second_x64: u128,
        pub growth_global_x64: u128,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct WhirlpoolBumps {
        pub whirlpool_bump: u8,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum CurrIndex {
        Below,
        Inside,
        Above,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum TickLabel {
        Upper,
        Lower,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum Direction {
        Left,
        Right,
    }
}
pub mod state {
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct WhirlpoolsConfig {
        pub fee_authority: ::solana_program::pubkey::Pubkey,
        pub collect_protocol_fees_authority: ::solana_program::pubkey::Pubkey,
        pub reward_emissions_super_authority: ::solana_program::pubkey::Pubkey,
        pub default_protocol_fee_rate: u16,
    }
    impl ::anchor_interface::Account for WhirlpoolsConfig {
        const DISCRIMINATOR: &'static [u8] = &[
            157u8,
            20u8,
            49u8,
            224u8,
            217u8,
            87u8,
            193u8,
            254u8,
        ];
    }
    impl ::anchor_interface::AccountSerialize for WhirlpoolsConfig {
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
    impl ::anchor_interface::AccountDeserialize for WhirlpoolsConfig {
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
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct FeeTier {
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub tick_spacing: u16,
        pub default_fee_rate: u16,
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
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct Position {
        pub whirlpool: ::solana_program::pubkey::Pubkey,
        pub position_mint: ::solana_program::pubkey::Pubkey,
        pub liquidity: u128,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub fee_growth_checkpoint_a: u128,
        pub fee_owed_a: u64,
        pub fee_growth_checkpoint_b: u128,
        pub fee_owed_b: u64,
        pub reward_infos: [PositionRewardInfo; 3usize],
    }
    impl ::anchor_interface::Account for Position {
        const DISCRIMINATOR: &'static [u8] = &[
            170u8,
            188u8,
            143u8,
            228u8,
            122u8,
            64u8,
            247u8,
            208u8,
        ];
    }
    impl ::anchor_interface::AccountSerialize for Position {
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
    impl ::anchor_interface::AccountDeserialize for Position {
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
    #[derive(Clone, Copy, ::bytemuck::Pod, ::bytemuck::Zeroable)]
    #[cfg_attr(not(target_arch = "bpf"), derive(Debug))]
    #[repr(C)]
    #[repr(packed)]
    pub struct TickArray {
        pub start_tick_index: i32,
        pub ticks: [Tick; 88usize],
        pub whirlpool: ::solana_program::pubkey::Pubkey,
    }
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
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct Whirlpool {
        pub whirlpools_config: ::solana_program::pubkey::Pubkey,
        pub whirlpool_bump: [u8; 1usize],
        pub tick_spacing: u16,
        pub tick_spacing_seed: [u8; 2usize],
        pub fee_rate: u16,
        pub protocol_fee_rate: u16,
        pub liquidity: u128,
        pub sqrt_price: u128,
        pub tick_current_index: i32,
        pub protocol_fee_owed_a: u64,
        pub protocol_fee_owed_b: u64,
        pub token_mint_a: ::solana_program::pubkey::Pubkey,
        pub token_vault_a: ::solana_program::pubkey::Pubkey,
        pub fee_growth_global_a: u128,
        pub token_mint_b: ::solana_program::pubkey::Pubkey,
        pub token_vault_b: ::solana_program::pubkey::Pubkey,
        pub fee_growth_global_b: u128,
        pub reward_last_updated_timestamp: u64,
        pub reward_infos: [WhirlpoolRewardInfo; 3usize],
    }
    impl ::anchor_interface::Account for Whirlpool {
        const DISCRIMINATOR: &'static [u8] = &[
            63u8,
            149u8,
            209u8,
            12u8,
            225u8,
            128u8,
            99u8,
            9u8,
        ];
    }
    impl ::anchor_interface::AccountSerialize for Whirlpool {
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
    impl ::anchor_interface::AccountDeserialize for Whirlpool {
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
}
pub mod error {
    #[allow(unused_imports)]
    use super::types::*;
    use ::num_derive::FromPrimitive;
    use ::thiserror::Error;
    use ::solana_program::{decode_error::DecodeError, program_error::ProgramError};
    #[derive(Error, Clone, Copy, Debug, FromPrimitive, PartialEq, Eq)]
    #[repr(u32)]
    pub enum WhirlpoolError {
        #[error("Enum value could not be converted")]
        InvalidEnum = 6000u32,
        #[error("Invalid start tick index provided.")]
        InvalidStartTick = 6001u32,
        #[error("Tick-array already exists in this whirlpool")]
        TickArrayExistInPool = 6002u32,
        #[error("Attempt to search for a tick-array failed")]
        TickArrayIndexOutofBounds = 6003u32,
        #[error("Tick-spacing is not supported")]
        InvalidTickSpacing = 6004u32,
        #[error("Position is not empty It cannot be closed")]
        ClosePositionNotEmpty = 6005u32,
        #[error("Unable to divide by zero")]
        DivideByZero = 6006u32,
        #[error("Unable to cast number into BigInt")]
        NumberCastError = 6007u32,
        #[error("Unable to down cast number")]
        NumberDownCastError = 6008u32,
        #[error("Tick not found within tick array")]
        TickNotFound = 6009u32,
        #[error("Provided tick index is either out of bounds or uninitializable")]
        InvalidTickIndex = 6010u32,
        #[error("Provided sqrt price out of bounds")]
        SqrtPriceOutOfBounds = 6011u32,
        #[error("Liquidity amount must be greater than zero")]
        LiquidityZero = 6012u32,
        #[error("Liquidity amount must be less than i64::MAX")]
        LiquidityTooHigh = 6013u32,
        #[error("Liquidity overflow")]
        LiquidityOverflow = 6014u32,
        #[error("Liquidity underflow")]
        LiquidityUnderflow = 6015u32,
        #[error("Tick liquidity net underflowed or overflowed")]
        LiquidityNetError = 6016u32,
        #[error("Exceeded token max")]
        TokenMaxExceeded = 6017u32,
        #[error("Did not meet token min")]
        TokenMinSubceeded = 6018u32,
        #[error("Position token account has a missing or invalid delegate")]
        MissingOrInvalidDelegate = 6019u32,
        #[error("Position token amount must be 1")]
        InvalidPositionTokenAmount = 6020u32,
        #[error("Timestamp should be convertible from i64 to u64")]
        InvalidTimestampConversion = 6021u32,
        #[error("Timestamp should be greater than the last updated timestamp")]
        InvalidTimestamp = 6022u32,
        #[error("Invalid tick array sequence provided for instruction.")]
        InvalidTickArraySequence = 6023u32,
        #[error("Token Mint in wrong order")]
        InvalidTokenMintOrder = 6024u32,
        #[error("Reward not initialized")]
        RewardNotInitialized = 6025u32,
        #[error("Invalid reward index")]
        InvalidRewardIndex = 6026u32,
        #[error(
            "Reward vault requires amount to support emissions for at least one day"
        )]
        RewardVaultAmountInsufficient = 6027u32,
        #[error("Exceeded max fee rate")]
        FeeRateMaxExceeded = 6028u32,
        #[error("Exceeded max protocol fee rate")]
        ProtocolFeeRateMaxExceeded = 6029u32,
        #[error("Multiplication with shift right overflow")]
        MultiplicationShiftRightOverflow = 6030u32,
        #[error("Muldiv overflow")]
        MulDivOverflow = 6031u32,
        #[error("Invalid div_u256 input")]
        MulDivInvalidInput = 6032u32,
        #[error("Multiplication overflow")]
        MultiplicationOverflow = 6033u32,
        #[error("Provided SqrtPriceLimit not in the same direction as the swap.")]
        InvalidSqrtPriceLimitDirection = 6034u32,
        #[error("There are no tradable amount to swap.")]
        ZeroTradableAmount = 6035u32,
        #[error("Amount out below minimum threshold")]
        AmountOutBelowMinimum = 6036u32,
        #[error("Amount in above maximum threshold")]
        AmountInAboveMaximum = 6037u32,
        #[error("Invalid index for tick array sequence")]
        TickArraySequenceInvalidIndex = 6038u32,
        #[error("Amount calculated overflows")]
        AmountCalcOverflow = 6039u32,
        #[error("Amount remaining overflows")]
        AmountRemainingOverflow = 6040u32,
    }
    impl DecodeError<WhirlpoolError> for WhirlpoolError {
        fn type_of() -> &'static str {
            "WhirlpoolError"
        }
    }
    impl From<WhirlpoolError> for ProgramError {
        fn from(err: WhirlpoolError) -> Self {
            Self::Custom(err as u32)
        }
    }
}

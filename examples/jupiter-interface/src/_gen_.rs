macro_rules! gen_crate_docs {
    () => {
        concat!(" ", "Jupiter", " v", "0.1.0",
        " program interface generated from Anchor IDL.")
    };
}
pub(crate) use gen_crate_docs;
pub use anchor_interface::prelude::*;
pub mod instruction {
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Debug)]
    pub enum JupiterInstruction {
        /// Route
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` token program
        /// 1. `[signer]` user transfer authority
        /// 2. `[]` destination token account
        Route {
            swap_leg: SwapLeg,
            in_amount: u64,
            quoted_out_amount: u64,
            slippage_bps: u16,
            platform_fee_bps: u8,
        },
        /// Whirlpool Swap Exact Output
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` token program
        /// 2. `[signer]` token authority
        /// 3. `[writable]` whirlpool
        /// 4. `[writable]` token owner account a
        /// 5. `[writable]` token vault a
        /// 6. `[writable]` token owner account b
        /// 7. `[writable]` token vault b
        /// 8. `[writable]` tick array0
        /// 9. `[writable]` tick array1
        /// 10. `[writable]` tick array2
        /// 11. `[]` oracle
        WhirlpoolSwapExactOutput {
            out_amount: u64,
            in_amount_with_slippage: AmountWithSlippage,
            a_to_b: bool,
            platform_fee_bps: u8,
        },
        /// Create Open Orders
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` open orders
        /// 1. `[signer, writable]` payer
        /// 2. `[]` dex program
        /// 3. `[]` system program
        /// 4. `[]` rent
        /// 5. `[]` market
        CreateOpenOrders,
        /// Mercurial Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` swap state
        /// 2. `[]` token program
        /// 3. `[]` pool authority
        /// 4. `[signer]` user transfer authority
        /// 5. `[writable]` source token account
        /// 6. `[writable]` destination token account
        MercurialSwap,
        /// Cykura Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[signer]` signer
        /// 2. `[]` factory state
        /// 3. `[writable]` pool state
        /// 4. `[writable]` input token account
        /// 5. `[writable]` output token account
        /// 6. `[writable]` input vault
        /// 7. `[writable]` output vault
        /// 8. `[writable]` last observation state
        /// 9. `[]` core program
        /// 10. `[]` token program
        CykuraSwap,
        /// Serum Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[writable]` market
        /// 1. `[writable]` open orders
        /// 2. `[writable]` request queue
        /// 3. `[writable]` event queue
        /// 4. `[writable]` bids
        /// 5. `[writable]` asks
        /// 6. `[writable]` coin vault
        /// 7. `[writable]` pc vault
        /// 8. `[]` vault signer
        /// 9. `[signer]` authority
        /// 10. `[writable]` order payer token account
        /// 11. `[writable]` coin wallet
        /// 12. `[writable]` pc wallet
        /// 13. `[]` dex program
        /// 14. `[]` token program
        /// 15. `[]` rent
        SerumSwap,
        /// Saber Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` token program
        /// 2. `[]` swap
        /// 3. `[]` swap authority
        /// 4. `[]` user authority
        /// 5. `[writable]` input user account
        /// 6. `[writable]` input token account
        /// 7. `[writable]` output user account
        /// 8. `[writable]` output token account
        /// 9. `[writable]` fees token account
        SaberSwap,
        /// Saber Add Decimals
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` add decimals program
        /// 1. `[]` wrapper
        /// 2. `[writable]` wrapper mint
        /// 3. `[writable]` wrapper underlying tokens
        /// 4. `[signer]` owner
        /// 5. `[writable]` user underlying tokens
        /// 6. `[writable]` user wrapped tokens
        /// 7. `[]` token program
        SaberAddDecimals,
        /// Token Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` token swap program
        /// 1. `[]` token program
        /// 2. `[]` swap
        /// 3. `[]` authority
        /// 4. `[signer]` user transfer authority
        /// 5. `[writable]` source
        /// 6. `[writable]` swap source
        /// 7. `[writable]` swap destination
        /// 8. `[writable]` destination
        /// 9. `[writable]` pool mint
        /// 10. `[writable]` pool fee
        TokenSwap,
        /// Sencha Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` token program
        /// 2. `[writable]` swap
        /// 3. `[]` user authority
        /// 4. `[writable]` input user account
        /// 5. `[writable]` input token account
        /// 6. `[writable]` input fees account
        /// 7. `[writable]` output user account
        /// 8. `[writable]` output token account
        /// 9. `[writable]` output fees account
        SenchaSwap,
        /// Step Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` token swap program
        /// 1. `[]` token program
        /// 2. `[]` swap
        /// 3. `[]` authority
        /// 4. `[signer]` user transfer authority
        /// 5. `[writable]` source
        /// 6. `[writable]` swap source
        /// 7. `[writable]` swap destination
        /// 8. `[writable]` destination
        /// 9. `[writable]` pool mint
        /// 10. `[writable]` pool fee
        StepSwap,
        /// Cropper Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` token swap program
        /// 1. `[]` token program
        /// 2. `[]` swap
        /// 3. `[]` swap state
        /// 4. `[]` authority
        /// 5. `[signer]` user transfer authority
        /// 6. `[writable]` source
        /// 7. `[writable]` swap source
        /// 8. `[writable]` swap destination
        /// 9. `[writable]` destination
        /// 10. `[writable]` pool mint
        /// 11. `[writable]` pool fee
        CropperSwap,
        /// Raydium Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` token program
        /// 2. `[writable]` amm id
        /// 3. `[]` amm authority
        /// 4. `[writable]` amm open orders
        /// 5. `[writable]` pool coin token account
        /// 6. `[writable]` pool pc token account
        /// 7. `[]` serum program id
        /// 8. `[writable]` serum market
        /// 9. `[writable]` serum bids
        /// 10. `[writable]` serum asks
        /// 11. `[writable]` serum event queue
        /// 12. `[writable]` serum coin vault account
        /// 13. `[writable]` serum pc vault account
        /// 14. `[]` serum vault signer
        /// 15. `[writable]` user source token account
        /// 16. `[writable]` user destination token account
        /// 17. `[signer]` user source owner
        RaydiumSwap,
        /// Crema Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[writable]` pool
        /// 2. `[]` pool signer
        /// 3. `[writable]` user source token account
        /// 4. `[writable]` user destination token account
        /// 5. `[writable]` pool source token account
        /// 6. `[writable]` pool destination token account
        /// 7. `[writable]` pool ticks account
        /// 8. `[signer]` wallet authority
        /// 9. `[]` token program
        CremaSwap,
        /// Lifinity Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` authority
        /// 2. `[]` amm
        /// 3. `[signer]` user transfer authority
        /// 4. `[writable]` source info
        /// 5. `[writable]` destination info
        /// 6. `[writable]` swap source
        /// 7. `[writable]` swap destination
        /// 8. `[writable]` pool mint
        /// 9. `[writable]` fee account
        /// 10. `[]` token program
        /// 11. `[]` pyth account
        /// 12. `[]` pyth pc account
        /// 13. `[writable]` config account
        LifinitySwap,
        /// Marinade Deposit
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` marinade finance program
        /// 1. `[writable]` state
        /// 2. `[writable]` msol mint
        /// 3. `[writable]` liq pool sol leg pda
        /// 4. `[writable]` liq pool msol leg
        /// 5. `[]` liq pool msol leg authority
        /// 6. `[writable]` reserve pda
        /// 7. `[writable]` transfer from
        /// 8. `[writable]` mint to
        /// 9. `[]` msol mint authority
        /// 10. `[]` system program
        /// 11. `[]` token program
        /// 12. `[writable]` user wsol token account
        /// 13. `[writable]` temp wsol token account
        /// 14. `[signer, writable]` user transfer authority
        /// 15. `[]` wsol mint
        /// 16. `[]` rent
        MarinadeDeposit,
        /// Marinade Unstake
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` marinade finance program
        /// 1. `[writable]` state
        /// 2. `[writable]` msol mint
        /// 3. `[writable]` liq pool sol leg pda
        /// 4. `[writable]` liq pool msol leg
        /// 5. `[writable]` treasury msol account
        /// 6. `[writable]` get msol from
        /// 7. `[signer]` get msol from authority
        /// 8. `[writable]` transfer sol to
        /// 9. `[]` system program
        /// 10. `[]` token program
        /// 11. `[writable]` user wsol token account
        MarinadeUnstake,
        /// Aldrin Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` pool
        /// 2. `[]` pool signer
        /// 3. `[writable]` pool mint
        /// 4. `[writable]` base token vault
        /// 5. `[writable]` quote token vault
        /// 6. `[writable]` fee pool token account
        /// 7. `[signer]` wallet authority
        /// 8. `[writable]` user base token account
        /// 9. `[writable]` user quote token account
        /// 10. `[]` token program
        AldrinSwap,
        /// Aldrin V2 Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` pool
        /// 2. `[]` pool signer
        /// 3. `[writable]` pool mint
        /// 4. `[writable]` base token vault
        /// 5. `[writable]` quote token vault
        /// 6. `[writable]` fee pool token account
        /// 7. `[signer]` wallet authority
        /// 8. `[writable]` user base token account
        /// 9. `[writable]` user quote token account
        /// 10. `[]` curve
        /// 11. `[]` token program
        AldrinV2Swap,
        /// Whirlpool Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` token program
        /// 2. `[signer]` token authority
        /// 3. `[writable]` whirlpool
        /// 4. `[writable]` token owner account a
        /// 5. `[writable]` token vault a
        /// 6. `[writable]` token owner account b
        /// 7. `[writable]` token vault b
        /// 8. `[writable]` tick array0
        /// 9. `[writable]` tick array1
        /// 10. `[writable]` tick array2
        /// 11. `[]` oracle
        WhirlpoolSwap,
        /// Invariant Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` state
        /// 2. `[writable]` pool
        /// 3. `[writable]` tickmap
        /// 4. `[writable]` account x
        /// 5. `[writable]` account y
        /// 6. `[writable]` reserve x
        /// 7. `[writable]` reserve y
        /// 8. `[signer]` owner
        /// 9. `[]` program authority
        /// 10. `[]` token program
        InvariantSwap,
        /// Meteora Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[writable]` pool
        /// 2. `[writable]` user source token
        /// 3. `[writable]` user destination token
        /// 4. `[writable]` a vault
        /// 5. `[writable]` b vault
        /// 6. `[writable]` a token vault
        /// 7. `[writable]` b token vault
        /// 8. `[writable]` a vault lp mint
        /// 9. `[writable]` b vault lp mint
        /// 10. `[writable]` a vault lp
        /// 11. `[writable]` b vault lp
        /// 12. `[writable]` admin token fee
        /// 13. `[signer]` user
        /// 14. `[]` vault program
        /// 15. `[]` token program
        MeteoraSwap,
        /// Goosefx Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` controller
        /// 2. `[writable]` pair
        /// 3. `[writable]` ssl in
        /// 4. `[writable]` ssl out
        /// 5. `[writable]` liability vault in
        /// 6. `[writable]` swapped liability vault in
        /// 7. `[writable]` liability vault out
        /// 8. `[writable]` swapped liability vault out
        /// 9. `[writable]` user in ata
        /// 10. `[writable]` user out ata
        /// 11. `[writable]` fee collector ata
        /// 12. `[signer]` user wallet
        /// 13. `[]` fee collector
        /// 14. `[]` token program
        GoosefxSwap,
        /// Deltafi Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` market config
        /// 2. `[writable]` swap info
        /// 3. `[writable]` user source token
        /// 4. `[writable]` user destination token
        /// 5. `[writable]` swap source token
        /// 6. `[writable]` swap destination token
        /// 7. `[writable]` deltafi user
        /// 8. `[writable]` admin destination token
        /// 9. `[]` pyth price base
        /// 10. `[]` pyth price quote
        /// 11. `[signer]` user authority
        /// 12. `[]` token program
        DeltafiSwap,
        /// Balansol Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[signer, writable]` authority
        /// 2. `[writable]` pool
        /// 3. `[writable]` tax man
        /// 4. `[]` bid mint
        /// 5. `[]` treasurer
        /// 6. `[writable]` src treasury
        /// 7. `[writable]` src associated token account
        /// 8. `[]` ask mint
        /// 9. `[writable]` dst treasury
        /// 10. `[writable]` dst associated token account
        /// 11. `[writable]` dst token account taxman
        /// 12. `[]` system program
        /// 13. `[]` token program
        /// 14. `[]` associated token program
        /// 15. `[]` rent
        BalansolSwap,
        /// Marco Polo Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[]` state
        /// 2. `[writable]` pool
        /// 3. `[]` token x
        /// 4. `[]` token y
        /// 5. `[writable]` pool x account
        /// 6. `[writable]` pool y account
        /// 7. `[writable]` swapper x account
        /// 8. `[writable]` swapper y account
        /// 9. `[signer, writable]` swapper
        /// 10. `[writable]` referrer x account
        /// 11. `[writable]` referrer y account
        /// 12. `[writable]` referrer
        /// 13. `[]` program authority
        /// 14. `[]` system program
        /// 15. `[]` token program
        /// 16. `[]` associated token program
        /// 17. `[]` rent
        MarcoPoloSwap,
        /// Dradex Swap
        ///
        /// Accounts expected by this instruction:
        /// 0. `[]` swap program
        /// 1. `[writable]` pair
        /// 2. `[writable]` market
        /// 3. `[writable]` event queue
        /// 4. `[]` dex user
        /// 5. `[writable]` market user
        /// 6. `[writable]` bids
        /// 7. `[writable]` asks
        /// 8. `[writable]` t0 vault
        /// 9. `[writable]` t1 vault
        /// 10. `[writable]` t0 user
        /// 11. `[writable]` t1 user
        /// 12. `[]` master
        /// 13. `[signer, writable]` signer
        /// 14. `[]` system program
        /// 15. `[]` token program
        /// 16. `[]` logger
        DradexSwap,
    }
    impl JupiterInstruction {
        pub fn discriminator(&self) -> &'static [u8; 8] {
            match self {
                Self::Route { .. } => {
                    &[229u8, 23u8, 203u8, 151u8, 122u8, 227u8, 173u8, 42u8]
                }
                Self::WhirlpoolSwapExactOutput { .. } => {
                    &[39u8, 58u8, 38u8, 128u8, 100u8, 62u8, 191u8, 249u8]
                }
                Self::CreateOpenOrders => {
                    &[229u8, 194u8, 212u8, 172u8, 8u8, 10u8, 134u8, 147u8]
                }
                Self::MercurialSwap => &[2u8, 5u8, 77u8, 173u8, 197u8, 0u8, 7u8, 157u8],
                Self::CykuraSwap => {
                    &[38u8, 241u8, 21u8, 107u8, 120u8, 59u8, 184u8, 249u8]
                }
                Self::SerumSwap => &[88u8, 183u8, 70u8, 249u8, 214u8, 118u8, 82u8, 210u8],
                Self::SaberSwap => &[64u8, 62u8, 98u8, 226u8, 52u8, 74u8, 37u8, 178u8],
                Self::SaberAddDecimals => {
                    &[36u8, 53u8, 231u8, 184u8, 7u8, 181u8, 5u8, 238u8]
                }
                Self::TokenSwap => {
                    &[187u8, 192u8, 118u8, 212u8, 62u8, 109u8, 28u8, 213u8]
                }
                Self::SenchaSwap => &[25u8, 50u8, 7u8, 21u8, 207u8, 248u8, 230u8, 194u8],
                Self::StepSwap => &[155u8, 56u8, 208u8, 198u8, 27u8, 61u8, 149u8, 233u8],
                Self::CropperSwap => {
                    &[230u8, 216u8, 47u8, 182u8, 165u8, 117u8, 210u8, 103u8]
                }
                Self::RaydiumSwap => {
                    &[177u8, 173u8, 42u8, 240u8, 184u8, 4u8, 124u8, 81u8]
                }
                Self::CremaSwap => {
                    &[169u8, 220u8, 41u8, 250u8, 35u8, 190u8, 133u8, 198u8]
                }
                Self::LifinitySwap => {
                    &[23u8, 96u8, 165u8, 33u8, 90u8, 214u8, 96u8, 153u8]
                }
                Self::MarinadeDeposit => {
                    &[62u8, 236u8, 248u8, 28u8, 222u8, 232u8, 182u8, 73u8]
                }
                Self::MarinadeUnstake => {
                    &[41u8, 120u8, 15u8, 0u8, 113u8, 219u8, 42u8, 1u8]
                }
                Self::AldrinSwap => {
                    &[251u8, 232u8, 119u8, 166u8, 225u8, 185u8, 169u8, 161u8]
                }
                Self::AldrinV2Swap => {
                    &[190u8, 166u8, 89u8, 139u8, 33u8, 152u8, 16u8, 10u8]
                }
                Self::WhirlpoolSwap => {
                    &[123u8, 229u8, 184u8, 63u8, 12u8, 0u8, 92u8, 145u8]
                }
                Self::InvariantSwap => {
                    &[187u8, 193u8, 40u8, 121u8, 47u8, 73u8, 144u8, 177u8]
                }
                Self::MeteoraSwap => {
                    &[127u8, 125u8, 226u8, 12u8, 81u8, 24u8, 204u8, 35u8]
                }
                Self::GoosefxSwap => {
                    &[222u8, 136u8, 46u8, 123u8, 189u8, 125u8, 124u8, 122u8]
                }
                Self::DeltafiSwap => {
                    &[132u8, 230u8, 102u8, 120u8, 205u8, 9u8, 237u8, 190u8]
                }
                Self::BalansolSwap => {
                    &[137u8, 109u8, 253u8, 253u8, 70u8, 109u8, 11u8, 100u8]
                }
                Self::MarcoPoloSwap => {
                    &[241u8, 147u8, 94u8, 15u8, 58u8, 108u8, 179u8, 68u8]
                }
                Self::DradexSwap => &[34u8, 146u8, 160u8, 38u8, 51u8, 85u8, 58u8, 151u8],
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
                    [229u8, 23u8, 203u8, 151u8, 122u8, 227u8, 173u8, 42u8] => {
                        RouteDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [39u8, 58u8, 38u8, 128u8, 100u8, 62u8, 191u8, 249u8] => {
                        WhirlpoolSwapExactOutputDeserializer::deserialize(&mut ix_data)?
                            .into()
                    }
                    [229u8, 194u8, 212u8, 172u8, 8u8, 10u8, 134u8, 147u8] => {
                        CreateOpenOrdersDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [2u8, 5u8, 77u8, 173u8, 197u8, 0u8, 7u8, 157u8] => {
                        MercurialSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [38u8, 241u8, 21u8, 107u8, 120u8, 59u8, 184u8, 249u8] => {
                        CykuraSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [88u8, 183u8, 70u8, 249u8, 214u8, 118u8, 82u8, 210u8] => {
                        SerumSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [64u8, 62u8, 98u8, 226u8, 52u8, 74u8, 37u8, 178u8] => {
                        SaberSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [36u8, 53u8, 231u8, 184u8, 7u8, 181u8, 5u8, 238u8] => {
                        SaberAddDecimalsDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [187u8, 192u8, 118u8, 212u8, 62u8, 109u8, 28u8, 213u8] => {
                        TokenSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [25u8, 50u8, 7u8, 21u8, 207u8, 248u8, 230u8, 194u8] => {
                        SenchaSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [155u8, 56u8, 208u8, 198u8, 27u8, 61u8, 149u8, 233u8] => {
                        StepSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [230u8, 216u8, 47u8, 182u8, 165u8, 117u8, 210u8, 103u8] => {
                        CropperSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [177u8, 173u8, 42u8, 240u8, 184u8, 4u8, 124u8, 81u8] => {
                        RaydiumSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [169u8, 220u8, 41u8, 250u8, 35u8, 190u8, 133u8, 198u8] => {
                        CremaSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [23u8, 96u8, 165u8, 33u8, 90u8, 214u8, 96u8, 153u8] => {
                        LifinitySwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [62u8, 236u8, 248u8, 28u8, 222u8, 232u8, 182u8, 73u8] => {
                        MarinadeDepositDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [41u8, 120u8, 15u8, 0u8, 113u8, 219u8, 42u8, 1u8] => {
                        MarinadeUnstakeDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [251u8, 232u8, 119u8, 166u8, 225u8, 185u8, 169u8, 161u8] => {
                        AldrinSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [190u8, 166u8, 89u8, 139u8, 33u8, 152u8, 16u8, 10u8] => {
                        AldrinV2SwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [123u8, 229u8, 184u8, 63u8, 12u8, 0u8, 92u8, 145u8] => {
                        WhirlpoolSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [187u8, 193u8, 40u8, 121u8, 47u8, 73u8, 144u8, 177u8] => {
                        InvariantSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [127u8, 125u8, 226u8, 12u8, 81u8, 24u8, 204u8, 35u8] => {
                        MeteoraSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [222u8, 136u8, 46u8, 123u8, 189u8, 125u8, 124u8, 122u8] => {
                        GoosefxSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [132u8, 230u8, 102u8, 120u8, 205u8, 9u8, 237u8, 190u8] => {
                        DeltafiSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [137u8, 109u8, 253u8, 253u8, 70u8, 109u8, 11u8, 100u8] => {
                        BalansolSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [241u8, 147u8, 94u8, 15u8, 58u8, 108u8, 179u8, 68u8] => {
                        MarcoPoloSwapDeserializer::deserialize(&mut ix_data)?.into()
                    }
                    [34u8, 146u8, 160u8, 38u8, 51u8, 85u8, 58u8, 151u8] => {
                        DradexSwapDeserializer::deserialize(&mut ix_data)?.into()
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
    impl ::borsh::BorshSerialize for JupiterInstruction {
        fn serialize<W: ::borsh::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), ::borsh::io::Error> {
            match self {
                Self::Route {
                    swap_leg,
                    in_amount,
                    quoted_out_amount,
                    slippage_bps,
                    platform_fee_bps,
                } => {
                    ::borsh::BorshSerialize::serialize(swap_leg, writer)?;
                    ::borsh::BorshSerialize::serialize(in_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(quoted_out_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(slippage_bps, writer)?;
                    ::borsh::BorshSerialize::serialize(platform_fee_bps, writer)?;
                }
                Self::WhirlpoolSwapExactOutput {
                    out_amount,
                    in_amount_with_slippage,
                    a_to_b,
                    platform_fee_bps,
                } => {
                    ::borsh::BorshSerialize::serialize(out_amount, writer)?;
                    ::borsh::BorshSerialize::serialize(in_amount_with_slippage, writer)?;
                    ::borsh::BorshSerialize::serialize(a_to_b, writer)?;
                    ::borsh::BorshSerialize::serialize(platform_fee_bps, writer)?;
                }
                Self::CreateOpenOrders => {}
                Self::MercurialSwap => {}
                Self::CykuraSwap => {}
                Self::SerumSwap => {}
                Self::SaberSwap => {}
                Self::SaberAddDecimals => {}
                Self::TokenSwap => {}
                Self::SenchaSwap => {}
                Self::StepSwap => {}
                Self::CropperSwap => {}
                Self::RaydiumSwap => {}
                Self::CremaSwap => {}
                Self::LifinitySwap => {}
                Self::MarinadeDeposit => {}
                Self::MarinadeUnstake => {}
                Self::AldrinSwap => {}
                Self::AldrinV2Swap => {}
                Self::WhirlpoolSwap => {}
                Self::InvariantSwap => {}
                Self::MeteoraSwap => {}
                Self::GoosefxSwap => {}
                Self::DeltafiSwap => {}
                Self::BalansolSwap => {}
                Self::MarcoPoloSwap => {}
                Self::DradexSwap => {}
            }
            Ok(())
        }
    }
    struct RouteDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for RouteDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::Route {
                    swap_leg: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    in_amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    quoted_out_amount: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    slippage_bps: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    platform_fee_bps: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<RouteDeserializer> for JupiterInstruction {
        fn from(helper: RouteDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct WhirlpoolSwapExactOutputDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for WhirlpoolSwapExactOutputDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::WhirlpoolSwapExactOutput {
                    out_amount: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    in_amount_with_slippage: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                    a_to_b: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,
                    platform_fee_bps: ::borsh::BorshDeserialize::deserialize_reader(
                        _reader,
                    )?,
                }),
            )
        }
    }
    impl From<WhirlpoolSwapExactOutputDeserializer> for JupiterInstruction {
        fn from(helper: WhirlpoolSwapExactOutputDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct CreateOpenOrdersDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for CreateOpenOrdersDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::CreateOpenOrders {
                }),
            )
        }
    }
    impl From<CreateOpenOrdersDeserializer> for JupiterInstruction {
        fn from(helper: CreateOpenOrdersDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct MercurialSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for MercurialSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::MercurialSwap {
                }),
            )
        }
    }
    impl From<MercurialSwapDeserializer> for JupiterInstruction {
        fn from(helper: MercurialSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct CykuraSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for CykuraSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::CykuraSwap {}))
        }
    }
    impl From<CykuraSwapDeserializer> for JupiterInstruction {
        fn from(helper: CykuraSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct SerumSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for SerumSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::SerumSwap {}))
        }
    }
    impl From<SerumSwapDeserializer> for JupiterInstruction {
        fn from(helper: SerumSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct SaberSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for SaberSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::SaberSwap {}))
        }
    }
    impl From<SaberSwapDeserializer> for JupiterInstruction {
        fn from(helper: SaberSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct SaberAddDecimalsDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for SaberAddDecimalsDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::SaberAddDecimals {
                }),
            )
        }
    }
    impl From<SaberAddDecimalsDeserializer> for JupiterInstruction {
        fn from(helper: SaberAddDecimalsDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct TokenSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for TokenSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::TokenSwap {}))
        }
    }
    impl From<TokenSwapDeserializer> for JupiterInstruction {
        fn from(helper: TokenSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct SenchaSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for SenchaSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::SenchaSwap {}))
        }
    }
    impl From<SenchaSwapDeserializer> for JupiterInstruction {
        fn from(helper: SenchaSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct StepSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for StepSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::StepSwap {}))
        }
    }
    impl From<StepSwapDeserializer> for JupiterInstruction {
        fn from(helper: StepSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct CropperSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for CropperSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::CropperSwap {}))
        }
    }
    impl From<CropperSwapDeserializer> for JupiterInstruction {
        fn from(helper: CropperSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct RaydiumSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for RaydiumSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::RaydiumSwap {}))
        }
    }
    impl From<RaydiumSwapDeserializer> for JupiterInstruction {
        fn from(helper: RaydiumSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct CremaSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for CremaSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::CremaSwap {}))
        }
    }
    impl From<CremaSwapDeserializer> for JupiterInstruction {
        fn from(helper: CremaSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct LifinitySwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for LifinitySwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::LifinitySwap {
                }),
            )
        }
    }
    impl From<LifinitySwapDeserializer> for JupiterInstruction {
        fn from(helper: LifinitySwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct MarinadeDepositDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for MarinadeDepositDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::MarinadeDeposit {
                }),
            )
        }
    }
    impl From<MarinadeDepositDeserializer> for JupiterInstruction {
        fn from(helper: MarinadeDepositDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct MarinadeUnstakeDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for MarinadeUnstakeDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::MarinadeUnstake {
                }),
            )
        }
    }
    impl From<MarinadeUnstakeDeserializer> for JupiterInstruction {
        fn from(helper: MarinadeUnstakeDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct AldrinSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for AldrinSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::AldrinSwap {}))
        }
    }
    impl From<AldrinSwapDeserializer> for JupiterInstruction {
        fn from(helper: AldrinSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct AldrinV2SwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for AldrinV2SwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::AldrinV2Swap {
                }),
            )
        }
    }
    impl From<AldrinV2SwapDeserializer> for JupiterInstruction {
        fn from(helper: AldrinV2SwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct WhirlpoolSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for WhirlpoolSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::WhirlpoolSwap {
                }),
            )
        }
    }
    impl From<WhirlpoolSwapDeserializer> for JupiterInstruction {
        fn from(helper: WhirlpoolSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct InvariantSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for InvariantSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::InvariantSwap {
                }),
            )
        }
    }
    impl From<InvariantSwapDeserializer> for JupiterInstruction {
        fn from(helper: InvariantSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct MeteoraSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for MeteoraSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::MeteoraSwap {}))
        }
    }
    impl From<MeteoraSwapDeserializer> for JupiterInstruction {
        fn from(helper: MeteoraSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct GoosefxSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for GoosefxSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::GoosefxSwap {}))
        }
    }
    impl From<GoosefxSwapDeserializer> for JupiterInstruction {
        fn from(helper: GoosefxSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct DeltafiSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for DeltafiSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::DeltafiSwap {}))
        }
    }
    impl From<DeltafiSwapDeserializer> for JupiterInstruction {
        fn from(helper: DeltafiSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct BalansolSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for BalansolSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::BalansolSwap {
                }),
            )
        }
    }
    impl From<BalansolSwapDeserializer> for JupiterInstruction {
        fn from(helper: BalansolSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct MarcoPoloSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for MarcoPoloSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(
                Self(JupiterInstruction::MarcoPoloSwap {
                }),
            )
        }
    }
    impl From<MarcoPoloSwapDeserializer> for JupiterInstruction {
        fn from(helper: MarcoPoloSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    struct DradexSwapDeserializer(JupiterInstruction);
    impl ::borsh::de::BorshDeserialize for DradexSwapDeserializer {
        fn deserialize_reader<R: std::io::prelude::Read>(
            _reader: &mut R,
        ) -> std::io::Result<Self> {
            Ok(Self(JupiterInstruction::DradexSwap {}))
        }
    }
    impl From<DradexSwapDeserializer> for JupiterInstruction {
        fn from(helper: DradexSwapDeserializer) -> JupiterInstruction {
            helper.0
        }
    }
    #[derive(Debug)]
    pub struct Route {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub user_transfer_authority: ::solana_program::pubkey::Pubkey,
        pub destination_token_account: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
        pub swap_leg: SwapLeg,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Route {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                token_program,
                user_transfer_authority,
                destination_token_account,
                trailing_accounts,
                swap_leg,
                in_amount,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_transfer_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(destination_token_account,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::Route {
                swap_leg,
                in_amount,
                quoted_out_amount,
                slippage_bps,
                platform_fee_bps,
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
    pub struct RouteAccountIndexes {
        pub token_program: usize,
        pub user_transfer_authority: usize,
        pub destination_token_account: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl RouteAccountIndexes {
        pub const TOKEN_PROGRAM: usize = 0usize;
        pub const USER_TRANSFER_AUTHORITY: usize = 1usize;
        pub const DESTINATION_TOKEN_ACCOUNT: usize = 2usize;
    }
    impl<'a> TryFrom<&'a [u8]> for RouteAccountIndexes {
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
                user_transfer_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_transfer_authority),
                            1usize,
                        ),
                    )?,
                destination_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(destination_token_account),
                            2usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct WhirlpoolSwapExactOutput {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
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
        pub out_amount: u64,
        pub in_amount_with_slippage: AmountWithSlippage,
        pub a_to_b: bool,
        pub platform_fee_bps: u8,
    }
    impl WhirlpoolSwapExactOutput {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
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
                out_amount,
                in_amount_with_slippage,
                a_to_b,
                platform_fee_bps,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
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
            let data = JupiterInstruction::WhirlpoolSwapExactOutput {
                out_amount,
                in_amount_with_slippage,
                a_to_b,
                platform_fee_bps,
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
    pub struct WhirlpoolSwapExactOutputAccountIndexes {
        pub swap_program: usize,
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
    impl WhirlpoolSwapExactOutputAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const TOKEN_AUTHORITY: usize = 2usize;
        pub const WHIRLPOOL: usize = 3usize;
        pub const TOKEN_OWNER_ACCOUNT_A: usize = 4usize;
        pub const TOKEN_VAULT_A: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_B: usize = 6usize;
        pub const TOKEN_VAULT_B: usize = 7usize;
        pub const TICK_ARRAY0: usize = 8usize;
        pub const TICK_ARRAY1: usize = 9usize;
        pub const TICK_ARRAY2: usize = 10usize;
        pub const ORACLE: usize = 11usize;
    }
    impl<'a> TryFrom<&'a [u8]> for WhirlpoolSwapExactOutputAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                token_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_authority),
                            2usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
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
                tick_array0: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array0),
                            8usize,
                        ),
                    )?,
                tick_array1: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array1),
                            9usize,
                        ),
                    )?,
                tick_array2: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array2),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CreateOpenOrders {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub open_orders: ::solana_program::pubkey::Pubkey,
        pub payer: ::solana_program::pubkey::Pubkey,
        pub dex_program: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub market: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CreateOpenOrders {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                open_orders,
                payer,
                dex_program,
                system_program,
                rent,
                market,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(open_orders, false),
                ::solana_program::instruction::AccountMeta::new(payer, true),
                ::solana_program::instruction::AccountMeta::new_readonly(dex_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false), ::solana_program::instruction::AccountMeta::new_readonly(market,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::CreateOpenOrders {
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
    pub struct CreateOpenOrdersAccountIndexes {
        pub open_orders: usize,
        pub payer: usize,
        pub dex_program: usize,
        pub system_program: usize,
        pub rent: usize,
        pub market: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CreateOpenOrdersAccountIndexes {
        pub const OPEN_ORDERS: usize = 0usize;
        pub const PAYER: usize = 1usize;
        pub const DEX_PROGRAM: usize = 2usize;
        pub const SYSTEM_PROGRAM: usize = 3usize;
        pub const RENT: usize = 4usize;
        pub const MARKET: usize = 5usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CreateOpenOrdersAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                open_orders: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(open_orders),
                            0usize,
                        ),
                    )?,
                payer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(payer),
                            1usize,
                        ),
                    )?,
                dex_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(dex_program),
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
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            4usize,
                        ),
                    )?,
                market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(market),
                            5usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct MercurialSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub swap_state: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub pool_authority: ::solana_program::pubkey::Pubkey,
        pub user_transfer_authority: ::solana_program::pubkey::Pubkey,
        pub source_token_account: ::solana_program::pubkey::Pubkey,
        pub destination_token_account: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl MercurialSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                swap_state,
                token_program,
                pool_authority,
                user_transfer_authority,
                source_token_account,
                destination_token_account,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(swap_state,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(pool_authority,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_transfer_authority,
                true),
                ::solana_program::instruction::AccountMeta::new(source_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(destination_token_account,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::MercurialSwap {
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
    pub struct MercurialSwapAccountIndexes {
        pub swap_program: usize,
        pub swap_state: usize,
        pub token_program: usize,
        pub pool_authority: usize,
        pub user_transfer_authority: usize,
        pub source_token_account: usize,
        pub destination_token_account: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl MercurialSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const SWAP_STATE: usize = 1usize;
        pub const TOKEN_PROGRAM: usize = 2usize;
        pub const POOL_AUTHORITY: usize = 3usize;
        pub const USER_TRANSFER_AUTHORITY: usize = 4usize;
        pub const SOURCE_TOKEN_ACCOUNT: usize = 5usize;
        pub const DESTINATION_TOKEN_ACCOUNT: usize = 6usize;
    }
    impl<'a> TryFrom<&'a [u8]> for MercurialSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                swap_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_state),
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
                pool_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_authority),
                            3usize,
                        ),
                    )?,
                user_transfer_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_transfer_authority),
                            4usize,
                        ),
                    )?,
                source_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(source_token_account),
                            5usize,
                        ),
                    )?,
                destination_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(destination_token_account),
                            6usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CykuraSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub signer: ::solana_program::pubkey::Pubkey,
        pub factory_state: ::solana_program::pubkey::Pubkey,
        pub pool_state: ::solana_program::pubkey::Pubkey,
        pub input_token_account: ::solana_program::pubkey::Pubkey,
        pub output_token_account: ::solana_program::pubkey::Pubkey,
        pub input_vault: ::solana_program::pubkey::Pubkey,
        pub output_vault: ::solana_program::pubkey::Pubkey,
        pub last_observation_state: ::solana_program::pubkey::Pubkey,
        pub core_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CykuraSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                signer,
                factory_state,
                pool_state,
                input_token_account,
                output_token_account,
                input_vault,
                output_vault,
                last_observation_state,
                core_program,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(signer,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(factory_state,
                false), ::solana_program::instruction::AccountMeta::new(pool_state,
                false),
                ::solana_program::instruction::AccountMeta::new(input_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(output_token_account,
                false), ::solana_program::instruction::AccountMeta::new(input_vault,
                false), ::solana_program::instruction::AccountMeta::new(output_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(last_observation_state,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(core_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::CykuraSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CykuraSwapAccountIndexes {
        pub swap_program: usize,
        pub signer: usize,
        pub factory_state: usize,
        pub pool_state: usize,
        pub input_token_account: usize,
        pub output_token_account: usize,
        pub input_vault: usize,
        pub output_vault: usize,
        pub last_observation_state: usize,
        pub core_program: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CykuraSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const SIGNER: usize = 1usize;
        pub const FACTORY_STATE: usize = 2usize;
        pub const POOL_STATE: usize = 3usize;
        pub const INPUT_TOKEN_ACCOUNT: usize = 4usize;
        pub const OUTPUT_TOKEN_ACCOUNT: usize = 5usize;
        pub const INPUT_VAULT: usize = 6usize;
        pub const OUTPUT_VAULT: usize = 7usize;
        pub const LAST_OBSERVATION_STATE: usize = 8usize;
        pub const CORE_PROGRAM: usize = 9usize;
        pub const TOKEN_PROGRAM: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CykuraSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                signer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(signer),
                            1usize,
                        ),
                    )?,
                factory_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(factory_state),
                            2usize,
                        ),
                    )?,
                pool_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_state),
                            3usize,
                        ),
                    )?,
                input_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(input_token_account),
                            4usize,
                        ),
                    )?,
                output_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(output_token_account),
                            5usize,
                        ),
                    )?,
                input_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(input_vault),
                            6usize,
                        ),
                    )?,
                output_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(output_vault),
                            7usize,
                        ),
                    )?,
                last_observation_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(last_observation_state),
                            8usize,
                        ),
                    )?,
                core_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(core_program),
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
    pub struct SerumSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub market: ::solana_program::pubkey::Pubkey,
        pub open_orders: ::solana_program::pubkey::Pubkey,
        pub request_queue: ::solana_program::pubkey::Pubkey,
        pub event_queue: ::solana_program::pubkey::Pubkey,
        pub bids: ::solana_program::pubkey::Pubkey,
        pub asks: ::solana_program::pubkey::Pubkey,
        pub coin_vault: ::solana_program::pubkey::Pubkey,
        pub pc_vault: ::solana_program::pubkey::Pubkey,
        pub vault_signer: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub order_payer_token_account: ::solana_program::pubkey::Pubkey,
        pub coin_wallet: ::solana_program::pubkey::Pubkey,
        pub pc_wallet: ::solana_program::pubkey::Pubkey,
        pub dex_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl SerumSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                market,
                open_orders,
                request_queue,
                event_queue,
                bids,
                asks,
                coin_vault,
                pc_vault,
                vault_signer,
                authority,
                order_payer_token_account,
                coin_wallet,
                pc_wallet,
                dex_program,
                token_program,
                rent,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new(market, false),
                ::solana_program::instruction::AccountMeta::new(open_orders, false),
                ::solana_program::instruction::AccountMeta::new(request_queue, false),
                ::solana_program::instruction::AccountMeta::new(event_queue, false),
                ::solana_program::instruction::AccountMeta::new(bids, false),
                ::solana_program::instruction::AccountMeta::new(asks, false),
                ::solana_program::instruction::AccountMeta::new(coin_vault, false),
                ::solana_program::instruction::AccountMeta::new(pc_vault, false),
                ::solana_program::instruction::AccountMeta::new_readonly(vault_signer,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                true),
                ::solana_program::instruction::AccountMeta::new(order_payer_token_account,
                false), ::solana_program::instruction::AccountMeta::new(coin_wallet,
                false), ::solana_program::instruction::AccountMeta::new(pc_wallet,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(dex_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::SerumSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct SerumSwapAccountIndexes {
        pub market: usize,
        pub open_orders: usize,
        pub request_queue: usize,
        pub event_queue: usize,
        pub bids: usize,
        pub asks: usize,
        pub coin_vault: usize,
        pub pc_vault: usize,
        pub vault_signer: usize,
        pub authority: usize,
        pub order_payer_token_account: usize,
        pub coin_wallet: usize,
        pub pc_wallet: usize,
        pub dex_program: usize,
        pub token_program: usize,
        pub rent: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SerumSwapAccountIndexes {
        pub const MARKET: usize = 0usize;
        pub const OPEN_ORDERS: usize = 1usize;
        pub const REQUEST_QUEUE: usize = 2usize;
        pub const EVENT_QUEUE: usize = 3usize;
        pub const BIDS: usize = 4usize;
        pub const ASKS: usize = 5usize;
        pub const COIN_VAULT: usize = 6usize;
        pub const PC_VAULT: usize = 7usize;
        pub const VAULT_SIGNER: usize = 8usize;
        pub const AUTHORITY: usize = 9usize;
        pub const ORDER_PAYER_TOKEN_ACCOUNT: usize = 10usize;
        pub const COIN_WALLET: usize = 11usize;
        pub const PC_WALLET: usize = 12usize;
        pub const DEX_PROGRAM: usize = 13usize;
        pub const TOKEN_PROGRAM: usize = 14usize;
        pub const RENT: usize = 15usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SerumSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(market),
                            0usize,
                        ),
                    )?,
                open_orders: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(open_orders),
                            1usize,
                        ),
                    )?,
                request_queue: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(request_queue),
                            2usize,
                        ),
                    )?,
                event_queue: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(event_queue),
                            3usize,
                        ),
                    )?,
                bids: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(bids),
                            4usize,
                        ),
                    )?,
                asks: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(asks),
                            5usize,
                        ),
                    )?,
                coin_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(coin_vault),
                            6usize,
                        ),
                    )?,
                pc_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pc_vault),
                            7usize,
                        ),
                    )?,
                vault_signer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(vault_signer),
                            8usize,
                        ),
                    )?,
                authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(authority),
                            9usize,
                        ),
                    )?,
                order_payer_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(order_payer_token_account),
                            10usize,
                        ),
                    )?,
                coin_wallet: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(coin_wallet),
                            11usize,
                        ),
                    )?,
                pc_wallet: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pc_wallet),
                            12usize,
                        ),
                    )?,
                dex_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(dex_program),
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
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            15usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SaberSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub swap: ::solana_program::pubkey::Pubkey,
        pub swap_authority: ::solana_program::pubkey::Pubkey,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub input_user_account: ::solana_program::pubkey::Pubkey,
        pub input_token_account: ::solana_program::pubkey::Pubkey,
        pub output_user_account: ::solana_program::pubkey::Pubkey,
        pub output_token_account: ::solana_program::pubkey::Pubkey,
        pub fees_token_account: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl SaberSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                token_program,
                swap,
                swap_authority,
                user_authority,
                input_user_account,
                input_token_account,
                output_user_account,
                output_token_account,
                fees_token_account,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(swap,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(swap_authority,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_authority,
                false),
                ::solana_program::instruction::AccountMeta::new(input_user_account,
                false),
                ::solana_program::instruction::AccountMeta::new(input_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(output_user_account,
                false),
                ::solana_program::instruction::AccountMeta::new(output_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(fees_token_account,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::SaberSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct SaberSwapAccountIndexes {
        pub swap_program: usize,
        pub token_program: usize,
        pub swap: usize,
        pub swap_authority: usize,
        pub user_authority: usize,
        pub input_user_account: usize,
        pub input_token_account: usize,
        pub output_user_account: usize,
        pub output_token_account: usize,
        pub fees_token_account: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SaberSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const SWAP: usize = 2usize;
        pub const SWAP_AUTHORITY: usize = 3usize;
        pub const USER_AUTHORITY: usize = 4usize;
        pub const INPUT_USER_ACCOUNT: usize = 5usize;
        pub const INPUT_TOKEN_ACCOUNT: usize = 6usize;
        pub const OUTPUT_USER_ACCOUNT: usize = 7usize;
        pub const OUTPUT_TOKEN_ACCOUNT: usize = 8usize;
        pub const FEES_TOKEN_ACCOUNT: usize = 9usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SaberSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                swap: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap),
                            2usize,
                        ),
                    )?,
                swap_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_authority),
                            3usize,
                        ),
                    )?,
                user_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_authority),
                            4usize,
                        ),
                    )?,
                input_user_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(input_user_account),
                            5usize,
                        ),
                    )?,
                input_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(input_token_account),
                            6usize,
                        ),
                    )?,
                output_user_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(output_user_account),
                            7usize,
                        ),
                    )?,
                output_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(output_token_account),
                            8usize,
                        ),
                    )?,
                fees_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fees_token_account),
                            9usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SaberAddDecimals {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub add_decimals_program: ::solana_program::pubkey::Pubkey,
        pub wrapper: ::solana_program::pubkey::Pubkey,
        pub wrapper_mint: ::solana_program::pubkey::Pubkey,
        pub wrapper_underlying_tokens: ::solana_program::pubkey::Pubkey,
        pub owner: ::solana_program::pubkey::Pubkey,
        pub user_underlying_tokens: ::solana_program::pubkey::Pubkey,
        pub user_wrapped_tokens: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl SaberAddDecimals {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                add_decimals_program,
                wrapper,
                wrapper_mint,
                wrapper_underlying_tokens,
                owner,
                user_underlying_tokens,
                user_wrapped_tokens,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(add_decimals_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(wrapper,
                false), ::solana_program::instruction::AccountMeta::new(wrapper_mint,
                false),
                ::solana_program::instruction::AccountMeta::new(wrapper_underlying_tokens,
                false), ::solana_program::instruction::AccountMeta::new_readonly(owner,
                true),
                ::solana_program::instruction::AccountMeta::new(user_underlying_tokens,
                false),
                ::solana_program::instruction::AccountMeta::new(user_wrapped_tokens,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::SaberAddDecimals {
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
    pub struct SaberAddDecimalsAccountIndexes {
        pub add_decimals_program: usize,
        pub wrapper: usize,
        pub wrapper_mint: usize,
        pub wrapper_underlying_tokens: usize,
        pub owner: usize,
        pub user_underlying_tokens: usize,
        pub user_wrapped_tokens: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SaberAddDecimalsAccountIndexes {
        pub const ADD_DECIMALS_PROGRAM: usize = 0usize;
        pub const WRAPPER: usize = 1usize;
        pub const WRAPPER_MINT: usize = 2usize;
        pub const WRAPPER_UNDERLYING_TOKENS: usize = 3usize;
        pub const OWNER: usize = 4usize;
        pub const USER_UNDERLYING_TOKENS: usize = 5usize;
        pub const USER_WRAPPED_TOKENS: usize = 6usize;
        pub const TOKEN_PROGRAM: usize = 7usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SaberAddDecimalsAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                add_decimals_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(add_decimals_program),
                            0usize,
                        ),
                    )?,
                wrapper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(wrapper),
                            1usize,
                        ),
                    )?,
                wrapper_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(wrapper_mint),
                            2usize,
                        ),
                    )?,
                wrapper_underlying_tokens: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(wrapper_underlying_tokens),
                            3usize,
                        ),
                    )?,
                owner: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(owner),
                            4usize,
                        ),
                    )?,
                user_underlying_tokens: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_underlying_tokens),
                            5usize,
                        ),
                    )?,
                user_wrapped_tokens: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_wrapped_tokens),
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
    pub struct TokenSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub token_swap_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub swap: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub user_transfer_authority: ::solana_program::pubkey::Pubkey,
        pub source: ::solana_program::pubkey::Pubkey,
        pub swap_source: ::solana_program::pubkey::Pubkey,
        pub swap_destination: ::solana_program::pubkey::Pubkey,
        pub destination: ::solana_program::pubkey::Pubkey,
        pub pool_mint: ::solana_program::pubkey::Pubkey,
        pub pool_fee: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl TokenSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                token_swap_program,
                token_program,
                swap,
                authority,
                user_transfer_authority,
                source,
                swap_source,
                swap_destination,
                destination,
                pool_mint,
                pool_fee,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(token_swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(swap,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_transfer_authority,
                true), ::solana_program::instruction::AccountMeta::new(source, false),
                ::solana_program::instruction::AccountMeta::new(swap_source, false),
                ::solana_program::instruction::AccountMeta::new(swap_destination, false),
                ::solana_program::instruction::AccountMeta::new(destination, false),
                ::solana_program::instruction::AccountMeta::new(pool_mint, false),
                ::solana_program::instruction::AccountMeta::new(pool_fee, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::TokenSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct TokenSwapAccountIndexes {
        pub token_swap_program: usize,
        pub token_program: usize,
        pub swap: usize,
        pub authority: usize,
        pub user_transfer_authority: usize,
        pub source: usize,
        pub swap_source: usize,
        pub swap_destination: usize,
        pub destination: usize,
        pub pool_mint: usize,
        pub pool_fee: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl TokenSwapAccountIndexes {
        pub const TOKEN_SWAP_PROGRAM: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const SWAP: usize = 2usize;
        pub const AUTHORITY: usize = 3usize;
        pub const USER_TRANSFER_AUTHORITY: usize = 4usize;
        pub const SOURCE: usize = 5usize;
        pub const SWAP_SOURCE: usize = 6usize;
        pub const SWAP_DESTINATION: usize = 7usize;
        pub const DESTINATION: usize = 8usize;
        pub const POOL_MINT: usize = 9usize;
        pub const POOL_FEE: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for TokenSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                token_swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_swap_program),
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
                swap: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap),
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
                user_transfer_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_transfer_authority),
                            4usize,
                        ),
                    )?,
                source: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(source),
                            5usize,
                        ),
                    )?,
                swap_source: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_source),
                            6usize,
                        ),
                    )?,
                swap_destination: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_destination),
                            7usize,
                        ),
                    )?,
                destination: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(destination),
                            8usize,
                        ),
                    )?,
                pool_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_mint),
                            9usize,
                        ),
                    )?,
                pool_fee: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_fee),
                            10usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct SenchaSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub swap: ::solana_program::pubkey::Pubkey,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub input_user_account: ::solana_program::pubkey::Pubkey,
        pub input_token_account: ::solana_program::pubkey::Pubkey,
        pub input_fees_account: ::solana_program::pubkey::Pubkey,
        pub output_user_account: ::solana_program::pubkey::Pubkey,
        pub output_token_account: ::solana_program::pubkey::Pubkey,
        pub output_fees_account: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl SenchaSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                token_program,
                swap,
                user_authority,
                input_user_account,
                input_token_account,
                input_fees_account,
                output_user_account,
                output_token_account,
                output_fees_account,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new(swap, false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_authority,
                false),
                ::solana_program::instruction::AccountMeta::new(input_user_account,
                false),
                ::solana_program::instruction::AccountMeta::new(input_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(input_fees_account,
                false),
                ::solana_program::instruction::AccountMeta::new(output_user_account,
                false),
                ::solana_program::instruction::AccountMeta::new(output_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(output_fees_account,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::SenchaSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct SenchaSwapAccountIndexes {
        pub swap_program: usize,
        pub token_program: usize,
        pub swap: usize,
        pub user_authority: usize,
        pub input_user_account: usize,
        pub input_token_account: usize,
        pub input_fees_account: usize,
        pub output_user_account: usize,
        pub output_token_account: usize,
        pub output_fees_account: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl SenchaSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const SWAP: usize = 2usize;
        pub const USER_AUTHORITY: usize = 3usize;
        pub const INPUT_USER_ACCOUNT: usize = 4usize;
        pub const INPUT_TOKEN_ACCOUNT: usize = 5usize;
        pub const INPUT_FEES_ACCOUNT: usize = 6usize;
        pub const OUTPUT_USER_ACCOUNT: usize = 7usize;
        pub const OUTPUT_TOKEN_ACCOUNT: usize = 8usize;
        pub const OUTPUT_FEES_ACCOUNT: usize = 9usize;
    }
    impl<'a> TryFrom<&'a [u8]> for SenchaSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                swap: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap),
                            2usize,
                        ),
                    )?,
                user_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_authority),
                            3usize,
                        ),
                    )?,
                input_user_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(input_user_account),
                            4usize,
                        ),
                    )?,
                input_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(input_token_account),
                            5usize,
                        ),
                    )?,
                input_fees_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(input_fees_account),
                            6usize,
                        ),
                    )?,
                output_user_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(output_user_account),
                            7usize,
                        ),
                    )?,
                output_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(output_token_account),
                            8usize,
                        ),
                    )?,
                output_fees_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(output_fees_account),
                            9usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct StepSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub token_swap_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub swap: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub user_transfer_authority: ::solana_program::pubkey::Pubkey,
        pub source: ::solana_program::pubkey::Pubkey,
        pub swap_source: ::solana_program::pubkey::Pubkey,
        pub swap_destination: ::solana_program::pubkey::Pubkey,
        pub destination: ::solana_program::pubkey::Pubkey,
        pub pool_mint: ::solana_program::pubkey::Pubkey,
        pub pool_fee: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl StepSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                token_swap_program,
                token_program,
                swap,
                authority,
                user_transfer_authority,
                source,
                swap_source,
                swap_destination,
                destination,
                pool_mint,
                pool_fee,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(token_swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(swap,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_transfer_authority,
                true), ::solana_program::instruction::AccountMeta::new(source, false),
                ::solana_program::instruction::AccountMeta::new(swap_source, false),
                ::solana_program::instruction::AccountMeta::new(swap_destination, false),
                ::solana_program::instruction::AccountMeta::new(destination, false),
                ::solana_program::instruction::AccountMeta::new(pool_mint, false),
                ::solana_program::instruction::AccountMeta::new(pool_fee, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::StepSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct StepSwapAccountIndexes {
        pub token_swap_program: usize,
        pub token_program: usize,
        pub swap: usize,
        pub authority: usize,
        pub user_transfer_authority: usize,
        pub source: usize,
        pub swap_source: usize,
        pub swap_destination: usize,
        pub destination: usize,
        pub pool_mint: usize,
        pub pool_fee: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl StepSwapAccountIndexes {
        pub const TOKEN_SWAP_PROGRAM: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const SWAP: usize = 2usize;
        pub const AUTHORITY: usize = 3usize;
        pub const USER_TRANSFER_AUTHORITY: usize = 4usize;
        pub const SOURCE: usize = 5usize;
        pub const SWAP_SOURCE: usize = 6usize;
        pub const SWAP_DESTINATION: usize = 7usize;
        pub const DESTINATION: usize = 8usize;
        pub const POOL_MINT: usize = 9usize;
        pub const POOL_FEE: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for StepSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                token_swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_swap_program),
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
                swap: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap),
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
                user_transfer_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_transfer_authority),
                            4usize,
                        ),
                    )?,
                source: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(source),
                            5usize,
                        ),
                    )?,
                swap_source: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_source),
                            6usize,
                        ),
                    )?,
                swap_destination: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_destination),
                            7usize,
                        ),
                    )?,
                destination: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(destination),
                            8usize,
                        ),
                    )?,
                pool_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_mint),
                            9usize,
                        ),
                    )?,
                pool_fee: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_fee),
                            10usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CropperSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub token_swap_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub swap: ::solana_program::pubkey::Pubkey,
        pub swap_state: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub user_transfer_authority: ::solana_program::pubkey::Pubkey,
        pub source: ::solana_program::pubkey::Pubkey,
        pub swap_source: ::solana_program::pubkey::Pubkey,
        pub swap_destination: ::solana_program::pubkey::Pubkey,
        pub destination: ::solana_program::pubkey::Pubkey,
        pub pool_mint: ::solana_program::pubkey::Pubkey,
        pub pool_fee: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CropperSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                token_swap_program,
                token_program,
                swap,
                swap_state,
                authority,
                user_transfer_authority,
                source,
                swap_source,
                swap_destination,
                destination,
                pool_mint,
                pool_fee,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(token_swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(swap,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(swap_state,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_transfer_authority,
                true), ::solana_program::instruction::AccountMeta::new(source, false),
                ::solana_program::instruction::AccountMeta::new(swap_source, false),
                ::solana_program::instruction::AccountMeta::new(swap_destination, false),
                ::solana_program::instruction::AccountMeta::new(destination, false),
                ::solana_program::instruction::AccountMeta::new(pool_mint, false),
                ::solana_program::instruction::AccountMeta::new(pool_fee, false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::CropperSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CropperSwapAccountIndexes {
        pub token_swap_program: usize,
        pub token_program: usize,
        pub swap: usize,
        pub swap_state: usize,
        pub authority: usize,
        pub user_transfer_authority: usize,
        pub source: usize,
        pub swap_source: usize,
        pub swap_destination: usize,
        pub destination: usize,
        pub pool_mint: usize,
        pub pool_fee: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CropperSwapAccountIndexes {
        pub const TOKEN_SWAP_PROGRAM: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const SWAP: usize = 2usize;
        pub const SWAP_STATE: usize = 3usize;
        pub const AUTHORITY: usize = 4usize;
        pub const USER_TRANSFER_AUTHORITY: usize = 5usize;
        pub const SOURCE: usize = 6usize;
        pub const SWAP_SOURCE: usize = 7usize;
        pub const SWAP_DESTINATION: usize = 8usize;
        pub const DESTINATION: usize = 9usize;
        pub const POOL_MINT: usize = 10usize;
        pub const POOL_FEE: usize = 11usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CropperSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                token_swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_swap_program),
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
                swap: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap),
                            2usize,
                        ),
                    )?,
                swap_state: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_state),
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
                user_transfer_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_transfer_authority),
                            5usize,
                        ),
                    )?,
                source: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(source),
                            6usize,
                        ),
                    )?,
                swap_source: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_source),
                            7usize,
                        ),
                    )?,
                swap_destination: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_destination),
                            8usize,
                        ),
                    )?,
                destination: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(destination),
                            9usize,
                        ),
                    )?,
                pool_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_mint),
                            10usize,
                        ),
                    )?,
                pool_fee: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_fee),
                            11usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct RaydiumSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub amm_id: ::solana_program::pubkey::Pubkey,
        pub amm_authority: ::solana_program::pubkey::Pubkey,
        pub amm_open_orders: ::solana_program::pubkey::Pubkey,
        pub pool_coin_token_account: ::solana_program::pubkey::Pubkey,
        pub pool_pc_token_account: ::solana_program::pubkey::Pubkey,
        pub serum_program_id: ::solana_program::pubkey::Pubkey,
        pub serum_market: ::solana_program::pubkey::Pubkey,
        pub serum_bids: ::solana_program::pubkey::Pubkey,
        pub serum_asks: ::solana_program::pubkey::Pubkey,
        pub serum_event_queue: ::solana_program::pubkey::Pubkey,
        pub serum_coin_vault_account: ::solana_program::pubkey::Pubkey,
        pub serum_pc_vault_account: ::solana_program::pubkey::Pubkey,
        pub serum_vault_signer: ::solana_program::pubkey::Pubkey,
        pub user_source_token_account: ::solana_program::pubkey::Pubkey,
        pub user_destination_token_account: ::solana_program::pubkey::Pubkey,
        pub user_source_owner: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl RaydiumSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                token_program,
                amm_id,
                amm_authority,
                amm_open_orders,
                pool_coin_token_account,
                pool_pc_token_account,
                serum_program_id,
                serum_market,
                serum_bids,
                serum_asks,
                serum_event_queue,
                serum_coin_vault_account,
                serum_pc_vault_account,
                serum_vault_signer,
                user_source_token_account,
                user_destination_token_account,
                user_source_owner,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new(amm_id, false),
                ::solana_program::instruction::AccountMeta::new_readonly(amm_authority,
                false), ::solana_program::instruction::AccountMeta::new(amm_open_orders,
                false),
                ::solana_program::instruction::AccountMeta::new(pool_coin_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(pool_pc_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(serum_program_id,
                false), ::solana_program::instruction::AccountMeta::new(serum_market,
                false), ::solana_program::instruction::AccountMeta::new(serum_bids,
                false), ::solana_program::instruction::AccountMeta::new(serum_asks,
                false),
                ::solana_program::instruction::AccountMeta::new(serum_event_queue,
                false),
                ::solana_program::instruction::AccountMeta::new(serum_coin_vault_account,
                false),
                ::solana_program::instruction::AccountMeta::new(serum_pc_vault_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(serum_vault_signer,
                false),
                ::solana_program::instruction::AccountMeta::new(user_source_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(user_destination_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_source_owner,
                true),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::RaydiumSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct RaydiumSwapAccountIndexes {
        pub swap_program: usize,
        pub token_program: usize,
        pub amm_id: usize,
        pub amm_authority: usize,
        pub amm_open_orders: usize,
        pub pool_coin_token_account: usize,
        pub pool_pc_token_account: usize,
        pub serum_program_id: usize,
        pub serum_market: usize,
        pub serum_bids: usize,
        pub serum_asks: usize,
        pub serum_event_queue: usize,
        pub serum_coin_vault_account: usize,
        pub serum_pc_vault_account: usize,
        pub serum_vault_signer: usize,
        pub user_source_token_account: usize,
        pub user_destination_token_account: usize,
        pub user_source_owner: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl RaydiumSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const AMM_ID: usize = 2usize;
        pub const AMM_AUTHORITY: usize = 3usize;
        pub const AMM_OPEN_ORDERS: usize = 4usize;
        pub const POOL_COIN_TOKEN_ACCOUNT: usize = 5usize;
        pub const POOL_PC_TOKEN_ACCOUNT: usize = 6usize;
        pub const SERUM_PROGRAM_ID: usize = 7usize;
        pub const SERUM_MARKET: usize = 8usize;
        pub const SERUM_BIDS: usize = 9usize;
        pub const SERUM_ASKS: usize = 10usize;
        pub const SERUM_EVENT_QUEUE: usize = 11usize;
        pub const SERUM_COIN_VAULT_ACCOUNT: usize = 12usize;
        pub const SERUM_PC_VAULT_ACCOUNT: usize = 13usize;
        pub const SERUM_VAULT_SIGNER: usize = 14usize;
        pub const USER_SOURCE_TOKEN_ACCOUNT: usize = 15usize;
        pub const USER_DESTINATION_TOKEN_ACCOUNT: usize = 16usize;
        pub const USER_SOURCE_OWNER: usize = 17usize;
    }
    impl<'a> TryFrom<&'a [u8]> for RaydiumSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                amm_id: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(amm_id),
                            2usize,
                        ),
                    )?,
                amm_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(amm_authority),
                            3usize,
                        ),
                    )?,
                amm_open_orders: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(amm_open_orders),
                            4usize,
                        ),
                    )?,
                pool_coin_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_coin_token_account),
                            5usize,
                        ),
                    )?,
                pool_pc_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_pc_token_account),
                            6usize,
                        ),
                    )?,
                serum_program_id: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(serum_program_id),
                            7usize,
                        ),
                    )?,
                serum_market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(serum_market),
                            8usize,
                        ),
                    )?,
                serum_bids: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(serum_bids),
                            9usize,
                        ),
                    )?,
                serum_asks: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(serum_asks),
                            10usize,
                        ),
                    )?,
                serum_event_queue: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(serum_event_queue),
                            11usize,
                        ),
                    )?,
                serum_coin_vault_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(serum_coin_vault_account),
                            12usize,
                        ),
                    )?,
                serum_pc_vault_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(serum_pc_vault_account),
                            13usize,
                        ),
                    )?,
                serum_vault_signer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(serum_vault_signer),
                            14usize,
                        ),
                    )?,
                user_source_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_source_token_account),
                            15usize,
                        ),
                    )?,
                user_destination_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_destination_token_account),
                            16usize,
                        ),
                    )?,
                user_source_owner: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_source_owner),
                            17usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct CremaSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub pool: ::solana_program::pubkey::Pubkey,
        pub pool_signer: ::solana_program::pubkey::Pubkey,
        pub user_source_token_account: ::solana_program::pubkey::Pubkey,
        pub user_destination_token_account: ::solana_program::pubkey::Pubkey,
        pub pool_source_token_account: ::solana_program::pubkey::Pubkey,
        pub pool_destination_token_account: ::solana_program::pubkey::Pubkey,
        pub pool_ticks_account: ::solana_program::pubkey::Pubkey,
        pub wallet_authority: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl CremaSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                pool,
                pool_signer,
                user_source_token_account,
                user_destination_token_account,
                pool_source_token_account,
                pool_destination_token_account,
                pool_ticks_account,
                wallet_authority,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new(pool, false),
                ::solana_program::instruction::AccountMeta::new_readonly(pool_signer,
                false),
                ::solana_program::instruction::AccountMeta::new(user_source_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(user_destination_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(pool_source_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(pool_destination_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(pool_ticks_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(wallet_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::CremaSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct CremaSwapAccountIndexes {
        pub swap_program: usize,
        pub pool: usize,
        pub pool_signer: usize,
        pub user_source_token_account: usize,
        pub user_destination_token_account: usize,
        pub pool_source_token_account: usize,
        pub pool_destination_token_account: usize,
        pub pool_ticks_account: usize,
        pub wallet_authority: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl CremaSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const POOL: usize = 1usize;
        pub const POOL_SIGNER: usize = 2usize;
        pub const USER_SOURCE_TOKEN_ACCOUNT: usize = 3usize;
        pub const USER_DESTINATION_TOKEN_ACCOUNT: usize = 4usize;
        pub const POOL_SOURCE_TOKEN_ACCOUNT: usize = 5usize;
        pub const POOL_DESTINATION_TOKEN_ACCOUNT: usize = 6usize;
        pub const POOL_TICKS_ACCOUNT: usize = 7usize;
        pub const WALLET_AUTHORITY: usize = 8usize;
        pub const TOKEN_PROGRAM: usize = 9usize;
    }
    impl<'a> TryFrom<&'a [u8]> for CremaSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                pool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool),
                            1usize,
                        ),
                    )?,
                pool_signer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_signer),
                            2usize,
                        ),
                    )?,
                user_source_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_source_token_account),
                            3usize,
                        ),
                    )?,
                user_destination_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_destination_token_account),
                            4usize,
                        ),
                    )?,
                pool_source_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_source_token_account),
                            5usize,
                        ),
                    )?,
                pool_destination_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_destination_token_account),
                            6usize,
                        ),
                    )?,
                pool_ticks_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_ticks_account),
                            7usize,
                        ),
                    )?,
                wallet_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(wallet_authority),
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
    pub struct LifinitySwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub amm: ::solana_program::pubkey::Pubkey,
        pub user_transfer_authority: ::solana_program::pubkey::Pubkey,
        pub source_info: ::solana_program::pubkey::Pubkey,
        pub destination_info: ::solana_program::pubkey::Pubkey,
        pub swap_source: ::solana_program::pubkey::Pubkey,
        pub swap_destination: ::solana_program::pubkey::Pubkey,
        pub pool_mint: ::solana_program::pubkey::Pubkey,
        pub fee_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub pyth_account: ::solana_program::pubkey::Pubkey,
        pub pyth_pc_account: ::solana_program::pubkey::Pubkey,
        pub config_account: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl LifinitySwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                authority,
                amm,
                user_transfer_authority,
                source_info,
                destination_info,
                swap_source,
                swap_destination,
                pool_mint,
                fee_account,
                token_program,
                pyth_account,
                pyth_pc_account,
                config_account,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(authority,
                false), ::solana_program::instruction::AccountMeta::new_readonly(amm,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_transfer_authority,
                true), ::solana_program::instruction::AccountMeta::new(source_info,
                false), ::solana_program::instruction::AccountMeta::new(destination_info,
                false), ::solana_program::instruction::AccountMeta::new(swap_source,
                false), ::solana_program::instruction::AccountMeta::new(swap_destination,
                false), ::solana_program::instruction::AccountMeta::new(pool_mint,
                false), ::solana_program::instruction::AccountMeta::new(fee_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(pyth_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(pyth_pc_account,
                false), ::solana_program::instruction::AccountMeta::new(config_account,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::LifinitySwap {
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
    pub struct LifinitySwapAccountIndexes {
        pub swap_program: usize,
        pub authority: usize,
        pub amm: usize,
        pub user_transfer_authority: usize,
        pub source_info: usize,
        pub destination_info: usize,
        pub swap_source: usize,
        pub swap_destination: usize,
        pub pool_mint: usize,
        pub fee_account: usize,
        pub token_program: usize,
        pub pyth_account: usize,
        pub pyth_pc_account: usize,
        pub config_account: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl LifinitySwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const AMM: usize = 2usize;
        pub const USER_TRANSFER_AUTHORITY: usize = 3usize;
        pub const SOURCE_INFO: usize = 4usize;
        pub const DESTINATION_INFO: usize = 5usize;
        pub const SWAP_SOURCE: usize = 6usize;
        pub const SWAP_DESTINATION: usize = 7usize;
        pub const POOL_MINT: usize = 8usize;
        pub const FEE_ACCOUNT: usize = 9usize;
        pub const TOKEN_PROGRAM: usize = 10usize;
        pub const PYTH_ACCOUNT: usize = 11usize;
        pub const PYTH_PC_ACCOUNT: usize = 12usize;
        pub const CONFIG_ACCOUNT: usize = 13usize;
    }
    impl<'a> TryFrom<&'a [u8]> for LifinitySwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                amm: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(amm),
                            2usize,
                        ),
                    )?,
                user_transfer_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_transfer_authority),
                            3usize,
                        ),
                    )?,
                source_info: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(source_info),
                            4usize,
                        ),
                    )?,
                destination_info: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(destination_info),
                            5usize,
                        ),
                    )?,
                swap_source: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_source),
                            6usize,
                        ),
                    )?,
                swap_destination: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_destination),
                            7usize,
                        ),
                    )?,
                pool_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_mint),
                            8usize,
                        ),
                    )?,
                fee_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_account),
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
                pyth_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pyth_account),
                            11usize,
                        ),
                    )?,
                pyth_pc_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pyth_pc_account),
                            12usize,
                        ),
                    )?,
                config_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(config_account),
                            13usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct MarinadeDeposit {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub marinade_finance_program: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub msol_mint: ::solana_program::pubkey::Pubkey,
        pub liq_pool_sol_leg_pda: ::solana_program::pubkey::Pubkey,
        pub liq_pool_msol_leg: ::solana_program::pubkey::Pubkey,
        pub liq_pool_msol_leg_authority: ::solana_program::pubkey::Pubkey,
        pub reserve_pda: ::solana_program::pubkey::Pubkey,
        pub transfer_from: ::solana_program::pubkey::Pubkey,
        pub mint_to: ::solana_program::pubkey::Pubkey,
        pub msol_mint_authority: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub user_wsol_token_account: ::solana_program::pubkey::Pubkey,
        pub temp_wsol_token_account: ::solana_program::pubkey::Pubkey,
        pub user_transfer_authority: ::solana_program::pubkey::Pubkey,
        pub wsol_mint: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl MarinadeDeposit {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                marinade_finance_program,
                state,
                msol_mint,
                liq_pool_sol_leg_pda,
                liq_pool_msol_leg,
                liq_pool_msol_leg_authority,
                reserve_pda,
                transfer_from,
                mint_to,
                msol_mint_authority,
                system_program,
                token_program,
                user_wsol_token_account,
                temp_wsol_token_account,
                user_transfer_authority,
                wsol_mint,
                rent,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(marinade_finance_program,
                false), ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(msol_mint, false),
                ::solana_program::instruction::AccountMeta::new(liq_pool_sol_leg_pda,
                false),
                ::solana_program::instruction::AccountMeta::new(liq_pool_msol_leg,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(liq_pool_msol_leg_authority,
                false), ::solana_program::instruction::AccountMeta::new(reserve_pda,
                false), ::solana_program::instruction::AccountMeta::new(transfer_from,
                false), ::solana_program::instruction::AccountMeta::new(mint_to, false),
                ::solana_program::instruction::AccountMeta::new_readonly(msol_mint_authority,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new(user_wsol_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(temp_wsol_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(user_transfer_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(wsol_mint,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::MarinadeDeposit {
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
    pub struct MarinadeDepositAccountIndexes {
        pub marinade_finance_program: usize,
        pub state: usize,
        pub msol_mint: usize,
        pub liq_pool_sol_leg_pda: usize,
        pub liq_pool_msol_leg: usize,
        pub liq_pool_msol_leg_authority: usize,
        pub reserve_pda: usize,
        pub transfer_from: usize,
        pub mint_to: usize,
        pub msol_mint_authority: usize,
        pub system_program: usize,
        pub token_program: usize,
        pub user_wsol_token_account: usize,
        pub temp_wsol_token_account: usize,
        pub user_transfer_authority: usize,
        pub wsol_mint: usize,
        pub rent: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl MarinadeDepositAccountIndexes {
        pub const MARINADE_FINANCE_PROGRAM: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const MSOL_MINT: usize = 2usize;
        pub const LIQ_POOL_SOL_LEG_PDA: usize = 3usize;
        pub const LIQ_POOL_MSOL_LEG: usize = 4usize;
        pub const LIQ_POOL_MSOL_LEG_AUTHORITY: usize = 5usize;
        pub const RESERVE_PDA: usize = 6usize;
        pub const TRANSFER_FROM: usize = 7usize;
        pub const MINT_TO: usize = 8usize;
        pub const MSOL_MINT_AUTHORITY: usize = 9usize;
        pub const SYSTEM_PROGRAM: usize = 10usize;
        pub const TOKEN_PROGRAM: usize = 11usize;
        pub const USER_WSOL_TOKEN_ACCOUNT: usize = 12usize;
        pub const TEMP_WSOL_TOKEN_ACCOUNT: usize = 13usize;
        pub const USER_TRANSFER_AUTHORITY: usize = 14usize;
        pub const WSOL_MINT: usize = 15usize;
        pub const RENT: usize = 16usize;
    }
    impl<'a> TryFrom<&'a [u8]> for MarinadeDepositAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                marinade_finance_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(marinade_finance_program),
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
                msol_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(msol_mint),
                            2usize,
                        ),
                    )?,
                liq_pool_sol_leg_pda: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(liq_pool_sol_leg_pda),
                            3usize,
                        ),
                    )?,
                liq_pool_msol_leg: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(liq_pool_msol_leg),
                            4usize,
                        ),
                    )?,
                liq_pool_msol_leg_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(liq_pool_msol_leg_authority),
                            5usize,
                        ),
                    )?,
                reserve_pda: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reserve_pda),
                            6usize,
                        ),
                    )?,
                transfer_from: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(transfer_from),
                            7usize,
                        ),
                    )?,
                mint_to: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(mint_to),
                            8usize,
                        ),
                    )?,
                msol_mint_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(msol_mint_authority),
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
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            11usize,
                        ),
                    )?,
                user_wsol_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_wsol_token_account),
                            12usize,
                        ),
                    )?,
                temp_wsol_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(temp_wsol_token_account),
                            13usize,
                        ),
                    )?,
                user_transfer_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_transfer_authority),
                            14usize,
                        ),
                    )?,
                wsol_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(wsol_mint),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct MarinadeUnstake {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub marinade_finance_program: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub msol_mint: ::solana_program::pubkey::Pubkey,
        pub liq_pool_sol_leg_pda: ::solana_program::pubkey::Pubkey,
        pub liq_pool_msol_leg: ::solana_program::pubkey::Pubkey,
        pub treasury_msol_account: ::solana_program::pubkey::Pubkey,
        pub get_msol_from: ::solana_program::pubkey::Pubkey,
        pub get_msol_from_authority: ::solana_program::pubkey::Pubkey,
        pub transfer_sol_to: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub user_wsol_token_account: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl MarinadeUnstake {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                marinade_finance_program,
                state,
                msol_mint,
                liq_pool_sol_leg_pda,
                liq_pool_msol_leg,
                treasury_msol_account,
                get_msol_from,
                get_msol_from_authority,
                transfer_sol_to,
                system_program,
                token_program,
                user_wsol_token_account,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(marinade_finance_program,
                false), ::solana_program::instruction::AccountMeta::new(state, false),
                ::solana_program::instruction::AccountMeta::new(msol_mint, false),
                ::solana_program::instruction::AccountMeta::new(liq_pool_sol_leg_pda,
                false),
                ::solana_program::instruction::AccountMeta::new(liq_pool_msol_leg,
                false),
                ::solana_program::instruction::AccountMeta::new(treasury_msol_account,
                false), ::solana_program::instruction::AccountMeta::new(get_msol_from,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(get_msol_from_authority,
                true), ::solana_program::instruction::AccountMeta::new(transfer_sol_to,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new(user_wsol_token_account,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::MarinadeUnstake {
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
    pub struct MarinadeUnstakeAccountIndexes {
        pub marinade_finance_program: usize,
        pub state: usize,
        pub msol_mint: usize,
        pub liq_pool_sol_leg_pda: usize,
        pub liq_pool_msol_leg: usize,
        pub treasury_msol_account: usize,
        pub get_msol_from: usize,
        pub get_msol_from_authority: usize,
        pub transfer_sol_to: usize,
        pub system_program: usize,
        pub token_program: usize,
        pub user_wsol_token_account: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl MarinadeUnstakeAccountIndexes {
        pub const MARINADE_FINANCE_PROGRAM: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const MSOL_MINT: usize = 2usize;
        pub const LIQ_POOL_SOL_LEG_PDA: usize = 3usize;
        pub const LIQ_POOL_MSOL_LEG: usize = 4usize;
        pub const TREASURY_MSOL_ACCOUNT: usize = 5usize;
        pub const GET_MSOL_FROM: usize = 6usize;
        pub const GET_MSOL_FROM_AUTHORITY: usize = 7usize;
        pub const TRANSFER_SOL_TO: usize = 8usize;
        pub const SYSTEM_PROGRAM: usize = 9usize;
        pub const TOKEN_PROGRAM: usize = 10usize;
        pub const USER_WSOL_TOKEN_ACCOUNT: usize = 11usize;
    }
    impl<'a> TryFrom<&'a [u8]> for MarinadeUnstakeAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                marinade_finance_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(marinade_finance_program),
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
                msol_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(msol_mint),
                            2usize,
                        ),
                    )?,
                liq_pool_sol_leg_pda: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(liq_pool_sol_leg_pda),
                            3usize,
                        ),
                    )?,
                liq_pool_msol_leg: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(liq_pool_msol_leg),
                            4usize,
                        ),
                    )?,
                treasury_msol_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(treasury_msol_account),
                            5usize,
                        ),
                    )?,
                get_msol_from: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(get_msol_from),
                            6usize,
                        ),
                    )?,
                get_msol_from_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(get_msol_from_authority),
                            7usize,
                        ),
                    )?,
                transfer_sol_to: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(transfer_sol_to),
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
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            10usize,
                        ),
                    )?,
                user_wsol_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_wsol_token_account),
                            11usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct AldrinSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub pool: ::solana_program::pubkey::Pubkey,
        pub pool_signer: ::solana_program::pubkey::Pubkey,
        pub pool_mint: ::solana_program::pubkey::Pubkey,
        pub base_token_vault: ::solana_program::pubkey::Pubkey,
        pub quote_token_vault: ::solana_program::pubkey::Pubkey,
        pub fee_pool_token_account: ::solana_program::pubkey::Pubkey,
        pub wallet_authority: ::solana_program::pubkey::Pubkey,
        pub user_base_token_account: ::solana_program::pubkey::Pubkey,
        pub user_quote_token_account: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl AldrinSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                pool,
                pool_signer,
                pool_mint,
                base_token_vault,
                quote_token_vault,
                fee_pool_token_account,
                wallet_authority,
                user_base_token_account,
                user_quote_token_account,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(pool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(pool_signer,
                false), ::solana_program::instruction::AccountMeta::new(pool_mint,
                false), ::solana_program::instruction::AccountMeta::new(base_token_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(quote_token_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(fee_pool_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(wallet_authority,
                true),
                ::solana_program::instruction::AccountMeta::new(user_base_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(user_quote_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::AldrinSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct AldrinSwapAccountIndexes {
        pub swap_program: usize,
        pub pool: usize,
        pub pool_signer: usize,
        pub pool_mint: usize,
        pub base_token_vault: usize,
        pub quote_token_vault: usize,
        pub fee_pool_token_account: usize,
        pub wallet_authority: usize,
        pub user_base_token_account: usize,
        pub user_quote_token_account: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl AldrinSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const POOL: usize = 1usize;
        pub const POOL_SIGNER: usize = 2usize;
        pub const POOL_MINT: usize = 3usize;
        pub const BASE_TOKEN_VAULT: usize = 4usize;
        pub const QUOTE_TOKEN_VAULT: usize = 5usize;
        pub const FEE_POOL_TOKEN_ACCOUNT: usize = 6usize;
        pub const WALLET_AUTHORITY: usize = 7usize;
        pub const USER_BASE_TOKEN_ACCOUNT: usize = 8usize;
        pub const USER_QUOTE_TOKEN_ACCOUNT: usize = 9usize;
        pub const TOKEN_PROGRAM: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for AldrinSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                pool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool),
                            1usize,
                        ),
                    )?,
                pool_signer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_signer),
                            2usize,
                        ),
                    )?,
                pool_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_mint),
                            3usize,
                        ),
                    )?,
                base_token_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_token_vault),
                            4usize,
                        ),
                    )?,
                quote_token_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_token_vault),
                            5usize,
                        ),
                    )?,
                fee_pool_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_pool_token_account),
                            6usize,
                        ),
                    )?,
                wallet_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(wallet_authority),
                            7usize,
                        ),
                    )?,
                user_base_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_base_token_account),
                            8usize,
                        ),
                    )?,
                user_quote_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_quote_token_account),
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
    pub struct AldrinV2Swap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub pool: ::solana_program::pubkey::Pubkey,
        pub pool_signer: ::solana_program::pubkey::Pubkey,
        pub pool_mint: ::solana_program::pubkey::Pubkey,
        pub base_token_vault: ::solana_program::pubkey::Pubkey,
        pub quote_token_vault: ::solana_program::pubkey::Pubkey,
        pub fee_pool_token_account: ::solana_program::pubkey::Pubkey,
        pub wallet_authority: ::solana_program::pubkey::Pubkey,
        pub user_base_token_account: ::solana_program::pubkey::Pubkey,
        pub user_quote_token_account: ::solana_program::pubkey::Pubkey,
        pub curve: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl AldrinV2Swap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                pool,
                pool_signer,
                pool_mint,
                base_token_vault,
                quote_token_vault,
                fee_pool_token_account,
                wallet_authority,
                user_base_token_account,
                user_quote_token_account,
                curve,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(pool,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(pool_signer,
                false), ::solana_program::instruction::AccountMeta::new(pool_mint,
                false), ::solana_program::instruction::AccountMeta::new(base_token_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(quote_token_vault,
                false),
                ::solana_program::instruction::AccountMeta::new(fee_pool_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(wallet_authority,
                true),
                ::solana_program::instruction::AccountMeta::new(user_base_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(user_quote_token_account,
                false), ::solana_program::instruction::AccountMeta::new_readonly(curve,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::AldrinV2Swap {
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
    pub struct AldrinV2SwapAccountIndexes {
        pub swap_program: usize,
        pub pool: usize,
        pub pool_signer: usize,
        pub pool_mint: usize,
        pub base_token_vault: usize,
        pub quote_token_vault: usize,
        pub fee_pool_token_account: usize,
        pub wallet_authority: usize,
        pub user_base_token_account: usize,
        pub user_quote_token_account: usize,
        pub curve: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl AldrinV2SwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const POOL: usize = 1usize;
        pub const POOL_SIGNER: usize = 2usize;
        pub const POOL_MINT: usize = 3usize;
        pub const BASE_TOKEN_VAULT: usize = 4usize;
        pub const QUOTE_TOKEN_VAULT: usize = 5usize;
        pub const FEE_POOL_TOKEN_ACCOUNT: usize = 6usize;
        pub const WALLET_AUTHORITY: usize = 7usize;
        pub const USER_BASE_TOKEN_ACCOUNT: usize = 8usize;
        pub const USER_QUOTE_TOKEN_ACCOUNT: usize = 9usize;
        pub const CURVE: usize = 10usize;
        pub const TOKEN_PROGRAM: usize = 11usize;
    }
    impl<'a> TryFrom<&'a [u8]> for AldrinV2SwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                pool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool),
                            1usize,
                        ),
                    )?,
                pool_signer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_signer),
                            2usize,
                        ),
                    )?,
                pool_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_mint),
                            3usize,
                        ),
                    )?,
                base_token_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(base_token_vault),
                            4usize,
                        ),
                    )?,
                quote_token_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(quote_token_vault),
                            5usize,
                        ),
                    )?,
                fee_pool_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_pool_token_account),
                            6usize,
                        ),
                    )?,
                wallet_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(wallet_authority),
                            7usize,
                        ),
                    )?,
                user_base_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_base_token_account),
                            8usize,
                        ),
                    )?,
                user_quote_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_quote_token_account),
                            9usize,
                        ),
                    )?,
                curve: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(curve),
                            10usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            11usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct WhirlpoolSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
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
    }
    impl WhirlpoolSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
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
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
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
            let data = JupiterInstruction::WhirlpoolSwap {
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
    pub struct WhirlpoolSwapAccountIndexes {
        pub swap_program: usize,
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
    impl WhirlpoolSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const TOKEN_PROGRAM: usize = 1usize;
        pub const TOKEN_AUTHORITY: usize = 2usize;
        pub const WHIRLPOOL: usize = 3usize;
        pub const TOKEN_OWNER_ACCOUNT_A: usize = 4usize;
        pub const TOKEN_VAULT_A: usize = 5usize;
        pub const TOKEN_OWNER_ACCOUNT_B: usize = 6usize;
        pub const TOKEN_VAULT_B: usize = 7usize;
        pub const TICK_ARRAY0: usize = 8usize;
        pub const TICK_ARRAY1: usize = 9usize;
        pub const TICK_ARRAY2: usize = 10usize;
        pub const ORACLE: usize = 11usize;
    }
    impl<'a> TryFrom<&'a [u8]> for WhirlpoolSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                token_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_authority),
                            2usize,
                        ),
                    )?,
                whirlpool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(whirlpool),
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
                tick_array0: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array0),
                            8usize,
                        ),
                    )?,
                tick_array1: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array1),
                            9usize,
                        ),
                    )?,
                tick_array2: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tick_array2),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct InvariantSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub pool: ::solana_program::pubkey::Pubkey,
        pub tickmap: ::solana_program::pubkey::Pubkey,
        pub account_x: ::solana_program::pubkey::Pubkey,
        pub account_y: ::solana_program::pubkey::Pubkey,
        pub reserve_x: ::solana_program::pubkey::Pubkey,
        pub reserve_y: ::solana_program::pubkey::Pubkey,
        pub owner: ::solana_program::pubkey::Pubkey,
        pub program_authority: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl InvariantSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                state,
                pool,
                tickmap,
                account_x,
                account_y,
                reserve_x,
                reserve_y,
                owner,
                program_authority,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(state,
                false), ::solana_program::instruction::AccountMeta::new(pool, false),
                ::solana_program::instruction::AccountMeta::new(tickmap, false),
                ::solana_program::instruction::AccountMeta::new(account_x, false),
                ::solana_program::instruction::AccountMeta::new(account_y, false),
                ::solana_program::instruction::AccountMeta::new(reserve_x, false),
                ::solana_program::instruction::AccountMeta::new(reserve_y, false),
                ::solana_program::instruction::AccountMeta::new_readonly(owner, true),
                ::solana_program::instruction::AccountMeta::new_readonly(program_authority,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::InvariantSwap {
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
    pub struct InvariantSwapAccountIndexes {
        pub swap_program: usize,
        pub state: usize,
        pub pool: usize,
        pub tickmap: usize,
        pub account_x: usize,
        pub account_y: usize,
        pub reserve_x: usize,
        pub reserve_y: usize,
        pub owner: usize,
        pub program_authority: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl InvariantSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const POOL: usize = 2usize;
        pub const TICKMAP: usize = 3usize;
        pub const ACCOUNT_X: usize = 4usize;
        pub const ACCOUNT_Y: usize = 5usize;
        pub const RESERVE_X: usize = 6usize;
        pub const RESERVE_Y: usize = 7usize;
        pub const OWNER: usize = 8usize;
        pub const PROGRAM_AUTHORITY: usize = 9usize;
        pub const TOKEN_PROGRAM: usize = 10usize;
    }
    impl<'a> TryFrom<&'a [u8]> for InvariantSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                pool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool),
                            2usize,
                        ),
                    )?,
                tickmap: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tickmap),
                            3usize,
                        ),
                    )?,
                account_x: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(account_x),
                            4usize,
                        ),
                    )?,
                account_y: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(account_y),
                            5usize,
                        ),
                    )?,
                reserve_x: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reserve_x),
                            6usize,
                        ),
                    )?,
                reserve_y: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(reserve_y),
                            7usize,
                        ),
                    )?,
                owner: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(owner),
                            8usize,
                        ),
                    )?,
                program_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(program_authority),
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
    pub struct MeteoraSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub pool: ::solana_program::pubkey::Pubkey,
        pub user_source_token: ::solana_program::pubkey::Pubkey,
        pub user_destination_token: ::solana_program::pubkey::Pubkey,
        pub a_vault: ::solana_program::pubkey::Pubkey,
        pub b_vault: ::solana_program::pubkey::Pubkey,
        pub a_token_vault: ::solana_program::pubkey::Pubkey,
        pub b_token_vault: ::solana_program::pubkey::Pubkey,
        pub a_vault_lp_mint: ::solana_program::pubkey::Pubkey,
        pub b_vault_lp_mint: ::solana_program::pubkey::Pubkey,
        pub a_vault_lp: ::solana_program::pubkey::Pubkey,
        pub b_vault_lp: ::solana_program::pubkey::Pubkey,
        pub admin_token_fee: ::solana_program::pubkey::Pubkey,
        pub user: ::solana_program::pubkey::Pubkey,
        pub vault_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl MeteoraSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                pool,
                user_source_token,
                user_destination_token,
                a_vault,
                b_vault,
                a_token_vault,
                b_token_vault,
                a_vault_lp_mint,
                b_vault_lp_mint,
                a_vault_lp,
                b_vault_lp,
                admin_token_fee,
                user,
                vault_program,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new(pool, false),
                ::solana_program::instruction::AccountMeta::new(user_source_token,
                false),
                ::solana_program::instruction::AccountMeta::new(user_destination_token,
                false), ::solana_program::instruction::AccountMeta::new(a_vault, false),
                ::solana_program::instruction::AccountMeta::new(b_vault, false),
                ::solana_program::instruction::AccountMeta::new(a_token_vault, false),
                ::solana_program::instruction::AccountMeta::new(b_token_vault, false),
                ::solana_program::instruction::AccountMeta::new(a_vault_lp_mint, false),
                ::solana_program::instruction::AccountMeta::new(b_vault_lp_mint, false),
                ::solana_program::instruction::AccountMeta::new(a_vault_lp, false),
                ::solana_program::instruction::AccountMeta::new(b_vault_lp, false),
                ::solana_program::instruction::AccountMeta::new(admin_token_fee, false),
                ::solana_program::instruction::AccountMeta::new_readonly(user, true),
                ::solana_program::instruction::AccountMeta::new_readonly(vault_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::MeteoraSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct MeteoraSwapAccountIndexes {
        pub swap_program: usize,
        pub pool: usize,
        pub user_source_token: usize,
        pub user_destination_token: usize,
        pub a_vault: usize,
        pub b_vault: usize,
        pub a_token_vault: usize,
        pub b_token_vault: usize,
        pub a_vault_lp_mint: usize,
        pub b_vault_lp_mint: usize,
        pub a_vault_lp: usize,
        pub b_vault_lp: usize,
        pub admin_token_fee: usize,
        pub user: usize,
        pub vault_program: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl MeteoraSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const POOL: usize = 1usize;
        pub const USER_SOURCE_TOKEN: usize = 2usize;
        pub const USER_DESTINATION_TOKEN: usize = 3usize;
        pub const A_VAULT: usize = 4usize;
        pub const B_VAULT: usize = 5usize;
        pub const A_TOKEN_VAULT: usize = 6usize;
        pub const B_TOKEN_VAULT: usize = 7usize;
        pub const A_VAULT_LP_MINT: usize = 8usize;
        pub const B_VAULT_LP_MINT: usize = 9usize;
        pub const A_VAULT_LP: usize = 10usize;
        pub const B_VAULT_LP: usize = 11usize;
        pub const ADMIN_TOKEN_FEE: usize = 12usize;
        pub const USER: usize = 13usize;
        pub const VAULT_PROGRAM: usize = 14usize;
        pub const TOKEN_PROGRAM: usize = 15usize;
    }
    impl<'a> TryFrom<&'a [u8]> for MeteoraSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                pool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool),
                            1usize,
                        ),
                    )?,
                user_source_token: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_source_token),
                            2usize,
                        ),
                    )?,
                user_destination_token: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_destination_token),
                            3usize,
                        ),
                    )?,
                a_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(a_vault),
                            4usize,
                        ),
                    )?,
                b_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(b_vault),
                            5usize,
                        ),
                    )?,
                a_token_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(a_token_vault),
                            6usize,
                        ),
                    )?,
                b_token_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(b_token_vault),
                            7usize,
                        ),
                    )?,
                a_vault_lp_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(a_vault_lp_mint),
                            8usize,
                        ),
                    )?,
                b_vault_lp_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(b_vault_lp_mint),
                            9usize,
                        ),
                    )?,
                a_vault_lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(a_vault_lp),
                            10usize,
                        ),
                    )?,
                b_vault_lp: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(b_vault_lp),
                            11usize,
                        ),
                    )?,
                admin_token_fee: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin_token_fee),
                            12usize,
                        ),
                    )?,
                user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user),
                            13usize,
                        ),
                    )?,
                vault_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(vault_program),
                            14usize,
                        ),
                    )?,
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            15usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct GoosefxSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub controller: ::solana_program::pubkey::Pubkey,
        pub pair: ::solana_program::pubkey::Pubkey,
        pub ssl_in: ::solana_program::pubkey::Pubkey,
        pub ssl_out: ::solana_program::pubkey::Pubkey,
        pub liability_vault_in: ::solana_program::pubkey::Pubkey,
        pub swapped_liability_vault_in: ::solana_program::pubkey::Pubkey,
        pub liability_vault_out: ::solana_program::pubkey::Pubkey,
        pub swapped_liability_vault_out: ::solana_program::pubkey::Pubkey,
        pub user_in_ata: ::solana_program::pubkey::Pubkey,
        pub user_out_ata: ::solana_program::pubkey::Pubkey,
        pub fee_collector_ata: ::solana_program::pubkey::Pubkey,
        pub user_wallet: ::solana_program::pubkey::Pubkey,
        pub fee_collector: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl GoosefxSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                controller,
                pair,
                ssl_in,
                ssl_out,
                liability_vault_in,
                swapped_liability_vault_in,
                liability_vault_out,
                swapped_liability_vault_out,
                user_in_ata,
                user_out_ata,
                fee_collector_ata,
                user_wallet,
                fee_collector,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(controller,
                false), ::solana_program::instruction::AccountMeta::new(pair, false),
                ::solana_program::instruction::AccountMeta::new(ssl_in, false),
                ::solana_program::instruction::AccountMeta::new(ssl_out, false),
                ::solana_program::instruction::AccountMeta::new(liability_vault_in,
                false),
                ::solana_program::instruction::AccountMeta::new(swapped_liability_vault_in,
                false),
                ::solana_program::instruction::AccountMeta::new(liability_vault_out,
                false),
                ::solana_program::instruction::AccountMeta::new(swapped_liability_vault_out,
                false), ::solana_program::instruction::AccountMeta::new(user_in_ata,
                false), ::solana_program::instruction::AccountMeta::new(user_out_ata,
                false),
                ::solana_program::instruction::AccountMeta::new(fee_collector_ata,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_wallet,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(fee_collector,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::GoosefxSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct GoosefxSwapAccountIndexes {
        pub swap_program: usize,
        pub controller: usize,
        pub pair: usize,
        pub ssl_in: usize,
        pub ssl_out: usize,
        pub liability_vault_in: usize,
        pub swapped_liability_vault_in: usize,
        pub liability_vault_out: usize,
        pub swapped_liability_vault_out: usize,
        pub user_in_ata: usize,
        pub user_out_ata: usize,
        pub fee_collector_ata: usize,
        pub user_wallet: usize,
        pub fee_collector: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl GoosefxSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const CONTROLLER: usize = 1usize;
        pub const PAIR: usize = 2usize;
        pub const SSL_IN: usize = 3usize;
        pub const SSL_OUT: usize = 4usize;
        pub const LIABILITY_VAULT_IN: usize = 5usize;
        pub const SWAPPED_LIABILITY_VAULT_IN: usize = 6usize;
        pub const LIABILITY_VAULT_OUT: usize = 7usize;
        pub const SWAPPED_LIABILITY_VAULT_OUT: usize = 8usize;
        pub const USER_IN_ATA: usize = 9usize;
        pub const USER_OUT_ATA: usize = 10usize;
        pub const FEE_COLLECTOR_ATA: usize = 11usize;
        pub const USER_WALLET: usize = 12usize;
        pub const FEE_COLLECTOR: usize = 13usize;
        pub const TOKEN_PROGRAM: usize = 14usize;
    }
    impl<'a> TryFrom<&'a [u8]> for GoosefxSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                controller: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(controller),
                            1usize,
                        ),
                    )?,
                pair: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pair),
                            2usize,
                        ),
                    )?,
                ssl_in: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(ssl_in),
                            3usize,
                        ),
                    )?,
                ssl_out: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(ssl_out),
                            4usize,
                        ),
                    )?,
                liability_vault_in: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(liability_vault_in),
                            5usize,
                        ),
                    )?,
                swapped_liability_vault_in: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swapped_liability_vault_in),
                            6usize,
                        ),
                    )?,
                liability_vault_out: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(liability_vault_out),
                            7usize,
                        ),
                    )?,
                swapped_liability_vault_out: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swapped_liability_vault_out),
                            8usize,
                        ),
                    )?,
                user_in_ata: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_in_ata),
                            9usize,
                        ),
                    )?,
                user_out_ata: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_out_ata),
                            10usize,
                        ),
                    )?,
                fee_collector_ata: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_collector_ata),
                            11usize,
                        ),
                    )?,
                user_wallet: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_wallet),
                            12usize,
                        ),
                    )?,
                fee_collector: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(fee_collector),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct DeltafiSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub market_config: ::solana_program::pubkey::Pubkey,
        pub swap_info: ::solana_program::pubkey::Pubkey,
        pub user_source_token: ::solana_program::pubkey::Pubkey,
        pub user_destination_token: ::solana_program::pubkey::Pubkey,
        pub swap_source_token: ::solana_program::pubkey::Pubkey,
        pub swap_destination_token: ::solana_program::pubkey::Pubkey,
        pub deltafi_user: ::solana_program::pubkey::Pubkey,
        pub admin_destination_token: ::solana_program::pubkey::Pubkey,
        pub pyth_price_base: ::solana_program::pubkey::Pubkey,
        pub pyth_price_quote: ::solana_program::pubkey::Pubkey,
        pub user_authority: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl DeltafiSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                market_config,
                swap_info,
                user_source_token,
                user_destination_token,
                swap_source_token,
                swap_destination_token,
                deltafi_user,
                admin_destination_token,
                pyth_price_base,
                pyth_price_quote,
                user_authority,
                token_program,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(market_config,
                false), ::solana_program::instruction::AccountMeta::new(swap_info,
                false),
                ::solana_program::instruction::AccountMeta::new(user_source_token,
                false),
                ::solana_program::instruction::AccountMeta::new(user_destination_token,
                false),
                ::solana_program::instruction::AccountMeta::new(swap_source_token,
                false),
                ::solana_program::instruction::AccountMeta::new(swap_destination_token,
                false), ::solana_program::instruction::AccountMeta::new(deltafi_user,
                false),
                ::solana_program::instruction::AccountMeta::new(admin_destination_token,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(pyth_price_base,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(pyth_price_quote,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(user_authority,
                true),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::DeltafiSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct DeltafiSwapAccountIndexes {
        pub swap_program: usize,
        pub market_config: usize,
        pub swap_info: usize,
        pub user_source_token: usize,
        pub user_destination_token: usize,
        pub swap_source_token: usize,
        pub swap_destination_token: usize,
        pub deltafi_user: usize,
        pub admin_destination_token: usize,
        pub pyth_price_base: usize,
        pub pyth_price_quote: usize,
        pub user_authority: usize,
        pub token_program: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl DeltafiSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const MARKET_CONFIG: usize = 1usize;
        pub const SWAP_INFO: usize = 2usize;
        pub const USER_SOURCE_TOKEN: usize = 3usize;
        pub const USER_DESTINATION_TOKEN: usize = 4usize;
        pub const SWAP_SOURCE_TOKEN: usize = 5usize;
        pub const SWAP_DESTINATION_TOKEN: usize = 6usize;
        pub const DELTAFI_USER: usize = 7usize;
        pub const ADMIN_DESTINATION_TOKEN: usize = 8usize;
        pub const PYTH_PRICE_BASE: usize = 9usize;
        pub const PYTH_PRICE_QUOTE: usize = 10usize;
        pub const USER_AUTHORITY: usize = 11usize;
        pub const TOKEN_PROGRAM: usize = 12usize;
    }
    impl<'a> TryFrom<&'a [u8]> for DeltafiSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                market_config: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(market_config),
                            1usize,
                        ),
                    )?,
                swap_info: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_info),
                            2usize,
                        ),
                    )?,
                user_source_token: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_source_token),
                            3usize,
                        ),
                    )?,
                user_destination_token: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_destination_token),
                            4usize,
                        ),
                    )?,
                swap_source_token: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_source_token),
                            5usize,
                        ),
                    )?,
                swap_destination_token: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_destination_token),
                            6usize,
                        ),
                    )?,
                deltafi_user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(deltafi_user),
                            7usize,
                        ),
                    )?,
                admin_destination_token: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(admin_destination_token),
                            8usize,
                        ),
                    )?,
                pyth_price_base: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pyth_price_base),
                            9usize,
                        ),
                    )?,
                pyth_price_quote: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pyth_price_quote),
                            10usize,
                        ),
                    )?,
                user_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(user_authority),
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
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct BalansolSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub authority: ::solana_program::pubkey::Pubkey,
        pub pool: ::solana_program::pubkey::Pubkey,
        pub tax_man: ::solana_program::pubkey::Pubkey,
        pub bid_mint: ::solana_program::pubkey::Pubkey,
        pub treasurer: ::solana_program::pubkey::Pubkey,
        pub src_treasury: ::solana_program::pubkey::Pubkey,
        pub src_associated_token_account: ::solana_program::pubkey::Pubkey,
        pub ask_mint: ::solana_program::pubkey::Pubkey,
        pub dst_treasury: ::solana_program::pubkey::Pubkey,
        pub dst_associated_token_account: ::solana_program::pubkey::Pubkey,
        pub dst_token_account_taxman: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl BalansolSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                authority,
                pool,
                tax_man,
                bid_mint,
                treasurer,
                src_treasury,
                src_associated_token_account,
                ask_mint,
                dst_treasury,
                dst_associated_token_account,
                dst_token_account_taxman,
                system_program,
                token_program,
                associated_token_program,
                rent,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new(authority, true),
                ::solana_program::instruction::AccountMeta::new(pool, false),
                ::solana_program::instruction::AccountMeta::new(tax_man, false),
                ::solana_program::instruction::AccountMeta::new_readonly(bid_mint,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(treasurer,
                false), ::solana_program::instruction::AccountMeta::new(src_treasury,
                false),
                ::solana_program::instruction::AccountMeta::new(src_associated_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(ask_mint,
                false), ::solana_program::instruction::AccountMeta::new(dst_treasury,
                false),
                ::solana_program::instruction::AccountMeta::new(dst_associated_token_account,
                false),
                ::solana_program::instruction::AccountMeta::new(dst_token_account_taxman,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::BalansolSwap {
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
    pub struct BalansolSwapAccountIndexes {
        pub swap_program: usize,
        pub authority: usize,
        pub pool: usize,
        pub tax_man: usize,
        pub bid_mint: usize,
        pub treasurer: usize,
        pub src_treasury: usize,
        pub src_associated_token_account: usize,
        pub ask_mint: usize,
        pub dst_treasury: usize,
        pub dst_associated_token_account: usize,
        pub dst_token_account_taxman: usize,
        pub system_program: usize,
        pub token_program: usize,
        pub associated_token_program: usize,
        pub rent: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl BalansolSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const AUTHORITY: usize = 1usize;
        pub const POOL: usize = 2usize;
        pub const TAX_MAN: usize = 3usize;
        pub const BID_MINT: usize = 4usize;
        pub const TREASURER: usize = 5usize;
        pub const SRC_TREASURY: usize = 6usize;
        pub const SRC_ASSOCIATED_TOKEN_ACCOUNT: usize = 7usize;
        pub const ASK_MINT: usize = 8usize;
        pub const DST_TREASURY: usize = 9usize;
        pub const DST_ASSOCIATED_TOKEN_ACCOUNT: usize = 10usize;
        pub const DST_TOKEN_ACCOUNT_TAXMAN: usize = 11usize;
        pub const SYSTEM_PROGRAM: usize = 12usize;
        pub const TOKEN_PROGRAM: usize = 13usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 14usize;
        pub const RENT: usize = 15usize;
    }
    impl<'a> TryFrom<&'a [u8]> for BalansolSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                pool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool),
                            2usize,
                        ),
                    )?,
                tax_man: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(tax_man),
                            3usize,
                        ),
                    )?,
                bid_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(bid_mint),
                            4usize,
                        ),
                    )?,
                treasurer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(treasurer),
                            5usize,
                        ),
                    )?,
                src_treasury: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(src_treasury),
                            6usize,
                        ),
                    )?,
                src_associated_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(src_associated_token_account),
                            7usize,
                        ),
                    )?,
                ask_mint: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(ask_mint),
                            8usize,
                        ),
                    )?,
                dst_treasury: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(dst_treasury),
                            9usize,
                        ),
                    )?,
                dst_associated_token_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(dst_associated_token_account),
                            10usize,
                        ),
                    )?,
                dst_token_account_taxman: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(dst_token_account_taxman),
                            11usize,
                        ),
                    )?,
                system_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(system_program),
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
                associated_token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(associated_token_program),
                            14usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            15usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct MarcoPoloSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub state: ::solana_program::pubkey::Pubkey,
        pub pool: ::solana_program::pubkey::Pubkey,
        pub token_x: ::solana_program::pubkey::Pubkey,
        pub token_y: ::solana_program::pubkey::Pubkey,
        pub pool_x_account: ::solana_program::pubkey::Pubkey,
        pub pool_y_account: ::solana_program::pubkey::Pubkey,
        pub swapper_x_account: ::solana_program::pubkey::Pubkey,
        pub swapper_y_account: ::solana_program::pubkey::Pubkey,
        pub swapper: ::solana_program::pubkey::Pubkey,
        pub referrer_x_account: ::solana_program::pubkey::Pubkey,
        pub referrer_y_account: ::solana_program::pubkey::Pubkey,
        pub referrer: ::solana_program::pubkey::Pubkey,
        pub program_authority: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub associated_token_program: ::solana_program::pubkey::Pubkey,
        pub rent: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl MarcoPoloSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                state,
                pool,
                token_x,
                token_y,
                pool_x_account,
                pool_y_account,
                swapper_x_account,
                swapper_y_account,
                swapper,
                referrer_x_account,
                referrer_y_account,
                referrer,
                program_authority,
                system_program,
                token_program,
                associated_token_program,
                rent,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(state,
                false), ::solana_program::instruction::AccountMeta::new(pool, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_x, false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_y, false),
                ::solana_program::instruction::AccountMeta::new(pool_x_account, false),
                ::solana_program::instruction::AccountMeta::new(pool_y_account, false),
                ::solana_program::instruction::AccountMeta::new(swapper_x_account,
                false),
                ::solana_program::instruction::AccountMeta::new(swapper_y_account,
                false), ::solana_program::instruction::AccountMeta::new(swapper, true),
                ::solana_program::instruction::AccountMeta::new(referrer_x_account,
                false),
                ::solana_program::instruction::AccountMeta::new(referrer_y_account,
                false), ::solana_program::instruction::AccountMeta::new(referrer, false),
                ::solana_program::instruction::AccountMeta::new_readonly(program_authority,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(associated_token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(rent,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::MarcoPoloSwap {
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
    pub struct MarcoPoloSwapAccountIndexes {
        pub swap_program: usize,
        pub state: usize,
        pub pool: usize,
        pub token_x: usize,
        pub token_y: usize,
        pub pool_x_account: usize,
        pub pool_y_account: usize,
        pub swapper_x_account: usize,
        pub swapper_y_account: usize,
        pub swapper: usize,
        pub referrer_x_account: usize,
        pub referrer_y_account: usize,
        pub referrer: usize,
        pub program_authority: usize,
        pub system_program: usize,
        pub token_program: usize,
        pub associated_token_program: usize,
        pub rent: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl MarcoPoloSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const STATE: usize = 1usize;
        pub const POOL: usize = 2usize;
        pub const TOKEN_X: usize = 3usize;
        pub const TOKEN_Y: usize = 4usize;
        pub const POOL_X_ACCOUNT: usize = 5usize;
        pub const POOL_Y_ACCOUNT: usize = 6usize;
        pub const SWAPPER_X_ACCOUNT: usize = 7usize;
        pub const SWAPPER_Y_ACCOUNT: usize = 8usize;
        pub const SWAPPER: usize = 9usize;
        pub const REFERRER_X_ACCOUNT: usize = 10usize;
        pub const REFERRER_Y_ACCOUNT: usize = 11usize;
        pub const REFERRER: usize = 12usize;
        pub const PROGRAM_AUTHORITY: usize = 13usize;
        pub const SYSTEM_PROGRAM: usize = 14usize;
        pub const TOKEN_PROGRAM: usize = 15usize;
        pub const ASSOCIATED_TOKEN_PROGRAM: usize = 16usize;
        pub const RENT: usize = 17usize;
    }
    impl<'a> TryFrom<&'a [u8]> for MarcoPoloSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
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
                pool: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool),
                            2usize,
                        ),
                    )?,
                token_x: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_x),
                            3usize,
                        ),
                    )?,
                token_y: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_y),
                            4usize,
                        ),
                    )?,
                pool_x_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_x_account),
                            5usize,
                        ),
                    )?,
                pool_y_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pool_y_account),
                            6usize,
                        ),
                    )?,
                swapper_x_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swapper_x_account),
                            7usize,
                        ),
                    )?,
                swapper_y_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swapper_y_account),
                            8usize,
                        ),
                    )?,
                swapper: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swapper),
                            9usize,
                        ),
                    )?,
                referrer_x_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(referrer_x_account),
                            10usize,
                        ),
                    )?,
                referrer_y_account: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(referrer_y_account),
                            11usize,
                        ),
                    )?,
                referrer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(referrer),
                            12usize,
                        ),
                    )?,
                program_authority: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(program_authority),
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
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            15usize,
                        ),
                    )?,
                associated_token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(associated_token_program),
                            16usize,
                        ),
                    )?,
                rent: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(rent),
                            17usize,
                        ),
                    )?,
                trailing_accounts: iter.collect(),
            })
        }
    }
    #[derive(Debug)]
    pub struct DradexSwap {
        pub program_id: ::solana_program::pubkey::Pubkey,
        pub swap_program: ::solana_program::pubkey::Pubkey,
        pub pair: ::solana_program::pubkey::Pubkey,
        pub market: ::solana_program::pubkey::Pubkey,
        pub event_queue: ::solana_program::pubkey::Pubkey,
        pub dex_user: ::solana_program::pubkey::Pubkey,
        pub market_user: ::solana_program::pubkey::Pubkey,
        pub bids: ::solana_program::pubkey::Pubkey,
        pub asks: ::solana_program::pubkey::Pubkey,
        pub t0_vault: ::solana_program::pubkey::Pubkey,
        pub t1_vault: ::solana_program::pubkey::Pubkey,
        pub t0_user: ::solana_program::pubkey::Pubkey,
        pub t1_user: ::solana_program::pubkey::Pubkey,
        pub master: ::solana_program::pubkey::Pubkey,
        pub signer: ::solana_program::pubkey::Pubkey,
        pub system_program: ::solana_program::pubkey::Pubkey,
        pub token_program: ::solana_program::pubkey::Pubkey,
        pub logger: ::solana_program::pubkey::Pubkey,
        pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,
    }
    impl DradexSwap {
        pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
            let Self {
                program_id,
                swap_program,
                pair,
                market,
                event_queue,
                dex_user,
                market_user,
                bids,
                asks,
                t0_vault,
                t1_vault,
                t0_user,
                t1_user,
                master,
                signer,
                system_program,
                token_program,
                logger,
                trailing_accounts,
            } = self;
            let mut accounts = vec![
                ::solana_program::instruction::AccountMeta::new_readonly(swap_program,
                false), ::solana_program::instruction::AccountMeta::new(pair, false),
                ::solana_program::instruction::AccountMeta::new(market, false),
                ::solana_program::instruction::AccountMeta::new(event_queue, false),
                ::solana_program::instruction::AccountMeta::new_readonly(dex_user,
                false), ::solana_program::instruction::AccountMeta::new(market_user,
                false), ::solana_program::instruction::AccountMeta::new(bids, false),
                ::solana_program::instruction::AccountMeta::new(asks, false),
                ::solana_program::instruction::AccountMeta::new(t0_vault, false),
                ::solana_program::instruction::AccountMeta::new(t1_vault, false),
                ::solana_program::instruction::AccountMeta::new(t0_user, false),
                ::solana_program::instruction::AccountMeta::new(t1_user, false),
                ::solana_program::instruction::AccountMeta::new_readonly(master, false),
                ::solana_program::instruction::AccountMeta::new(signer, true),
                ::solana_program::instruction::AccountMeta::new_readonly(system_program,
                false),
                ::solana_program::instruction::AccountMeta::new_readonly(token_program,
                false), ::solana_program::instruction::AccountMeta::new_readonly(logger,
                false),
            ];
            if !trailing_accounts.is_empty() {
                accounts.extend(trailing_accounts);
            }
            let data = JupiterInstruction::DradexSwap {}.pack();
            ::solana_program::instruction::Instruction {
                program_id,
                data,
                accounts,
            }
        }
    }
    #[derive(Debug)]
    pub struct DradexSwapAccountIndexes {
        pub swap_program: usize,
        pub pair: usize,
        pub market: usize,
        pub event_queue: usize,
        pub dex_user: usize,
        pub market_user: usize,
        pub bids: usize,
        pub asks: usize,
        pub t0_vault: usize,
        pub t1_vault: usize,
        pub t0_user: usize,
        pub t1_user: usize,
        pub master: usize,
        pub signer: usize,
        pub system_program: usize,
        pub token_program: usize,
        pub logger: usize,
        pub trailing_accounts: Vec<usize>,
    }
    impl DradexSwapAccountIndexes {
        pub const SWAP_PROGRAM: usize = 0usize;
        pub const PAIR: usize = 1usize;
        pub const MARKET: usize = 2usize;
        pub const EVENT_QUEUE: usize = 3usize;
        pub const DEX_USER: usize = 4usize;
        pub const MARKET_USER: usize = 5usize;
        pub const BIDS: usize = 6usize;
        pub const ASKS: usize = 7usize;
        pub const T0_VAULT: usize = 8usize;
        pub const T1_VAULT: usize = 9usize;
        pub const T0_USER: usize = 10usize;
        pub const T1_USER: usize = 11usize;
        pub const MASTER: usize = 12usize;
        pub const SIGNER: usize = 13usize;
        pub const SYSTEM_PROGRAM: usize = 14usize;
        pub const TOKEN_PROGRAM: usize = 15usize;
        pub const LOGGER: usize = 16usize;
    }
    impl<'a> TryFrom<&'a [u8]> for DradexSwapAccountIndexes {
        type Error = ::anchor_interface::errors::TryAccountIndexesError;
        fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
            let mut iter = indexes.iter().map(|idx| (*idx) as usize);
            Ok(Self {
                swap_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(swap_program),
                            0usize,
                        ),
                    )?,
                pair: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(pair),
                            1usize,
                        ),
                    )?,
                market: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(market),
                            2usize,
                        ),
                    )?,
                event_queue: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(event_queue),
                            3usize,
                        ),
                    )?,
                dex_user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(dex_user),
                            4usize,
                        ),
                    )?,
                market_user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(market_user),
                            5usize,
                        ),
                    )?,
                bids: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(bids),
                            6usize,
                        ),
                    )?,
                asks: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(asks),
                            7usize,
                        ),
                    )?,
                t0_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(t0_vault),
                            8usize,
                        ),
                    )?,
                t1_vault: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(t1_vault),
                            9usize,
                        ),
                    )?,
                t0_user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(t0_user),
                            10usize,
                        ),
                    )?,
                t1_user: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(t1_user),
                            11usize,
                        ),
                    )?,
                master: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(master),
                            12usize,
                        ),
                    )?,
                signer: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(signer),
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
                token_program: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(token_program),
                            15usize,
                        ),
                    )?,
                logger: iter
                    .next()
                    .ok_or(
                        ::anchor_interface::errors::TryAccountIndexesError::GetIndex(
                            stringify!(logger),
                            16usize,
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
    pub struct AmountWithSlippage {
        pub amount: u64,
        pub slippage_bps: u16,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct SplitLegDeeper {
        pub percent: u8,
        pub swap_leg: SwapLegSwap,
    }
    #[derive(Clone, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct SplitLeg {
        pub percent: u8,
        pub swap_leg: SwapLegDeeper,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum SwapInstrution {
        Swap(Swap),
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum Side {
        Bid,
        Ask,
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum SwapLegSwap {
        PlaceholderOne,
        PlaceholderTwo,
        Swap { swap: Swap },
    }
    #[derive(Clone, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum SwapLegDeeper {
        Chain { swap_legs: Vec<SwapLegSwap> },
        Split { split_legs: Vec<SplitLegDeeper> },
        Swap { swap: Swap },
    }
    #[derive(Clone, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum SwapLeg {
        Chain { swap_legs: Vec<SwapLegDeeper> },
        Split { split_legs: Vec<SplitLeg> },
        Swap { swap: Swap },
    }
    #[derive(Clone, Copy, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum Swap {
        Saber,
        SaberAddDecimalsDeposit,
        SaberAddDecimalsWithdraw,
        TokenSwap,
        Sencha,
        Step,
        Cropper,
        Raydium,
        Crema,
        Lifinity,
        Mercurial,
        Cykura,
        Serum { side: Side },
        MarinadeDeposit,
        MarinadeUnstake,
        Aldrin { side: Side },
        AldrinV2 { side: Side },
        Whirlpool { a_to_b: bool },
        Invariant { x_to_y: bool },
        Meteora,
        GooseFx,
        DeltaFi { stable: bool },
        Balansol,
        MarcoPolo { x_to_y: bool },
        Dradex { side: Side },
    }
    #[derive(Clone, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub enum SwapAction {
        SetupSplit { percents: Vec<u8> },
        NextSplitLeg,
        MergeSplit,
        Swap { swap: Swap },
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct SwapE {
        pub amm: ::solana_program::pubkey::Pubkey,
        pub input_mint: ::solana_program::pubkey::Pubkey,
        pub input_amount: u64,
        pub output_mint: ::solana_program::pubkey::Pubkey,
        pub output_amount: u64,
    }
    #[derive(Clone, Copy, Default, ::borsh::BorshDeserialize, ::borsh::BorshSerialize)]
    #[derive(Debug)]
    pub struct Fee {
        pub account: ::solana_program::pubkey::Pubkey,
        pub mint: ::solana_program::pubkey::Pubkey,
        pub amount: u64,
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
    pub enum JupiterError {
        #[error("Empty route")]
        EmptyRoute = 6000u32,
        #[error("Slippage tolerance exceeded")]
        SlippageToleranceExceeded = 6001u32,
        #[error("Invalid calculation")]
        InvalidCalculation = 6002u32,
        #[error("Missing platform fee account")]
        MissingPlatformFeeAccount = 6003u32,
        #[error("Invalid slippage")]
        InvalidSlippage = 6004u32,
        #[error("Not enough percent to 100")]
        NotEnoughPercent = 6005u32,
        #[error("In amounts stack is empty")]
        InAmountsStackIsEmpty = 6006u32,
        #[error("Out amounts stack is empty")]
        OutAmountsStackIsEmpty = 6007u32,
        #[error("Not Enough Account keys")]
        NotEnoughAccountKeys = 6008u32,
    }
    impl DecodeError<JupiterError> for JupiterError {
        fn type_of() -> &'static str {
            "JupiterError"
        }
    }
    impl From<JupiterError> for ProgramError {
        fn from(err: JupiterError) -> Self {
            Self::Custom(err as u32)
        }
    }
}

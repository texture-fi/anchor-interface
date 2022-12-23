use solana_program::pubkey::Pubkey;

pub const PDA_WHIRLPOOL_SEED: &[u8] = b"whirlpool";
pub const PDA_POSITION_SEED: &[u8] = b"position";
pub const PDA_METADATA_SEED: &[u8] = b"metadata";
pub const PDA_TICK_ARRAY_SEED: &[u8] = b"tick_array";
pub const PDA_FEE_TIER_SEED: &[u8] = b"fee_tier";
pub const PDA_ORACLE_SEED: &[u8] = b"oracle";

pub fn get_whirlpool(
    program_id: &Pubkey,
    whirlpools_config: &Pubkey,
    token_mint_a: &Pubkey,
    token_mint_b: &Pubkey,
    tick_spacing: u16,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PDA_WHIRLPOOL_SEED,
            whirlpools_config.as_ref(),
            token_mint_a.as_ref(),
            token_mint_b.as_ref(),
            &tick_spacing.to_le_bytes(),
        ],
        program_id,
    )
}

pub fn get_position(program_id: &Pubkey, position_mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PDA_POSITION_SEED, position_mint.as_ref()], program_id)
}

pub fn get_fee_tier(
    program_id: &Pubkey,
    whirlpools_config: &Pubkey,
    tick_spacing: u16,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PDA_FEE_TIER_SEED,
            whirlpools_config.as_ref(),
            &tick_spacing.to_le_bytes(),
        ],
        program_id,
    )
}

pub fn get_oracle(program_id: &Pubkey, whirlpool: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[PDA_ORACLE_SEED, whirlpool.as_ref()], program_id)
}

pub fn get_tick_array(program_id: &Pubkey, whirlpool: &Pubkey, start_index: i32) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PDA_TICK_ARRAY_SEED,
            whirlpool.as_ref(),
            start_index.to_string().as_bytes(),
        ],
        program_id,
    )
}

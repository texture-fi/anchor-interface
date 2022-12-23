#![doc = gen_crate_docs!()]

use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_program::sysvar;

// We can generate code in-place
#[cfg(not(feature = "gen-file"))]
anchor_interface_gen::program!(idl = "psy_american.json");

// Or we can generate rust-file and then include or import that (easier to debug)
#[cfg(feature = "gen-file")]
anchor_interface_gen::program!(idl = "psy_american.json", out_file = "src/_gen_.rs");
#[cfg(feature = "gen-file")]
mod _gen_;
#[cfg(feature = "gen-file")]
pub use _gen_::*;

solana_program::declare_id!("R2y9ip6mxmWUj4pt54jP2hz2dgvMozy9VTSwMWE7evs");

pub mod fee_owner {
    solana_program::declare_id!("6c33US7ErPmLXZog9SyChQUYUrrJY51k4GmzdhrbhNnD");
}

pub mod address {
    use solana_program::pubkey::Pubkey;

    pub fn option_market(
        underlying_token: &Pubkey,
        quote_token: &Pubkey,
        underlying_amount_per_contract: u64,
        quote_amount_per_contract: u64,
        expiration: i64,
        program_id: &Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                underlying_token.as_ref(),
                quote_token.as_ref(),
                &underlying_amount_per_contract.to_le_bytes(),
                &quote_amount_per_contract.to_le_bytes(),
                &expiration.to_le_bytes(),
            ],
            program_id,
        )
    }

    pub const OPTION_MINT_SEED: &[u8] = b"optionToken";
    pub fn option_mint(option_market: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[option_market.as_ref(), OPTION_MINT_SEED], program_id)
    }

    pub const WRITER_MINT_SEED: &[u8] = b"writerToken";
    pub fn writer_mint(option_market: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[option_market.as_ref(), WRITER_MINT_SEED], program_id)
    }

    pub const QUOTE_ASSET_POOL_SEED: &[u8] = b"quoteAssetPool";
    pub fn quote_asset_pool(option_market: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[option_market.as_ref(), QUOTE_ASSET_POOL_SEED], program_id)
    }

    pub const UNDERLYING_ASSET_POOL_SEED: &[u8] = b"underlyingAssetPool";
    pub fn underlying_asset_pool(option_market: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[option_market.as_ref(), UNDERLYING_ASSET_POOL_SEED],
            program_id,
        )
    }
}

const DEFAULT_PUBKEY: Pubkey = Pubkey::new_from_array([255; 32]);

impl Default for instruction::InitializeMarket {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            system_program: system_program::id(),
            token_program: spl_token::id(),
            associated_token_program: spl_associated_token_account::id(),
            rent: sysvar::rent::id(),
            clock: sysvar::clock::id(),
            fee_owner: fee_owner::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            authority: DEFAULT_PUBKEY,
            underlying_asset_mint: DEFAULT_PUBKEY,
            quote_asset_mint: DEFAULT_PUBKEY,
            option_mint: DEFAULT_PUBKEY,
            writer_token_mint: DEFAULT_PUBKEY,
            quote_asset_pool: DEFAULT_PUBKEY,
            underlying_asset_pool: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            underlying_amount_per_contract: 0,
            quote_amount_per_contract: 0,
            expiration_unix_timestamp: 0,
            bump_seed: 0,
        }
    }
}

impl Default for instruction::MintOption {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            system_program: system_program::id(),
            token_program: spl_token::id(),
            associated_token_program: spl_associated_token_account::id(),
            rent: sysvar::rent::id(),
            clock: sysvar::clock::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            user_authority: DEFAULT_PUBKEY,
            underlying_asset_mint: DEFAULT_PUBKEY,
            underlying_asset_pool: DEFAULT_PUBKEY,
            underlying_asset_src: DEFAULT_PUBKEY,
            option_mint: DEFAULT_PUBKEY,
            minted_option_dest: DEFAULT_PUBKEY,
            writer_token_mint: DEFAULT_PUBKEY,
            minted_writer_token_dest: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            fee_owner: DEFAULT_PUBKEY,
            size: 0,
        }
    }
}

impl Default for instruction::MintOptionV2 {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            token_program: spl_token::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            user_authority: DEFAULT_PUBKEY,
            underlying_asset_mint: DEFAULT_PUBKEY,
            underlying_asset_pool: DEFAULT_PUBKEY,
            underlying_asset_src: DEFAULT_PUBKEY,
            option_mint: DEFAULT_PUBKEY,
            minted_option_dest: DEFAULT_PUBKEY,
            writer_token_mint: DEFAULT_PUBKEY,
            minted_writer_token_dest: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            size: 0,
        }
    }
}

impl Default for instruction::ExerciseOption {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            system_program: system_program::id(),
            token_program: spl_token::id(),
            clock: sysvar::clock::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            user_authority: DEFAULT_PUBKEY,
            option_authority: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            option_mint: DEFAULT_PUBKEY,
            exerciser_option_token_src: DEFAULT_PUBKEY,
            underlying_asset_pool: DEFAULT_PUBKEY,
            underlying_asset_dest: DEFAULT_PUBKEY,
            quote_asset_pool: DEFAULT_PUBKEY,
            quote_asset_src: DEFAULT_PUBKEY,
            fee_owner: DEFAULT_PUBKEY,
            size: 0,
        }
    }
}

impl Default for instruction::ExerciseOptionV2 {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            token_program: spl_token::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            user_authority: DEFAULT_PUBKEY,
            option_authority: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            option_mint: DEFAULT_PUBKEY,
            exerciser_option_token_src: DEFAULT_PUBKEY,
            underlying_asset_pool: DEFAULT_PUBKEY,
            underlying_asset_dest: DEFAULT_PUBKEY,
            quote_asset_pool: DEFAULT_PUBKEY,
            quote_asset_src: DEFAULT_PUBKEY,
            size: 0,
        }
    }
}

impl Default for instruction::ClosePostExpiration {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            token_program: spl_token::id(),
            clock: sysvar::clock::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            user_authority: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            writer_token_mint: DEFAULT_PUBKEY,
            writer_token_src: DEFAULT_PUBKEY,
            underlying_asset_pool: DEFAULT_PUBKEY,
            underlying_asset_dest: DEFAULT_PUBKEY,
            size: 0,
        }
    }
}

impl Default for instruction::CloseOptionPosition {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            token_program: spl_token::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            user_authority: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            writer_token_mint: DEFAULT_PUBKEY,
            writer_token_src: DEFAULT_PUBKEY,
            option_token_mint: DEFAULT_PUBKEY,
            option_token_src: DEFAULT_PUBKEY,
            underlying_asset_pool: DEFAULT_PUBKEY,
            underlying_asset_dest: DEFAULT_PUBKEY,
            size: 0,
        }
    }
}

impl Default for instruction::BurnWriterForQuote {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            token_program: spl_token::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            user_authority: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            writer_token_mint: DEFAULT_PUBKEY,
            writer_token_src: DEFAULT_PUBKEY,
            quote_asset_pool: DEFAULT_PUBKEY,
            writer_quote_dest: DEFAULT_PUBKEY,
            size: 0,
        }
    }
}

impl Default for instruction::InitSerumMarket {
    fn default() -> Self {
        Self {
            // valid values
            program_id: id(),
            system_program: system_program::id(),
            token_program: spl_token::id(),
            rent: sysvar::rent::id(),
            trailing_accounts: Vec::new(),

            // invalid values
            user_authority: DEFAULT_PUBKEY,
            option_market: DEFAULT_PUBKEY,
            serum_market: DEFAULT_PUBKEY,
            dex_program: DEFAULT_PUBKEY,
            pc_mint: DEFAULT_PUBKEY,
            option_mint: DEFAULT_PUBKEY,
            request_queue: DEFAULT_PUBKEY,
            event_queue: DEFAULT_PUBKEY,
            bids: DEFAULT_PUBKEY,
            asks: DEFAULT_PUBKEY,
            coin_vault: DEFAULT_PUBKEY,
            pc_vault: DEFAULT_PUBKEY,
            vault_signer: DEFAULT_PUBKEY,
            market_authority: DEFAULT_PUBKEY,
            market_space: 0,
            vault_signer_nonce: 0,
            coin_lot_size: 0,
            pc_lot_size: 0,
            pc_dust_threshold: 0,
        }
    }
}

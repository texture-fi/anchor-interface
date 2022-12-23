#![doc = gen_crate_docs!()]

// We can generate code in-place
#[cfg(not(feature = "gen-file"))]
anchor_interface_gen::program!(
    idl = "whirlpool.json",
    zero_copy(TickArray, Tick),
    packed(TickArray, Tick)
);

// Or we can generate rust-file and then include or import that (easier to debug)
#[cfg(feature = "gen-file")]
anchor_interface_gen::program!(
    idl = "whirlpool.json",
    zero_copy(TickArray, Tick),
    packed(TickArray, Tick),
    out_file = "src/_gen_.rs"
);
#[cfg(feature = "gen-file")]
mod _gen_;
#[cfg(feature = "gen-file")]
pub use _gen_::*;

solana_program::declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

pub mod pda;

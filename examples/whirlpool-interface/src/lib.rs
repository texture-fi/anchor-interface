#![doc = gen_crate_docs!()]

// We can generate code in-place
#[cfg(not(feature = "gen-file"))]
anchor_interface_gen::program!(
    idl = "whirlpool.json",
    zero_copy(TickArray, Tick),
    packed(TickArray, Tick)
);

// Or we can generate included rust-file (easier to debug)
#[cfg(feature = "gen-file")]
anchor_interface_gen::program!(
    out_dir = "src",
    out_mod = "_gen_",
    idl = "whirlpool.json",
    zero_copy(TickArray, Tick),
    packed(TickArray, Tick)
);

solana_program::declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

pub mod pda;

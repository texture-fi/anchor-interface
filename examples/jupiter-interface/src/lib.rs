#![doc = gen_crate_docs!()]

// We can generate code in-place
#[cfg(not(feature = "gen-file"))]
anchor_interface_gen::program!(idl = "jupiter.json");

// Or we can generate included rust-file (easier to debug)
#[cfg(feature = "gen-file")]
anchor_interface_gen::program!(out_dir = "src", out_mod = "_gen_", idl = "jupiter.json",);

solana_program::declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

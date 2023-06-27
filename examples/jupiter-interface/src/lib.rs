#![doc = gen_crate_docs!()]

// We can generate code in-place
#[cfg(not(feature = "gen-file"))]
anchor_interface_gen::program!(idl = "jupiter.json");

// Or we can generate rust-file with build script and then include or import that (easier to debug)
#[cfg(feature = "gen-file")]
mod _gen_;
#[cfg(feature = "gen-file")]
pub use _gen_::*;

solana_program::declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");

#![doc = gen_crate_docs!()]

// We can generate code in-place
#[cfg(not(feature = "gen-file"))]
anchor_interface_gen::program!(idl = "ratex.json", with_borsh(Order));

// Or we can generate included rust-file (easier to debug)
#[cfg(feature = "gen-file")]
anchor_interface_gen::program!(
    out_dir = "src",
    out_mod = "_gen_",
    idl = "ratex.json",
    with_borsh(Order),
    attr(
        names(MarketStatus, EpochUpdateStatus, MarginType, MarketType),
        attr(repr(u8)),
        attr(derive(num_enum::TryFromPrimitive)),
    ),
);

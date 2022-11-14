use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub use anchor_syn::idl;

pub mod common;
pub mod macros;

pub mod accounts;
pub mod errors;
pub mod exports;
pub mod instructions;

pub fn load_idl<P: AsRef<Path>>(path: P) -> idl::Idl {
    let path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join(path);
    let idl = fs::read_to_string(&path).expect("IDL path");
    serde_json::from_str(&idl).expect("IDL data")
}

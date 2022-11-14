use anchor_syn::idl::Idl;
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen(_: &Idl) -> TokenStream {
    quote! {
        pub use anchor_interface::prelude::*;
    }
}

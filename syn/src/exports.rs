use proc_macro2::TokenStream;
use quote::quote;

use crate::Generator;

impl Generator {
    pub fn gen_exports(&self) -> TokenStream {
        quote! {
            pub use anchor_interface::prelude::*;
        }
    }
}

use proc_macro2::TokenStream;
use quote::quote;

use crate::{common::typedef_gen, Generator};

impl Generator {
    pub fn gen_types(&self) -> TokenStream {
        let types = self
            .idl
            .types
            .iter()
            .map(|ty| typedef_gen(&self.idl.types, &self.struct_opts, ty).0);
        quote!(#(#types)*)
    }
}

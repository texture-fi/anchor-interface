use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;

use anchor_interface_syn::{Generator, GeneratorOptions};

#[proc_macro]
pub fn program(input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(input.into()) {
        Ok(v) => v,
        Err(e) => {
            return darling::Error::from(e).write_errors().into();
        }
    };
    let opts = GeneratorOptions::from_list(&attr_args).expect("parse options");
    Generator::from(opts).generate().into()
}

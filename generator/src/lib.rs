use darling::FromMeta;
use syn::parse_macro_input;

use anchor_interface_syn::{Generator, GeneratorOptions};

#[proc_macro]
pub fn program(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let generator = {
        let attr_args = parse_macro_input!(input as syn::AttributeArgs);
        let opts = GeneratorOptions::from_list(&attr_args).expect("parse options");
        Generator::from(&opts)
    };
    generator.gen_program().into()
}

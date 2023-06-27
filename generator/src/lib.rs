use darling::FromMeta;
use syn::parse_macro_input;

use anchor_interface_syn::{Generator, GeneratorOptions};

macro_rules! parse_generator {
    ($input:ident) => {{
        let attr_args = parse_macro_input!($input as syn::AttributeArgs);
        let opts = GeneratorOptions::from_list(&attr_args).expect("parse options");
        Generator::from(&opts)
    }};
}

#[proc_macro]
pub fn program(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let generator = parse_generator!(input);
    generator.gen_program_stream().into()
}

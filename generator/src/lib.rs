use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitStr};

use anchor_interface_syn::{accounts, errors, exports, idl::Idl, instructions, load_idl, macros};

fn mod_gen<G>(idl: &Idl, name: &Ident, gen: G) -> TokenStream
where
    G: FnOnce(&Idl) -> TokenStream,
{
    let stream = gen(idl);
    if stream.is_empty() {
        quote!()
    } else {
        quote! {
            pub mod #name {
                #stream
            }
        }
    }
}

#[proc_macro]
pub fn program(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let idl = {
        let idl_path = parse_macro_input!(input as LitStr).value();
        load_idl(&idl_path)
    };

    let macros = macros::gen(&idl);
    let exports = exports::gen(&idl);

    let instruction_mod = mod_gen(&idl, &format_ident!("instruction"), instructions::gen);
    let state_mod = mod_gen(&idl, &format_ident!("state"), accounts::gen);
    let error_mod = mod_gen(&idl, &format_ident!("error"), errors::gen);

    quote! {
        #macros
        #exports
        #instruction_mod
        #state_mod
        #error_mod
    }
    .into()
}

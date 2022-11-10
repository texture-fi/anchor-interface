use anchor_syn::idl::Idl;
use heck::ToTitleCase;
use proc_macro2::TokenStream;
use quote::quote;

pub fn gen(idl: &Idl) -> TokenStream {
    let program_name = &idl.name;
    let program_title = program_name.replace('_', " ").to_title_case();
    let program_version = &idl.version;

    quote! {
        macro_rules! gen_crate_docs {
            () => {
                concat!(
                    " ", #program_title, " v", #program_version,
                    " program interface generated from Anchor IDL."
                )
            }
        }
    }
}

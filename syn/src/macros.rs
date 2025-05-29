use heck::ToTitleCase;
use proc_macro2::TokenStream;
use quote::quote;

use crate::Generator;

impl Generator {
    pub fn gen_macros(&self) -> TokenStream {
        let program_name = &self.idl.metadata.name;
        let program_title = program_name.replace('_', " ").to_title_case();
        let program_version = &self.idl.metadata.version;

        quote! {
            macro_rules! gen_crate_docs {
                () => {
                    concat!(
                        " ", #program_title, " v", #program_version,
                        " program interface generated from Anchor IDL."
                    )
                }
            }
            pub(crate) use gen_crate_docs;
        }
    }
}

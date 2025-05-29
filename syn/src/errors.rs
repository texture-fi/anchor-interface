use heck::{ToTitleCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{common::item_gen, Generator};

impl Generator {
    pub fn gen_errors(&self) -> TokenStream {
        let error_type = format!("{}Error", self.idl.metadata.name.to_upper_camel_case());
        let error_name = format_ident!("{}", &error_type);

        let errors = self.idl.errors.iter().map(|err| {
            let name = item_gen(&err.name);
            let code = err.code;
            let msg = if let Some(msg) = &err.msg {
                msg.clone()
            } else {
                err.name.to_title_case()
            };
            quote! {
                #[error(#msg)]
                #name = #code
            }
        });

        quote! {
            use ::num_derive::FromPrimitive;
            use ::thiserror::Error;
            use ::solana_program::{decode_error::DecodeError, program_error::ProgramError};

            #[derive(Error, Clone, Copy, Debug, FromPrimitive, PartialEq, Eq)]
            #[repr(u32)]
            pub enum #error_name {
                #(#errors,)*
            }

            impl DecodeError<#error_name> for #error_name {
                fn type_of() -> &'static str {
                    #error_type
                }
            }

            impl From<#error_name> for ProgramError {
                fn from(err: #error_name) -> Self {
                    Self::Custom(err as u32)
                }
            }
        }
    }
}

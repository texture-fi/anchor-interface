use anchor_syn::idl::Idl;
use heck::{ToTitleCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

use crate::common::item_gen;

pub fn gen(idl: &Idl) -> TokenStream {
    let error_type = format!("{}Error", idl.name.to_upper_camel_case());
    let error_name = format_ident!("{}", &error_type);

    let errors = if let Some(errors) = &idl.errors {
        errors
    } else {
        return quote!();
    };

    let errors = errors.iter().map(|err| {
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
        use num_derive::FromPrimitive;
        use solana_program::{decode_error::DecodeError, program_error::ProgramError};
        use thiserror::Error;

        #[derive(Error, Clone, Copy, Debug, FromPrimitive)]
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

use anchor_syn::codegen::program::common::sighash;
use anchor_syn::idl::{Idl, IdlTypeDefinitionTy};
use proc_macro2::TokenStream;
use quote::quote;

use crate::common::{docs_gen, item_gen, pub_fields_decl_gen};

pub fn gen(idl: &Idl) -> TokenStream {
    let accounts = idl.accounts.iter().map(|acc| {
        let docs = docs_gen(&acc.docs);
        let name = item_gen(&acc.name);
        let namespace = "account"; // TODO: add other namespaces support
        let discriminator = sighash(namespace, &name.to_string());
        let impl_discriminator = quote!(
            impl anchor_interface::Account for #name {
                fn discriminator() -> &'static [u8; 8] {
                    &[#(#discriminator),*]
                }
            }
        );
        match &acc.ty {
            IdlTypeDefinitionTy::Struct { fields } => {
                let fields = pub_fields_decl_gen(fields, &acc.name);
                quote! {
                    #docs
                    #[derive(
                        Debug,
                        Clone,
                        borsh::BorshDeserialize,
                        borsh::BorshSerialize,
                    )]
                    pub struct #name {
                        #fields
                    }
                    #impl_discriminator
                }
            }
            IdlTypeDefinitionTy::Enum { variants: _ } => {
                unimplemented!("enum accounts not supported yet")
                // TODO: check and finish
                // let variants = variants.iter().map(|var| {
                //     let docs = docs_gen(&var.docs);
                //     let name = format_ident!("{}", var.name);
                //     let fields = match &var.fields {
                //         Some(EnumFields::Named(fields)) => {
                //             let fields = fields_gen(fields, false, &format!(
                //                 "account {} variant {}",
                //                 &acc.name, &var.name
                //             ));
                //             quote!({ #fields })
                //         }
                //         Some(EnumFields::Tuple(types)) => {
                //             let types = types_gen(types, );
                //             quote!(( #(#types),* ))
                //         }
                //         None => quote!(),
                //     };
                //     quote! {
                //         #docs
                //         #name #fields
                //     }
                // });
                // quote! {
                //     #docs
                //     #[derive(
                //         Debug,
                //         borsh::BorshDeserialize,
                //         borsh::BorshSerialize,
                //     )]
                //     pub enum #name {
                //         #(#variants),*
                //     }
                //     #impl_discriminator
                // }
            }
        }
    });
    quote! {
        #(#accounts)*
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminator() {
        let account_name = "WhirlpoolsConfig";
        let namespace = "account";
        let sighash = sighash(namespace, account_name);
        let expected = [157, 20, 49, 224, 217, 87, 193, 254];
        assert_eq!(sighash, expected);
    }
}

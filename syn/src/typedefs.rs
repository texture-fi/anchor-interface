use std::collections::BTreeMap;

use anchor_syn::idl::{EnumFields, IdlTypeDefinition, IdlTypeDefinitionTy};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::common::*;
use crate::{Generator, StructOpts};

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

pub fn typedef_gen(
    defs: &[IdlTypeDefinition],
    opts: &BTreeMap<Ident, StructOpts>,
    ty: &IdlTypeDefinition,
) -> (TokenStream, Ident, StructOpts) {
    let docs = docs_gen(&ty.docs);
    let name = item_gen(&ty.name);
    let opts = opts.get(&name).copied().unwrap_or_default();
    let derive_debug = if opts.packed {
        // NOTE: pointers to the fields of the boxed structure are not aligned,
        // so they need to be copied to a local variable,
        // which can lead to stack overflow of a BPF-program
        quote!(#[cfg_attr(not(target_arch = "bpf"), derive(Debug))])
    } else {
        quote!(#[derive(Debug)])
    };
    let typedef = match &ty.ty {
        IdlTypeDefinitionTy::Struct { fields } => {
            let (derive, attributes) =
                get_field_list_properties(defs, fields).derive_and_attrs(&opts);
            let fields = pub_fields_decl_gen(fields, &opts);
            quote! {
                #docs
                #[derive(#(#derive),*)]
                #derive_debug
                #attributes
                pub struct #name {
                    #fields
                }
            }
        }
        IdlTypeDefinitionTy::Enum { variants } => {
            let (derive, attributes) = get_variant_list_properties(defs, variants)
                .whitout_default()
                .derive_and_attrs(&opts);
            let variants = variants.iter().map(|var| {
                let name = item_gen(&var.name);
                let fields = match &var.fields {
                    Some(EnumFields::Named(fields)) => {
                        let fields = fields_decl_gen(fields, &opts);
                        quote!({ #fields })
                    }
                    Some(EnumFields::Tuple(types)) => {
                        let types = types_gen(types, &opts);
                        quote!(( #types ))
                    }
                    None => quote!(),
                };
                quote! {
                    #docs
                    #name #fields
                }
            });
            quote! {
                #docs
                #[derive(#(#derive),*)]
                #attributes
                pub enum #name {
                    #(#variants),*
                }
            }
        }
    };
    (typedef, name, opts)
}

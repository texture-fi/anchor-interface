use std::collections::BTreeMap;

use anchor_lang_idl::types::{IdlDefinedFields, IdlSerialization, IdlTypeDef, IdlTypeDefTy};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::common::*;
use crate::{Generator, TypeDefOpts};

impl Generator {
    pub fn gen_types(&self) -> TokenStream {
        let types = self
            .idl
            .types
            .iter()
            .filter(|ty| !self.account_type_idx_by_name.contains_key(&ty.name))
            .map(|ty| typedef_gen(&self.idl.types, &self.typedef_opts, ty).0);
        quote!(#(#types)*)
    }
}

pub fn typedef_gen(
    defs: &[IdlTypeDef],
    opts: &BTreeMap<Ident, TypeDefOpts>,
    ty: &IdlTypeDef,
) -> (TokenStream, Ident, TypeDefOpts) {
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
        IdlTypeDefTy::Struct { fields } => {
            let (derive, attributes) = get_def_field_list_properties(defs, fields)
                .derive_and_attrs(&opts, &ty.serialization, &ty.repr);
            let fields = pub_def_fields_decl_gen(fields, &opts);
            let unsafe_bytemuck_impls =
                if matches!(ty.serialization, IdlSerialization::BytemuckUnsafe) {
                    quote! {
                        unsafe impl ::bytemuck::Pod for #name {}
                        unsafe impl ::bytemuck::Zeroable for #name {}
                    }
                } else {
                    quote!()
                };
            quote! {
                #docs
                #[derive(#(#derive),*)]
                #derive_debug
                #(#attributes)*
                pub struct #name #fields
                #unsafe_bytemuck_impls
            }
        }
        IdlTypeDefTy::Enum { variants } => {
            let (derive, attributes) = get_variant_list_properties(defs, variants)
                .whitout_default()
                .derive_and_attrs(&opts, &ty.serialization, &ty.repr);
            let variants = variants.iter().map(|var| {
                let name = item_gen(&var.name);
                let fields = match &var.fields {
                    Some(IdlDefinedFields::Named(fields)) => {
                        let fields = fields_decl_gen(fields, &opts);
                        quote!({ #fields })
                    }
                    Some(IdlDefinedFields::Tuple(types)) => {
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
                #derive_debug
                #(#attributes)*
                pub enum #name {
                    #(#variants),*
                }
            }
        }
        IdlTypeDefTy::Type { alias } => {
            let (_derive, attributes) = get_type_properties(defs, alias)
                .whitout_default()
                .derive_and_attrs(&opts, &ty.serialization, &ty.repr);
            let alias = type_gen(alias, &opts);
            quote! {
                #docs
                #(#attributes)*
                pub type #name = #alias;
            }
        }
    };
    (typedef, name, opts)
}

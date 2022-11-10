#![allow(dead_code)]

use anchor_syn::idl::{IdlField, IdlType};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, LexError, TokenStream};
use quote::{format_ident, quote};

/// Converts an [IdlType] to a [String] of the Rust representation.
pub fn to_rust_type(ty: &IdlType) -> String {
    match ty {
        IdlType::Bool => "bool".to_string(),
        IdlType::U8 => "u8".to_string(),
        IdlType::I8 => "i8".to_string(),
        IdlType::U16 => "u16".to_string(),
        IdlType::I16 => "i16".to_string(),
        IdlType::U32 => "u32".to_string(),
        IdlType::I32 => "i32".to_string(),
        IdlType::F32 => "f32".to_string(),
        IdlType::U64 => "u64".to_string(),
        IdlType::I64 => "i64".to_string(),
        IdlType::F64 => "f64".to_string(),
        IdlType::U128 => "u128".to_string(),
        IdlType::I128 => "i128".to_string(),
        IdlType::Bytes => "Vec<u8>".to_string(),
        IdlType::String => "String".to_string(),
        IdlType::PublicKey => "::solana_program::pubkey::Pubkey".to_string(),
        IdlType::Option(inner) => format!("Option<{}>", to_rust_type(inner)),
        IdlType::Vec(inner) => format!("Vec<{}>", to_rust_type(inner)),
        IdlType::Array(ty, size) => format!("[{}; {}]", to_rust_type(ty), size),
        IdlType::Defined(name) => name.to_string(),
    }
}

pub fn docs_gen(docs: &Option<Vec<String>>) -> TokenStream {
    let docs = docs.as_ref().map(|d| d.iter()).unwrap_or_else(|| [].iter());
    quote!(#(#[doc = concat!(" ", #docs)])*)
}

pub fn item_gen(name: &str) -> Ident {
    format_ident!("{}", name.to_upper_camel_case())
}

pub fn type_gen(ty: &IdlType) -> Result<TokenStream, LexError> {
    let ty = to_rust_type(ty);
    ty.parse()
}

pub fn types_gen(types: &[IdlType], err_head: &str) -> TokenStream {
    let types = types.iter().map(|ty| {
        type_gen(ty).unwrap_or_else(|err| panic!("{} type {:?}: {}", err_head, &ty, err))
    });
    quote!(#(#types),*)
}

pub struct Field {
    pub docs: TokenStream,
    pub ident: Ident,
    pub ty: TokenStream,
}
impl Field {
    pub fn parse(field: &IdlField) -> Result<Self, LexError> {
        Ok(Field {
            docs: docs_gen(&field.docs),
            ident: format_ident!("{}", field.name.to_snake_case()),
            ty: type_gen(&field.ty)?,
        })
    }

    pub fn decl_gen(&self) -> TokenStream {
        let Self { docs, ident, ty } = self;
        quote! {
            #docs
            #ident: #ty
        }
    }
    pub fn pub_decl_gen(&self) -> TokenStream {
        let Self { docs, ident, ty } = self;
        quote! {
            #docs
            pub #ident: #ty
        }
    }
}

pub fn fields_decl_gen(fields: &[IdlField], err_head: &str) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let field = Field::parse(field)
            .unwrap_or_else(|err| panic!("{} field {}: {}", err_head, &field.name, err));
        field.decl_gen()
    });
    quote!(#(#fields),*)
}
pub fn pub_fields_decl_gen(fields: &[IdlField], err_head: &str) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let field = Field::parse(field)
            .unwrap_or_else(|err| panic!("{} field {}: {}", err_head, &field.name, err));
        field.pub_decl_gen()
    });
    quote!(#(#fields),*)
}

#![allow(dead_code)]

use anchor_syn::idl::{EnumFields, IdlEnumVariant, IdlField, IdlType, IdlTypeDefinition};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::StructOpts;

#[derive(Copy, Clone, Debug, Default)]
pub struct FieldListProperties {
    pub can_copy: bool,
    pub can_derive_default: bool,
}
impl FieldListProperties {
    pub fn whitout_default(self) -> Self {
        Self {
            can_derive_default: false,
            ..self
        }
    }
    pub fn derive_and_attrs(&self, opts: &StructOpts) -> (Vec<TokenStream>, TokenStream) {
        let derive = {
            let mut vec = vec![];
            vec.push(quote!(Clone));
            if self.can_copy {
                vec.push(quote!(Copy));
            }
            if self.can_derive_default {
                vec.push(quote!(Default));
            }
            if !opts.zero_copy {
                vec.push(quote!(::borsh::BorshDeserialize));
                vec.push(quote!(::borsh::BorshSerialize));
            } else {
                vec.push(quote!(::bytemuck::Pod));
                vec.push(quote!(::bytemuck::Zeroable));
            }
            vec
        };
        let attrs = {
            let repr_c = if opts.zero_copy {
                quote!(#[repr(C)])
            } else {
                quote!()
            };
            let repr_packed = if opts.packed {
                quote!(#[repr(packed)])
            } else {
                quote!()
            };
            quote! {
                #repr_c
                #repr_packed
            }
        };
        (derive, attrs)
    }
}

pub fn get_field_list_properties(
    defs: &[IdlTypeDefinition],
    fields: &[IdlField],
) -> FieldListProperties {
    get_type_list_properties(
        defs,
        &fields.iter().map(|f| f.ty.clone()).collect::<Vec<_>>(),
    )
}

pub fn get_type_list_properties(
    defs: &[IdlTypeDefinition],
    fields: &[IdlType],
) -> FieldListProperties {
    fields.iter().fold(
        FieldListProperties {
            can_copy: true,
            can_derive_default: true,
        },
        |acc, el| {
            let inner_props = get_type_properties(defs, el);
            let can_copy = acc.can_copy && inner_props.can_copy;
            let can_derive_default = acc.can_derive_default && inner_props.can_derive_default;
            FieldListProperties {
                can_copy,
                can_derive_default,
            }
        },
    )
}

pub fn get_variant_list_properties(
    defs: &[IdlTypeDefinition],
    variants: &[IdlEnumVariant],
) -> FieldListProperties {
    variants.iter().fold(
        FieldListProperties {
            can_copy: true,
            can_derive_default: true,
        },
        |acc, el| {
            let props = match &el.fields {
                Some(EnumFields::Named(fields)) => get_field_list_properties(defs, fields),
                Some(EnumFields::Tuple(fields)) => get_type_list_properties(defs, fields),
                None => acc,
            };
            FieldListProperties {
                can_copy: acc.can_copy && props.can_copy,
                can_derive_default: acc.can_derive_default && props.can_derive_default,
            }
        },
    )
}

pub fn get_type_properties(defs: &[IdlTypeDefinition], ty: &IdlType) -> FieldListProperties {
    match ty {
        IdlType::Bool
        | IdlType::U8
        | IdlType::I8
        | IdlType::U16
        | IdlType::I16
        | IdlType::U32
        | IdlType::I32
        | IdlType::F32
        | IdlType::U64
        | IdlType::I64
        | IdlType::F64
        | IdlType::U128
        | IdlType::I128
        | IdlType::PublicKey => FieldListProperties {
            can_copy: true,
            can_derive_default: true,
        },
        IdlType::Bytes => FieldListProperties {
            can_copy: false,
            can_derive_default: false,
        },
        IdlType::String | IdlType::Vec(_) => FieldListProperties {
            can_copy: false,
            can_derive_default: true,
        },
        IdlType::Defined(inner) => {
            let def = defs.iter().find(|def| def.name == *inner).unwrap();
            match &def.ty {
                anchor_syn::idl::IdlTypeDefinitionTy::Struct { fields } => {
                    get_field_list_properties(defs, fields)
                }
                anchor_syn::idl::IdlTypeDefinitionTy::Enum { variants } => {
                    get_variant_list_properties(defs, variants).whitout_default()
                }
            }
        }
        IdlType::Option(inner) => get_type_properties(defs, inner),
        IdlType::Array(inner, len) => {
            let inner = get_type_properties(defs, inner);
            let can_derive_array_len = *len <= 32;
            FieldListProperties {
                can_copy: inner.can_copy,
                can_derive_default: can_derive_array_len && inner.can_derive_default,
            }
        }
    }
}

pub fn docs_gen(docs: &Option<Vec<String>>) -> TokenStream {
    let docs = docs.as_ref().map(|d| d.iter()).unwrap_or_else(|| [].iter());
    quote!(#(#[doc = concat!(" ", #docs)])*)
}

pub fn item_gen(name: &str) -> Ident {
    format_ident!("{}", name.to_upper_camel_case())
}

pub fn type_gen(ty: &IdlType, opts: &StructOpts) -> TokenStream {
    match ty {
        IdlType::Bool => {
            if opts.zero_copy {
                quote!(u8)
            } else {
                quote!(bool)
            }
        }
        IdlType::U8 => quote!(u8),
        IdlType::I8 => quote!(i8),
        IdlType::U16 => quote!(u16),
        IdlType::I16 => quote!(i16),
        IdlType::U32 => quote!(u32),
        IdlType::I32 => quote!(i32),
        IdlType::F32 => quote!(f32),
        IdlType::U64 => quote!(u64),
        IdlType::I64 => quote!(i64),
        IdlType::F64 => quote!(f64),
        IdlType::U128 => quote!(u128),
        IdlType::I128 => quote!(i128),
        IdlType::Bytes => quote!(Vec<u8>),
        IdlType::String => quote!(String),
        IdlType::PublicKey => quote!(::solana_program::pubkey::Pubkey),
        IdlType::Option(inner) => {
            let inner = type_gen(inner, opts);
            quote!(Option<#inner>)
        }
        IdlType::Vec(inner) => {
            let inner = type_gen(inner, opts);
            quote!(Vec<#inner>)
        }
        IdlType::Array(ty, size) => {
            let ty = type_gen(ty, opts);
            quote!([#ty; #size])
        }
        IdlType::Defined(name) => {
            let name = format_ident!("{}", name);
            quote!(#name)
        }
    }
}

pub fn types_gen(types: &[IdlType], opts: &StructOpts) -> TokenStream {
    let types = types.iter().map(|ty| type_gen(ty, opts));
    quote!(#(#types),*)
}

pub fn field_attrs(ty: &IdlType, opts: &StructOpts) -> TokenStream {
    match (opts.zero_copy, ty) {
        (false, IdlType::Array(item, size)) => {
            quote!()
        }
        _ => quote!()
    }
}

pub struct Field {
    pub docs: TokenStream,
    pub attrs: TokenStream,
    pub ident: Ident,
    pub ty: TokenStream,
}
impl Field {
    pub fn parse(field: &IdlField, opts: &StructOpts) -> Self {
        Field {
            docs: docs_gen(&field.docs),
            attrs: ,
            ident: format_ident!("{}", field.name.to_snake_case()),
            ty: type_gen(&field.ty, opts),
        }
    }

    pub fn decl_gen(&self) -> TokenStream {
        let Self { docs, attrs, ident, ty } = self;
        quote! {
            #docs
            #attrs
            #ident: #ty
        }
    }
    pub fn pub_decl_gen(&self) -> TokenStream {
        let Self { docs, attrs, ident, ty } = self;
        quote! {
            #docs
            #attrs
            pub #ident: #ty
        }
    }
}

pub fn fields_decl_gen(fields: &[IdlField], opts: &StructOpts) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let field = Field::parse(field, opts);
        field.decl_gen()
    });
    quote!(#(#fields),*)
}
pub fn pub_fields_decl_gen(fields: &[IdlField], opts: &StructOpts) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let field = Field::parse(field, opts);
        field.pub_decl_gen()
    });
    quote!(#(#fields),*)
}

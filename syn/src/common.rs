#![allow(dead_code)]

use anchor_lang_idl::types::{
    IdlArrayLen, IdlDefinedFields, IdlEnumVariant, IdlField, IdlRepr, IdlSerialization, IdlType,
    IdlTypeDef, IdlTypeDefTy,
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::TypeDefOpts;

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
    pub fn derive_and_attrs(
        &self,
        opts: &TypeDefOpts,
        serialization: &IdlSerialization,
        repr: &Option<IdlRepr>,
    ) -> (Vec<TokenStream>, Vec<TokenStream>) {
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
                if opts.with_borsh || matches!(serialization, IdlSerialization::Borsh) {
                    vec.push(quote!(::borsh::BorshDeserialize));
                    vec.push(quote!(::borsh::BorshSerialize));
                }
                match serialization {
                    IdlSerialization::Borsh => {
                        // NOTE: derives already added
                    }
                    IdlSerialization::Bytemuck => {
                        vec.push(quote!(::bytemuck::Pod));
                        vec.push(quote!(::bytemuck::Zeroable));
                    }
                    IdlSerialization::BytemuckUnsafe => {
                        // NOTE: must be implemented manually
                    }
                    IdlSerialization::Custom(ser) => {
                        todo!("serialization `{ser}` not supported yet")
                    }
                    _ => panic!("unknown serialization `{serialization:?}`"),
                }
            } else {
                if opts.with_borsh {
                    vec.push(quote!(::borsh::BorshDeserialize));
                    vec.push(quote!(::borsh::BorshSerialize));
                }
                if !matches!(serialization, IdlSerialization::BytemuckUnsafe) {
                    vec.push(quote!(::bytemuck::Pod));
                    vec.push(quote!(::bytemuck::Zeroable));
                }
            }
            vec
        };
        let attrs = {
            let mut vec = vec![];

            if opts.zero_copy || opts.packed {
                if opts.zero_copy {
                    vec.push(quote!(#[repr(C)]))
                }
                if opts.packed {
                    vec.push(quote!(#[repr(packed)]))
                }
            } else if let Some(repr) = repr {
                let repr_modifier = match repr {
                    IdlRepr::Rust(idl_repr_modifier) => Some(idl_repr_modifier),
                    IdlRepr::C(idl_repr_modifier) => {
                        vec.push(quote!(#[repr(C)]));
                        Some(idl_repr_modifier)
                    }
                    IdlRepr::Transparent => {
                        vec.push(quote!(#[repr(transparent)]));
                        None
                    }
                    _ => panic!("unknown repr `{repr:?}`"),
                };
                if let Some(repr_modifier) = repr_modifier {
                    if repr_modifier.packed {
                        let align = repr_modifier.align.iter();
                        vec.push(quote!(#[repr(packed #((#align))*)]));
                    }
                }
            } else if matches!(
                serialization,
                IdlSerialization::Bytemuck | IdlSerialization::BytemuckUnsafe
            ) {
                vec.push(quote!(#[repr(C)]))
            }

            opts.custom_attr
                .iter()
                .for_each(|attr| vec.push(quote!(#[#attr])));

            vec
        };
        (derive, attrs)
    }
}

pub fn get_def_field_list_properties(
    defs: &[IdlTypeDef],
    fields: &Option<IdlDefinedFields>,
) -> FieldListProperties {
    match fields {
        Some(IdlDefinedFields::Named(fields)) => get_field_list_properties(defs, fields),
        Some(IdlDefinedFields::Tuple(fields)) => get_type_list_properties(defs, fields),
        None => FieldListProperties {
            can_copy: true,
            can_derive_default: true,
        },
    }
}

pub fn get_field_list_properties(defs: &[IdlTypeDef], fields: &[IdlField]) -> FieldListProperties {
    get_type_list_properties(
        defs,
        &fields.iter().map(|f| f.ty.clone()).collect::<Vec<_>>(),
    )
}

pub fn get_type_list_properties(defs: &[IdlTypeDef], types: &[IdlType]) -> FieldListProperties {
    types.iter().fold(
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
    defs: &[IdlTypeDef],
    variants: &[IdlEnumVariant],
) -> FieldListProperties {
    variants.iter().fold(
        FieldListProperties {
            can_copy: true,
            can_derive_default: true,
        },
        |acc, el| {
            let props = match &el.fields {
                Some(IdlDefinedFields::Named(fields)) => get_field_list_properties(defs, fields),
                Some(IdlDefinedFields::Tuple(fields)) => get_type_list_properties(defs, fields),
                None => acc,
            };
            FieldListProperties {
                can_copy: acc.can_copy && props.can_copy,
                can_derive_default: acc.can_derive_default && props.can_derive_default,
            }
        },
    )
}

pub fn get_type_properties(defs: &[IdlTypeDef], ty: &IdlType) -> FieldListProperties {
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
        | IdlType::U256
        | IdlType::I256
        | IdlType::Pubkey => FieldListProperties {
            can_copy: true,
            can_derive_default: true,
        },
        IdlType::Bytes => FieldListProperties {
            can_copy: false,
            can_derive_default: false,
        },
        IdlType::String | IdlType::Vec(_) | IdlType::Generic(_) => FieldListProperties {
            can_copy: false,
            can_derive_default: true,
        },
        IdlType::Defined { name, generics: _ } => {
            let def = defs.iter().find(|def| &def.name == name).unwrap();
            match &def.ty {
                IdlTypeDefTy::Struct { fields } => get_def_field_list_properties(defs, fields),
                IdlTypeDefTy::Enum { variants } => {
                    get_variant_list_properties(defs, variants).whitout_default()
                }
                IdlTypeDefTy::Type { alias } => get_type_properties(defs, alias),
            }
        }
        IdlType::Option(inner) => get_type_properties(defs, inner),
        IdlType::Array(ty, len) => {
            let ty = get_type_properties(defs, ty);
            let can_derive_array_len = match len {
                IdlArrayLen::Generic(_) => false,
                IdlArrayLen::Value(len) => *len <= 32,
            };
            FieldListProperties {
                can_copy: ty.can_copy,
                can_derive_default: can_derive_array_len && ty.can_derive_default,
            }
        }
        _ => {
            panic!("variant '{ty:?}' not suppurted yet")
        }
    }
}

pub fn docs_gen(docs: &[String]) -> TokenStream {
    quote!(#(#[doc = concat!(" ", #docs)])*)
}

pub fn item_gen(name: &str) -> Ident {
    format_ident!("{}", name.to_upper_camel_case())
}

pub fn type_gen(ty: &IdlType, opts: &TypeDefOpts) -> TokenStream {
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
        IdlType::U256 => quote!(u256),
        IdlType::I256 => quote!(i256),
        IdlType::Bytes => quote!(Vec<u8>),
        IdlType::String => quote!(String),
        IdlType::Pubkey => quote!(::solana_program::pubkey::Pubkey),
        IdlType::Option(inner) => {
            let inner = type_gen(inner, opts);
            quote!(Option<#inner>)
        }
        IdlType::Vec(inner) => {
            let inner = type_gen(inner, opts);
            quote!(Vec<#inner>)
        }
        IdlType::Array(ty, size) => {
            let size = array_type_size(ty, size);
            let ty = type_gen(ty, opts);
            quote!([#ty; #size])
        }
        IdlType::Defined { name, generics } => {
            let name = item_gen(name);
            let generics = if generics.is_empty() {
                quote!()
            } else {
                todo!("defined generics not suppurted yet")
            };
            quote!(#name #generics)
        }
        IdlType::Generic(_) => {
            todo!("generic not suppurted yet")
        }
        _ => {
            panic!("variant '{ty:?}' not suppurted yet")
        }
    }
}

pub fn types_gen(types: &[IdlType], opts: &TypeDefOpts) -> TokenStream {
    let types = types.iter().map(|ty| type_gen(ty, opts));
    quote!(#(#types),*)
}

pub fn array_type_size(ty: &IdlType, size: &IdlArrayLen) -> TokenStream {
    match size {
        anchor_lang_idl::types::IdlArrayLen::Generic(str) => match str.parse() {
            Ok(out) => out,
            Err(err) => {
                panic!("parse array `{ty:?}` size as generic `{str}`: {err}")
            }
        },
        anchor_lang_idl::types::IdlArrayLen::Value(size) => quote!(#size),
    }
}
pub struct Field {
    pub docs: TokenStream,
    pub ident: Option<Ident>,
    pub ty: TokenStream,
}
impl Field {
    pub fn named(field: &IdlField, opts: &TypeDefOpts) -> Self {
        Field {
            docs: docs_gen(&field.docs),
            ident: Some(format_ident!("{}", field.name.to_snake_case())),
            ty: type_gen(&field.ty, opts),
        }
    }

    pub fn tuple(ty: &IdlType, docs: &[String], opts: &TypeDefOpts) -> Self {
        Field {
            docs: docs_gen(docs),
            ident: None,
            ty: type_gen(ty, opts),
        }
    }

    pub fn decl_gen(&self) -> TokenStream {
        let Self { docs, ident, ty } = self;
        let ident = ident.iter();
        quote! {
            #docs
            #(#ident:)* #ty
        }
    }
    pub fn pub_decl_gen(&self) -> TokenStream {
        let Self { docs, ident, ty } = self;
        let ident = ident.iter();
        quote! {
            #docs
            pub #(#ident:)* #ty
        }
    }
}

pub fn fields_decl_gen(fields: &[IdlField], opts: &TypeDefOpts) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let field = Field::named(field, opts);
        field.decl_gen()
    });
    quote!(#(#fields),*)
}
pub fn pub_fields_decl_gen(fields: &[IdlField], opts: &TypeDefOpts) -> TokenStream {
    let fields = fields.iter().map(|field| {
        let field = Field::named(field, opts);
        field.pub_decl_gen()
    });
    quote!(#(#fields),*)
}

pub fn tuple_decl_gen(types: &[IdlType], opts: &TypeDefOpts) -> TokenStream {
    let fields = types.iter().map(|ty| {
        let field = Field::tuple(ty, &[], opts);
        field.decl_gen()
    });
    quote!(#(#fields),*)
}
pub fn pub_tuple_decl_gen(types: &[IdlType], opts: &TypeDefOpts) -> TokenStream {
    let fields = types.iter().map(|ty| {
        let field = Field::tuple(ty, &[], opts);
        field.pub_decl_gen()
    });
    quote!(#(#fields),*)
}

pub fn def_fields_decl_gen(fields: &Option<IdlDefinedFields>, opts: &TypeDefOpts) -> TokenStream {
    match fields {
        Some(IdlDefinedFields::Named(fields)) => {
            let fields = fields_decl_gen(fields, opts);
            quote!({ #fields })
        }
        Some(IdlDefinedFields::Tuple(types)) => {
            let types = tuple_decl_gen(types, opts);
            quote!(( #types );)
        }
        None => quote!(()),
    }
}
pub fn pub_def_fields_decl_gen(
    fields: &Option<IdlDefinedFields>,
    opts: &TypeDefOpts,
) -> TokenStream {
    match fields {
        Some(IdlDefinedFields::Named(fields)) => {
            let fields = pub_fields_decl_gen(fields, opts);
            quote!({ #fields })
        }
        Some(IdlDefinedFields::Tuple(types)) => {
            let types = pub_tuple_decl_gen(types, opts);
            quote!(( #types );)
        }
        None => quote!(();),
    }
}

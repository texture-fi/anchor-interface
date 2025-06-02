use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
};

use anchor_lang_idl::types::{Idl, IdlRepr};
use common::item_gen;
use darling::{util::PathList, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Meta;
use typed_builder::TypedBuilder;

use rust_format::{Formatter, PrettyPlease};

pub mod common;
pub mod macros;

pub mod accounts;
pub mod errors;
pub mod exports;
pub mod instructions;
pub mod typedefs;

#[derive(Default, FromMeta, TypedBuilder)]
pub struct GeneratorOptions {
    /// Module name.
    pub out_mod: Option<String>,
    /// Path to the out module directory.
    pub out_dir: Option<String>,

    /// Path to the IDL.
    #[builder(setter(into))]
    pub idl: String,

    /// List of structs with implemented `borsh` always.
    #[builder(default, setter(transform = |list: &[&str]| {
        Some(parse_path_list(list))
    }))]
    pub with_borsh: Option<PathList>,

    /// List of zero-copy structs.
    #[builder(default, setter(transform = |list: &[&str]| {
        Some(parse_path_list(list))
    }))]
    pub zero_copy: Option<PathList>,

    /// List of `repr(packed)` structs.
    #[builder(default, setter(transform = |list: &[&str]| {
        Some(parse_path_list(list))
    }))]
    pub packed: Option<PathList>,

    /// List of additional attributes.
    #[darling(multiple)]
    pub attr: Vec<AttrOptions>,
}

#[derive(FromMeta, TypedBuilder)]
pub struct AttrOptions {
    #[darling(multiple, with = parse_submeta)]
    pub attr: Vec<Meta>,
    #[builder(default, setter(transform = |list: &[&str]| {
        parse_path_list(list)
    }))]
    pub names: PathList,
}

pub fn parse_submeta(meta: &Meta) -> darling::Result<Meta> {
    match meta {
        Meta::Path(_) => Err(darling::Error::unsupported_format("path").with_span(meta)),
        Meta::List(meta_list) => syn::parse2(meta_list.tokens.clone()).map_err(Into::into),
        Meta::NameValue(_) => Err(darling::Error::unsupported_format("name-value").with_span(meta)),
    }
}

pub fn parse_path_list(list: &[&str]) -> PathList {
    let list: Vec<syn::Path> = list
        .iter()
        .map(|&path| {
            let leading_colon = path
                .starts_with("..")
                .then_some(<syn::Token![::]>::default());
            let idents = path
                .split("::")
                .map(|ident| syn::PathSegment::from(format_ident!("{}", ident)));

            syn::Path {
                leading_colon,
                segments: syn::punctuated::Punctuated::from_iter(idents),
            }
        })
        .collect();
    PathList::new(list)
}

#[derive(Clone, Default)]
pub struct TypeDefOpts {
    pub with_borsh: bool,
    pub packed: bool,
    pub zero_copy: bool,
    pub custom_attr: Vec<Meta>,
}

pub struct Generator {
    pub cargo_manifest_dir: String,
    pub out_dir: Option<String>,
    pub out_mod: Option<String>,
    pub idl_file: String,
    pub idl: Idl,
    pub typedef_opts: BTreeMap<Ident, TypeDefOpts>,
    pub account_type_idx_by_name: BTreeMap<String, usize>,
}

impl From<GeneratorOptions> for Generator {
    fn from(opt: GeneratorOptions) -> Self {
        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let idl_path = PathBuf::from(cargo_manifest_dir.clone()).join(&opt.idl);
        let idl = load_idl(idl_path);

        let mut typedef_opts = BTreeMap::new();

        let manually_with_borsh = pathlist_to_idents(opt.with_borsh.as_ref());
        let manually_zero_copy = pathlist_to_idents(opt.zero_copy.as_ref());
        let manually_packed = pathlist_to_idents(opt.packed.as_ref());
        let manually_repr = opt.attr.iter().fold(HashMap::new(), |mut out, attr_opts| {
            let names = pathlist_to_idents(Some(&attr_opts.names));
            names.iter().for_each(|&name| {
                attr_opts.attr.iter().for_each(|attr| {
                    out.entry(name)
                        .and_modify(|reprs: &mut Vec<Meta>| {
                            reprs.push(attr.clone());
                        })
                        .or_insert(vec![attr.clone()]);
                })
            });
            out
        });
        idl.types.iter().for_each(|ty| {
            let name = item_gen(&ty.name);
            let zero_copy = match &ty.serialization {
                anchor_lang_idl::types::IdlSerialization::Borsh => false,
                anchor_lang_idl::types::IdlSerialization::Bytemuck
                | anchor_lang_idl::types::IdlSerialization::BytemuckUnsafe => true,
                anchor_lang_idl::types::IdlSerialization::Custom(custom) => {
                    todo!("serialization `{custom}` not supported yet")
                }
                _ => panic!("unknown serialization `{:?}`", ty.serialization),
            };
            let packed = match &ty.repr {
                Some(IdlRepr::C(modifier) | IdlRepr::Rust(modifier)) => modifier.packed,
                _ => false,
            };
            typedef_opts.insert(
                name.clone(),
                TypeDefOpts {
                    with_borsh: manually_with_borsh.contains(&name),
                    packed: manually_packed.contains(&name) || packed,
                    zero_copy: manually_zero_copy.contains(&name) || zero_copy,
                    custom_attr: manually_repr.get(&name).cloned().unwrap_or_default(),
                },
            );
        });

        let account_names: BTreeSet<_> = idl.accounts.iter().map(|acc| acc.name.clone()).collect();

        let account_type_idx_by_name = idl
            .types
            .iter()
            .enumerate()
            .filter(|(_, ty)| account_names.contains(&ty.name))
            .map(|(idx, ty)| (ty.name.clone(), idx))
            .collect();

        Generator {
            cargo_manifest_dir,
            out_dir: opt.out_dir,
            out_mod: opt.out_mod,
            idl_file: opt.idl.clone(),
            idl,
            typedef_opts,
            account_type_idx_by_name,
        }
    }
}

impl Generator {
    pub fn generate(&self) -> TokenStream {
        let stream = self.gen_program_stream();

        if self.out_dir.is_some() {
            let (rs_mod_ident, out_file_full) = self.write_stream_to_file(stream);

            quote! {
                #[rustfmt::skip]
                #[path = #out_file_full]
                mod #rs_mod_ident;
                pub use #rs_mod_ident::*;
            }
        } else {
            stream
        }
    }

    pub fn gen_program_file(&self) {
        self.write_stream_to_file(self.gen_program_stream());
    }

    pub fn gen_program_stream(&self) -> TokenStream {
        let macros = self.gen_macros();
        let exports = self.gen_exports();

        let types = self.mod_gen(&format_ident!("types"), Self::gen_types, false, false);
        let instruction_mod = self.mod_gen(
            &format_ident!("instruction"),
            Self::gen_instructions,
            true,
            true,
        );
        let state_mod = self.mod_gen(&format_ident!("state"), Self::gen_accounts, true, false);
        let error_mod = self.mod_gen(&format_ident!("error"), Self::gen_errors, true, false);

        quote! {
            #macros
            #exports
            #instruction_mod
            #types
            #state_mod
            #error_mod
        }
    }

    fn write_stream_to_file(&self, stream: TokenStream) -> (Ident, String) {
        let out_dir = self.out_dir.as_ref().expect("out dir not set");

        let rs_mod_name = self.out_mod.as_deref().unwrap_or("_gen_");
        let rs_mod_ident = format_ident!("{rs_mod_name}");

        let out_dir_full = format!("{}/{out_dir}", self.cargo_manifest_dir);
        let out_file_full = format!("{out_dir_full}/{rs_mod_name}.rs");

        let raw = stream.to_string();
        let new = match PrettyPlease::default().format_tokens(stream) {
            Ok(formated) => formated,
            Err(err) => format!("compile_error!(\"{err}\");\n\n\n{raw}"),
        }
        .into_bytes();

        let already_generated = fs::read(&out_file_full)
            .map(|old| new == old)
            .unwrap_or_default();

        if !already_generated {
            std::fs::create_dir_all(&out_dir_full).expect("create out dir");
            fs::write(&out_file_full, new).unwrap();
        }

        (rs_mod_ident, out_file_full)
    }

    fn mod_gen<G>(&self, name: &Ident, gen: G, need_types: bool, need_state: bool) -> TokenStream
    where
        G: FnOnce(&Self) -> TokenStream,
    {
        let stream = gen(self);
        if stream.is_empty() {
            quote!()
        } else {
            let types_import = if self.idl.types.is_empty() || !need_types {
                quote!()
            } else {
                quote! {
                    #[allow(unused_imports)]
                    use super::types::*;
                }
            };
            let state_import = if self.idl.accounts.is_empty() || !need_state {
                quote!()
            } else {
                quote! {
                    #[allow(unused_imports)]
                    use super::state::*;
                }
            };
            quote! {
                pub mod #name {
                    #types_import
                    #state_import
                    #stream
                }
            }
        }
    }
}

fn pathlist_to_idents(list: Option<&PathList>) -> HashSet<&Ident> {
    list.map(|el| el.iter().map(|el| el.get_ident().unwrap()).collect())
        .unwrap_or_default()
}

pub fn load_idl<P: AsRef<Path>>(path: P) -> Idl {
    let path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join(path);
    let idl = fs::read(path).expect("IDL path");
    anchor_lang_idl::convert::convert_idl(&idl).expect("IDL data")
}

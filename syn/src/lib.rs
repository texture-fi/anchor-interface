use std::{
    collections::{BTreeMap, HashSet},
    env, fs,
    path::{Path, PathBuf},
};

use darling::{util::PathList, FromMeta};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use typed_builder::TypedBuilder;

pub use anchor_syn::idl;
use rust_format::{Formatter, RustFmt};

pub mod common;
pub mod macros;

pub mod accounts;
pub mod errors;
pub mod exports;
pub mod instructions;
pub mod typedefs;

#[derive(Default, FromMeta, TypedBuilder)]
pub struct GeneratorOptions {
    /// Path to the IDL.
    #[builder(setter(into))]
    pub idl: String,

    /// List of zero copy structs.
    #[builder(default, setter(transform = |list: &[&str]| {
        Some(parse_path_list(list))
    }))]
    pub zero_copy: Option<PathList>,
    /// List of `repr(packed)` structs.
    #[builder(default, setter(transform = |list: &[&str]| {
        Some(parse_path_list(list))
    }))]
    pub packed: Option<PathList>,
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

#[derive(Clone, Copy, Default)]
pub struct StructOpts {
    pub packed: bool,
    pub zero_copy: bool,
}

pub struct Generator {
    pub cargo_manifest_dir: String,
    pub idl_file: String,
    pub idl: idl::Idl,
    pub struct_opts: BTreeMap<Ident, StructOpts>,
}

impl From<&GeneratorOptions> for Generator {
    fn from(opt: &GeneratorOptions) -> Self {
        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

        let idl_path = PathBuf::from(cargo_manifest_dir.clone()).join(&opt.idl);
        let idl_contents = fs::read_to_string(idl_path).unwrap();
        let idl: anchor_syn::idl::Idl = serde_json::from_str(&idl_contents).unwrap();

        let zero_copy = pathlist_to_idents(opt.zero_copy.as_ref());
        let packed = pathlist_to_idents(opt.packed.as_ref());

        let mut struct_opts = BTreeMap::new();
        let all_structs: HashSet<&&Ident> = zero_copy.union(&packed).collect::<HashSet<_>>();
        all_structs.into_iter().for_each(|&name| {
            struct_opts.insert(
                name.clone(),
                StructOpts {
                    zero_copy: zero_copy.contains(name),
                    packed: packed.contains(name),
                },
            );
        });

        Generator {
            cargo_manifest_dir,
            idl_file: opt.idl.clone(),
            idl,
            struct_opts,
        }
    }
}

impl Generator {
    pub fn gen_program_stream(&self) -> TokenStream {
        let macros = self.gen_macros();
        let exports = self.gen_exports();

        let types = self.mod_gen(&format_ident!("types"), Self::gen_types, false);
        let instruction_mod =
            self.mod_gen(&format_ident!("instruction"), Self::gen_instructions, true);
        let state_mod = self.mod_gen(&format_ident!("state"), Self::gen_accounts, true);
        let error_mod = self.mod_gen(&format_ident!("error"), Self::gen_errors, true);

        quote! {
            #macros
            #exports
            #instruction_mod
            #types
            #state_mod
            #error_mod
        }
    }

    fn write_stream_to_file(&self, stream: TokenStream, out_file: impl AsRef<Path>) {
        let out_file = PathBuf::from(&self.cargo_manifest_dir).join(out_file);

        let new = RustFmt::default()
            .format_str(stream.to_string())
            .unwrap()
            .into_bytes();

        if let Ok(old) = fs::read(&out_file) {
            if new == old {
                return;
            }
        }

        fs::write(out_file, new).unwrap();
    }

    pub fn gen_program_file(&self, out: impl AsRef<Path>) {
        self.write_stream_to_file(self.gen_program_stream(), out)
    }

    fn mod_gen<G>(&self, name: &Ident, gen: G, need_types: bool) -> TokenStream
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
            quote! {
                pub mod #name {
                    #types_import
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

pub fn load_idl<P: AsRef<Path>>(path: P) -> idl::Idl {
    let path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join(path);
    let idl = fs::read_to_string(path).expect("IDL path");
    serde_json::from_str(&idl).expect("IDL data")
}

use anchor_lang_idl::types::{IdlInstruction, IdlInstructionAccount, IdlInstructionAccountItem};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::common::{docs_gen, item_gen, Field};
use crate::Generator;

impl Generator {
    pub fn gen_instructions(&self) -> TokenStream {
        let master_enum_name = item_gen(&format!(
            "{}Instruction",
            self.idl.metadata.name.to_upper_camel_case()
        ));
        let ixs: Vec<_> = self.idl.instructions.iter().map(Instruction::new).collect();
        let master_enum = master_enum_gen(&master_enum_name, &ixs);
        let ix_builders_and_parsers = ix_builders_and_parsers_gen(&master_enum_name, &ixs);
        quote! {
            #master_enum
            #ix_builders_and_parsers
        }
    }
}

struct Instruction<'a> {
    pub idl: &'a IdlInstruction,
    pub ident: Ident,
    pub args: Vec<Field>,

    pub borsh_deser_ident: Ident,
}
impl<'a> Instruction<'a> {
    pub fn new(ix: &'a IdlInstruction) -> Self {
        let ident = item_gen(&ix.name);
        let borsh_deser_ident = format_ident!("{}Deserializer", ident);
        Self {
            idl: ix,
            ident,
            args: ix
                .args
                .iter()
                .map(|arg| Field::named(arg, &Default::default()))
                .collect(),
            borsh_deser_ident,
        }
    }
}

fn master_enum_gen(master_enum_name: &Ident, ixs: &[Instruction<'_>]) -> TokenStream {
    let ixs_decl = ixs.iter().map(|ix| {
        let docs = if !ix.idl.docs.is_empty() {
            docs_gen(&ix.idl.docs)
        } else {
            let head = format!(" {}", ix.idl.name.to_title_case());
            quote!(#[doc = #head])
        };
        let acc_docs = acc_docs_gen(&ix.idl.accounts);
        let name = &ix.ident;
        let args = if ix.idl.args.is_empty() {
            quote!()
        } else {
            let args = ix.args.iter().map(Field::decl_gen);
            quote!({ #(#args),* })
        };
        quote! {
            #docs
            #[doc = " "]
            #acc_docs
            #name #args
        }
    });

    let discriminator_matches = ixs.iter().map(|ix| {
        let name = &ix.ident;
        let args = if ix.args.is_empty() {
            quote!()
        } else {
            quote!({ .. })
        };
        let discriminator = &ix.idl.discriminator;
        quote!( Self::#name #args => &[#(#discriminator),*], )
    });

    let borsh_serialize_matches = ixs.iter().map(|ix| {
        let name = &ix.ident;
        let (args, serialize_args) = if ix.args.is_empty() {
            (quote!(), quote!())
        } else {
            let args = ix.args.iter().map(|arg| &arg.ident);
            let serialize_args = ix.args.iter().map(|arg| {
                let name = &arg.ident;
                quote!(::borsh::BorshSerialize::serialize(#name, writer)?;)
            });
            (quote!({ #(#args),* }), quote!(#(#serialize_args)*))
        };
        quote!(Self::#name #args => { #serialize_args })
    });

    let borsh_deserializers = ixs.iter().map(|ix| -> TokenStream {
        let name = &ix.ident;
        let helper_name = &ix.borsh_deser_ident;
        let args = ix.args.iter().map(|arg| &arg.ident);
        quote! {
            struct #helper_name ( #master_enum_name );
            impl ::borsh::de::BorshDeserialize for #helper_name {
                fn deserialize_reader<R: std::io::prelude::Read>(_reader: &mut R) -> std::io::Result<Self> {
                    Ok(Self ( #master_enum_name::#name {
                        #(#args: ::borsh::BorshDeserialize::deserialize_reader(_reader)?,)*
                    }))}
            }
            impl From<#helper_name> for #master_enum_name {
                fn from(helper: #helper_name) -> #master_enum_name {
                    helper.0
                }
            }
        }
    });

    let unpack_matches = ixs.iter().map(|ix| {
        let discriminator = &ix.idl.discriminator;
        let helper_name = &ix.borsh_deser_ident;
        quote!([#(#discriminator),*] => {
            #helper_name::deserialize(&mut ix_data)?.into()
        })
    });

    quote! {
        #[derive(Debug)]
        pub enum #master_enum_name {
            #(#ixs_decl),*
        }
        impl #master_enum_name {
            pub fn discriminator(&self) -> &'static [u8; 8] {
                match self {
                    #(#discriminator_matches)*
                }
            }
            pub fn pack(self) -> Vec<u8> {let mut out = Vec::new();
                out.extend(self.discriminator());

                let data = ::borsh::to_vec(&self).unwrap();
                out.extend(data);

                out
            }
            pub fn unpack(data: &[u8]) -> ::std::io::Result<Self> {
                use ::borsh::BorshDeserialize;

                let (discriminator, mut ix_data) = data.split_at(8);

                Ok(match discriminator {
                    #(#unpack_matches)*
                    _ => return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid discriminator",
                    )),
                })
            }
        }

        impl ::borsh::BorshSerialize for #master_enum_name {
            fn serialize<W: ::borsh::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::io::Error> {
                match self {
                    #(#borsh_serialize_matches)*
                }
                Ok(())
            }
        }

        #(#borsh_deserializers)*
    }
}

fn acc_docs(acc: &IdlInstructionAccount, idx: &mut usize) -> String {
    let out = format!(
        " {}. `[{}{}{}]` {}",
        idx,
        if acc.signer { "signer" } else { "" },
        if acc.signer && acc.writable { ", " } else { "" },
        if acc.writable { "writable" } else { "" },
        acc.name.to_title_case().to_lowercase(),
    );
    *idx += 1;
    out
}
fn acc_item_docs(acc: &IdlInstructionAccountItem, idx: &mut usize) -> Vec<String> {
    let mut out = Vec::new();
    match acc {
        IdlInstructionAccountItem::Single(acc) => {
            out.push(acc_docs(acc, idx));
        }
        IdlInstructionAccountItem::Composite(accs) => {
            accs.accounts.iter().for_each(|acc| {
                out.extend(acc_item_docs(acc, idx));
            });
        }
    }
    out
}
fn acc_docs_gen(accounts: &[IdlInstructionAccountItem]) -> TokenStream {
    if accounts.is_empty() {
        return quote!();
    }

    let mut idx = 0;
    let mut docs = Vec::new();
    accounts.iter().for_each(|acc| {
        docs.extend(acc_item_docs(acc, &mut idx));
    });
    quote! {
        #[doc = " Accounts expected by this instruction:"]
        #(#[doc = #docs])*
    }
}

fn acc_name(acc: &IdlInstructionAccount, upper: bool) -> TokenStream {
    let name = format_ident!(
        "{}",
        if upper {
            acc.name.to_shouty_snake_case()
        } else {
            acc.name.to_snake_case()
        }
    );
    quote!(#name)
}
fn acc_item_name(acc: &IdlInstructionAccountItem, upper: bool) -> Vec<TokenStream> {
    let mut out = Vec::new();
    match acc {
        IdlInstructionAccountItem::Single(acc) => out.push(acc_name(acc, upper)),
        IdlInstructionAccountItem::Composite(accs) => {
            accs.accounts
                .iter()
                .for_each(|acc| out.extend(acc_item_name(acc, upper)));
        }
    }
    out
}

fn acc_meta(acc: &IdlInstructionAccount) -> TokenStream {
    let name = format_ident!("{}", acc.name.to_snake_case());
    let is_signer = acc.signer;
    let new = format_ident!("{}", if acc.writable { "new" } else { "new_readonly" });
    quote!(::solana_program::instruction::AccountMeta::#new(#name, #is_signer))
}
fn acc_item_meta(acc: &IdlInstructionAccountItem) -> Vec<TokenStream> {
    let mut out = Vec::new();
    match acc {
        IdlInstructionAccountItem::Single(acc) => out.push(acc_meta(acc)),
        IdlInstructionAccountItem::Composite(accs) => {
            accs.accounts
                .iter()
                .for_each(|acc| out.extend(acc_item_meta(acc)));
        }
    }
    out
}

fn ix_builders_and_parsers_gen(master_enum_name: &Ident, ixs: &[Instruction<'_>]) -> TokenStream {
    ixs.iter()
        .map(|ix| {
            let name = &ix.ident;
            let accounts: Vec<TokenStream> = ix
                .idl
                .accounts
                .iter()
                .flat_map(|acc| acc_item_name(acc, false))
                .collect();
            let upper_accounts: Vec<TokenStream> = ix
                .idl
                .accounts
                .iter()
                .flat_map(|acc| acc_item_name(acc, true))
                .collect();
            let accounts_decl = accounts.clone();
            let account_metas: Vec<TokenStream> =
                ix.idl.accounts.iter().flat_map(acc_item_meta).collect();
            let params_decl = ix.args.iter().map(Field::pub_decl_gen);
            let params = ix.args.iter().map(|arg| &arg.ident);
            let ix_args = params.clone();
            let account_idxs_name = format_ident!("{}AccountIndexes", name);
            let try_acc_idx = accounts.clone();
            let try_acc_idx_iter_mut = if accounts.is_empty() {
                quote!()
            } else {
                quote!(mut)
            };
            let try_acc_idx_idx_const = 0..accounts.len();
            let try_acc_idx_idx = 0..accounts.len();
            quote! {
                #[derive(Debug)]
                pub struct #name {
                    pub program_id: ::solana_program::pubkey::Pubkey,

                    // Accounts
                    #(pub #accounts_decl: ::solana_program::pubkey::Pubkey,)*
                    pub trailing_accounts: Vec<::solana_program::instruction::AccountMeta>,

                    // Params
                    #(#params_decl,)*
                }
                impl #name {
                    pub fn into_instruction(self) -> ::solana_program::instruction::Instruction {
                        let Self {
                            program_id,
                            #(#accounts,)*
                            trailing_accounts,
                            #(#params,)*
                        } = self;

                        let mut accounts = vec![
                            #(#account_metas,)*
                        ];
                        if !trailing_accounts.is_empty() {
                            accounts.extend(trailing_accounts);
                        }

                        let data = #master_enum_name::#name {
                            #(#ix_args,)*
                        }
                        .pack();

                        ::solana_program::instruction::Instruction {
                            program_id,
                            data,
                            accounts,
                        }
                    }
                }

                #[derive(Debug)]
                pub struct #account_idxs_name {
                    #(pub #accounts_decl: usize,)*
                    pub trailing_accounts: Vec<usize>,
                }
                impl #account_idxs_name {
                    #(
                        pub const #upper_accounts: usize = #try_acc_idx_idx_const;
                    )*
                }
                impl<'a> TryFrom<&'a [u8]> for #account_idxs_name {
                    type Error = ::anchor_interface::errors::TryAccountIndexesError;
                    fn try_from(indexes: &'a [u8]) -> Result<Self, Self::Error> {
                        let #try_acc_idx_iter_mut iter = indexes.iter().map(|idx| (*idx) as usize);
                        Ok(Self {
                            #(
                                #try_acc_idx: iter.next()
                                    .ok_or(::anchor_interface::errors::TryAccountIndexesError
                                        ::GetIndex(stringify!(#try_acc_idx), #try_acc_idx_idx)
                                    )?,
                            )*
                            trailing_accounts: iter.collect(),
                        })
                    }
                }
            }
        })
        .collect()
}

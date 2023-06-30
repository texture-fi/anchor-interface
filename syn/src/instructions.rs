use anchor_syn::codegen::program::common::{sighash, SIGHASH_GLOBAL_NAMESPACE};
use anchor_syn::idl::{IdlAccount, IdlAccountItem, IdlInstruction};
use heck::{ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::common::{docs_gen, item_gen, Field};
use crate::Generator;

impl Generator {
    pub fn gen_instructions(&self) -> TokenStream {
        let master_enum_name = item_gen(&format!(
            "{}Instruction",
            self.idl.name.to_upper_camel_case()
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
    pub sighash: TokenStream,
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
            sighash: {
                let h = sighash(SIGHASH_GLOBAL_NAMESPACE, &ix.name.to_snake_case());
                quote!([#(#h),*])
            },
            args: ix
                .args
                .iter()
                .map(|arg| Field::parse(arg, &Default::default()))
                .collect(),
            borsh_deser_ident,
        }
    }
}

fn master_enum_gen(master_enum_name: &Ident, ixs: &[Instruction<'_>]) -> TokenStream {
    let ixs_decl = ixs.iter().map(|ix| {
        let docs = if ix.idl.docs.is_some() {
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

    let sighashes_matches = ixs.iter().map(|ix| {
        let name = &ix.ident;
        let args = if ix.args.is_empty() {
            quote!()
        } else {
            quote!({ .. })
        };
        let sighash = &ix.sighash;
        quote!( Self::#name #args => &#sighash )
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
                fn deserialize(
                    _buf: &mut &[u8],
                ) -> ::core::result::Result<Self, ::borsh::maybestd::io::Error> {
                    Ok(Self ( #master_enum_name::#name {
                        #(#args: ::borsh::BorshDeserialize::deserialize(_buf)?,)*
                    }))
                }
            }
            impl From<#helper_name> for #master_enum_name {
                fn from(helper: #helper_name) -> #master_enum_name {
                    helper.0
                }
            }
        }
    });

    let unpack_matches = ixs.iter().map(|ix| {
        let sighash = &ix.sighash;
        let helper_name = &ix.borsh_deser_ident;
        quote!(#sighash => {
            #helper_name::try_from_slice(ix_data)?.into()
        })
    });

    quote! {
        #[derive(Debug)]
        pub enum #master_enum_name {
            #(#ixs_decl),*
        }
        impl #master_enum_name {
            pub fn sighash(&self) -> &'static [u8; 8] {
                match self {
                    #(#sighashes_matches),*
                }
            }
            pub fn pack(self) -> Vec<u8> {
                use ::borsh::BorshSerialize;

                let mut out = Vec::new();
                out.extend(self.sighash());

                let data = self.try_to_vec().unwrap();
                out.extend(data);

                out
            }
            pub fn unpack(data: &[u8]) -> ::std::io::Result<Self> {
                use ::borsh::BorshDeserialize;

                let (sighash, ix_data) = data.split_at(8);

                Ok(match sighash {
                    #(#unpack_matches)*
                    _ => return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid sighash",
                    )),
                })
            }
        }

        impl ::borsh::BorshSerialize for #master_enum_name {
            fn serialize<W: ::borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), ::borsh::maybestd::io::Error> {
                match self {
                    #(#borsh_serialize_matches)*
                }
                Ok(())
            }
        }

        #(#borsh_deserializers)*
    }
}

fn acc_docs(acc: &IdlAccount, idx: &mut usize) -> String {
    let out = format!(
        " {}. `[{}{}{}]` {}",
        idx,
        if acc.is_signer { "signer" } else { "" },
        if acc.is_signer && acc.is_mut {
            ", "
        } else {
            ""
        },
        if acc.is_mut { "writable" } else { "" },
        acc.name.to_title_case().to_lowercase(),
    );
    *idx += 1;
    out
}
fn acc_item_docs(acc: &IdlAccountItem, idx: &mut usize) -> Vec<String> {
    let mut out = Vec::new();
    match acc {
        IdlAccountItem::IdlAccount(acc) => {
            out.push(acc_docs(acc, idx));
        }
        IdlAccountItem::IdlAccounts(accs) => {
            accs.accounts.iter().for_each(|acc| {
                out.extend(acc_item_docs(acc, idx));
            });
        }
    }
    out
}
fn acc_docs_gen(accounts: &[IdlAccountItem]) -> TokenStream {
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

fn acc_name(acc: &IdlAccount, upper: bool) -> TokenStream {
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
fn acc_item_name(acc: &IdlAccountItem, upper: bool) -> Vec<TokenStream> {
    let mut out = Vec::new();
    match acc {
        IdlAccountItem::IdlAccount(acc) => out.push(acc_name(acc, upper)),
        IdlAccountItem::IdlAccounts(accs) => {
            accs.accounts
                .iter()
                .for_each(|acc| out.extend(acc_item_name(acc, upper)));
        }
    }
    out
}

fn acc_meta(acc: &IdlAccount) -> TokenStream {
    let name = format_ident!("{}", acc.name.to_snake_case());
    let is_signer = acc.is_signer;
    let new = format_ident!("{}", if acc.is_mut { "new" } else { "new_readonly" });
    quote!(AccountMeta::#new(#name, #is_signer))
}
fn acc_item_meta(acc: &IdlAccountItem) -> Vec<TokenStream> {
    let mut out = Vec::new();
    match acc {
        IdlAccountItem::IdlAccount(acc) => out.push(acc_meta(acc)),
        IdlAccountItem::IdlAccounts(accs) => {
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
            let accounts = {
                let mut out = Vec::new();
                ix.idl
                    .accounts
                    .iter()
                    .for_each(|acc| out.extend(acc_item_name(acc, false)));
                out
            };
            let upper_accounts = {
                let mut out = Vec::new();
                ix.idl
                    .accounts
                    .iter()
                    .for_each(|acc| out.extend(acc_item_name(acc, true)));
                out
            };
            let accounts_decl = accounts.clone();
            let account_metas = {
                let mut out = Vec::new();
                ix.idl
                    .accounts
                    .iter()
                    .for_each(|acc| out.extend(acc_item_meta(acc)));
                out
            };
            let params_decl = ix.args.iter().map(Field::pub_decl_gen);
            let params = ix.args.iter().map(|arg| &arg.ident);
            let ix_args = params.clone();
            let account_idxs_name = format_ident!("{}AccountIndexes", name);
            let try_acc_idx = accounts.clone();
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
                        use ::solana_program::instruction::{AccountMeta, Instruction};

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

                        Instruction {
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
                        let mut iter = indexes.iter().map(|idx| (*idx) as usize);
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

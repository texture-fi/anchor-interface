use anchor_syn::codegen::program::common::{sighash, SIGHASH_GLOBAL_NAMESPACE};
use anchor_syn::idl::{Idl, IdlAccountItem, IdlInstruction};
use heck::{ToSnakeCase, ToTitleCase, ToUpperCamelCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::common::{docs_gen, item_gen, Field};

pub fn gen(idl: &Idl) -> TokenStream {
    let master_enum_name = item_gen(&format!("{}Instruction", idl.name.to_upper_camel_case()));
    let ixs: Vec<_> = idl.instructions.iter().map(Instruction::new).collect();
    let master_enum = master_enum_gen(&master_enum_name, &ixs);
    let ix_builders = ix_builders_gen(&master_enum_name, &ixs);
    quote! {
        #master_enum
        #ix_builders
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
                .map(|arg| {
                    Field::parse(arg).unwrap_or_else(|err| {
                        panic!("Instruction {} arg {}: {}", &ix.name, &arg.name, err)
                    })
                })
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
                    buf: &mut &[u8],
                ) -> ::core::result::Result<Self, ::borsh::maybestd::io::Error> {
                    Ok(Self ( #master_enum_name::#name {
                        #(#args: borsh::BorshDeserialize::deserialize(buf)?,)*
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

fn acc_docs_gen(accounts: &[IdlAccountItem]) -> TokenStream {
    if accounts.is_empty() {
        return quote!();
    }

    let mut idx = 0;
    let docs = accounts.iter().map(|acc| match acc {
        IdlAccountItem::IdlAccount(acc) => {
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
            idx += 1;
            out
        }
        IdlAccountItem::IdlAccounts(_accs) => {
            todo!("multiple accounts not supported yet")
        }
    });
    quote! {
        #[doc = " Accounts expected by this instruction:"]
        #(#[doc = #docs])*
    }
}

fn ix_builders_gen(master_enum_name: &Ident, ixs: &[Instruction<'_>]) -> TokenStream {
    ixs.iter()
        .map(|ix| {
            let name = &ix.ident;
            let accounts = ix.idl.accounts.iter().map(|acc| match acc {
                IdlAccountItem::IdlAccount(acc) => {
                    let name = format_ident!("{}", acc.name.to_snake_case());
                    quote!(#name)
                }
                IdlAccountItem::IdlAccounts(_) => todo!("multiple accounts not supported yet"),
            });
            let accounts_decl = accounts.clone();
            let account_metas = ix.idl.accounts.iter().map(|acc| match acc {
                IdlAccountItem::IdlAccount(acc) => {
                    let name = format_ident!("{}", acc.name.to_snake_case());
                    let is_signer = acc.is_signer;
                    let new = format_ident!("{}", if acc.is_mut { "new" } else { "new_readonly" });
                    quote!(AccountMeta::#new(#name, #is_signer))
                }
                IdlAccountItem::IdlAccounts(_) => todo!("multiple accounts not supported yet"),
            });
            let params_decl = ix.args.iter().map(Field::pub_decl_gen);
            let params = ix.args.iter().map(|arg| &arg.ident);
            let ix_args = params.clone();
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
            }
        })
        .collect()
}

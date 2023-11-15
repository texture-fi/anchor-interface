use anchor_syn::codegen::program::common::sighash;
use proc_macro2::TokenStream;
use quote::quote;

use crate::typedefs::typedef_gen;
use crate::Generator;

impl Generator {
    pub fn gen_accounts(&self) -> TokenStream {
        let accounts = self.idl.accounts.iter().map(|acc| {
            let (typedef, name, opts) = typedef_gen(&self.idl.types, &self.struct_opts, acc);
            let namespace = "account"; // TODO: add other namespaces support
            let discriminator = sighash(namespace, &name.to_string());
            let (serialize, deserialize) = if opts.zero_copy {
                (
                    quote!(writer.write_all(::bytemuck::bytes_of(self))),
                    quote!(
                        ::bytemuck::try_pod_read_unaligned(&data[8..]).map_err(|err| {
                            ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                        })
                    ),
                )
            } else {
                (
                    quote!(::borsh::BorshSerialize::serialize(self, writer)),
                    quote!(::borsh::BorshDeserialize::deserialize(&mut &data[8..])),
                )
            };
            let impl_serialize_and_deserialize = quote! {
                impl ::anchor_interface::Account for #name {
                    fn discriminator() -> &'static [u8; 8] {
                        &[#(#discriminator),*]
                    }
                }
                impl ::anchor_interface::AccountSerialize for #name {
                    fn try_serialize<W: std::io::Write>(&self, writer: &mut W)
                        -> std::io::Result<()>
                    {
                        use ::anchor_interface::Account;
                        writer.write_all(Self::discriminator())?;
                        #serialize?;
                        Ok(())
                    }
                }
                impl ::anchor_interface::AccountDeserialize for #name {
                    fn try_deserialize(data: &mut &[u8]) -> std::io::Result<Self> {
                        use ::anchor_interface::Account;
                        if data.len() < 8 || &data[..8] != Self::discriminator() {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "invalid discriminator",
                            ));
                        }
                        let t = #deserialize?;
                        Ok(t)
                    }
                }
            };
            quote! {
                #typedef
                #impl_serialize_and_deserialize
            }
        });
        quote! {
            #(#accounts)*
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discriminator() {
        let account_name = "WhirlpoolsConfig";
        let namespace = "account";
        let sighash = sighash(namespace, account_name);
        let expected = [157, 20, 49, 224, 217, 87, 193, 254];
        assert_eq!(sighash, expected);
    }
}

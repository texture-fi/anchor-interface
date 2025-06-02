use proc_macro2::TokenStream;
use quote::quote;

use crate::typedefs::typedef_gen;
use crate::Generator;

impl Generator {
    pub fn gen_accounts(&self) -> TokenStream {
        let accounts = self.idl.accounts.iter().map(|acc| {
            let ty = {
                let Some(&idx) = self.account_type_idx_by_name.get(&acc.name) else {
                    panic!("account `{}` type not found", &acc.name);
                };
                &self.idl.types[idx]
            };

            let (typedef, name, opts) = typedef_gen(&self.idl.types, &self.typedef_opts, ty);

            let discriminator = &acc.discriminator;
            let discriminator_len = discriminator.len();

            let impl_account = quote! {
                impl ::anchor_interface::Account for #name {
                    const DISCRIMINATOR: &'static [u8] = &[#(#discriminator),*];
                }
            };
            let check_discriminator = quote! {
                if data.len() < #discriminator_len || &data[..#discriminator_len] != Self::DISCRIMINATOR {
                    return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::InvalidData,
                        format!(
                            "invalid discriminator: got `{:?}`, expected `{:?}`",
                            &data[..#discriminator_len.min(data.len())],
                            Self::DISCRIMINATOR,
                        ),
                    ));
                }
            };
            let impl_serialize_and_deserialize = if opts.zero_copy {
                let check_size = quote! {
                    if data.len() < #discriminator_len + std::mem::size_of::<Self>() {
                        return Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidData,
                            format!(
                                "not enough data: got `{}`, expected `{}`",
                                data.len(),
                                #discriminator_len + std::mem::size_of::<Self>(),
                            ),
                        ));
                    }
                };
                let from_bytes_checks = quote! {
                    #check_discriminator
                    #check_size
                };
                let payload = quote! {
                    data[#discriminator_len..#discriminator_len + std::mem::size_of::<Self>()]
                };
                quote! {
                    impl ::anchor_interface::PodAccount for #name {
                        fn try_init_bytes(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
                            use ::anchor_interface::Account;

                            #check_size

                            data[..#discriminator_len].copy_from_slice(Self::DISCRIMINATOR);

                            let payload = &mut #payload;
                            ::bytemuck::try_from_bytes_mut(payload).map_err(|err| {
                                ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                            })
                        }

                        fn try_from_bytes_mut(data: &mut [u8]) -> ::std::io::Result<&mut Self> {
                            use ::anchor_interface::Account;

                            #from_bytes_checks

                            let payload = &mut #payload;
                            ::bytemuck::try_from_bytes_mut(payload).map_err(|err| {
                                ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                            })
                        }

                        fn try_from_bytes(data: &[u8]) -> ::std::io::Result<&Self> {
                            use ::anchor_interface::Account;

                            #from_bytes_checks

                            let payload = &#payload;
                            ::bytemuck::try_from_bytes(payload).map_err(|err| {
                                ::std::io::Error::new(::std::io::ErrorKind::InvalidData, err)
                            })
                        }
                    }
                }
            } else {
                quote! {
                    impl ::anchor_interface::AccountSerialize for #name {
                        fn try_serialize<W: std::io::Write>(&self, writer: &mut W)
                            -> std::io::Result<()>
                        {
                            use ::anchor_interface::Account;
                            writer.write_all(Self::DISCRIMINATOR)?;
                            ::borsh::BorshSerialize::serialize(self, writer)?;
                            Ok(())
                        }
                    }
                    impl ::anchor_interface::AccountDeserialize for #name {
                        fn try_deserialize(data: &mut &[u8]) -> std::io::Result<Self> {
                            use ::anchor_interface::Account;
                            #check_discriminator
                            let t = ::borsh::BorshDeserialize::deserialize(&mut &data[#discriminator_len..])?;
                            Ok(t)
                        }
                    }
                }
            };
            quote! {
                #typedef
                #impl_account
                #impl_serialize_and_deserialize
            }
        });
        quote! {
            #(#accounts)*
        }
    }
}

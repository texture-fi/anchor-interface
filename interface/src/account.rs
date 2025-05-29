use std::io::{Result, Write};

pub trait Account: Sized {
    const DISCRIMINATOR: &'static [u8];
}

pub trait AccountSerialize: Account {
    fn try_serialize<W: Write>(&self, writer: &mut W) -> Result<()>;
}

pub trait AccountDeserialize: Account {
    fn try_deserialize(data: &mut &[u8]) -> Result<Self>;
}

pub trait PodAccount: Account + Copy {
    fn try_init_bytes(bytes: &mut [u8]) -> Result<&mut Self>;
    fn try_from_bytes_mut(bytes: &mut [u8]) -> Result<&mut Self>;
    fn try_from_bytes(bytes: &[u8]) -> Result<&Self>;
}

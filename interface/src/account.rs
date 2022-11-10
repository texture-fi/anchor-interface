use borsh::{BorshDeserialize, BorshSerialize};

pub trait Account: BorshSerialize + BorshDeserialize {
    fn discriminator() -> &'static [u8; 8];
}

pub trait AccountSerialize: Account {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(Self::discriminator())?;
        BorshSerialize::serialize(self, writer)?;
        Ok(())
    }
}

pub trait AccountDeserialize: Account {
    fn try_deserialize(data: &mut &[u8]) -> std::io::Result<Self> {
        if data.len() < 8 || &data[..8] != Self::discriminator() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid discriminator",
            ));
        }
        BorshDeserialize::try_from_slice(&data[8..]).map_err(Into::into)
    }
}

impl<T: Account> AccountSerialize for T {}
impl<T: Account> AccountDeserialize for T {}

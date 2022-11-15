pub trait Account: Sized {
    fn discriminator() -> &'static [u8; 8];
}

pub trait AccountSerialize: Account {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()>;
}

pub trait AccountDeserialize: Account {
    fn try_deserialize(data: &mut &[u8]) -> std::io::Result<Self>;
}

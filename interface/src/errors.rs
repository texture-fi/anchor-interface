use thiserror::Error;

#[derive(Debug, Error)]
pub enum TryAccountIndexesError {
    #[error("get index for account '{0}' with index {1} failure")]
    GetIndex(&'static str, usize),
}

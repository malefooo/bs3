use crate::Result;
use alloc::vec::Vec;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Update(Vec<u8>),
    Delete,
}

pub trait Store {
    /// Provide this method to get value from backend.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;

    /// Provide this method to execute transaction.
    fn execute(&self, batch: Vec<(&[u8], &Operation)>) -> Result<()>;
}

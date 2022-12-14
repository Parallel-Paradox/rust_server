#![allow(unused)]

use std::fmt::Display;

pub type Result<T> = std::result::Result<T, ErrCode>;

#[derive(Debug)]
pub enum ErrCode {
    /// Mongo account should be set manually.
    MongoCredentialUnset,
    /// Mongo read write error, See [`mongodb::error::Error`]
    MongoRwError(mongodb::error::Error),
    ObjectNotExist,
}

impl ErrCode {
    fn get_code(&self) -> usize {
        match &self {
            Self::MongoCredentialUnset => 120,
            Self::MongoRwError(_) => 121,
            Self::ObjectNotExist => 122,
        }
    }
}

impl Display for ErrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "err: {} ", self.get_code())?;
        match &self {
            _ => { /* Ignore the enum that has no content inside */ },
            // TODO A more readable error information.
            Self::MongoRwError(err) => { write!(f, "{:?}", err)?; },
        }
        Ok(())
    }
}

impl From<mongodb::error::Error> for ErrCode {
    fn from(err: mongodb::error::Error) -> Self { Self::MongoRwError(err) }
}

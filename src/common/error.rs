use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum MyError {
    #[error("DBError:")]
    DBError(String),

    #[error("ActixError:")]
    ActixError(String),

    #[error("NotFound:")]
    NotFound(String),
}

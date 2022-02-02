use crate::errors::AppError;
use std::result;

pub type Result<T> = result::Result<T, AppError>;
pub type Byte = u8;
pub type Bytes = Vec<Byte>;

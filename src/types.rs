use crate::errors::AppError;
use std::result;

pub type Result<T> = result::Result<T, AppError>;

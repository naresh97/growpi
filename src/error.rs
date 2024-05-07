use std::error::Error;

pub type GenericResult<T> = Result<T, Box<dyn Error>>;

pub fn lock_err<T: std::fmt::Display>(e: T) -> String {
    format!("{}", e)
}

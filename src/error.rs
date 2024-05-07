use std::error::Error;

pub type GenericResult<T> = Result<T, Box<dyn Error>>;

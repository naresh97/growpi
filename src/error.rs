use std::error::Error;

type GenericResult<T> = Result<T, Box<dyn Error>>;

use anyhow::Error;

pub trait IntoResult<T> {
    fn into_result(self) -> Result<T, Error>;
}

impl<T> IntoResult<T> for Option<T> {
    fn into_result(self) -> Result<T, Error> {
        match self {
            Some(t) => Ok(t),
            None => Err(Error::msg("None")),
        }
    }
}

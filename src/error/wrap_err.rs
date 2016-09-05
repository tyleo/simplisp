use std::error::Error;
use error::Result as LispResult;
use error::WrapError;

pub trait WrapErr<TResult> {
    fn wrap_err_to_err(self) -> LispResult<TResult>;
}

impl <TResult, TError> WrapErr<TResult> for Result<TResult, TError>
    where TError: Error + Send + 'static {
    fn wrap_err_to_err(self) -> LispResult<TResult> {
        match self  {
            Ok(ok) => Ok(ok),
            Err(error) => error.wrap_error_to_err(),
        }
    }
}

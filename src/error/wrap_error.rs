use std::error::Error;
use error::Error as LispError;
use error::ErrorKind as LispErrorKind;
use error::Result as LispResult;

pub trait WrapError {
    fn wrap_error_to_err<TResult>(self) -> LispResult<TResult>;

    fn wrap_error_to_error(self) -> LispError;
}

impl <TError> WrapError for TError
    where TError: Error + Send + 'static {
    fn wrap_error_to_err<TResult>(self) -> LispResult<TResult> {
        Err(self.wrap_error_to_error())
    }

    fn wrap_error_to_error(self) -> LispError {
        let err = Box::new(self);
        LispErrorKind::ErrorWrapper(err).into()
    }
}

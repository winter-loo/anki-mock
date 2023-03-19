
pub trait OrInvalid {
    type Value;
    fn or_invalid(self, message: impl Into<String>) -> super::Result<Self::Value>;
}

impl<T> OrInvalid for Option<T> {
    type Value = T;

    fn or_invalid(self, message: impl Into<String>) -> super::Result<T> {
        match self {
            Some(t) => Ok(t),
            None => Err(super::AnkiError::ProtoError {
                info: message.into(),
            }),
        }
    }
}

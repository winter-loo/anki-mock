mod invalid_input;

use crate::pb;

pub use self::invalid_input::OrInvalid;

pub type Result<T, E = AnkiError> = std::result::Result<T, E>;

pub enum AnkiError {
    InvalidInput(&'static str),
    ProtoError{
        info: String
    }
}

impl From<prost::EncodeError> for AnkiError {
    fn from(err: prost::EncodeError) -> Self {
        AnkiError::ProtoError {
            info: err.to_string(),
        }
    }
}

impl From<prost::DecodeError> for AnkiError {
    fn from(err: prost::DecodeError) -> Self {
        AnkiError::ProtoError {
            info: err.to_string(),
        }
    }
}

impl AnkiError {
    pub fn into_protobuf(&self) -> pb::backend::BackendError {
        pb::backend::BackendError{
            message: "backend error".into(),
            kind: 0,
            context: "backend".into(),
            backtrace: "no backtrace".into(),
        }
    }
}
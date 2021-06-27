//! Crate level errors.

use http::StatusCode;
use std::str::Utf8Error;
use tokio::task::JoinError;

pub type ApolloClientResult<T> = Result<T, ApolloClientError>;

#[derive(thiserror::Error, Debug)]
pub enum ApolloClientError {
    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    Http(#[from] http::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[cfg(feature = "conf")]
    #[cfg_attr(docsrs, doc(cfg(feature = "conf")))]
    #[error(transparent)]
    IniParse(#[from] ini::ParseError),

    #[error(transparent)]
    Join(#[from] JoinError),

    #[error(transparent)]
    ApolloResponse(#[from] ApolloResponseError),

    #[error("this URL is cannot-be-a-base")]
    UrlCannotBeABase,

    #[error("Config is empty")]
    EmptyConfig,
}

#[derive(thiserror::Error, Debug)]
pub enum ApolloResponseError {
    #[error("not modified")]
    NotModified,
    #[error("bad request")]
    BadRequest,
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("method not allowed")]
    MethodNotAllowed,
    #[error("internal server error")]
    InternalServerError,
    #[error("unknown error, status code: {0}")]
    Unknown(StatusCode),
}

impl ApolloResponseError {
    pub(crate) fn from_status_code(status: StatusCode) -> Option<Self> {
        match status {
            StatusCode::OK => None,
            StatusCode::NOT_MODIFIED => Some(Self::NotModified),
            StatusCode::BAD_REQUEST => Some(Self::BadRequest),
            StatusCode::UNAUTHORIZED => Some(Self::Unauthorized),
            StatusCode::FORBIDDEN => Some(Self::Forbidden),
            StatusCode::NOT_FOUND => Some(Self::NotFound),
            StatusCode::METHOD_NOT_ALLOWED => Some(Self::MethodNotAllowed),
            StatusCode::INTERNAL_SERVER_ERROR => Some(Self::InternalServerError),
            s => Some(Self::Unknown(s)),
        }
    }
}

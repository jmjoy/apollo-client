use http::StatusCode;
use crate::open::errors::OpenApiResponseError::Unknown;

pub type OpenApiClientResult<T> = Result<T, OpenApiClientError>;

#[derive(thiserror::Error, Debug)]
pub enum OpenApiClientError {
    #[error(transparent)]
    Http(#[from] http::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    OpenApiResponse(#[from] OpenApiResponseError),

    #[error("this URL is cannot-be-a-base")]
    UrlCannotBeABase,
}

#[derive(thiserror::Error, Debug)]
pub enum OpenApiResponseError {
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

impl OpenApiResponseError {
    pub(crate) fn from_status_code(status: StatusCode) -> Option<Self> {
        match status {
            StatusCode::OK => None,
            StatusCode::BAD_REQUEST => Some(Self::BadRequest),
            StatusCode::UNAUTHORIZED => Some(Self::Unauthorized),
            StatusCode::FORBIDDEN => Some(Self::Forbidden),
            StatusCode::NOT_FOUND => Some(Self::NotFound),
            StatusCode::METHOD_NOT_ALLOWED => Some(Self::MethodNotAllowed),
            StatusCode::INTERNAL_SERVER_ERROR => Some(Self::InternalServerError),
            s => Some(Unknown(s)),
        }
    }
}

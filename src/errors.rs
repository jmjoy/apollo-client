//! Crate level errors.

use http::StatusCode;
use reqwest::Response;
use std::str::Utf8Error;

/// Crate level result.
pub type ApolloClientResult<T> = Result<T, ApolloClientError>;

/// Crate level error.
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
    ApolloResponse(#[from] ApolloResponseError),

    #[error("this URL is cannot-be-a-base")]
    UrlCannotBeABase,
}

/// Apollo api response error, when http status is not success.
#[derive(thiserror::Error, Debug)]
#[error(r#"error occurred when apollo response, status: {status}, body: "{body}""#)]
pub struct ApolloResponseError {
    /// Http response status.
    pub status: StatusCode,
    /// Http response body, mainly the error reason.
    pub body: String,
}

impl ApolloResponseError {
    pub(crate) async fn from_response(response: Response) -> Result<Response, Self> {
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(Self {
                status: response.status(),
                body: response.text().await.unwrap_or_default(),
            })
        }
    }
}

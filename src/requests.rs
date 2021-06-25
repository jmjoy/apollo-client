use http::Method;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

pub trait PerformRequest {
    type Response: DeserializeOwned;

    fn path(&self) -> String;

    fn method(&self) -> http::Method {
        Method::GET
    }

    fn query(&self) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        vec![]
    }
}

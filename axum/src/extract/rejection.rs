//! Rejection response types.

use crate::{
    body::{boxed, Full},
    response::{IntoResponse, Response},
    BoxError, Error,
};

pub use crate::extract::path::FailedToDeserializePathParams;
pub use axum_core::extract::rejection::*;

#[cfg(feature = "json")]
define_rejection! {
    #[status = UNPROCESSABLE_ENTITY]
    #[body = "Failed to parse the request body as JSON"]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    /// Rejection type for [`Json`](super::Json).
    pub struct InvalidJsonBody(Error);
}

#[cfg(feature = "json")]
define_rejection! {
    #[status = UNSUPPORTED_MEDIA_TYPE]
    #[body = "Expected request with `Content-Type: application/json`"]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    /// Rejection type for [`Json`](super::Json) used if the `Content-Type`
    /// header is missing.
    pub struct MissingJsonContentType;
}

define_rejection! {
    #[status = INTERNAL_SERVER_ERROR]
    #[body = "Missing request extension"]
    /// Rejection type for [`Extension`](super::Extension) if an expected
    /// request extension was not found.
    pub struct MissingExtension(Error);
}

define_rejection! {
    #[status = PAYLOAD_TOO_LARGE]
    #[body = "Request payload is too large"]
    /// Rejection type for [`ContentLengthLimit`](super::ContentLengthLimit) if
    /// the request body is too large.
    pub struct PayloadTooLarge;
}

define_rejection! {
    #[status = LENGTH_REQUIRED]
    #[body = "Content length header is required"]
    /// Rejection type for [`ContentLengthLimit`](super::ContentLengthLimit) if
    /// the request is missing the `Content-Length` header or it is invalid.
    pub struct LengthRequired;
}

define_rejection! {
    #[status = INTERNAL_SERVER_ERROR]
    #[body = "No paths parameters found for matched route. Are you also extracting `Request<_>`?"]
    /// Rejection type used if axum's internal representation of path parameters
    /// is missing. This is commonly caused by extracting `Request<_>`. `Path`
    /// must be extracted first.
    pub struct MissingPathParams;
}

define_rejection! {
    #[status = UNSUPPORTED_MEDIA_TYPE]
    #[body = "Form requests must have `Content-Type: x-www-form-urlencoded`"]
    /// Rejection type used if you try and extract the request more than once.
    pub struct InvalidFormContentType;
}

/// Rejection type for extractors that deserialize query strings if the input
/// couldn't be deserialized into the target type.
#[derive(Debug)]
pub struct FailedToDeserializeQueryString {
    error: Error,
    type_name: &'static str,
}

impl FailedToDeserializeQueryString {
    pub(super) fn new<T, E>(error: E) -> Self
    where
        E: Into<BoxError>,
    {
        FailedToDeserializeQueryString {
            error: Error::new(error),
            type_name: std::any::type_name::<T>(),
        }
    }
}

impl IntoResponse for FailedToDeserializeQueryString {
    fn into_response(self) -> Response {
        let mut res = Response::new(boxed(Full::from(self.to_string())));
        *res.status_mut() = http::StatusCode::UNPROCESSABLE_ENTITY;
        res
    }
}

impl std::fmt::Display for FailedToDeserializeQueryString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to deserialize query string. Expected something of type `{}`. Error: {}",
            self.type_name, self.error,
        )
    }
}

impl std::error::Error for FailedToDeserializeQueryString {}

composite_rejection! {
    /// Rejection used for [`Query`](super::Query).
    ///
    /// Contains one variant for each way the [`Query`](super::Query) extractor
    /// can fail.
    pub enum QueryRejection {
        FailedToDeserializeQueryString,
    }
}

composite_rejection! {
    /// Rejection used for [`Form`](super::Form).
    ///
    /// Contains one variant for each way the [`Form`](super::Form) extractor
    /// can fail.
    pub enum FormRejection {
        InvalidFormContentType,
        FailedToDeserializeQueryString,
        BytesRejection,
    }
}

#[cfg(feature = "json")]
composite_rejection! {
    /// Rejection used for [`Json`](super::Json).
    ///
    /// Contains one variant for each way the [`Json`](super::Json) extractor
    /// can fail.
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub enum JsonRejection {
        InvalidJsonBody,
        MissingJsonContentType,
        BytesRejection,
    }
}

composite_rejection! {
    /// Rejection used for [`Extension`](super::Extension).
    ///
    /// Contains one variant for each way the [`Extension`](super::Extension) extractor
    /// can fail.
    pub enum ExtensionRejection {
        MissingExtension,
    }
}

composite_rejection! {
    /// Rejection used for [`Path`](super::Path).
    ///
    /// Contains one variant for each way the [`Path`](super::Path) extractor
    /// can fail.
    pub enum PathRejection {
        FailedToDeserializePathParams,
        MissingPathParams,
    }
}

#[cfg(feature = "matched-path")]
define_rejection! {
    #[status = INTERNAL_SERVER_ERROR]
    #[body = "No matched path found"]
    /// Rejection if no matched path could be found.
    ///
    /// See [`MatchedPath`](super::MatchedPath) for more details.
    #[cfg_attr(docsrs, doc(cfg(feature = "matched-path")))]
    pub struct MatchedPathMissing;
}

#[cfg(feature = "matched-path")]
composite_rejection! {
    /// Rejection used for [`MatchedPath`](super::MatchedPath).
    #[cfg_attr(docsrs, doc(cfg(feature = "matched-path")))]
    pub enum MatchedPathRejection {
        MatchedPathMissing,
    }
}

/// Rejection used for [`ContentLengthLimit`](super::ContentLengthLimit).
///
/// Contains one variant for each way the
/// [`ContentLengthLimit`](super::ContentLengthLimit) extractor can fail.
#[derive(Debug)]
#[non_exhaustive]
pub enum ContentLengthLimitRejection<T> {
    #[allow(missing_docs)]
    PayloadTooLarge(PayloadTooLarge),
    #[allow(missing_docs)]
    LengthRequired(LengthRequired),
    #[allow(missing_docs)]
    Inner(T),
}

impl<T> IntoResponse for ContentLengthLimitRejection<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Self::PayloadTooLarge(inner) => inner.into_response(),
            Self::LengthRequired(inner) => inner.into_response(),
            Self::Inner(inner) => inner.into_response(),
        }
    }
}

impl<T> std::fmt::Display for ContentLengthLimitRejection<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PayloadTooLarge(inner) => inner.fmt(f),
            Self::LengthRequired(inner) => inner.fmt(f),
            Self::Inner(inner) => inner.fmt(f),
        }
    }
}

impl<T> std::error::Error for ContentLengthLimitRejection<T>
where
    T: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PayloadTooLarge(inner) => Some(inner),
            Self::LengthRequired(inner) => Some(inner),
            Self::Inner(inner) => Some(inner),
        }
    }
}

#[cfg(feature = "headers")]
pub use super::typed_header::{TypedHeaderRejection, TypedHeaderRejectionReason};

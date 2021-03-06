use ruma::api::client::r0::uiaa::UiaaResponse;
use ruma::api::client::Error as ApiError;
use ruma::api::exports::http;
use ruma_client::Error as InnerClientError;
use std::fmt::{self, Display};

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug)]
pub enum ClientError {
    /// Error occurred during an IO operation.
    IOError(std::io::Error),
    /// Error occurred while parsing a string as URL.
    URLParse(String, http::uri::InvalidUri),
    /// Error occurred in the Matrix client library.
    Internal(InnerClientError<ApiError>),
    /// Error occurred in the Matrix client library.
    InternalUiaa(InnerClientError<UiaaResponse>),
    /// The user is already logged in.
    AlreadyLoggedIn,
    /// Not all required login information was provided.
    MissingLoginInfo,
    /// Custom error
    Custom(String),
}

impl Clone for ClientError {
    fn clone(&self) -> Self {
        use ClientError::*;

        match self {
            AlreadyLoggedIn => AlreadyLoggedIn,
            MissingLoginInfo => MissingLoginInfo,
            Custom(err) => Custom(err.clone()),
            _ => Custom(self.to_string()),
        }
    }
}

impl From<InnerClientError<ApiError>> for ClientError {
    fn from(other: InnerClientError<ApiError>) -> Self {
        Self::Internal(other)
    }
}

impl From<InnerClientError<UiaaResponse>> for ClientError {
    fn from(other: InnerClientError<UiaaResponse>) -> Self {
        Self::InternalUiaa(other)
    }
}

impl From<std::io::Error> for ClientError {
    fn from(other: std::io::Error) -> Self {
        Self::IOError(other)
    }
}

impl Display for ClientError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use ruma::{api::client::error::ErrorKind as ClientAPIErrorKind, api::error::*};

        match self {
            ClientError::URLParse(string, err) => {
                write!(fmt, "Could not parse URL '{}': {}", string, err)
            }
            ClientError::Internal(err) => {
                match err {
                    InnerClientError::FromHttpResponse(FromHttpResponseError::Http(
                        ServerError::Known(err),
                    )) => match err.kind {
                        ClientAPIErrorKind::Forbidden => {
                            return write!(
                                fmt,
                                "It's forbidden to perform this operation: {}",
                                err.message
                            );
                        }
                        ClientAPIErrorKind::Unauthorized => {
                            return write!(
                                fmt,
                                "You are unauthorized to perform an operation: {}",
                                err.message
                            );
                        }
                        ClientAPIErrorKind::UnknownToken { soft_logout: _ } => {
                            return write!(
                                fmt,
                                "Your session has expired, please login again: {}",
                                err.message
                            );
                        }
                        _ => {}
                    },
                    InnerClientError::Response(_) => {
                        return write!(
                            fmt,
                            "Please check if you can connect to the internet and try again: {}",
                            err,
                        );
                    }
                    InnerClientError::AuthenticationRequired => {
                        return write!(
                            fmt,
                            "Authentication is required for an operation, please login (again)",
                        );
                    }
                    _ => {}
                }
                write!(fmt, "An internal error occurred: {}", err)
            }
            ClientError::InternalUiaa(err) => {
                write!(
                    fmt,
                    "An internal error occured while trying to register: {}",
                    err
                )
            }
            ClientError::IOError(err) => write!(fmt, "An IO error occurred: {}", err),
            ClientError::AlreadyLoggedIn => write!(fmt, "Already logged in with another user."),
            ClientError::MissingLoginInfo => {
                write!(fmt, "Missing required login information, can't login.")
            }
            ClientError::Custom(msg) => write!(fmt, "{}", msg),
        }
    }
}

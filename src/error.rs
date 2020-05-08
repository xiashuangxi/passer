use serde::Deserialize;
use std::fmt;
use std::str;

#[derive(Debug, Deserialize)]
pub struct RequestError {
    message: ResultErrorJson,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ResultErrorJson {
    code: Option<String>,
    status: Option<u32>,
    key: Option<String>,
    message: Option<String>,
}

impl fmt::Display for ResultErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "code: {}, key: {}, message: {}",
            self.code.as_ref().unwrap(),
            self.key.as_ref().unwrap(),
            self.message.as_ref().unwrap()
        )
    }
}

impl RequestError {
    pub fn new<T>(json: &str) -> Result<T, ClientError> {
        Err(ClientError::Http(RequestError {
            message: serde_json::from_str::<ResultErrorJson>(json).unwrap(),
        }))
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HTTP Error: {}", self.message)
    }
}

#[derive(Debug)]
pub struct InternalError {
    pub message: String,
}

impl InternalError {
    pub fn new<T>(message: &str) -> Result<T, ClientError> {
        Err(ClientError::Internal(InternalError {
            message: message.to_string(),
        }))
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal Error: {}", self.message)
    }
}

#[derive(Debug)]
pub enum ClientError {
    Http(RequestError),
    Internal(InternalError),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ClientError::Http(ref e) => write!(f, "{}", e),
            &ClientError::Internal(ref e) => write!(f, "{}", e),
        }
    }
}

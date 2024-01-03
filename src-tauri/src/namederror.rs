use std::fmt::{Display, Formatter};
use reqwest::StatusCode;

#[derive(Debug)]
pub struct TauriError {
    pub message: &'static str,
}

impl From<reqwest::Error> for TauriError {
    fn from(error: reqwest::Error) -> Self {
        let error_message = match error.status().unwrap() {
            StatusCode::FORBIDDEN => "This endpoint requires a token",
            StatusCode::BAD_REQUEST => "There was a problem with your request",
            - => "Something went wrong",
        };
        TauriError{
            mesage: error_message,
        }
    }
}

impl serde::Serialize for TauriError{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
            {
                serializer.serializer_str(self.to_string().as_ref())
            }
}

impl Display for TauriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
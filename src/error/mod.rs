use std::error::Error;
use std::fmt;

extern crate serde_json;
extern crate hyper;

use self::ClarifaiError::{ConversionError, HttpStatusError, HyperError, MissingFieldError,
                          SerdeError};

#[derive(Debug)]
pub enum ClarifaiError {
    ConversionError,
    HttpStatusError,
    HyperError(hyper::Error),
    MissingFieldError,
    SerdeError(serde_json::Error),
}

impl fmt::Display for ClarifaiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl Error for ClarifaiError {
    fn description(&self) -> &str {
        match *self {
            ConversionError => "Could not convert json",
            HttpStatusError => "Non-200 HTTP response from server",
            HyperError(ref e) => e.description(),
            MissingFieldError => "No such field exists",
            SerdeError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            HyperError(ref e) => Some(e),
            SerdeError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<hyper::Error> for ClarifaiError {
    fn from(err: hyper::Error) -> ClarifaiError {
        HyperError(err)
    }
}

impl From<serde_json::Error> for ClarifaiError {
    fn from(err: serde_json::Error) -> ClarifaiError {
        SerdeError(err)
    }
}

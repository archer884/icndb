use hyper;
use std::error;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    cause: Option<Box<error::Error>>,
}

#[derive(Debug)]
pub enum ErrorKind {
    /// An error returned by the ICNDB API.
    ///
    /// The ICNDB API does not return useful error codes in most cases. It doesn't even return a 
    /// JSON error response; it just spits out some nonsense about a call to an undefined method
    /// (ChuckAPI::echoException()) in /home/alumni/mdecat/chuck/api-github... etc. etc. I'm 
    /// guessing it's a bug that just isn't worth fixing.
    Api,

    /// An error in decoding the API response.
    ///
    /// This is generated by `std::io` when attempting to read a response into a string.
    IO,

    /// An error in contacting the ICNDB API.
    ///
    /// This is generated by hyper when a request doesn't work correctly.
    Network,
}

impl Error {
    pub fn api() -> Error {
        Error {
            kind: ErrorKind::Api,
            cause: None,
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(error: hyper::Error) -> Self {
        Error {
            kind: ErrorKind::Network,
            cause: Some(Box::new(error)),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            kind: ErrorKind::IO,
            cause: Some(Box::new(error)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Api => "ICNDB returned an error",
            ErrorKind::IO => "unable to decode response",
            ErrorKind::Network => "unable to contact ICNDB",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.cause {
            Some(ref cause) => Some(cause.as_ref()),
            None => None,
        }
    }
}

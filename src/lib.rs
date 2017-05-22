//! `icndb` provides a simple interface to the [Internet Chuck Norris Database](http://www.icndb.com)
//!
//! ## Example
//!
//! The code below calls the API to get a random joke with the custom
//! name "Maximus Hardcorion", because why would you not?
//!
//! ```rust
//! extern crate icndb;
//!
//! let response = icndb::next_with_names("Maximus", "Hardcorion").unwrap();
//!
//! assert!(response.content.contains("Maximus Hardcorion"));
//! ```
//!
//! The big thing to keep in mind, here, is that Maximus Hardcorion is
//! Chuck Norris' Latin name.

#[cfg(feature = "ssl")]
extern crate hyper_native_tls;

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde_json;

mod error;

use hyper::client;
use std::result;

pub use error::*;

#[cfg(not(feature="ssl"))]
static PROTOCOL: &str = "http";

#[cfg(feature="ssl")]
static PROTOCOL: &str = "https";

pub struct ApiClient {
    client: hyper::Client,
}

impl ApiClient {
    /// Create a new API client.
    pub fn new() -> ApiClient {
        ApiClient { client: create_client() }
    }

    /// Get a random joke from the ICNDB.
    ///
    /// Returns an option value containing a random joke from the API
    /// or, failing that, None.
    pub fn next(&self) -> Result<Joke> {
        let request_url = format!("{}://api.icndb.com/jokes/random", PROTOCOL);
        let response = self.execute_request(&request_url);
        unwrap_response(response)
    }

    /// Get a random joke from the ICNDB, replacing the names in the joke.
    ///
    /// Returns an option value containing a random joke from the API
    /// using the names supplied to the function instead of the default
    /// name (Chuck Norris) or, failing that, None.
    pub fn next_with_names(&self, first: &str, last: &str) -> Result<Joke> {
        let request_url = format!("{}://api.icndb.com/jokes/random?firstName={}&lastName={}", PROTOCOL, first, last);
        let response = self.execute_request(&request_url);
        unwrap_response(response)
    }

    /// Get a specific joke from the ICNDB.
    ///
    /// Returns an option value containing a specified joke from the API
    /// or, failing that, None.
    pub fn get_by_id(&self, id: u64) -> Result<Joke> {
        let request_url = format!("{}://api.icndb.com/jokes/{}", PROTOCOL, id);
        let response = self.execute_request(&request_url);
        unwrap_response(response)
    }

    /// Get a specific joke with specified names.
    ///
    /// Returns an option value containing a specified joke from the API
    /// using the names supplied to the function instead of the default
    /// name (Chuck Norris) or, failing that, None.
    pub fn get_by_id_with_names(&self, id: u64, first: &str, last: &str) -> Result<Joke> {
        let request_url = format!("{}://api.icndb.com/jokes/{}?firstName={}&lastName={}", PROTOCOL, id, first, last);
        let response = self.execute_request(&request_url);
        unwrap_response(response)
    }

    fn execute_request(&self, url: &str) -> Result<ApiResponseWrapper> {
        read_response(self.client.get(url).send())
    }
}

// Wraps an API response from the `api.icndb.com`. The authors'
// intent appears to have been to provide an interface for both
// failed and successful requests, but it has been difficult to
// represent the full wrapper in Rust, and the wrapper adds no
// real value.
#[derive(Deserialize)]
struct ApiResponseWrapper {
    value: ApiResponse,
}

#[derive(Deserialize)]
struct ApiResponse {
    pub id: u64,
    pub joke: String,
    pub categories: Box<[String]>,
}

/// Response containing a Chuck Norris joke.
///
/// Represents a single joke provided by the ICNDB. The `id` field
/// uniquely identifies this specific joke, which allows the user
/// to get this joke again at a later time if he or she so desires.
#[derive(Debug)]
pub struct Joke {
    pub id: u64,
    pub content: String,
    pub categories: Box<[String]>,
}

impl From<ApiResponse> for Joke {
    #[inline]
    fn from(response: ApiResponse) -> Self {
        Joke {
            id: response.id,
            content: response.joke,
            categories: response.categories,
        }
    }
}

// Parses the response returned by a query against the ICNDB API
// into an Joke or, failing that, None.
fn unwrap_response(response: Result<ApiResponseWrapper>) -> Result<Joke> {
    response.map(|res| unescape_content(res.value))
}

// Unescape HTML entities found in joke contents.
//
// The ICNDB represents some values as HTML entities in the json
// packets returned to the caller and rustc_deserialize does not
// unescape these entities upon deserializing the json packet.
// This function deals with that by taking the (potentially) escaped
// values and unescaping any HTML entities contained therein before
// returning a new Joke struct containing the unescaped
// values.
//
// ** Entities handled include: **
// - &quot;
//
// Hopefully we won't discover anymore.
fn unescape_content(response: ApiResponse) -> Joke {
    if response.joke.contains("&quot;") {
        Joke {
            id: response.id,
            content: response.joke.replace("&quot;", "\""),
            categories: response.categories,
        }
    } else {
        response.into()
    }
}

#[cfg(not(feature="ssl"))]
fn create_client() -> hyper::Client {
    Client::new()
}

#[cfg(feature="ssl")]
fn create_client() -> hyper::Client {
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);

    hyper::Client::with_connector(connector)
}

fn read_response(response: result::Result<client::Response, hyper::Error>) -> Result<ApiResponseWrapper> {
    use std::io::Read;

    let mut buf = String::new();
    response?.read_to_string(&mut buf)?;

    match serde_json::from_str(&buf) {
        Ok(result) => Ok(result),
        Err(_) => Err(Error::api()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::next();
        assert!(result.is_ok(), format!("{:?}", result));
    }
}

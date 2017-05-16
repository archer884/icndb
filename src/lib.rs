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

use hyper::Client;
use std::io::Read;

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

/// Get a random joke from the ICNDB.
///
/// Returns an option value containing a random joke from the API
/// or, failing that, None.
pub fn next() -> Option<Joke> {
    unwrap_response(execute_request("://api.icndb.com/jokes/random"))
}

/// Get a random joke from the ICNDB, replacing the names in the joke.
///
/// Returns an option value containing a random joke from the API
/// using the names supplied to the function instead of the default
/// name (Chuck Norris) or, failing that, None.
pub fn next_with_names(first: &str, last: &str) -> Option<Joke> {
    let request_url = format!("://api.icndb.com/jokes/random?firstName={}&lastName={}", first, last);
    unwrap_response(execute_request(&request_url))
}

/// Get a specific joke from the ICNDB.
///
/// Returns an option value containing a specified joke from the API
/// or, failing that, None.
pub fn get_by_id(id: u64) -> Option<Joke> {
    unwrap_response(execute_request(&format!("://api.icndb.com/jokes/{}", id)))
}

/// Get a specific joke with specified names.
///
/// Returns an option value containing a specified joke from the API
/// using the names supplied to the function instead of the default
/// name (Chuck Norris) or, failing that, None.
pub fn get_by_id_with_names(id: u64, first: &str, last: &str) -> Option<Joke> {
    let request_url = format!("://api.icndb.com/jokes/{}?firstName={}&lastName={}", id, first, last);
    unwrap_response(execute_request(&request_url))
}

// Parses the response returned by a query against the ICNDB API
// into an Joke or, failing that, None.
fn unwrap_response(response: Option<String>) -> Option<Joke> {
    let raw_response = match response {
        Some(response) => response,
        None => return None,
    };

    serde_json::from_str::<ApiResponseWrapper>(&raw_response).ok()
        .and_then(|result| unescape_content(result.value.into()))
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
fn unescape_content(response: Joke) -> Option<Joke> {
    if response.content.contains("&quot;") {
        Some(Joke {
            id: response.id,
            content: response.content.replace("&quot;", "\""),
            categories: response.categories,
        })
    } else {
        Some(response)
    }
}

#[cfg(not(feature = "ssl"))]
fn execute_request(request: &str) -> Option<String> {
    let client = Client::new();
    client.get(&format!("http{}", request))
        .send()
        .map(|mut res| {
            let mut buf = String::new();
            res.read_to_string(&mut buf).ok();
            buf
        })
        .ok()
}

#[cfg(feature = "ssl")]
fn execute_request(request: &str) -> Option<String> {
    use hyper::net::HttpsConnector;
    use hyper_native_tls::NativeTlsClient;

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    client.get(&format!("https{}", request))
        .send()
        .map(|mut res| {
            let mut buf = String::new();
            res.read_to_string(&mut buf).ok();
            buf
        })
        .ok()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = super::next();
        assert!(result.is_some(), format!("{:?}", result));
    }
}

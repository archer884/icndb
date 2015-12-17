//! `icndb` provides a simple interface to `api.icndb.com` and is
//! designed especially for use with `fun`. Extracting this functionality
//! into a library grants us greater flexibility with regard to our
//! base application and will permit a large number of improvements
//! in the future.
//!
//! ## Example
//!
//! The code below calls the API to get a random joke with the custom
//! name "Maximus Hardcorion", because why would you not?
//!
//! ```
//! extern crate icndb;
//!
//! let response = icndb::next_with_names("Maximus", "Hardcorion").unwrap();
//!
//! assert!(response.joke.contains("Maximus Hardcorion"));
//! ```
//!
//! The big thing to keep in mind, here, is that Maximus Hardcorion is
//! Chuck Norris' Latin name.

extern crate hyper;
extern crate rustc_serialize;

use hyper::Client;
use rustc_serialize::json;
use std::io::Read;

/// Wraps an API response from the `api.icndb.com`. The authors'
/// intent appears to have been to provide an interface for both
/// failed and successful requests, but it has been difficult to
/// represent the full wrapper in Rust, and the wrapper adds no
/// real value.
#[derive(RustcDecodable, RustcEncodable)]
struct ApiResponseWrapper {
    // type: String, // terrible field name
    value: ApiResponse,
}

/// Represents a single joke provided by the ICNDB. The `id` field
/// uniquely identifies this specific joke, which allows the user
/// to get this joke again at a later time if he or she so desires.
#[derive(RustcDecodable, RustcEncodable)]
pub struct ApiResponse {
    pub id: u64,
    pub joke: String,
    pub categories: Box<[String]>
}

/// Returns an option value containing a random joke from the API
/// or, failing that, None.
pub fn next() -> Option<ApiResponse> {
    unwrap_response(execute_request("http://api.icndb.com/jokes/random"))
}

/// Returns an option value containing a random joke from the API
/// using the names supplied to the function instead of the default
/// name (Chuck Norris) or, failing that, None.
pub fn next_with_names(first: &str, last: &str) -> Option<ApiResponse> {
    unwrap_response(execute_request(&format!("http://api.icndb.com/jokes/random?firstName={}&lastName={}", first, last)))
}

/// Returns an option value containing a specified joke from the API
/// or, failing that, None.
pub fn get_by_id(id: u64) -> Option<ApiResponse> {
    unwrap_response(execute_request(&format!("http://api.icndb.com/jokes/{}", id)))
}

/// Returns an option value containing a specified joke from the API
/// using the names supplied to the function instead of the default
/// name (Chuck Norris) or, failing that, None.
pub fn get_by_id_with_names(id: u64, first: &str, last: &str) -> Option<ApiResponse> {
    unwrap_response(
        execute_request(
            &format!(
                "http://api.icndb.com/jokes/{}?firstName={}&lastName={}",
                id,
                first,
                last)))
}

/// Parses the response returned by a query against the ICNDB API
/// into an ApiResponse or, failing that, None.
fn unwrap_response(response: Option<String>) -> Option<ApiResponse> {
    let raw_response = match response {
        Some(response) => response,
        None => return None,
    };

    match json::decode::<ApiResponseWrapper>(&raw_response) {
        Ok(result) => unescape_content(result.value),
        _ => None
    }
}

/// Unescape HTML entities found in joke contents.
///
/// The ICNDB represents some values as HTML entities in the json
/// packets returned to the caller and rustc_deserialize does not
/// unescape these entities upon deserializing the json packet.
/// This function deals with that by taking the (potentially) escaped
/// values and unescaping any HTML entities contained therein before
/// returning a new ApiResponse struct containing the unescaped
/// values.
///
/// ** Entities handled include: **
/// - &quot;
///
/// Hopefully we won't discover anymore.
fn unescape_content(response: ApiResponse) -> Option<ApiResponse> {
    if response.joke.contains("&quot;") {
        Some(ApiResponse {
            id: response.id,
            joke: response.joke.replace("&quot;", "\""),
            categories: response.categories
        })
    } else {
        Some(response)
    }
}

/// Executes an arbitrary request against the ICNDB API.
fn execute_request(request: &str) -> Option<String> {
    let client = Client::new();
    client.get(request).send().map(|mut res| {
        let mut buf = String::new();
        res.read_to_string(&mut buf).ok();
        buf
    }).ok()
}

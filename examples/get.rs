extern crate icndb;

fn main() {
    let client = icndb::ApiClient::new();
    let joke = client.get_by_id(23);

    if let Some(joke) = joke.ok() {
        // Time waits for no man. Unless that man is Chuck Norris.
        println!("{}", joke.content);
    }
}

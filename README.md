# ICNDB

> Provides a barebones API for the most important database in the universe.

## Usage

```rust
let client = icndb::ApiClient::new();
let joke = client.get_by_id(23);

if let Some(joke) = joke.ok() {
    // Time waits for no man. Unless that man is Chuck Norris.
    println!("{}", joke.content);
}
```

I don't currently provide any options with regard to creating the client; if you want TLS, compile it in. If not, don't.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE][apc] or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[apc]:https://github.com/archer884/icndb/blob/master/LICENSE-APACHE
[mit]:https://github.com/archer884/icndb/blob/master/LICENSE-MIT

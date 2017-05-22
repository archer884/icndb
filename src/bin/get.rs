extern crate icndb;

fn main() {
    let identifiers = std::env::args().skip(1).filter_map(|n| n.parse::<u64>().ok());
    let client = icndb::ApiClient::new();

    for id in identifiers {
        println!("{:?}", client.get_by_id(id));
    }
}

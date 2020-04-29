extern crate bson;
extern crate mongodb;
extern crate serde;
extern crate serde_json;
use mongodb::{options::ClientOptions, Client};

fn main() {
    let uri = "mongodb://localhost:27017/";

    // Setup options
    let mut options = ClientOptions::parse(uri).unwrap();
    options.direct_connection = Some(true);

    // Create the Client
    let client = Client::with_options(options).unwrap();

    let db = client.database("bitcoin");
    let coll = db.collection("blocks");
    let cursor = coll.find(None, None).ok().expect("Failed to execute find.");

    let docs: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();

    let serialized = serde_json::to_string(&docs).unwrap();

    println!("{}", serialized);
}

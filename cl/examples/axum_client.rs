use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// the output to our `create_user` handler
#[derive(Serialize, Debug, Deserialize)]
struct User {
    id: u64,
    username: String,
}

#[tokio::main]
async fn main() {
    let mut map = HashMap::new();
    map.insert("username", "rust");

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:3000/users")
        .json(&map)
        .send()
        .await.expect("client.post");
    // println!("{:#?}", res);

    match res.json::<User>().await {
        Ok(parsed) => println!("Success! {:?}", parsed),
        Err(_) => println!("Hm, the response didn't match the shape we expected."),
    };
}

use cl::server::SaveRequest;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let mut map = HashMap::new();
    map.insert("opt", "put");
    map.insert("opt", "hello_world.file");

    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3000/users")
        .json(&map)
        .send()
        .await
        .expect("client.post");
    // println!("{:#?}", res);

    match res.json::<SaveRequest>().await {
        Ok(parsed) => println!("Success! {:?}", parsed),
        Err(_) => println!("Hm, the response didn't match the shape we expected."),
    };
}

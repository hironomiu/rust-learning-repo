use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Post {
    #[serde(rename(serialize = "userId", deserialize = "userId"))]
    user_id: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn fetch(request_url: &String) -> Result<Vec<Post>, Error> {
    let response = reqwest::get(request_url).await?;
    let json_data: Vec<Post> = match response.json::<Vec<Post>>().await {
        Ok(parsed) => parsed,
        Err(_) => panic!("error"),
    };
    Ok(json_data)
}

#[derive(Deserialize)]
struct Res {
    id: u32,
}

#[tokio::main]
async fn post(request_url: &String) -> Result<(), Error> {
    let response = reqwest::Client::new().post(request_url).send().await?;
    match response.json::<Res>().await {
        Ok(parsed) => println!("id: {}", parsed.id),
        Err(_) => println!("error"),
    }
    Ok(())
}

fn main() -> Result<(), Error> {
    let request_url: String = String::from("https://jsonplaceholder.typicode.com/posts");

    let json_data: Vec<Post> = match fetch(&request_url) {
        Ok(json_data) => json_data,
        Err(_) => panic!("error"),
    };

    for data in json_data {
        println!("{},{},{},{}", data.user_id, data.id, data.title, data.body);
    }

    match post(&request_url) {
        Ok(_) => (),
        Err(_) => (),
    };
    Ok(())
}

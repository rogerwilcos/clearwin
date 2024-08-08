use reqwest::{header::CONTENT_TYPE, Client};
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
pub async fn http_get(uri: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(uri)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    //  .json::<GETAPIResponse>()
    //  .await?;

    // println!("{:#?}", resp);

    match resp.status() {
        reqwest::StatusCode::OK => {
            println!("Success!");
            println!("resp:{:?} ", resp);
            // let resp_json = resp.json::<GETAPIResponse>().await?;
            Ok(())
        }
        // "NOT_FOUND - 404
        // reqwest::StatusCode::NOT_FOUND => {
        //     println!("Got 404! Haven't found resource!");
        //     let resp_404 = resp.json::<GETAPIResponse>().await?;
        //     println!("{:#?}", resp_404);
        //     Ok(resp_404)
        // }
        _ => {
            panic!("Okay... this shouldn't happen...");
        }
    }
}

#[tokio::main]
pub async fn http_post(client: &Client, uri: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let resp_json = client
        .post(uri)
        // .json(&map)
        .send()
        .await?;
    // .json::<JSONResponse>()
    // .await?;

    println!("{:#?}", resp_json);

    Ok(())
}

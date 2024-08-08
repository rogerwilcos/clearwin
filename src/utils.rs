use reqwest::header::{ACCEPT, AUTHORIZATION, X_CONTENT_TYPE_OPTIONS};
use reqwest::{header::CONTENT_TYPE, Client};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use teloxide::dispatching::dialogue::serializer::Json;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

//, "gists_url\":\"https://api.github.com/users/rogerwilcos/gists{/gist_id}\",\"starred_url\":\"https://api.github.com/users/rogerwilcos/starred{/owner}{/repo}\",\"subscriptions_url\":\"https://api.github.com/users/rogerwilcos/subscriptions\",\"organizations_url\":\"https://api.github.com/users/rogerwilcos/orgs\",\"repos_url\":\"https://api.github.com/users/rogerwilcos/repos\",\"events_url\":\"https://api.github.com/users/rogerwilcos/events{/privacy}\",\"received_events_url\":\"https://api.github.com/users/rogerwilcos/received_events\",\"type\":\"User\",\"site_admin\":false,\"name\":\"Roger Wilco\",\"company\":null,\"blog\":\"\",\"location\":\"Bishkek\",\"email\":null,\"hireable\":null,\"bio\":\"rubbish programmer\",\"twitter_username\":null,\"public_repos\":10,\"public_gists\":0,\"followers\":0,\"following\":1,\"created_at\":\"2016-01-07T14:15:18Z\",\"updated_at\":\"2024-08-03T07:28:42Z\",\"private_gists\":3,\"total_private_repos\":0,\"owned_private_repos\":0,\"disk_usage\":99545,\"collaborators\":0,\"two_factor_authentication\":false,\"plan\":{\"name\":\"free\",\"space\":976562499,\"collaborators\":0,\"private_repos\":10000}}"

#[derive(Deserialize, Serialize, Debug)]
struct UserInfo {
    login: String,
    id: u64,
    node_id: String,
    // avatar: String,
    // gravatar: String,
    // url: String,
    // html_url: String,
    // followers_url: String,
    r#type: String,
}

// {
//     "login": "rogerwilcos",
//     "id": 16594560,
//     "node_id": "MDQ6VXNlcjE2NTk0NTYw",
//     "avatar_url": "https://avatars.githubusercontent.com/u/16594560?v=4",
//     "gravatar_id": "",
//     "url": "https://api.github.com/users/rogerwilcos",
//     "html_url": "https://github.com/rogerwilcos",
//     "followers_url": "https://api.github.com/users/rogerwilcos/followers",
//     "following_url": "https://api.github.com/users/rogerwilcos/following{/other_user}",
//     "gists_url": "https://api.github.com/users/rogerwilcos/gists{/gist_id}",
//     "starred_url": "https://api.github.com/users/rogerwilcos/starred{/owner}{/repo}",
//     "subscriptions_url": "https://api.github.com/users/rogerwilcos/subscriptions",
//     "organizations_url": "https://api.github.com/users/rogerwilcos/orgs",
//     "repos_url": "https://api.github.com/users/rogerwilcos/repos",
//     "events_url": "https://api.github.com/users/rogerwilcos/events{/privacy}",
//     "received_events_url": "https://api.github.com/users/rogerwilcos/received_events",
//     "type": "User",
//     "site_admin": false,
//     "name": "Roger Wilco",
//     "company": null,
//     "blog": "",
//     "location": "Bishkek",
//     "email": null,
//     "hireable": null,
//     "bio": "rubbish programmer",
//     "twitter_username": null,
//     "public_repos": 10,
//     "public_gists": 0,
//     "followers": 0,
//     "following": 1,
//     "created_at": "2016-01-07T14:15:18Z",
//     "updated_at": "2024-08-03T07:28:42Z",
//     "private_gists": 3,
//     "total_private_repos": 0,
//     "owned_private_repos": 0,
//     "disk_usage": 99545,
//     "collaborators": 0,
//     "two_factor_authentication": false,
//     "plan": {
//         "name": "free",
//         "space": 976562499,
//         "collaborators": 0,
//         "private_repos": 10000
//     }
// }
#[tokio::main]
pub async fn http_get(uri: &str) -> Result<(), Box<dyn Error>> {
    let token = env::var("TELOXIDE_TOKEN").unwrap();

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let resp = client
        .get(uri)
        .header(ACCEPT, "application/vnd.github+json")
        .header(AUTHORIZATION, "Bearer ".to_owned() + &token)
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            println!("Success!");
            // println!("{}", resp.text().await?);
            let resp_json = resp.json::<UserInfo>().await?;
            println!("response json: {:?}", resp_json.r#type);
            Ok(())
        }
        reqwest::StatusCode::FORBIDDEN => {
            println!("403 Forbidden!");
            println!("resp: {:?} ", resp);
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
            println!("ERROR: resp.status: {}", resp.status());
            Ok(())
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

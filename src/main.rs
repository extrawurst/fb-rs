use std::env;

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: Option<String>,
    id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Picture {
    height: i32,
    is_silhouette: bool,
    url: String,
    width: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseFriends {
    data: Option<Vec<User>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ResponsePicture {
    data: Option<Picture>,
}

#[derive(Default, Debug)]
struct GraphAPI {}

const BASE_URL: &str = "https://graph.facebook.com/v8.0";

impl GraphAPI {
    async fn me(&self, token: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/me?fields=id%2Cname%2Cfirst_name%2Clast_name%2Cemail&access_token={}",
            BASE_URL, token
        );

        let url: Url = url.parse()?;

        let resp = reqwest::get(url).await?.json::<User>().await?;

        Ok(resp)
    }

    async fn friends(
        &self,
        token: &str,
    ) -> Result<ResponseFriends, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/me/friends?fields=id%2Cname%2Cfirst_name%2Clast_name&access_token={}",
            BASE_URL, token
        );

        let url: Url = url.parse()?;

        let resp = reqwest::get(url).await?.json::<ResponseFriends>().await?;

        Ok(resp)
    }

    async fn my_picture(
        &self,
        token: &str,
    ) -> Result<ResponsePicture, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "{}/me/picture?redirect=false&access_token={}",
            BASE_URL, token
        );

        let url: Url = url.parse()?;

        let resp = reqwest::get(url).await?.json::<ResponsePicture>().await?;

        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = env::var("ACCESS_TOKEN").unwrap_or_default();
    let me = GraphAPI::default().me(&token).await?;
    println!("me:\n{:#?}", me);

    let friends = GraphAPI::default().friends(&token).await?;
    println!("friends:\n{:#?}", friends.data);

    let pic = GraphAPI::default().my_picture(&token).await?;
    println!("pic:\n{:#?}", pic.data);

    Ok(())
}

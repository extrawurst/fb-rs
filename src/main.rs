use std::env;

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ResponseMe {
    name: Option<String>,
    id: Option<String>,
}

#[derive(Default, Debug)]
struct GraphAPI {}

impl GraphAPI {
    async fn me(
        &self,
        token: &str,
    ) -> Result<ResponseMe, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://graph.facebook.com/v8.0/me?fields=id%2Cname%2Cemail&access_token={}",
            token
        );

        let url: Url = url.parse()?;

        let resp = reqwest::get(url).await?.json::<ResponseMe>().await?;

        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = env::var("ACCESS_TOKEN").unwrap_or_default();
    let me = GraphAPI::default().me(&token).await?;
    println!("{:#?}", me);

    Ok(())
}

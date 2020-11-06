use crate::{
    error::Result,
    types::{ResponseFriends, ResponsePicture, User},
};
use async_trait::async_trait;
use reqwest::Url;

#[async_trait]
pub trait GraphAPI {
    async fn me(&self, token: &str) -> Result<User>;
    async fn friends(&self, token: &str) -> Result<ResponseFriends>;
    async fn my_picture(&self, token: &str) -> Result<ResponsePicture>;
}

#[derive(Default, Debug)]
pub struct FBGraphAPI {}

const BASE_URL: &str = "https://graph.facebook.com/v8.0";

#[async_trait]
impl GraphAPI for FBGraphAPI {
    async fn me(&self, token: &str) -> Result<User> {
        let url = format!(
            "{}/me?fields=id%2Cname%2Cfirst_name%2Clast_name%2Cemail&access_token={}",
            BASE_URL, token
        );

        let url: Url = url.parse()?;

        let resp = reqwest::get(url).await?.json::<User>().await?;

        Ok(resp)
    }

    async fn friends(&self, token: &str) -> Result<ResponseFriends> {
        let url = format!(
            "{}/me/friends?fields=id%2Cname%2Cfirst_name%2Clast_name&access_token={}",
            BASE_URL, token
        );

        let url: Url = url.parse()?;

        let resp = reqwest::get(url).await?.json::<ResponseFriends>().await?;

        Ok(resp)
    }

    async fn my_picture(&self, token: &str) -> Result<ResponsePicture> {
        let url = format!(
            "{}/me/picture?redirect=false&access_token={}",
            BASE_URL, token
        );

        let url: Url = url.parse()?;

        let resp = reqwest::get(url).await?.json::<ResponsePicture>().await?;

        Ok(resp)
    }
}

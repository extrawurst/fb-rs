mod api;
mod error;
mod types;

use api::{FBGraphAPI, GraphAPI};
use error::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("ACCESS_TOKEN").unwrap_or_default();
    let me = FBGraphAPI::default().me(&token).await?;
    println!("me:\n{:#?}", me);

    let friends = FBGraphAPI::default().friends(&token).await?;
    println!("friends:\n{:#?}", friends.data);

    let pic = FBGraphAPI::default().my_picture(&token).await?;
    println!("pic:\n{:#?}", pic.data);

    Ok(())
}

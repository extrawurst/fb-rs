mod api;
mod error;
mod types;

use api::{FBGraphAPI, GraphAPI};
use error::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("ACCESS_TOKEN").unwrap_or_default();
    let api = FBGraphAPI::default();

    let me = api.me(&token).await?;
    println!(
        "me:\n id:\t{}\n name:\t{}",
        me.id.unwrap(),
        me.name.unwrap()
    );

    let friends = api.friends(&token).await?;
    println!("friends:\n{:#?}", friends.data);

    let pic = api.my_picture(&token).await?;
    println!("pic:\n{:#?}", pic.data);

    Ok(())
}

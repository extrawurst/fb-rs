# fb-rs
Facebook Graph API for Rust

Supported endpoints:

* `/me`
* `/me/friends`
* `/me/picture`

Example:

```rust
let api:GraphAPI = FBGraphAPI::default();

let me = api.me("<CLIENT_TOKEN>").await?;
println!(
    "me:\n id:\t{}\n name:\t{}",
    me.id.unwrap(),
    me.name.unwrap()
);
```

prints:
```
me:
 id:    <fbid>
 name:  <your fb name>
```
use serenity::all::standard::macros::*;
use serenity::all::Message;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};

static DEFAULT_PLAYLIST_ID: &str = "PLSVx1ksr-QK3hDUzO3s1zUNgcq4akDOA8";

#[command]
#[aliases("bp")]
#[only_in(guilds)]
#[owners_only(true)]
async fn bind_playlist(_context: &Context, _message: &Message, args: Args) -> CommandResult {
    let playlist_id = args.clone().single::<String>().or::<String>(Ok(DEFAULT_PLAYLIST_ID.to_string())).unwrap();
    println!("{playlist_id:?}");

    let res = reqwest::get("https://www.googleapis.com/youtube/v3/channels?part=contentDetails").await.unwrap();
    println!("{res:#?}");
    Ok(())
}

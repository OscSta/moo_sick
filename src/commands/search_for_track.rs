use reqwest;
use std::env;

use serenity::all::standard::macros::*;
use serenity::all::Message;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};

const MAX_RESULTS: u32 = 1;

#[command]
#[aliases("qs", "search")]
#[only_in(guilds)]
#[owners_only(true)]
async fn search_for_track(_context: &Context, _message: &Message, args: Args) -> CommandResult {
    let search_term = args.parse::<String>();
    if search_term.is_err() {
        eprintln!("Error parsing args as a search query");
        return Ok(());
    }
    let search_term = search_term.unwrap();
    let yt_api_key = env::var("YT_API_KEY").expect("Should find YoutTube API key");

    let response = reqwest::get(format!(
        "https://youtube.googleapis.com/youtube/v3/search?part=snippet&type=video&maxResults={}&q={}&key={}",
        MAX_RESULTS, search_term, yt_api_key,
    )).await.unwrap().json::<serde_json::Value>().await.unwrap();

    let video_json = &response["items"][0];
    let video_id = video_json["id"]["videoId"].clone().to_string();
    println!("Video ID is: {}", video_id);

    Ok(())
}

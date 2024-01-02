use reqwest;
use std::env;

use serenity::all::standard::macros::*;
use serenity::all::Message;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult, Delimiter};

use crate::commands::queue_track;

const MAX_RESULTS: u32 = 1;

#[command]
#[aliases("qs", "search")]
#[only_in(guilds)]
#[owners_only(false)]
async fn search_for_track(context: &Context, message: &Message, args: Args) -> CommandResult {
    let search_term = args.message();
    if search_term.is_empty() {
        eprintln!("Error parsing args as a search query");
        return Ok(());
    }
    // let search_term = search_term.unwrap();
    let yt_api_key = env::var("YT_API_KEY").expect("Should find YoutTube API key");

    let response = reqwest::get(
        format!(
            "https://youtube.googleapis.com/youtube/v3/search?part=snippet&type=video&maxResults={}&q={}&key={}",
            MAX_RESULTS,
            search_term,
            yt_api_key
        )
    )
    .await
    .expect("Failed to unwrap response from request")
    .json::<serde_json::Value>()
    .await
    .expect("Failed to unwrap response JSON");

    let video_json = &response["items"][0];
    let video_id = video_json["id"]["videoId"].clone().to_string().replace("\"", "");
    println!(
        "Video ID found for query |{}| is: {}",
        search_term, video_id
    );

    // println!("Passing {} on to queue_track", format!("https://www.youtube.com/watch?v={}", video_id).as_str());
    queue_track::queue_track(
        context,
        message,
        Args::new(
            format!("https://www.youtube.com/watch?v={}", video_id).as_str(),
            &[Delimiter::Single(' ')]
        ),
    ).await.ok();

    Ok(())
}

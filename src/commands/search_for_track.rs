use reqwest;
use serenity::builder::CreateEmbed;
use std::env;

use serenity::all::standard::macros::*;
use serenity::all::Message;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult, Delimiter};

use crate::commands::queue_track;

const MAX_RESULTS: u32 = 5;

// Vec<title, artist, id>
async fn user_choose_track(choices: &Vec<(&str, &str, &str)>, yt_api_key: String) -> String {
    // let response = reqwest::get(format!(
    //     "https://youtube.googleapis.com/youtube/v3/videos?part=fileDetails&id={}&key={}",
    //     id, yt_api_key
    // ))
    // .await
    // .unwrap()
    // .json::<serde_json::Value>()
    // .await
    // .unwrap();
    "WtOskFeLmr4".to_string()
}

#[command]
#[aliases("qs", "search")]
#[only_in(guilds)]
#[owners_only(false)]
async fn search_for_track(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let search_term = Args::new(args.message(), &[Delimiter::Single(',')])
        .single::<String>()
        .unwrap();
    let num_results = args.single::<u32>().unwrap_or(MAX_RESULTS);
    if search_term.is_empty() {
        eprintln!("Error parsing args as a search query");
        return Ok(());
    }
    // let search_term = search_term.unwrap();
    let yt_api_key = env::var("YT_API_KEY").expect("Should find YoutTube API key");

    let response = reqwest::get(
        format!(
            "https://youtube.googleapis.com/youtube/v3/search?part=snippet&type=video&maxResults={}&q={}&key={}",
            num_results,
            search_term,
            yt_api_key
        ))
        .await
        .expect("Failed to unwrap response from request")
        .json::<serde_json::Value>()
        .await
        .expect("Failed to unwrap response JSON");

    let items = &response["items"];
    let item0_json = &items[0];
    let item0_id = item0_json["id"]["videoId"].as_str().unwrap();
    let item0_link = format!("https://www.youtube.com/watch?v={}", item0_id);
    let item_iter = items.as_array().unwrap();

    let mut choices: Vec<(&str, &str, &str)> = Vec::new();
    for item in item_iter {
        let item_map = item.as_object().unwrap();
        // This is kinda ugly
        let item_id = item["id"]["videoId"].as_str().unwrap();
        let item_snippet = item_map.get("snippet").unwrap().as_object().unwrap();
        let channel_name = item_snippet.get("channelTitle").unwrap().as_str().unwrap();
        let item_title = item_snippet.get("title").unwrap().as_str().unwrap();
        // println!("Item is {} {} {}", item_id, channel_name, item_title);
        choices.push((item_title, channel_name, item_id));
    }

    let chosen_id = user_choose_track(&choices, yt_api_key).await;

    println!(
        "Video ID chosen for query |{}| is: {} - passing on {} to track queue",
        search_term, item0_id, &item0_link
    );

    queue_track::queue_track(
        context,
        message,
        Args::new(&item0_link, &[Delimiter::Single(' ')]),
    )
    .await
    .ok();

    Ok(())
}

use reqwest;
use serenity::builder::{CreateEmbed, CreateMessage};
use std::env;
use std::time::Duration;

use serenity::all::standard::macros::*;
use serenity::all::Message;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult, Delimiter};

use crate::music::commands::queue_track;

const MAX_RESULTS: u32 = 5;

// Vec<id>
async fn user_choose_track(
    choices: &Vec<&str>,
    yt_api_key: String,
    message: &Message,
    context: &Context,
) -> Option<String> {
    let joined_choices = choices
        .iter()
        .map(|choice| choice.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let response = reqwest::get(format!(
        "https://youtube.googleapis.com/youtube/v3/videos?part=snippet,contentDetails&id={}&key={}",
        joined_choices, yt_api_key
    ))
    .await
    .unwrap()
    .json::<serde_json::Value>()
    .await
    .unwrap();

    let embed_items = response["items"]
        .as_array()
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(i, serde_value)| {
            let duration: iso8601::Duration =
                iso8601::duration(serde_value["contentDetails"]["duration"].as_str().unwrap())
                    .unwrap();
            let (minutes, seconds) = match duration {
                iso8601::Duration::YMDHMS {
                    hour,
                    minute,
                    second,
                    ..
                } => {
                    let minutes = u32::from(minute) + 60 * u32::from(hour);
                    let seconds = u32::from(second);
                    (minutes, seconds)
                }
                _ => (0, 0),
            };
            let title = format!(
                "{}. {} ({}:{:0>2})",
                i + 1,
                serde_value["snippet"]["title"].as_str().unwrap(),
                minutes,
                seconds,
            );
            let artist = format!(
                "Uploaded by {}",
                serde_value["snippet"]["channelTitle"].as_str().unwrap()
            );
            (title, artist, false)
        })
        .collect::<Vec<(String, String, bool)>>();

    let embed = CreateEmbed::new().title("Choose song").fields(embed_items);
    let msg_builder = CreateMessage::new().embed(embed);
    let msg = message
        .channel_id
        .send_message(&context.http, msg_builder)
        .await;
    if let Err(why) = &msg {
        eprintln!("Error sending embed message: {why:?}");
    }

    let collector = message
        .author
        .await_reply(&context.shard)
        .timeout(Duration::from_secs(30));

    // This paragraph is a bit of a mess, but it works fine
    let selection;
    if let Some(content) = collector.await {
        selection = content.content.parse::<usize>().unwrap_or(0);
    } else {
        let _ = message.reply(&context.http, "Timed Out").await;
        return None;
    }
    if (1..choices.len()).contains(&selection) {
        let track_id = choices[selection - 1];
        msg.unwrap().delete(&context).await.unwrap();
        return Some(track_id.to_string());
    } else {
        msg.unwrap().delete(&context).await.unwrap();
        return None;
    }
}

#[command]
#[aliases("qs", "search")]
#[only_in(guilds)]
#[owners_only(false)]
async fn search_for_track(context: &Context, message: &Message, args: Args) -> CommandResult {
    let mut args = Args::new(args.message(), &[Delimiter::Single(',')]);
    let search_term = args.single::<String>().unwrap();
    let num_results = args.single::<u32>().unwrap_or(MAX_RESULTS);
    if search_term.is_empty() {
        eprintln!("Error parsing a search query");
        return Ok(());
    }

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
    // let item0_json = &items[0];
    // let item0_id = item0_json["id"]["videoId"].as_str().unwrap();
    // let item0_link = format!("https://www.youtube.com/watch?v={}", item0_id);
    let item_iter = items.as_array().unwrap();

    let mut choices: Vec<&str> = Vec::new();
    for item in item_iter {
        let item_id = item["id"]["videoId"].as_str().unwrap();
        choices.push(item_id);
    }
    let chosen_id = user_choose_track(&choices, yt_api_key, message, context).await;
    let chosen_id_link = if chosen_id.is_some() {
        format!(
            "https://www.youtube.com/watch?v={}",
            chosen_id.clone().unwrap()
        )
    } else {
        return Ok(());
    };

    println!(
        "Video ID chosen for query |{}| is: {} - passing on {} to track queue",
        search_term,
        chosen_id.unwrap(),
        chosen_id_link
    );
    queue_track::queue_track_from_link(
        context,
        message,
        Args::new(&chosen_id_link, &[Delimiter::Single(' ')]),
    )
    .await
    .ok();

    Ok(())
}

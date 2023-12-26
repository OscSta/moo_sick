use serenity::all::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::model::prelude::Message;
use serenity::{framework::standard::CommandResult, prelude::Context};
use songbird::input::{YoutubeDl, Input};
use songbird::typemap::TypeMapKey;

use reqwest::Client as HttpClient;

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[command]
#[only_in(guilds)]
async fn play(context: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            let _ = msg
                .channel_id
                .say(&context.http, "Must provied URL to playable")
                .await;
            return Ok(());
        }
    };

    let do_search = !url.starts_with("http");
    let guild_id = msg.guild_id.unwrap();

    let http_client = {
        let data = context.data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Could not find HTTP Client in typemap")
    };

    let manager = songbird::get(context)
        .await
        .expect("Could not get songbird client")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let src;
        if do_search {
            let _ = msg
                .channel_id
                .say(&context.http, "Please include link starting with http")
                .await;
            return Ok(());
        } else {
            src = YoutubeDl::new(http_client, url);
        };
        let _th = handler
            .enqueue_input(
                src.clone()
                .into()) // Here .into() turns the YoutubeDl struct into an Input struct since .play_input expects an Input struct. I assume?
                .await
            .set_volume(0.4);
        if let Ok(audio_meta) = Input::from(src).aux_metadata().await {
            let _ = msg.channel_id.say(&context.http, format!("Playing {}", audio_meta.title.unwrap_or("audio".to_string()))).await;
        }
        println!("Playing audio");
    } else {
        let _ = msg
            .channel_id
            .say(
                &context.http,
                "Could not play playable, are you in a voice channel?",
            )
            .await;
    }

    Ok(())
}

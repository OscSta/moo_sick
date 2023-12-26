use serenity::all::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::model::prelude::Message;
use serenity::{framework::standard::CommandResult, prelude::Context};
use songbird::input::{Input, YoutubeDl};
use songbird::typemap::TypeMapKey;

use reqwest::Client as HttpClient;

pub struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[command]
#[only_in(guilds)]
async fn play(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            let _ = message
                .channel_id
                .say(&context.http, "You must provide a URL to a playable track")
                .await;
            return Ok(());
        }
    };

    let do_search = !url.starts_with("http");
    let guild_id = message.guild_id.unwrap();

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
        let src = if do_search {
            let _ = message
                .channel_id
                .say(&context.http, "Please include a link starting with http")
                .await;
            return Ok(());
        } else {
            YoutubeDl::new(http_client, url)
        };
        let _th = handler
            .enqueue_input(src.clone().into()) // Here .into() turns the YoutubeDl struct into an Input struct since .play_input expects an Input struct. I assume?
            .await
            .set_volume(0.4);

        if let Ok(audio_meta) = Input::from(src).aux_metadata().await {
            let audio_title = audio_meta.title.unwrap_or("audio".to_string());
            let _ = message
                .channel_id
                .say(&context.http, format!("Playing {}", audio_title))
                .await;
            println!("Enqueueing audio - {}", audio_title);
        } else {
            println!("Playing audio - UNKNOWN TITLE");
        }
    } else {
        let _ = message
            .channel_id
            .say(
                &context.http,
                "Could not play track, are you in a voice channel?",
            )
            .await;
    }

    Ok(())
}

use serenity::all::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::model::prelude::Message;
use serenity::{framework::standard::CommandResult, prelude::Context};
use songbird::input::YoutubeDl;
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
        let th = handler.play_input(src.clone().into());
        let _ = msg.channel_id.say(&context.http, "Playing playable").await;
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

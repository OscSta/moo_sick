use std::sync::Arc;

use serenity::all::standard::macros::command;
use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::Message;
use serenity::{framework::standard::CommandResult, prelude::Context};
use songbird::input::{Input, YoutubeDl};
use songbird::tracks::PlayMode;
use songbird::typemap::TypeMapKey;
use songbird::{Event, EventContext, EventHandler as SongbirdEventHandler};

use reqwest::Client as HttpClient;

pub struct HttpKey;
impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

struct SongNowPlayingNotifier {
    channel_id: ChannelId,
    http: Arc<Http>,
}
#[async_trait]
impl SongbirdEventHandler for SongNowPlayingNotifier {
    async fn act(&self, context: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = context {
            for (state, _handle) in *track_list {
                if state.playing == PlayMode::Play {
                    print!("Playing new song");
                    let _ = self
                        .channel_id
                        .say(&self.http, format!(r#"Now playing "{}""#, "song"))
                        .await;
                }
            }
        };
        None
    }
}

#[command]
#[aliases("q")]
#[only_in(guilds)]
async fn queue(context: &Context, message: &Message, mut args: Args) -> CommandResult {
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
                .say(
                    &context.http,
                    r#"Please include a link in the form: "http(s)://[website].[domain]""#,
                )
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
            let audio_title = audio_meta.title.unwrap_or("track".to_string());
            let _ = message
                .channel_id
                .say(
                    &context.http,
                    format!(r#"Added to queue: "{}""#, audio_title),
                )
                .await;
            println!("Enqueueing audio - {}", audio_title);
        } else {
            println!("Enqueueing audio - Could not fetch metadata");
        }
    } else {
        let _ = message
            .channel_id
            .say(
                &context.http,
                "Could not play track, make sure you and moosick are in the same voice channel",
            )
            .await;
    }

    Ok(())
}

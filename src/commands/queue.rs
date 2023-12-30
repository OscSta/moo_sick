use std::sync::Arc;

use serenity::all::standard::macros::command;
use serenity::async_trait;
use serenity::framework::standard::Args;
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::Message;
use serenity::{framework::standard::CommandResult, prelude::Context};
use songbird::input::Compose;
use songbird::input::YoutubeDl;
use songbird::tracks::PlayMode;
use songbird::typemap::TypeMapKey;
use songbird::TrackEvent;
use songbird::{Event, EventContext, EventHandler as SongbirdEventHandler};

use reqwest::Client as HttpClient;

use crate::commands::join;

pub struct TrackTitle;
impl TypeMapKey for TrackTitle {
    type Value = String;
}

pub struct HttpKey;
impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

struct SongNowPlayingNotifier {
    channel_id: ChannelId,
    http: Arc<Http>,
    track_title: String,
}
#[async_trait]
impl SongbirdEventHandler for SongNowPlayingNotifier {
    async fn act(&self, context: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = context {
            for (state, _handle) in *track_list {
                if state.playing == PlayMode::Play {
                    println!("Playing new song");
                    let _ = self
                        .channel_id
                        .say(&self.http, format!(r#"Now playing "{}""#, self.track_title))
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
#[owners_only(false)]
async fn queue(context: &Context, message: &Message, args: Args) -> CommandResult {
    let url = match args.clone().single::<String>() {
        Ok(url) => url,
        Err(_) => {
            let _ = message
                .channel_id
                .say(&context.http, "You must provide a URL to a playable track")
                .await;
            return Ok(());
        }
    };

    let not_valid_link = !url.starts_with("http");
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

        let src = if not_valid_link {
            let _ = message
                .channel_id
                .say(
                    &context.http,
                    r#"Please include a link in the form: "http(s)://[website]""#,
                )
                .await;
            return Ok(());
        } else {
            YoutubeDl::new(http_client, url)
        };

        let audio_title;
        if let Ok(audio_meta) = src.clone().aux_metadata().await {
            audio_title = audio_meta.title.unwrap_or("track".to_string());
            if !handler.queue().is_empty() { 
                let _ = message
                .channel_id
                .say(
                    &context.http,
                    format!(r#"Added to queue: "{}""#, audio_title),
                )
                .await;
            };
            println!("Enqueueing audio - {}", audio_title);
        } else {
            audio_title = "Next Track".to_string();
            println!("Enqueueing audio - Could not fetch metadata");
        }

        let track_handle = handler
            .enqueue_input(src.clone().into()) // Here .into() turns the YoutubeDl struct into an Input struct since .play_input expects an Input struct. I assume?
            .await;

        let _ = track_handle.add_event(
            TrackEvent::Play.into(),
            SongNowPlayingNotifier {
                channel_id: message.channel_id,
                http: context.http.clone(),
                track_title: audio_title.clone(),
            },
        );

        let _ = track_handle.set_volume(0.4);
        let _ = track_handle
            .typemap()
            .write()
            .await
            .insert::<TrackTitle>(audio_title.clone());
    } else {
        let _ = message
            .channel_id
            .say(
                &context.http,
                "**Warning: Calling ?queue before ?join implicitly calls ?join for you, this can lead to unexpected behaviour**",
            )
            .await;
        let _ = join::join(context, message, args.clone()).await;
        let _ = self::queue(context, message, args).await;
    }

    Ok(())
}

use serenity::all::standard::macros::command;

use serenity::model::prelude::Message;
use serenity::{async_trait, framework::standard::CommandResult, prelude::Context};

use songbird::{Event, EventContext, EventHandler as SongbirdEventHandler, TrackEvent};

struct TrackErrorNotifier;
#[async_trait]
impl SongbirdEventHandler for TrackErrorNotifier {
    async fn act(&self, context: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = context {
            for (state, handle) in *track_list {
                println!(
                    "Track {:?} encountered an error {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        };
        None
    }
}

#[command]
#[aliases("j")]
#[only_in(guilds)]
pub async fn join(context: &Context, message: &Message) -> CommandResult {
    let (guild_id, channel_id) = {
        let guild = message.guild(&context.cache).unwrap();
        let channel_id = guild
            .voice_states
            .get(&message.author.id)
            .and_then(|voice_state| voice_state.channel_id);
        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            let _ = message
                .reply(context, "Join a voice channel before calling ?join")
                .await;
            return Ok(());
        }
    };

    let manager = songbird::get(context)
        .await
        .expect("Could not retrieve songbird client")
        .clone();

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;

        handler.add_global_event(TrackEvent::Error.into(), TrackErrorNotifier);

        if let Ok(channel_name) = connect_to.name(&context.http).await {
            println!("Joined voice channel {:?}", channel_name);
        }
    }

    Ok(())
}

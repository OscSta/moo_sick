use serenity::all::standard::macros::command;
use serenity::model::prelude::Message;
use serenity::{async_trait, framework::standard::CommandResult, prelude::Context};
use songbird::{Event, EventContext, EventHandler, TrackEvent};

pub struct TrackErrorNotifier;
#[async_trait]
impl EventHandler for TrackErrorNotifier {
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
#[only_in(guilds)]
async fn join(context: &Context, msg: &Message) -> CommandResult {
    let (guild_id, channel_id) = {
        let guild = msg.guild(&context.cache).unwrap();
        let channel_id = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);
        (guild.id, channel_id)
    };

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            let _ = msg.reply(context, "User not in voice channel").await;
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
    }

    Ok(())
}

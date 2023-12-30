use crate::TrackTitle;
use serenity::{
    all::Message,
    framework::standard::{macros::command, CommandResult},
    prelude::*,
};

#[command]
#[owners_only(true)]
#[aliases("l")]
#[owners_only(false)]
async fn list(context: &Context, message: &Message) -> CommandResult {
    let guild_id = message.guild_id.unwrap();
    let manager = songbird::get(context)
        .await
        .expect("Could not get songbird client")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if queue.is_empty() {
            println!("Queue is empty");
            return Ok(());
        }

        let current_tracks = queue.current_queue();
        let mut current_tracks_string = String::from("Current track queue:\n");
        for (n, th) in current_tracks.into_iter().take(5).enumerate() {
            let typemap = th.typemap().read().await;
            let audio_title = typemap.get::<TrackTitle>().unwrap();
            current_tracks_string.push_str(format!("{}. \"{}\"\n", n, audio_title).as_str());
        }

        let _ = message
            .channel_id
            .say(&context.http, current_tracks_string)
            .await;
    } else {
        println!("Failed to lock Songbird manager");
    }

    Ok(())
}

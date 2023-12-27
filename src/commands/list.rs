use serenity::{
    all::Message,
    framework::standard::{macros::command, CommandResult},
    prelude::*,
};
use songbird::tracks::PlayMode;

#[command]
#[owners_only(true)]
#[aliases("l")]
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
        for th in current_tracks.into_iter() {
            let _typemap = th.typemap().read().await;
            current_tracks_string.push_str(
                format!(
                    "- {}\n",
                    match th
                        .get_info()
                        .await
                        .expect("Could not get trackhandle info")
                        .playing
                    {
                        PlayMode::Play => {
                            "Playing"
                        }
                        PlayMode::End => {
                            "Has Ended"
                        }
                        _ => {
                            "Unknown"
                        }
                    }
                )
                .as_str(),
            );
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

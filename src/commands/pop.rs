use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[only_in(guilds)]
async fn pop(context: &Context, message: &Message) -> CommandResult {
    let guild_id = message.guild_id.unwrap();
    let manager = songbird::get(context)
        .await
        .expect("Could not get songbird client")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if queue.is_empty() {
            println!("Could not skip current track");
            return Ok(());
        }
        let _ = message
            .channel_id
            .say(&context.http, "Skipping current track")
            .await;
    } else {
        println!("Failed to lock Songbird manager");
    }

    Ok(())
}

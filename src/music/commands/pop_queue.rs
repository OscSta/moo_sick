use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[aliases("skip", "p", "pop")]
#[only_in(guilds)]
#[owners_only(false)]
async fn pop_queue(context: &Context, message: &Message, args: Args) -> CommandResult {
    let guild_id = message.guild_id.unwrap();
    let manager = songbird::get(context)
        .await
        .expect("Could not get songbird client")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        if queue.is_empty() {
            println!("Could not skip current track, queue is empty");
            let _ = message
                .channel_id
                .say(&context.http, "Queue is empty, cannot skip")
                .await;
            return Ok(());
        }

        let skip_count = match args.clone().single::<u32>() {
            Ok(n) => n,
            Err(_) => {
                let _ = queue.skip();
                1
            }
        };

        let _ = message
            .channel_id
            .say(&context.http, format!("Skipping {} track(s)", skip_count))
            .await;
    } else {
        println!("Failed to lock Songbird manager");
    }

    Ok(())
}

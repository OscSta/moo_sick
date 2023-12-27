use serenity::all::standard::macros::command;
use serenity::{all::Message, client::Context, framework::standard::CommandResult};

#[command]
#[aliases("fuckoff")]
#[only_in(guilds)]
async fn leave(context: &Context, message: &Message) -> CommandResult {
    let guild_id = message.guild_id.unwrap();
    let manager = songbird::get(context)
        .await
        .expect("Could not get songbird client")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let _ = handler.leave().await;
        println!("Left channel");
    }

    Ok(())
}

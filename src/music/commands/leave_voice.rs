use serenity::all::standard::macros::command;
use serenity::{all::Message, client::Context, framework::standard::CommandResult};

#[command]
#[aliases("fuckoff", "leave")]
#[only_in(guilds)]
#[owners_only(false)]
async fn leave_voice(context: &Context, message: &Message) -> CommandResult {
    let guild_id = message.guild_id.unwrap();
    let manager = songbird::get(context)
        .await
        .expect("Could not get songbird client")
        .clone();

    match manager.remove(guild_id).await {
        Ok(()) => {
            println!("Left and removed");
        }
        Err(why) => {
            println!("Could not leave and remove: {why}");
        }
    };

    Ok(())
}

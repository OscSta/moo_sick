use serenity::all::standard::macros::*;
use serenity::all::Message;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};

#[command]
#[aliases("s")]
#[only_in(guilds)]
#[owners_only(true)]
async fn bind_playlist(_context: &Context, _message: &Message, _args: Args) -> CommandResult {

    Ok(())
}

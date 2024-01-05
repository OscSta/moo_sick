use serenity::all::standard::macros::*;
use serenity::all::Message;
use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};

#[command]
#[aliases("alias")]
#[only_in(guilds)]
#[owners_only(true)]
async fn command(_context: &Context, _message: &Message, _args: Args) -> CommandResult {
    Ok(())
}

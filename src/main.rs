mod commands;

use serenity::{async_trait, model::channel::Message, model::gateway::Ready, prelude::*};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }

    async fn message(&self, context: Context, msg: Message) {
        if msg.content == "~ping" {
            match msg.channel_id.say(&context.http, "~pong").await {
                Ok(msg) => {
                    println!("Responded with {:?}", msg.content);
                }
                Err(why) => {
                    println!("Error sending message {why:?}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Could not read .env file");
    let discord_bot_token =
        env::var("DISCORD_BOT_TOKEN").expect("Could not find DISCORD_BOT_TOKEN");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&discord_bot_token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

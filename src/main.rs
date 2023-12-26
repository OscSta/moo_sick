mod commands;
use crate::commands::join::*;
use crate::commands::play::*;
use crate::commands::pop::*;

use reqwest::Client as HttpClient;
use serenity::{
    async_trait,
    framework::{
        standard::{macros::group, Configuration},
        StandardFramework,
    },
    model::gateway::Ready,
    client::EventHandler as SerenityEventHandler,
    prelude::*,
};
use songbird::{self, SerenityInit};
use std::env;

struct Handler;

#[group]
#[commands(join, queue, pop)]
struct All;

#[async_trait]
impl SerenityEventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let discord_bot_token =
        env::var("DISCORD_BOT_TOKEN").expect("Could not find DISCORD_BOT_TOKEN");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::non_privileged();

    let framework = StandardFramework::new().group(&ALL_GROUP);
    framework.configure(Configuration::new().prefix("?"));

    let mut client = Client::builder(&discord_bot_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .type_map_insert::<HttpKey>(HttpClient::new())
        .await
        .expect("Error creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended because: {:?}", why));
    });

    let _err_signal = tokio::signal::ctrl_c().await;
    println!("Ctrc-C signal, exiting");
}

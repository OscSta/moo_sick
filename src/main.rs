mod commands;
use crate::commands::join::*;
use crate::commands::leave::*;
use crate::commands::list::*;
use crate::commands::pop::*;
use crate::commands::queue::*;

use reqwest::Client as HttpClient;
use serenity::http::Http;
use serenity::{
    async_trait,
    client::EventHandler as SerenityEventHandler,
    framework::{
        standard::{macros::group, Configuration},
        StandardFramework,
    },
    model::gateway::Ready,
    prelude::*,
};
use songbird::{self, SerenityInit};
use std::collections::HashSet;
use std::env;

struct Handler;

#[group]
#[commands(join, queue, pop, leave, list)]
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
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::non_privileged(); // Don't need all non-priveleged, but idk which ones are neccessary

    let framework = StandardFramework::new().group(&ALL_GROUP);
    framework.configure(
        Configuration::new()
            .owners({
                let http = Http::new(&discord_bot_token);
                match http.get_current_application_info().await {
                    Ok(info) => {
                        let mut owner_set = HashSet::new();
                        if let Some(owner) = &info.owner {
                            owner_set.insert(owner.id);
                            println!("Owner ID is {} with name {}", owner.id, owner.name);
                        }
                        owner_set
                    }
                    Err(why) => panic!("Could not access application info {:?}", why),
                }
            })
            .prefix("?"),
    );

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

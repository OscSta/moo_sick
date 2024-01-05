mod music;
use crate::music::commands::bind_playlist::*;
use crate::music::commands::join_voice::*;
use crate::music::commands::leave_voice::*;
use crate::music::commands::list_current_queue::*;
use crate::music::commands::pop_queue::*;
use crate::music::commands::queue_track::*;
use crate::music::commands::search_for_track::*;

use reqwest::Client as HttpClient;
use serenity::http::Http;
use serenity::{
    async_trait,
    client::EventHandler as SerenityEventHandler,
    framework::{
        standard::{macros::*, Configuration},
        StandardFramework,
    },
    model::gateway::Ready,
    prelude::*,
};
use songbird::{self, SerenityInit};
use std::collections::HashSet;
use std::env;

struct Handler;
#[async_trait]
impl SerenityEventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }
}

#[group]
#[commands(
    join_voice,
    queue_track_from_link,
    pop_queue,
    leave_voice,
    list_current_queue,
    bind_playlist,
    search_for_track
)]
struct All;

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

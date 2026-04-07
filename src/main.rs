use public_ip::addr;
use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use std::process::Command;
struct Handler {
    wol: String,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ip" {
            let gate = addr().await.unwrap();
            if let Err(why) = msg.channel_id.say(&ctx.http, gate.to_string()).await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content == "!wol" {
            Command::new("wol")
                .arg(self.wol.clone())
                .status()
                .expect("failed to execute process");
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let wol_token = env::var("WOL_MAC").expect("Expected WOL token to wake up");
    let handler = Handler { wol: wol_token };
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

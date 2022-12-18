use std::env;
use dotenv::dotenv;

mod commands;

// This trait adds the `register_songbird` and `register_songbird_with` methods
// to the client builder below, making it easy to install this voice client.
// The voice client can be retrieved in any command using `songbird::get(ctx).await`.
use songbird::SerenityInit;

// Import the `Context` to handle commands.
use serenity::client::Context;

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    model::{
        gateway::Ready,
        application::interaction::{Interaction, InteractionResponseType},
        application::command::Command
    },
    prelude::GatewayIntents,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction.clone() {
            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "join" => commands::join::run(&command.data.options, &ctx, &interaction).await.unwrap(),
                "leave" => commands::leave::run(&command.data.options, &ctx, &interaction).await.unwrap(),
                "play" => commands::play::run(&command.data.options, &ctx, &interaction).await.unwrap(),
                "mute" => commands::mute::run(&command.data.options, &ctx, &interaction).await.unwrap(),
                "unmute" => commands::unmute::run(&command.data.options, &ctx, &interaction).await.unwrap(),
                "deafen" => commands::deafen::run(&command.data.options, &ctx, &interaction).await.unwrap(),
                "undeafen" => commands::undeafen::run(&command.data.options, &ctx, &interaction).await.unwrap(),
                _ => "not implemented :(".to_string(),
            };
            
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // register slash commands
        Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        }).await.expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            commands::join::register(command)
        }).await.expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            commands::leave::register(command)
        }).await.expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            commands::play::register(command)
        }).await.expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            commands::mute::register(command)
        }).await.expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            commands::unmute::register(command)
        }).await.expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            commands::deafen::register(command)
        }).await.expect("couldn't create command");
        Command::create_global_application_command(&ctx.http, |command| {
            commands::undeafen::register(command)
        }).await.expect("couldn't create command");
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    // load the bot token from a `.env` file
    let token = env::var("BOT_TOKEN")
        .expect("Expected a token in the environment");

    // specify intents
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });
    
    tokio::signal::ctrl_c().await.expect("failed to listen for Ctrl-C");
    println!("Received Ctrl-C, shutting down.");

}

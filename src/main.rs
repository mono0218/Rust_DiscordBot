use std::env;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::Interaction::ApplicationCommand;

struct Handler;

#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guild_id = GuildId(1161529325674311771);
        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("hello").description("Say hello")
            }).create_application_command(|command| {
                command.name("bye").description("Say Good Bye")
            })
        }).await.unwrap();

        println!("I now have the following guild slash commands: {:#?}",commands);

        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            let userid: String = msg.author.id.to_string();
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Hello <@{userid}>")).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn interaction_create(&self, ctx: Context,interaction: Interaction){
        if let ApplicationCommand(command) = interaction{
            let response_content = match command.data.name.as_str(){
                "hello" => "hello!!".to_owned(),
                "bye" => "good bye!!".to_owned(),
                command=> unreachable!("unknown command: {}",command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response|{
                    response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message|message.content(response_content))
                });

            println!("{:?}",create_interaction_response.await)
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
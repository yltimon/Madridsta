use std::env;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    model::{channel::Message, 
            gateway::Ready, // Correct import for Ready
            prelude::GatewayIntents},
};
use tokio::time::{sleep, Duration};

#[group]
#[commands(uno, sui, vamos, calma)]
struct General; 

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, _ctx: Context, msg: Message) {
        println!("Received a message: {}", msg.content);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    // Load the Discord token from the environment variable
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    
    // Create the client
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    // Set the necessary Gateway Intents
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // Start the bot
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn uno(ctx: &Context, msg: &Message) -> CommandResult {
    // loggin debug
    println!("Received !uno command from {}", msg.author.name);
    // Send "dos"
    msg.channel_id.say(&ctx.http, "dos").await?;

    // Wait for 1 second
    sleep(Duration::from_secs(1)).await;

    // Send "tres"
    msg.channel_id.say(&ctx.http, "tres").await?;

    // Wait for 1 second
    sleep(Duration::from_secs(1)).await;

    // Send the video
    let video_path = "./madrid.mp4"; // Local path to the video file
    msg.channel_id.send_message(ctx, |m| {
        m.content("HALA MADRID")
            .add_file(video_path)
    }).await?;

    Ok(())
}

#[command]
async fn sui(ctx: &Context, msg: &Message) -> CommandResult {
    // Logging debug
    println!("Received !sui command from {}", msg.author.name);

    // Send the video
    let video_path = "./sui.mp4"; // Local path to the new video file
    msg.channel_id.send_message(&ctx.http, |m| {
        m.content("SUIIIIII")
            .add_file(video_path)
    }).await?;

    Ok(())
}

#[command]
async fn vamos(ctx: &Context, msg: &Message) -> CommandResult {
    // Logging debug
    println!("Received !vamos command from {}", msg.author.name);

    // Send the video
    let video_path = "./vamos.mp4"; // Local path to the video file for `vamos`
    msg.channel_id.send_message(&ctx.http, |m| {
        m.content("Vamos!")
            .add_file(video_path)
    }).await?;

    Ok(())
}

#[command]
async fn calma(ctx: &Context, msg: &Message) -> CommandResult {
    // Logging debug
    println!("Received !calma command from {}", msg.author.name);

    // Send the video
    let video_path = "./calma.mp4"; // Local path to the video file for `vamos`
    msg.channel_id.send_message(&ctx.http, |m| {
        m.content("CALMA!")
            .add_file(video_path)
    }).await?;

    Ok(())
}
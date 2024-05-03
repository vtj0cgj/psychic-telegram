use serenity::{
    async_trait,
    model::channel::Message, // Import Message from channel module
    prelude::{Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
};

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Initialize the bot with your Discord bot token
    let token = "YOUR_TOKEN_HERE";

    // Create a new instance of the Discord client
    let mut client = serenity::Client::builder(token)
        .event_handler(Handler)
        .framework(StandardFramework::new().group(&GENERAL_GROUP))
        .await
        .expect("Error creating client");

    // Start the bot
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

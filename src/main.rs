use serenity::{
    async_trait,
    model::{
        channel::Message,
        id::UserId, // Import UserId for sending DMs
    },
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
    async fn ready(&self, ctx: Context, ready: serenity::model::gateway::Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Initialize the bot with your Discord bot token
    let token = "insert token here";

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
    let user_id = msg.author.id;
    let dm = user_id
        .create_dm_channel(&ctx.http)
        .await
        .expect("Failed to create DM channel");
    dm.say(&ctx.http, "Pong!").await?;
    Ok(())
}

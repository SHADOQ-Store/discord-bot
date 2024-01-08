use poise::serenity_prelude as serenity;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

struct Data {
    database: Surreal<Client>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Ping the bot
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env!("DATABASE_URL");
    let database = Surreal::new::<Ws>(database_url).await?;

    database
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;

    database.use_ns("discord_bot").use_db("discord_bot").await?;

    let bot_token = env!("BOT_TOKEN");
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping()],
            ..Default::default()
        })
        .token(bot_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { database })
            })
        });

    framework.run().await?;
    Ok(())
}

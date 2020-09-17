use redis::Commands;
use std::env;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::utils::MessageBuilder;


#[command]
pub async fn start(ctx: &Context, msg: &Message) -> CommandResult {
    let response = MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&msg.author.name)
        .push(" just started a JungleSpeed game. Use ")
        .push_mono_safe("~join")
        .push(" to join the game!")
        .build();

    let addr = format!("redis://{}/", env::var("REDIS_ADDR").expect("Expected a redis addr in env"));
    let client = redis::Client::open(addr)?;
    let mut con = client.get_connection()?;
    con.set("my_key", 42)?;


    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}

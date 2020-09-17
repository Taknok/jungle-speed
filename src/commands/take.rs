use redis::Commands;
use std::env;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn take(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>()?;
    let two = args.single::<f64>()?;

    let product = one * two;


    let addr = format!("redis://{}/", env::var("REDIS_ADDR").expect("Expected a redis addr in env"));
    let client = redis::Client::open(addr)?;
    let mut con = client.get_connection()?;
    con.set("my_key", 2)?;
    assert_eq!(con.get("my_key"), Ok(2));

    msg.channel_id.say(&ctx.http, product).await?;

    Ok(())
}

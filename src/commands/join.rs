use redis::Commands;
use std::env;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::utils::MessageBuilder;
use log::{error, info};


#[command]
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let response = MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&msg.author.name)
        .push(" joined the game!")
        .build();
    
    let addr = format!("redis://{}/", env::var("REDIS_ADDR").expect("Expected a redis addr in env"));
    let client = redis::Client::open(addr)?;
    let mut con = client.get_connection()?;
/*    let my_var = match con.get::<&str, String>("my_key") {
        Ok(v) => info!("secret: {}", v),
        Err(e) => error!("{}", e),
    };*/

    let my_var: String = con.get("my_key")?;
    info!("secret: {}", my_var);

    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}

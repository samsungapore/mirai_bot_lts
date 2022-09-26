mod bot;
mod bot_handler;
mod log;
mod utils;
mod mirai_bot;

extern crate chrono;
extern crate chrono_tz;

#[tokio::main]
async fn main() {
    let mut mirai_bot = bot::DiscordBot::new()
        .set_token(std::env::var("bot_token").expect("bot_token env variable not set").as_str())
        .set_prefix("/");

    mirai_bot.setup_client().await;

    if let Err(why) = mirai_bot.client.expect("Client is not correctly initialized").start()
        .await {
        eprintln!("Client error: {:?}", why);
    }
}

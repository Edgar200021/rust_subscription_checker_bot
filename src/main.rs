use dotenv::dotenv;
use teloxide::{adaptors::Throttle, prelude::*};
use teloxide_core::{adaptors::throttle::Limits, requests::RequesterExt, Bot};
use tg_bot_rust::{
    configuration::{self},
    handlers,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let configuration =
        configuration::Settings::get_configuration().expect("failed to read configuration");

    pretty_env_logger::init();

    let bot = Bot::new(configuration.bot.bot_token).throttle(Limits {
        messages_per_min_channel: 100,
        messages_per_sec_chat: 10,
        messages_per_sec_overall: 100,
        messages_per_min_chat: 100,
    });

    let main_chat_id = configuration.bot.main_chat_id;
    let main_chat_url = configuration.bot.main_chat_url;
    let bot_id = configuration.bot.bot_id;

    teloxide::repl(bot, move |bot: Throttle<Bot>, msg: Message| {
        handlers::answer(
            bot,
            msg,
            main_chat_id.clone(),
            main_chat_url.clone(),
            bot_id.clone(),
        )
    })
    .await
}

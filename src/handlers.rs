use teloxide::{
    adaptors::Throttle,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

use crate::WrappedI64;

pub async fn answer(
    bot: Throttle<Bot>,
    msg: Message,
    main_chat_id: i64,
    main_chat_url: String,
    bot_id: u64,
) -> ResponseResult<()> {
    if msg.from.is_none() || msg.chat.id.0 == main_chat_id {
        return Ok(());
    }

    let user_id = msg.from.unwrap().id;

    let is_admin = bot
        .get_chat_administrators(msg.chat.id)
        .await?
        .iter()
        .any(|a| a.user.id == user_id || a.user.id.0 != bot_id);

    if is_admin {
        return Ok(());
    }

    let chat_member = bot
        .get_chat_member(WrappedI64(main_chat_id), user_id)
        .await?;

    if chat_member.is_administrator() || chat_member.is_member() || chat_member.is_owner() {
        return Ok(());
    }

    let url = reqwest::Url::parse(main_chat_url.as_str()).unwrap();

    let urls = InlineKeyboardMarkup::default().append_row(
        vec![url]
            .into_iter()
            .map(|url| InlineKeyboardButton::url("Канал", url)),
    );

    let (first, second) = tokio::join!(
        bot.delete_message(msg.chat.id, msg.id),
        bot.send_message(
            msg.chat.id,
            "Пожалуйста, вступите в наш канал, чтобы продолжить",
        )
        .reply_markup(urls)
    );

    first?;
    second?;

    Ok(())
}

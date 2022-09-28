use std::sync::Arc;


use serenity::builder::CreateEmbedAuthor;
use serenity::http::Http;
use serenity::model::channel::Message;
use serenity::model::guild::Member;
use serenity::model::id::ChannelId;
use crate::bot::BOT_TIMEZONE;

use crate::log::{MiraiLog, MiraiLogger};

use crate::mirai_bot::color::MIRAI_BOT_COLOR;
use crate::mirai_bot::image::PROLOGUE_DR2_STUDENTS_IMG_LINK;
use crate::utils::time::FRENCH_TIME_FORMAT;

pub async fn on_new_member(http: &Arc<Http>, system_channel: ChannelId, new_member: Member) -> serenity::Result<Message> {
    let avatar_url = new_member.avatar_url().unwrap_or_default();
    let mut author = CreateEmbedAuthor::default();
    author.name(new_member.display_name());

    if !avatar_url.is_empty() {
        author.icon_url(avatar_url.clone());
    }

    MiraiLogger::debug(format!(
        "Try to send welcome member message to [{}] - {} at {} on {}",
        new_member.user.id,
        new_member.display_name(),
        system_channel,
        new_member.guild_id
    ));
    system_channel.send_message(http, |msg| {
        msg.embed(|embed| {
            embed.color(MIRAI_BOT_COLOR);
            embed.title("Bienvenue");
            embed.description(
                "*Viendrais-tu par hasard chercher ton bonheur avec Danganronpa 2 ?*"
            );
            embed.image(PROLOGUE_DR2_STUDENTS_IMG_LINK);
            embed.set_author(author);

            if !avatar_url.is_empty() {
                embed.thumbnail(avatar_url);
            }

            if let Some(joined_at) = new_member.joined_at {
                let time = format!(
                    "{}", joined_at.with_timezone(&BOT_TIMEZONE).format(FRENCH_TIME_FORMAT)
                );
                MiraiLogger::debug(format!(
                    "Formatted joined_at time of {} is: {}",
                    new_member.display_name(),
                    time
                ));
                embed.footer(|footer| {
                    footer.text(format!(
                        "{} nous rejoint en cette date mémorable du {}",
                        new_member.display_name(),
                        time
                    ));
                    footer
                });
            }

            embed
        });
        msg
    }).await
}

#[cfg(test)]
mod tests {
    use serenity::model::Timestamp;
    use crate::bot::BOT_TIMEZONE;
    use crate::mirai_bot::on_new_member::FRENCH_TIME_FORMAT;

    #[test]
    fn test_date_format() {
        let date = Timestamp::now();
        let formatted_date = date.format(FRENCH_TIME_FORMAT);

        let local_date = date.with_timezone(&BOT_TIMEZONE);
        let formatted_local_date = local_date.format(FRENCH_TIME_FORMAT);

        println!("{}", format!("{}", date));
        println!("{}", format!("{}", date.date()));
        println!("{}", format!("{}", date.time()));
        println!();
        println!("{}", format!("{}", local_date));
        println!("{}", format!("{}", local_date.date()));
        println!("{}", format!("{}", local_date.time()));

        println!();

        println!("{}", format!("{}", formatted_date));
        println!("{}", format!("{}", formatted_local_date));
    }
}

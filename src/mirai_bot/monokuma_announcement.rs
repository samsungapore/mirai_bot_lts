use chrono::Timelike;
use serenity::builder::CreateEmbedAuthor;
use serenity::http::{Http};
use serenity::model::prelude::ChannelId;
use serenity::model::Timestamp;
use crate::log::{MiraiLog, MiraiLogger};

use crate::utils::time::{local_timestamp_now, sync_at, UTC_OFFSET};

async fn send_monokuma_morning_announcement(http: &Http, channel: ChannelId) -> bool {
    if let Err(err) = channel.send_message(http, |msg| {
        msg.embed(|embed| {
            let mut msg_author = CreateEmbedAuthor::default();
            msg_author.name("Monokuma".to_string());
            embed.set_author(msg_author);
            embed.field(
                "Bonjour, tout le monde !", "Il est maintenant 7h du matin
et la période de nuit est officiellement terminée !
Il est l'heure de se lever !\n\n
Préparez-vous à accueillir un autre jour meeeeerveilleux !", false);
            embed
        });
        msg
    }).await {
        MiraiLogger::error(format!("Could not send monokuma evening announcement: {}", err));
        return false;
    }

    true
}

async fn send_monokuma_evening_announcement(http: &Http, channel: ChannelId) -> bool {
    if let Err(err) = channel.send_message(http, |msg| {
        msg.embed(|embed| {
            let mut msg_author = CreateEmbedAuthor::default();
            msg_author.name("Monokuma".to_string());
            embed.set_author(msg_author);
            embed.field(
                "Mm, ahem, ceci est une annonce de l'école.", "Il est maintenant 22 h.\n\n
Autrement dit, c'est officiellement la période de nuit.
Les salons discord vont bientôt être fermés, et y discuter à
partir de maintenant est strictement interdit.
Maintenant, faites de beaux rêves ! Le marchand de sable va bientôt passer...", false);
            embed
        });
        msg
    }).await {
        MiraiLogger::error(format!("Could not send monokuma evening announcement: {}", err));
        return false;
    }

    true
}

pub async fn setup_monokuma_announcement(channel: ChannelId) -> (tokio::task::JoinHandle<()>,
                                                              tokio::task::JoinHandle<()>) {
    let http = Http::new(std::env::var("bot_token").expect("Wrong bot token").as_str());
    let another_http = Http::new(std::env::var("bot_token").expect("Wrong bot token").as_str());
    let chan = channel.clone();

    let morning_handle = tokio::task::spawn(async move {
        let running = true;
        let time_limit = None;
        let now = local_timestamp_now();
        let morning_monokuma_time = match now.hour() + UTC_OFFSET < 7 {
            true => {
                Timestamp::from(now.with_hour(7).unwrap().with_minute(0).unwrap())
            }
            false => {
                Timestamp::from(
                    now.with_hour(7).unwrap().with_minute(0).unwrap() +
                        chrono::Duration::days(1)
                )
            }
        };

        MiraiLogger::debug(format!("Morning announcement expected at {}", morning_monokuma_time));
        let mut morning_announcement = sync_at(
            &running,
            &time_limit,
            morning_monokuma_time,
            Some(std::time::Duration::new(60 * 60 * 24, 0))).await.unwrap();

        while running {
            send_monokuma_morning_announcement(&http, chan).await;
            morning_announcement.tick().await;
        }
    });

    let evening_handle = tokio::task::spawn(async move {
        let running = true;
        let time_limit = None;
        let now = local_timestamp_now();
        let evening_monokuma_time = match now.hour() + UTC_OFFSET < 22 {
            true => {
                Timestamp::from(now.with_hour(22).unwrap().with_minute(0).unwrap())
            }
            false => {
                Timestamp::from(now.with_hour(22).unwrap().with_minute(0).unwrap() +
                    chrono::Duration::days(1))
            }
        };

        MiraiLogger::debug(format!("Evening announcement expected at {}", evening_monokuma_time));
        let mut evening_announcement = sync_at(
            &running,
            &time_limit,
            evening_monokuma_time,
            Some(std::time::Duration::new(60 * 60 * 24, 0))).await.unwrap();

        while running {
            send_monokuma_evening_announcement(&another_http, chan).await;
            evening_announcement.tick().await;
        }
    });

    (morning_handle, evening_handle)
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;
    use serenity::model::Timestamp;

    use crate::mirai_bot::monokuma_announcement::setup_monokuma_announcement;
    use crate::utils::time::local_timestamp_now;

    #[tokio::test]
    async fn test_find_interval() {
        let running = true;

        // setup_monokuma_announcement(&running, Some(chrono::Duration::seconds(4))).await;
        println!("End of 4 seconds waiting");
    }

    #[tokio::test]
    async fn test_monokuma_morning_timestamp() {
        let morning_timestamp = local_timestamp_now();
        let m_timestamp = Timestamp::from(morning_timestamp.with_hour(7).unwrap());

        println!("{}", m_timestamp);
    }

    #[tokio::test]
    async fn test_locale_timestamp() {
        let time = local_timestamp_now();
        println!("{}", time);
    }
}
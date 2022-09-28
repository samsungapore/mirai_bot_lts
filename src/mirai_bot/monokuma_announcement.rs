use std::string::ToString;
use chrono::{Timelike, TimeZone, Utc};
use date_component::date_component::calculate;
use rand::Rng;
use serenity::builder::CreateEmbedAuthor;
use serenity::http::{Http};
use serenity::model::prelude::ChannelId;
use serenity::model::Timestamp;
use crate::log::{MiraiLog, MiraiLogger};
use crate::mirai_bot::color::MIRAI_BOT_COLOR;

use crate::utils::time::{local_timestamp_now, sync_at, UTC_OFFSET};

const MONOKUMA_AVATAR: &str = "https://avatars.githubusercontent.com/u/13270208?v=4";

const MONOKUMA_IMGS: [&str; 7] = [
    "https://vignette.wikia.nocookie.net/bloodbrothersgame/images/5/53/Monokuma.jpg/revision/latest/scale-to-width-down/640?cb=20131210191609",
    "http://2.bp.blogspot.com/-E5L7PG07qbk/U7zPtDHk_9I/AAAAAAAAAt4/UzoKWesIqWE/s1600/Danganronpa-Episode-07-Monokuma.jpg",
    "https://i.pinimg.com/236x/cc/c5/b1/ccc5b19b6d41e45d108e57433b5c4469.jpg",
    "http://i.imgur.com/T5s569W.gif",
    "https://i.imgur.com/K14wGy5.jpg?1",
    "https://i.imgur.com/aH1xD9S.gif",
    "https://c.tenor.com/svobtzY8wm4AAAAC/monokuma-danganronpa.gif"
];

fn get_random_in_str_array(arr: Box<[&str]>) -> &str {
    let mut rng = rand::thread_rng();

    return arr[rng.gen_range(0..arr.len())];
}

async fn send_monokuma_morning_announcement(http: &Http, channel: ChannelId) -> bool {
    let date1 = Utc.ymd(2016, 4, 10).and_hms(10, 0, 0);
    let date2 =  Utc::now();
    let date_interval = calculate(&date1, &date2);
    let footer = format!("Ainsi débute le jour {} à l'Académie du Pic de l'Espoir", date_interval.interval_days);

    if let Err(err) = channel.send_message(http, |msg| {
        msg.embed(|embed| {
            let mut msg_author = CreateEmbedAuthor::default();
            msg_author.name("Monokuma".to_string());
            msg_author.icon_url(MONOKUMA_AVATAR);
            embed.set_author(msg_author);
            embed.color(MIRAI_BOT_COLOR);
            embed.field(
                "Bonjour, tout le monde !", "Il est maintenant 7h du matin
et la période de nuit est officiellement terminée !
Il est l'heure de se lever !\n\n
Préparez-vous à accueillir un autre jour meeeeerveilleux !", false);
            embed.image(get_random_in_str_array(Box::new(MONOKUMA_IMGS)));
            embed.footer(|f| {
                f.text(footer);
                f
            });
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
            msg_author.icon_url(MONOKUMA_AVATAR);
            embed.set_author(msg_author);
            embed.color(MIRAI_BOT_COLOR);
            embed.field(
                "Mm, ahem, ceci est une annonce de l'école.", "Il est maintenant 22 h.\n\n
Autrement dit, c'est officiellement la période de nuit.
Les salons discord vont bientôt être fermés, et y discuter à
partir de maintenant est strictement interdit.
Maintenant, faites de beaux rêves ! Le marchand de sable va bientôt passer...", false);
            embed.image(get_random_in_str_array(Box::new(MONOKUMA_IMGS)));
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
    use serenity::http::Http;
    use serenity::model::id::ChannelId;
    use serenity::model::Timestamp;

    use crate::mirai_bot::monokuma_announcement::{get_random_in_str_array, MONOKUMA_IMGS, send_monokuma_evening_announcement, send_monokuma_morning_announcement};
    use crate::utils::time::local_timestamp_now;

    #[tokio::test]
    async fn test_get_random_in_array() {
        println!("{}", get_random_in_str_array(Box::new(MONOKUMA_IMGS)));
    }

    #[tokio::test]
    async fn test_morning_announcement() {
        let http = Http::new(std::env::var("test_bot_token").expect("Wrong test bot token").as_str
        ());
        let test_channel_id: ChannelId = ChannelId(std::env::var("test_bot_channel_id").expect
        ("Wrong test \
        channel id").as_str().parse::<u64>().expect("Could not parse channel id"));

        send_monokuma_morning_announcement(&http, test_channel_id).await;
    }

    #[tokio::test]
    async fn test_evening_announcement() {
        let http = Http::new(std::env::var("test_bot_token").expect("Wrong test bot token").as_str
        ());
        let test_channel_id: ChannelId = ChannelId(std::env::var("test_bot_channel_id").expect
        ("Wrong test \
        channel id").as_str().parse::<u64>().expect("Could not parse channel id"));

        send_monokuma_evening_announcement(&http, test_channel_id).await;
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
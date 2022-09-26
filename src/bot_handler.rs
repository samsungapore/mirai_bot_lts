use serenity::client::{Context, EventHandler};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;

use crate::bot::DiscordBot;
use crate::log::{MiraiLog, MiraiLogger};
use crate::mirai_bot::guild::MIRAI_TEAM_GUILD_ID;
use crate::mirai_bot::monokuma_announcement::setup_monokuma_announcement;
use crate::mirai_bot::on_new_member::on_new_member;
use crate::utils;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn guild_member_addition(&self, _ctx: Context, _new_member: Member) {
        let guild_name = _new_member.guild_id.name(&_ctx.cache).unwrap_or_default();
        MiraiLogger::info(
            format!("{} joined {}", _new_member.display_name(), guild_name).trim().to_string()
        );

        if let Some(system_channel) = utils::guild_fcts::find_guild_system_channel(
            &_ctx.cache, _new_member.guild_id
        ) {
            if let Err(err) = on_new_member(&_ctx.http, system_channel, _new_member).await {
                MiraiLogger::error(format!("Error on new member: {}", err));
            }
        }


    }

    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.is_own(&ctx.cache) {
            let mut debug_msg = format!(
                "Received new message by [{}] {}: {}",
                msg.author.id,
                msg.author.name,
                msg.content
            );

            if let Some(guild_id) = msg.guild_id {
                if let Some(guild_name) = guild_id.name(&ctx.cache) {
                    if let Some(channel_name) = msg.channel_id.name(&ctx.cache).await {
                        debug_msg = format!(
                            "Received new message on {} - {} by [{}] {}: {}",
                            guild_name,
                            channel_name,
                            msg.author.id,
                            msg.author.name,
                            msg.content
                        );
                    }
                }
            }

            MiraiLogger::debug(debug_msg);
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        MiraiLogger::info(format!("{} is connected!", ready.user.name));

        let bot = ctx.data.read().await.get::<DiscordBot>()
            .expect("Did not find DiscordBot").clone();

        match bot.admins.len() {
            0 => {
                MiraiLogger::info(format!("There are no admins on this bot"));
            }
            number_of_admins => {
                MiraiLogger::info(format!("There are {} admins on this bot owo", number_of_admins));
            }
        }

        if let Some(creator_id) = bot.creator {
            if let Some(creator) = ctx.cache.user(creator_id) {
                MiraiLogger::info(format!("The creator of the bot is {}", creator.name));
            } else {
                MiraiLogger::error(format!(
                    "Could not find the creator of the bot corresponding to user id {}",
                    creator_id
                ));
            }
        }

        match ctx.cache.guilds().len() {
            0 => {
                MiraiLogger::info(format!("No guilds on cache"));
            },
            1 => {
                if let Some(guild_name) = ctx.cache.guilds()[0].name(&ctx.cache) {
                    MiraiLogger::info(format!("There is one guild on cache ! It's {}.", guild_name));
                } else {
                    MiraiLogger::info(format!("There is one guild on cache !"));
                }
            },
            guild_nb => {
                MiraiLogger::info(format!("There are {} guilds on cache !", guild_nb));
            }
        }

        let mirai_team_guild = ctx.http.get_guild(MIRAI_TEAM_GUILD_ID.0).await;

        if let Ok(guild) = mirai_team_guild {
            MiraiLogger::debug(format!("Found guild {}", guild.name));
            if let Some(system_channel) = guild.system_channel_id {

                setup_monokuma_announcement(system_channel).await;

                /*if let Err(err) = system_channel.send_message(&ctx.http, |msg| {
                    msg.content("Hello! I'm the ultimate flashcard bot :3");
                    msg
                }).await {
                    MiraiLogger::error(format!("Could not send message to guild {}: {}", guild.name, err));
                }*/
            }
        }
    }
}
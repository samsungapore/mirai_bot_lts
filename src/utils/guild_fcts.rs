use std::sync::Arc;
use serenity::cache::Cache;
use serenity::model::id::{ChannelId, GuildId};

pub fn find_guild_system_channel(cache: &Arc<Cache>, guild_id: GuildId) -> Option<ChannelId> {
    match cache.guild(guild_id) {
        None => { None }
        Some(guild) => {
            guild.system_channel_id
        }
    }
}
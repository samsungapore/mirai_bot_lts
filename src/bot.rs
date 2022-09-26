use serenity::Client;
use serenity::framework::StandardFramework;
use serenity::model::id::UserId;
use serenity::prelude::{GatewayIntents, TypeMapKey};
use uuid::Uuid;

use crate::{bot_handler};
use crate::log::{MiraiLog, MiraiLogger};

pub const BOT_TIMEZONE: chrono_tz::Tz = chrono_tz::Europe::Paris;

pub struct DiscordBot {
    pub id: Uuid,
    pub token: String,
    pub prefix: String,
    pub creator: Option<UserId>,
    pub admins: Vec<UserId>,
    pub client: Option<Client>,
}

impl TypeMapKey for DiscordBot {
    type Value = DiscordBot;
}

impl DiscordBot {
    pub fn new() -> Self {
        let default_token: String = String::new();
        let default_prefix: String = String::from("/");
        MiraiLogger::debug(format!("Created new DiscordBot object with prefix {}", default_prefix));

        Self{
            id: Uuid::new_v4(),
            token: default_token,
            prefix: default_prefix,
            creator: None,
            admins: Vec::new(),
            client: None,
        }
    }

    pub fn set_token(mut self, token: &str) -> Self {
        self.token = token.to_string();
        self
    }

    pub fn set_prefix(mut self, prefix: &str) -> Self {
        self.prefix = prefix.to_string();
        self
    }

    pub async fn setup_client(&mut self) -> bool {
        MiraiLogger::debug(format!("Starting DiscordBot {}", self.id));

        let discord_framework = StandardFramework::new()
            .configure(|c| c.with_whitespace(true).prefix(self.prefix.as_str()));

        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::GUILDS
            | GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let client = Client::builder(self.token.as_str(), intents)
            .event_handler(bot_handler::Handler)
            .framework(discord_framework)
            .await
            .expect("Error creating client");

        client.data.write().await.insert::<DiscordBot>(self.clone());

        self.client = Some(client);
        true
    }
}

impl Default for DiscordBot {
    fn default() -> Self { Self::new() }
}

impl Clone for DiscordBot {
    fn clone(&self) -> Self {
        Self{
            id: self.id,
            token: self.token.clone(),
            admins: self.admins.clone(),
            prefix: self.prefix.clone(),
            creator: self.creator,
            client: None,
        }
    }
}

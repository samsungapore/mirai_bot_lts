import { ConsoleLogger, Injectable } from '@nestjs/common';
import {
  Client,
  ColorResolvable,
  GuildMember,
  Intents,
  Message,
  MessageEmbed,
  TextChannel,
} from 'discord.js';

@Injectable()
export class MiraiBotService extends Client {
  public readonly color: ColorResolvable;

  constructor(private readonly logger: ConsoleLogger) {
    super({
      intents: [
        Intents.FLAGS.DIRECT_MESSAGES,
        Intents.FLAGS.DIRECT_MESSAGE_TYPING,
        Intents.FLAGS.GUILDS,
        Intents.FLAGS.GUILD_MEMBERS,
      ],
    });
    this.token = process.env.mirai_bot_token;
    this.color = '#5afcf7';

    this.logger.log(`Mirai Bot constructed.`);

    this.setupEvents();
  }

  private setupEvents() {
    this.on('ready', this.onReady);
    this.on('error', this.logger.error);
    this.on('messageCreate', this.onMessage);
    this.on('guildMemberAdd', this.onNewMember);
  }

  private onReady() {
    this.logger.log('Mirai Bot is ready.');
  }

  private onMessage(message: Message) {
    this.logger.log(message.content);
  }

  private async onNewMember(member: GuildMember) {
    this.logger.log(`${member.displayName} joined ${member.guild.name}`);
    const systemChann = member.guild.systemChannel;
    if (systemChann) {
      const embed = new MessageEmbed({
        color: this.color,
        title: 'Bienvenue',
        description:
          '*Viendrais-tu par hasard chercher ton bonheur avec Danganronpa 2 ?*',
      });
      embed.setImage(
        'https://media.discordapp.net/attachments/168673025460273152/903227160494485504/latest_6.png?width=784&height=296',
      );
      embed.setAuthor(
        member.displayName,
        member.user.avatarURL({ dynamic: false }),
      );
      embed.setThumbnail(member.user.avatarURL({ dynamic: false }));
      embed.setFooter(
        `${
          member.displayName
        } nous rejoint en cette date mémorable du ${member.joinedAt.toLocaleString(
          'fr-FR',
        )}`,
        this.user.avatarURL({ dynamic: false }),
      );
      await this.sendMessageToTextChannel(systemChann, embed);
    }
  }

  public async sendMessageToTextChannel(
    channel: TextChannel,
    embed: MessageEmbed,
  ) {
    await channel.send({
      embeds: [embed],
    });
  }

  public run(): Promise<string> {
    return this.login(this.token);
  }
}

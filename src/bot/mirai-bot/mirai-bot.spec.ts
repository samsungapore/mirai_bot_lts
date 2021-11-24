import { Test, TestingModule } from '@nestjs/testing';
import { MiraiBotService } from './mirai-bot.service';
import { ConsoleLogger } from '@nestjs/common';
import { Guild, Message, MessageEmbed, TextChannel } from 'discord.js';

describe('MiraiBot', () => {
  const discordTestingGuildId = '550023853745766401';
  const testChannId = '903383503868133467';

  let miraibotService: MiraiBotService;
  let testGuild: Guild;
  let testChann: TextChannel;

  beforeEach(async () => {
    const app: TestingModule = await Test.createTestingModule({
      providers: [MiraiBotService, ConsoleLogger],
    }).compile();

    miraibotService = app.get<MiraiBotService>(MiraiBotService);

    testGuild = await miraibotService.guilds.fetch(discordTestingGuildId);
    testChann = (await testGuild.channels.fetch(testChannId)) as TextChannel;
  });

  describe('mirai bot', () => {
    it('should be able to send a message to test channel"', async () => {
      const msg: Message = await testChann.send({
        embeds: [
          new MessageEmbed({
            title: 'Mirai Bot',
            description: 'Hello',
            color: miraibotService.color,
          }),
        ],
      });

      setTimeout(() => {
        msg.delete();
      }, 4000);
    });
  });
});

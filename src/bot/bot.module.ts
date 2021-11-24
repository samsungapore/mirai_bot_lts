import { ConsoleLogger, Module } from '@nestjs/common';
import { MiraiBotService } from './mirai-bot/mirai-bot.service';

@Module({
  providers: [MiraiBotService, ConsoleLogger],
})
export class BotModule {}

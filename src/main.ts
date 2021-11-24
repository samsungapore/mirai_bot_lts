import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';
import { MiraiBotService } from './bot/mirai-bot/mirai-bot.service';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  const MiraiBot = app.get(MiraiBotService);
  await MiraiBot.run();
}
bootstrap().catch(console.error);

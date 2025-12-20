import { z } from "zod";

import { DefaultBoolean } from "../common-schemas";

export const TwitchChatSettingsSchema = z.object({
  // isActive: DefaultBoolean,
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type TwitchChatSettings = z.infer<typeof TwitchChatSettingsSchema>;

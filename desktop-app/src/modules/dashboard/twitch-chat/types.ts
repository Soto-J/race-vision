import z from "zod";

export const TwitchChatSettingsSchema = z.object({
  isActive: z.boolean(),
  //   general: GeneralSchema,
  //   content: ConetentSchama,
  //   header: HeaderSchema,
  //   footer: FooterSchema,
});

export type TwitchChatSettings = z.infer<typeof TwitchChatSettingsSchema>;

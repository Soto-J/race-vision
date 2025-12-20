import { z } from "zod";

export const DefaultBoolean = z.boolean().default(false);

export const GeneralSchema = z.object({
  showOverlayWhen: z.object({
    inCar: DefaultBoolean,
    outOfCar: DefaultBoolean,
    spotting: DefaultBoolean,
    inGarage: DefaultBoolean,
  }),
  showFlags: z.boolean().default(false),
});

export const ActiveAndDisplayInSchema = z.object({
  isActive: DefaultBoolean,
  displayIn: z.object({
    race: DefaultBoolean,
    qualy: DefaultBoolean,
    practice: DefaultBoolean,
  }),
});

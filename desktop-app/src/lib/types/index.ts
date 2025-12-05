import { z } from "zod";

const Chars8Schema = z.object({ Chars8: z.array(z.number()) });
const BoolSchema = z.object({ Bool: z.array(z.boolean()) });
const BitfieldSchema = z.object({ Bitfield: z.array(z.number()) });
const I32Schema = z.object({ I32: z.array(z.number()) });
const F32Schema = z.object({ F32: z.array(z.number()) });
const F64Schema = z.object({ F64: z.array(z.number()) });

export const VarKindSchema = z.union([
  Chars8Schema,
  BoolSchema,
  I32Schema,
  BitfieldSchema,
  F32Schema,
  F64Schema,
]);

export type VarKind = z.infer<typeof VarKindSchema>;

export const parseToValue = (value: VarKind): number | boolean | string => {
  if ("F32" in value) return value.F32[0];
  if ("F64" in value) return value.F64[0];
  if ("I32" in value) return value.I32[0];
  if ("Bool" in value) return value.Bool[0];
  if ("Chars8" in value) return String.fromCharCode(...value.Chars8);

  throw new Error("Unsupported type");
};

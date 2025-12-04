import { z } from "zod";

const Chars8Schema = z.object({
  Chars8: z.instanceof(Uint8Array).or(z.array(z.number())),
});
const BoolSchema = z.object({ Bool: z.array(z.boolean()) });
const I32Schema = z.object({ I32: z.array(z.number()) });
const BitfieldSchema = z.object({ Bitfield: z.array(z.number()) });
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

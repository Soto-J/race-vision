import { z } from "zod";

const NumArr = z.array(z.number()).min(1);
// -------------------------
// Raw incoming data
// -------------------------
const RawChars8Schema = z.object({ Chars8: NumArr });
const RawBitfieldSchema = z.object({ Bitfield: NumArr });
const RawI32Schema = z.object({ I32: NumArr });
const RawF32Schema = z.object({ F32: NumArr });
const RawF64Schema = z.object({ F64: NumArr });
const RawBoolSchema = z.object({ Bool: z.array(z.boolean()).min(1) });

export const RawVarKindSchema = z.union([
  RawChars8Schema,
  RawBitfieldSchema,
  RawI32Schema,
  RawF32Schema,
  RawF64Schema,
  RawBoolSchema,
]);

export type RawVarKind = z.infer<typeof RawVarKindSchema>;

// -------------------------
// Prasrsed Raw data
// -------------------------
const Chars8Schema = RawChars8Schema.transform(({ Chars8 }) => Chars8[0]);

const BitfieldSchema = RawBitfieldSchema.transform(
  ({ Bitfield }) => Bitfield[0],
);
const I32Schema = RawI32Schema.transform(({ I32 }) => I32[0]);
const F32Schema = RawF32Schema.transform(({ F32 }) => F32[0]);
const F64Schema = RawF64Schema.transform(({ F64 }) => F64[0]);
const BoolSchema = RawBoolSchema.transform(({ Bool }) => (Bool[0] ? 1 : 0));

export const VarKindSchema = z
  .union([
    Chars8Schema,
    BoolSchema,
    I32Schema,
    BitfieldSchema,
    F32Schema,
    F64Schema,
  ])
  .transform((obj) => {
    // Extract the first field’s value no matter which variant it is
    const key = Object.keys(obj)[0] as keyof typeof obj;
    return obj[key]; // already transformed by child schemas
  });

export type VarKind = z.infer<typeof VarKindSchema>;

export const valueSchema = z.array(z.number()).transform((v) => v[0]);
export const TelemetryValueSchema = z.discriminatedUnion("kind", [
  z.object({ kind: z.literal("F32"), value: valueSchema }),
  z.object({ kind: z.literal("F64"), value: valueSchema }),
  z.object({ kind: z.literal("I32"), value: valueSchema }),
  z.object({ kind: z.literal("Chars8"), value: valueSchema }),
  z.object({ kind: z.literal("Bitfield"), value: valueSchema }),
  z.object({
    kind: z.literal("Bool"),
    value: z.array(z.boolean()).transform((v) => v[0]),
  }),
]);

export type TelemetryValue = z.infer<typeof TelemetryValueSchema>;

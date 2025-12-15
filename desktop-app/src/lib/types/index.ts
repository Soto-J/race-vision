import { z } from "zod";
/**
 * Telemetry value schemas for Rust → Frontend IPC.
 *
 * Rust sends telemetry values using an externally tagged enum:
 *
 *   #[serde(tag = "kind", content = "value")]
 *   pub enum TelemetryValue {
 *       Chars8(Vec<u8>),
 *       Bool(Vec<bool>),
 *       I32(Vec<i32>),
 *       Bitfield(Vec<u32>),
 *       F32(Vec<f32>),
 *       F64(Vec<f64>),
 *   }
 *
 * On the wire, each telemetry value has the shape:
 *
 *   {
 *     kind: "F32" | "F64" | "I32" | "Bitfield" | "Chars8" | "Bool",
 *     value: T[]
 *   }
 *
 * Notes:
 * - `value` is always an array, even for single-element telemetry variables.
 * - This schema models the IPC payload exactly as Rust emits it.
 * - Do NOT normalize or flatten values inside the schema.
 *   Validation and interpretation are intentionally separated.
 */

/**
 * Runtime validation for a single telemetry value.
 * Matches the Rust `TelemetryValue` enum 1:1.
 */
export const TelemetryValueSchema = z.discriminatedUnion("kind", [
  z.object({
    kind: z.literal("F32"),
    value: z.array(z.number()).min(1),
  }),
  z.object({
    kind: z.literal("F64"),
    value: z.array(z.number()).min(1),
  }),
  z.object({
    kind: z.literal("I32"),
    value: z.array(z.number().int()).min(1),
  }),
  z.object({
    kind: z.literal("Bitfield"),
    value: z.array(z.number().int()).min(1),
  }),
  z.object({
    kind: z.literal("Chars8"),
    value: z.array(z.number().int()).min(1),
  }),
  z.object({
    kind: z.literal("Bool"),
    value: z.array(z.boolean()).min(1),
  }),
]);

export type TelemetryValue = z.infer<typeof TelemetryValueSchema>;

/**
 * A full telemetry snapshot.
 *
 * Rust emits:
 *   HashMap<String, TelemetryValue>
 *
 * Frontend receives:
 *   Record<string, TelemetryValue>
 */
export const TelemetrySnapshotSchema = z.record(
  z.string(),
  TelemetryValueSchema,
);

export type TelemetrySnapshot = z.infer<typeof TelemetrySnapshotSchema>;

/**
 * Decode a validated telemetry value into a display-friendly primitive.
 *
 * - Numeric and boolean telemetry returns the first value
 * - Char8 values are decoded as UTF-8 strings
 *
 * This function is intentionally NOT part of the Zod schema.
 * Validation and interpretation are kept separate by design.
 */
export function decodeTelemetryValue(v: TelemetryValue) {
  switch (v.kind) {
    case "F32":
    case "F64":
    case "I32":
    case "Bitfield":
      return v.value[0];

    case "Bool":
      return v.value[0];

    case "Chars8":
      return new TextDecoder().decode(Uint8Array.from(v.value));
  }
}

/**
 * @module parameters
 * @description Demonstrates different PolarCodec parameter combinations:
 *   - Frozen-bits methods (5G, RM, GA)
 *   - CRC options (none, built-in, user-defined)
 */

import { PolarCodec, Crc } from "polar_codec_wasm";
import { to_llr, bytes_eq, bits_array } from "./utils.ts";

let failed = false;

// ---------------------------------------------------------------------------
// Helper: round-trip encode -> add noise -> decode and check
// ---------------------------------------------------------------------------

function round_trip(
  label: string,
  codec: PolarCodec,
  data: Uint8Array,
  noise = 0,
): boolean {
  const encoded = codec.encode_bit(data);
  const llr = to_llr(encoded);

  // Optionally corrupt the LLR to simulate a noisy channel
  if (noise > 0) {
    for (let i = 0; i < llr.length; i++) {
      llr[i] += (Math.random() - 0.5) * 2 * noise;
    }
  }

  const decoded = codec.decode_bit(llr);
  const ok = bytes_eq(data, decoded);
  const status = ok ? "PASS" : "FAIL";
  console.log(`  [${status}] ${label}`);
  if (!ok) failed = true;
  return ok;
}

// ---------------------------------------------------------------------------
// 1. Frozen-bits methods
// ---------------------------------------------------------------------------

console.log("=== Frozen-bits methods ===");

const K = 32;
const N = 64;

const codec5g  = new PolarCodec(K, N, 4, { frozenBits: "5G" });
const codecRM  = new PolarCodec(K, N, 4, { frozenBits: "RM" });
const codecGA  = new PolarCodec(K, N, 4, { frozenBits: { type: "GA", sigma: 0.5 } });
const codecAuto = new PolarCodec(K, N);   // auto-selects based on N

const data = bits_array(K);

round_trip("5G  (frozenBits: '5G')",       codec5g,  data);
round_trip("RM  (frozenBits: 'RM')",       codecRM,  data);
round_trip("GA  (frozenBits: {type:'GA'})", codecGA,  data);
round_trip("Auto (default, N=64)",         codecAuto, data);

// ---------------------------------------------------------------------------
// 2. CRC options
// ---------------------------------------------------------------------------

console.log("\n=== CRC options ===");

const codecNoCrc   = new PolarCodec(K, N, 4, { crc: null });
const codecCrc16   = new PolarCodec(K, N, 4, { crc: Crc.CRC_16_UMTS });
const codecCrc24   = new PolarCodec(K, N, 4, { crc: Crc.CRC_24_LTE_A });

round_trip("No CRC",                  codecNoCrc, data);
round_trip("CRC-16/UMTS",             codecCrc16, data);
round_trip("CRC-24/LTE-A",            codecCrc24, data);

// ---------------------------------------------------------------------------
// 3. User-defined CRC
// ---------------------------------------------------------------------------

console.log("\n=== User-defined CRC ===");

// Reproduce CRC-16/USB manually:
//   width=16, poly=0x8005, init=0xFFFF, refin=true, refout=true, xorout=0xFFFF
const codecUserCrc = new PolarCodec(K, N, 4, {
  crc: {
    name: "CRC-16/USB (manual)",
    width: 16,
    poly: 0x8005,
    init: 0xFFFF,
    refin: true,
    refout: true,
    xorout: 0xFFFF,
  },
});

round_trip("User-defined CRC-16/USB", codecUserCrc, data);

// ---------------------------------------------------------------------------
// 4. List size (L) effect
// ---------------------------------------------------------------------------

console.log("\n=== List size (L) ===");

const codecL1 = new PolarCodec(K, N, 1);
const codecL4 = new PolarCodec(K, N, 4);
const codecL8 = new PolarCodec(K, N, 8);

round_trip("L=1 (SC only)",  codecL1, data);
round_trip("L=4 (default)",  codecL4, data);
round_trip("L=8 (better)",   codecL8, data);

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

console.log(failed ? "\nSome tests FAILED." : "\nAll tests passed.");
if (failed) process.exit(1);

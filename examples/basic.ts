/**
 * @module basic
 * @description Minimal round-trip examples for the Polar Codec.
 *
 * Two demos:
 *   1. Bit-level encode/decode with raw 0/1 data.
 *   2. Byte-level encode/decode with a UTF-8 string.
 */

import { PolarCodec } from "polar_codec_wasm";
import { to_llr, bytes_eq, from_str, to_str } from "./utils.ts";

/**
 * Bit-level round-trip: encode 8 random bits and decode them back.
 * Returns `true` if the test failed (i.e. decoded != original).
 */
function simple() {
  console.log("=============================");
  console.log("Simple Polar Codec Example");
  const data = new Uint8Array([1, 0, 0, 1, 1, 1, 0, 0]);

  const codec  = new PolarCodec(data.length, 16);
  const encoded = codec.encode_bit(data);
  const llr     = to_llr(encoded);
  const decoded = codec.decode_bit(llr);

  console.log("Original:", data);
  console.log("Encoded:", encoded);
  const PASS = bytes_eq(data, decoded);
  console.log("Eq:", PASS);
  return !PASS;
}

/**
 * Byte-level round-trip: encode the string "Hello, world!" and decode it.
 * Returns `true` if the test failed.
 */
function withText() {
  console.log("=============================");
  console.log("Polar Codec Example with Text");
  const str  = "Hello, world!";
  const data = from_str(str);

  const codec  = new PolarCodec(data.length * 8, 128);
  const encoded = codec.encode(data);
  const llr     = to_llr(encoded);
  const decoded = codec.decode(llr);

  const text = to_str(decoded);
  console.log("Original:", str);
  console.log("Decoded:", text);
  const PASS = str === text;
  console.log("Eq:", PASS);
  return !PASS;
}

// ---------------------------------------------------------------------------
// Run
// ---------------------------------------------------------------------------

if (simple() || withText()) {
  throw new Error("\nExample Failed!");
} else {
  console.log("\nExample Passed.");
}
/**
 * @module noise
 * @description Demonstrates Bit Error Rate (BER) performance under Gaussian noise
 * for Polar Codes with and without CRC-aided SCL decoding.
 *
 * For each noise level the script runs many random trials and reports the
 * fraction of information bits that were decoded incorrectly.
 */

import { PolarCodec, Crc } from "polar_codec_wasm";
import { bits_array, normal_dist } from "./utils.ts";

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const K       = 32;          // information bits
const N       = 64;          // codeword length
const L       = 4;           // SCL list size
const TRIALS  = 5000;        // number of random codewords per noise level

/** Noise levels (sigma) to sweep. */
const SIGMA_LEVELS = [0.2, 0.4, 0.6, 0.8, 1.0, 1.2, 1.4, 1.6];

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/**
 * Run a single encode -> noisy channel -> decode trial.
 * Returns the number of bit errors in the decoded information word.
 */
function trial(codec: PolarCodec, data: Uint8Array, sigma: number): number {
  const encoded = codec.encode_bit(data);

  // Convert bits to LLR and add AWGN noise
  const llr = new Float32Array(encoded.length);
  for (let i = 0; i < encoded.length; i++) {
    // Clean LLR: bit 1 -> -1, bit 0 -> +1
    const clean = encoded[i] ? -1 : 1;
    llr[i] = clean + normal_dist() * sigma;
  }

  const decoded = codec.decode_bit(llr);

  // Count bit errors
  let errors = 0;
  for (let i = 0; i < data.length; i++) {
    if (data[i] !== decoded[i]) errors++;
  }
  return errors;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

console.log("Polar Codec BER Simulation");
console.log(`K=${K}  N=${N}  L=${L}  trials=${TRIALS}`);
console.log("─".repeat(52));

// Two codecs: one without CRC, one with CRC-24
const codecNoCrc = new PolarCodec(K, N, L, { crc: null });
const codecCrc   = new PolarCodec(K, N, L, { crc: Crc.CRC_24_LTE_A });

// Header
console.log(
  "sigma".padStart(6),
  "BER(no CRC)".padStart(14),
  "BER(CRC-24)".padStart(14),
);

for (const sigma of SIGMA_LEVELS) {
  let totalBitsNoCrc = 0;
  let totalErrsNoCrc = 0;
  let totalBitsCrc   = 0;
  let totalErrsCrc   = 0;

  for (let t = 0; t < TRIALS; t++) {
    const data = bits_array(K);

    totalErrsNoCrc += trial(codecNoCrc, data, sigma);
    totalBitsNoCrc += K;

    totalErrsCrc += trial(codecCrc, data, sigma);
    totalBitsCrc += K;
  }

  const berNoCrc = totalErrsNoCrc / totalBitsNoCrc;
  const berCrc   = totalErrsCrc   / totalBitsCrc;

  console.log(
    sigma.toFixed(1).padStart(6),
    berNoCrc.toExponential(3).padStart(14),
    berCrc.toExponential(3).padStart(14),
  );
}

console.log("─".repeat(52));
console.log("Done.");

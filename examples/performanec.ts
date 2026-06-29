/**
 * @module performance
 * @description Benchmarks Polar Codec encode + noisy-channel + decode latency.
 *
 * The script runs 1000 groups.  In each group:
 *   1. A fresh codec is created with GA frozen-bits (N=2048).
 *   2. 1000 random information bits are encoded, corrupted with noise, and
 *      decoded -- repeated 100 times.
 *   3. Each full encode+decode cycle is timed with `process.hrtime.bigint()`.
 *
 * After all groups, mean and standard deviation of the per-cycle latency are
 * reported.
 */

import { PolarCodec } from "polar_codec_wasm";
import { to_llr, bits_array, normal_dist } from "./utils.ts";

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

const K        = 1000;    // information bits per codeword
const N        = 2048;    // codeword length (power of two)
const L        = 4;       // SCL list size
const GROUPS   = 100;    // number of benchmark groups
const ITERS    = 100;     // encode+decode cycles per group
const SIGMA    = 1.0;     // noise level

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/** High-resolution timer helpers (Node.js only). */
function now(): bigint {
  return process.hrtime.bigint();
}

/** Convert nanoseconds to a human-readable string. */
function fmtNs(ns: number): string {
  if (ns >= 1_000_000_000) return `${(ns / 1_000_000_000).toFixed(2)} s`;
  if (ns >= 1_000_000)     return `${(ns / 1_000_000).toFixed(2)} ms`;
  if (ns >= 1_000)         return `${(ns / 1_000).toFixed(2)} us`;
  return `${ns.toFixed(0)} ns`;
}

// ---------------------------------------------------------------------------
// Main benchmark
// ---------------------------------------------------------------------------

console.log("Polar Codec Performance Benchmark");
console.log(`K=${K}  N=${N}  L=${L}  groups=${GROUPS}  iterations/group=${ITERS}  sigma=${SIGMA}`);
console.log("─".repeat(60));

/** Collects the per-iteration latency (in ns) across all groups. */
const allLatencies: number[] = [];

const tStart = now();

for (let g = 0; g < GROUPS; g++) {
  // Create a fresh codec each group to also measure construction cost.
  const codec = new PolarCodec(K, N, L, { frozenBits: { type: "GA", sigma: 0.5 } });

  for (let i = 0; i < ITERS; i++) {
    const data = bits_array(K);

    const t0 = now();
    const encoded = codec.encode_bit(data);

    // Simulate AWGN channel
    const llr = new Float32Array(encoded.length);
    for (let j = 0; j < encoded.length; j++) {
      const clean = encoded[j] ? -1 : 1;
      llr[j] = clean + normal_dist() * SIGMA;
    }

    codec.decode_bit(llr);
    const t1 = now();

    allLatencies.push(Number(t1 - t0));
  }
}

const tEnd = now();
const totalMs = Number(tEnd - tStart) / 1_000_000;

// ---------------------------------------------------------------------------
// Statistics
// ---------------------------------------------------------------------------

const n = allLatencies.length;
const sum  = allLatencies.reduce((a, b) => a + b, 0);
const mean = sum / n;

const variance = allLatencies.reduce((acc, v) => acc + (v - mean) ** 2, 0) / n;
const stdDev   = Math.sqrt(variance);

// Percentiles (sorted copy)
const sorted = [...allLatencies].sort((a, b) => a - b);
const p50 = sorted[Math.floor(n * 0.50)];
const p95 = sorted[Math.floor(n * 0.95)];
const p99 = sorted[Math.floor(n * 0.99)];

console.log(`Total time      : ${fmtNs(Number(tEnd - tStart))}`);
console.log(`Total iterations: ${n}`);
console.log("");
console.log("Per-iteration latency (encode + noise + decode):");
console.log(`  Mean   : ${fmtNs(mean)}`);
console.log(`  StdDev : ${fmtNs(stdDev)}`);
console.log(`  P50    : ${fmtNs(p50)}`);
console.log(`  P95    : ${fmtNs(p95)}`);
console.log(`  P99    : ${fmtNs(p99)}`);
console.log("");
console.log(`Effective throughput: ~${(K / (mean / 1_000_000_000)).toFixed(0)} info bits/s`);
console.log("─".repeat(60));
console.log("Done.");

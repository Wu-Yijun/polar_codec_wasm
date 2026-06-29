# polar_codec_wasm

[![npm version](https://img.shields.io/npm/v/polar_codec_wasm.svg)](https://www.npmjs.com/package/polar_codec_wasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/%3C%2F%3E-TypeScript-blue.svg)](https://www.typescriptlang.org/)

A high-performance, **zero-dependency** WebAssembly implementation of **Polar Codes** — a class of Forward Error Correction (FEC) codes that achieve the capacity of symmetric binary-input discrete memoryless channels.

The heavy-lifting core is written in Rust ([`polar_codec`](https://crates.io/crates/polar_codec)), compiled to WebAssembly, and embedded directly via Base64. This architectural design completely eliminates runtime garbage collection (GC) overhead during intensive decoding arrays, while bypassing the headache of asynchronous `.wasm` file fetching. 

It works entirely synchronously out-of-the-box in **Node.js**, **Browsers**, and **Deno**.

## Features

- **Zero Dependencies:** The Wasm binary is base64-inlined. Import it and run it synchronously anywhere.
- **High Performance:** Core SCL decoding operations are executed in Rust, circumventing JS engine limitations and GC pauses.
- **SCL Decoding:** Successive Cancellation List decoding with a configurable list size `L`.
- **CRC-Aided:** 80+ built-in CRC algorithms (CRC-8 through CRC-82) + full support for user-defined polynomials.
- **Frozen Bits Generation:** Out-of-the-box support for 5G NR, Reed-Muller, and Gaussian Approximation (GA) construction methods.
- **Bit & Byte APIs:** Effortlessly encode/decode raw byte arrays (`Uint8Array`) or specific 0/1 bit arrays.

## Install

### Package Managers

```bash
npm install polar_codec_wasm
# or
pnpm add polar_codec_wasm

```

### Via CDN (Direct Browser Usage)

For native browser environments without bundlers, you can include the IIFE bundle directly via **jsDelivr** or **unpkg**. The package will automatically expose a global `PolarCodec` constructor on the `window` object.

```html
<script src="https://cdn.jsdelivr.net/npm/polar_codec_wasm@1.0.0/dist/index.iife.min.js"></script>

<script src="https://unpkg.com/polar_codec_wasm@1.0.0/dist/index.iife.js"></script>

```

## ⚡ Quick Start

### 1. Browser Environment (via CDN)

```html
<script src="https://cdn.jsdelivr.net/npm/polar_codec_wasm/dist/index.iife.js"></script>
<script>
  // Encode string data at byte-level
  const inputString = "Hello, Polar Codec!";
  const data = new TextEncoder().encode(inputString);

  // Initialize Codec (K = data.length * 8 bits)
  const codec = new PolarCodec(data.length * 8);
  const encoded = codec.encode(data);

  // Simulate BPSK channel (bits to Log-Likelihood Ratios)
  const llr = PolarCodec.bits_to_llr(encoded);

  // Decode back to bytes
  const decoded = codec.decode(llr);
  console.log("Decoded:", new TextDecoder().decode(decoded)); // "Hello, Polar Codec!"
</script>

```

### 2. Bundler / Node.js Environment (Byte-Level)

```ts
import { PolarCodec } from "polar_codec_wasm";

const data = new Uint8Array([72, 101, 108, 108, 111]); // "Hello"
const codec = new PolarCodec(data.length * 8, 64);

const encoded = codec.encode(data);
const llr = PolarCodec.bits_to_llr(encoded);
const decoded = codec.decode(llr);

```

### 3. Bit-Level Communication Simulation

```ts
import { PolarCodec, Crc } from "polar_codec_wasm";

// K=96 info bits, N=128 codeword length, List Size=4, 5G Frozen bits + CRC-16
const codec = new PolarCodec(96, 128, 4, { 
  frozenBits: "5G",
  crc: Crc.CRC_16_UMTS 
});

const dataBits = new Uint8Array([1, 0, 1, 1, 0, 0, 1, 0 /* ... up to 96 bits */]);
const encodedBits = codec.encode_bit(dataBits);

// Receiver side LLR mapping: negative = bit 1, positive = bit 0
const llr = PolarCodec.bits_to_llr(encodedBits);
const decodedBits = codec.decode_bit(llr);

```

## 🛠 API Reference

### `new PolarCodec(k, n?, l?, options?)`

| Param | Type | Default | Description |
| --- | --- | --- | --- |
| `k` | `number` |  | Information bits per codeword |
| `n` | `number` | `2^ceil(log2(k))` | Codeword length (must be power of two, $\ge k$) |
| `l` | `number` | `4` | SCL list size (higher = better correction, slower execution) |
| `options.frozenBits` | `FrozenBitsMethod` | `auto` | `"5G"`, `"RM"`, or `{ type: "GA", sigma?: number }` |
| `options.crc` | `Crc | DefinedCrc | null` | `null` | CRC algorithm or `null` for none |

> **Note on Browser (CDN) Built-in CRCs:** When using the global CDN script, built-in CRC enums can be accessed via `polar_codec.Crc` (e.g., `polar_codec.Crc.CRC_16_UMTS`).

### Core Methods

| Method | Input | Output | Description |
| --- | --- | --- | --- |
| `encode(src)` | `Uint8Array` (bytes) | `Uint8Array` (packed bytes) | Byte-level encode |
| `decode(llr)` | `Float32Array` (length n) | `Uint8Array` (packed bytes) | Byte-level SCL decode |
| `encode_bit(src)` | `Uint8Array` of 0/1 | `Uint8Array` of 0/1 | Bit-level encode |
| `decode_bit(llr)` | `Float32Array` (length n) | `Uint8Array` of 0/1 | Bit-level SCL decode |
| `PolarCodec.bits_to_llr(bits)` | `Uint8Array` | `Float32Array` | Utility: Maps `0 -> +10.0`, `1 -> -10.0` |

### User-Defined CRC

You can strictly define custom polynomials if the built-in standards don't match your protocol:

```ts
import { PolarCodec } from "polar_codec_wasm";

const codec = new PolarCodec(96, 128, 4, {
  crc: {
    name: "My Custom CRC",
    width: 16,
    poly: 0x8005,
    init: 0xFFFF,
    refin: true,
    refout: true,
    xorout: 0xFFFF,
  },
});

```

## 🧪 Examples

Runnable demos are located in the `examples/` directory. Clone the repo and run:

```bash
# Basic round-trip (bit-level and byte-level)
npx tsx examples/basic.ts

# Frozen-bits and CRC parameter combinations
npx tsx examples/parameters.ts

# BER (Bit Error Rate) simulation under Gaussian noise
npx tsx examples/noise.ts

# Encode/decode latency and throughput benchmark
npx tsx examples/performance.ts

```

## 🏗 Architecture & Build

```text
TypeScript API  ──►  wasm-bindgen  ──►  Rust Codec  ──►  polar_codec crate
       │
       └──  Wasm binary embedded as Base64 (No asynchronous file I/O)

```

Building from source:

```bash
git clone https://github.com/Wu-Yijun/polar_codec_wasm.git
cd polar_codec_wasm
pnpm install
pnpm run build

```

## License

[MIT](LICENSE) © Aluria


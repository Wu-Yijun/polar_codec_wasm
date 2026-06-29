# polar_codec_wasm

A WebAssembly implementation of **Polar Codes** -- a class of error-correcting codes that achieve the capacity of symmetric binary-input discrete memoryless channels.

The codec is written in Rust ([`polar_codec`](https://crates.io/crates/polar_codec)), compiled to WebAssembly via [wasm-pack](https://rustwasm.github.io/wasm-pack/), and wrapped in a zero-dependency TypeScript package that works in **Node.js**, **browsers**, and **Deno**.

## Install

```bash
npm install polar_codec_wasm
# or
pnpm add polar_codec_wasm
# or
yarn add polar_codec_wasm
```

## Quick Start

```ts
import { PolarCodec, Crc } from "polar_codec_wasm";

// K=96 information bits, N=128 codeword length (default 5G frozen bits, no CRC)
const codec = new PolarCodec(96, 128);

// Encode
const data    = new Uint8Array([1, 0, 1, 1, 0, 0, 1, 0 /* ... */]);
const encoded = codec.encode_bit(data);

// Decode (LLR: negative = bit 1, positive = bit 0)
const llr     = to_llr(encoded);
const decoded = codec.decode_bit(llr);
```

## Features

| Feature | Description |
|---------|-------------|
| **SCL decoding** | Successive Cancellation List decoding with configurable list size `L` |
| **CRC-aided** | 80+ built-in CRC algorithms (CRC-8 through CRC-82) + user-defined |
| **Frozen bits** | 5G NR, Reed-Muller, and Gaussian Approximation construction methods |
| **Byte & bit APIs** | `encode` / `decode` for byte arrays, `encode_bit` / `decode_bit` for 0/1 arrays |
| **Zero deps** | The Wasm binary is embedded as Base64 -- no file loading at runtime |
| **Cross-platform** | Node.js >= 16, browsers (ESM / IIFE), Deno |

## API

### `new PolarCodec(k, n, l?, options?)`

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| `k` | `number` | | Information bits per codeword |
| `n` | `number` | | Codeword length (power of two, >= k) |
| `l` | `number` | `4` | SCL list size (higher = better correction, slower) |
| `options.frozenBits` | `FrozenBitsMethod` | auto | `"5G"`, `"RM"`, or `{ type: "GA", sigma?: number }` |
| `options.crc` | `Crc \| DefinedCrc \| null` | `null` | CRC algorithm or `null` for none |

### Methods

| Method | Input | Output | Description |
|--------|-------|--------|-------------|
| `encode_bit(src)` | `Uint8Array` of 0/1 | `Uint8Array` of 0/1 (length n) | Encode information bits |
| `decode_bit(llr)` | `Float32Array` (length n) | `Uint8Array` of 0/1 (length k) | Decode LLRs to information bits |
| `encode(src)` | `Uint8Array` (bytes) | `Uint8Array` (packed bytes) | Byte-level encode |
| `decode(llr)` | `Float32Array` (length n) | `Uint8Array` (packed bytes) | Byte-level decode |

### Frozen-bits Methods

| Method | Description |
|--------|-------------|
| `"5G"` | 5G NR standard frozen-bits pattern (default for N <= 1024) |
| `"RM"` | Reed-Muller based construction (default for N > 1024) |
| `"GA"` | Gaussian Approximation with configurable `sigma` parameter |

### User-defined CRC

```ts
import { PolarCodec, Crc } from "polar_codec_wasm";

const codec = new PolarCodec(96, 128, 4, {
  crc: {
    name: "My CRC",
    width: 16,
    poly: 0x8005,
    init: 0xFFFF,
    refin: true,
    refout: true,
    xorout: 0xFFFF,
  },
});
```

### Built-in CRC Algorithms

All variants from the `crc` crate are available via the `Crc` enum:

```ts
import { Crc } from "polar_codec_wasm";

new PolarCodec(96, 128, 4, { crc: Crc.CRC_16_UMTS });
new PolarCodec(96, 128, 4, { crc: Crc.CRC_24_LTE_A });
new PolarCodec(96, 128, 4, { crc: Crc.CRC_32_ISCSI });
```

## Examples

The `examples/` directory contains runnable demos:

```bash
# Basic round-trip (bit-level and byte-level)
npx tsx examples/basic.ts

# Frozen-bits and CRC parameter combinations
npx tsx examples/paramaters.ts

# BER simulation under Gaussian noise
npx tsx examples/noise.ts

# Encode/decode latency benchmark
npx tsx examples/performanec.ts
```

## Architecture

```
TypeScript API  ──►  wasm-bindgen  ──►  Rust Codec  ──►  polar_codec crate
       │
       └──  Wasm binary embedded as Base64 (no file I/O needed)
```

The build pipeline (`pnpm run build`):

1. Compiles Rust to `wasm32-unknown-unknown` via `wasm-pack build --target web`.
2. Converts the `.wasm` file to a Base64 string embedded in `lib/wasm-b64.ts`.
3. Bundles three output formats via esbuild: **ESM**, **CJS**, and **IIFE** (for `<script>` tags).
4. Generates `.d.ts` declaration files with `tsc`.

## Build from Source

Requires: [Rust](https://rustup.rs/), [wasm-pack](https://rustwasm.github.io/wasm-pack/), [pnpm](https://pnpm.io/).

```bash
git clone https://github.com/Wu-Yijun/polar-codec-wasm.git
cd polar-codec-wasm
pnpm install
pnpm run build     # compiles Rust -> wasm -> Base64 -> ESM/CJS/IIFE bundles
```

## License

MIT

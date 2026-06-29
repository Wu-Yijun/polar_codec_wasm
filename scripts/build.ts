import { execSync } from 'node:child_process';
import fs from 'node:fs';
import { build } from 'tsdown';

console.log("🚀 [1/4] Compiling Rust to WebAssembly...");
// Compile in release mode, targeting the web
execSync('wasm-pack build --target web --release', { stdio: 'inherit' });

console.log("📦 [2/4] Converting Wasm into an inline Base64 module...");
const wasmBuffer = fs.readFileSync('./pkg/polar_codec_wasm_bg.wasm');
const base64Str = wasmBuffer.toString('base64');
// Generate a dummy JS file for TypeScript imports
fs.writeFileSync('./lib/wasm-b64.ts', `export const WASM_BASE64: string = "${base64Str}";\n`);

console.log("🗜️  [3/4] Bundling CJS and ESM outputs using rolldown...");
build({
  entry: './lib/index.ts',
  format: "esm",
});
build({
  entry: './lib/index.ts',
  format: "cjs",
});
build({
  entry: './lib/index.ts',
  format: "iife",
  globalName: 'polar_codec',
  platform: 'browser',
  // minify: true,
  define: { 'import.meta': "{/* import.meta */}" },
  footer: { js: 'window.PolarCodec = polar_codec.PolarCodec;' }
});

// console.log("📝 [4/4] Generating TypeScript declaration files (.d.ts)...");
// execSync('pnpm run tsc', { stdio: 'inherit' });

console.log("✅ Build complete! Outputs saved to the /dist directory.");


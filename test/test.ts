import { PolarCodec } from "polar_codec_wasm";

const str = "Hello, world!";
const data = (new TextEncoder()).encode(str);
// const data = new Uint8Array([1, 0, 0, 1, 1, 1, 0, 0]);
console.log("Raw data:", data);

const codec = new PolarCodec(data.length, 16, 8);

console.log("Polar Codec", codec);
const encoded = new Uint8Array(codec.N);

codec.encode(data, encoded);
// console.log("encoded", encoded);

const llr = new Float32Array(codec.N);
for (let i = 0; i < codec.N; i++) {
  llr[i] = encoded[i] ? -1 : 1;
}

const decoded = new Uint8Array(codec.K);
codec.decode(llr, decoded);
console.log("decoded", decoded);
const text = (new TextDecoder()).decode(decoded);
console.log("Text: ", text);
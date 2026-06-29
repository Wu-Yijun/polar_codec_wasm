import { PolarCodec } from "polar_codec_wasm";
import { to_llr, bytes_eq, from_str, to_str } from "./utils.ts";

function simple() {
  console.log("=============================");
  console.log("Simple Polar Codec Example");
  // Note that data should be a 0/1 array.
  const data = new Uint8Array([1, 0, 0, 1, 1, 1, 0, 0]);

  const codec = new PolarCodec(data.length, 16);
  const encoded = codec.encode_bit(data);
  const llr = to_llr(encoded);
  const decoded = codec.decode_bit(llr);

  console.log("Original:", data);
  console.log("Encoded:", encoded);
  const PASS = bytes_eq(data, decoded);
  console.log("Eq:", PASS);
  return !PASS;
}

function withText() {
  console.log("=============================");
  console.log("Polar Codec Example with Text");
  // This time data is a normal BYTE array
  const str = "Hello, world!";
  const data = from_str(str);

  const codec = new PolarCodec(data.length * 8, 128);
  const encoded = codec.encode(data);
  const llr = to_llr(encoded);
  const decoded = codec.decode(llr);

  const text = to_str(decoded);
  console.log("Original:", str);
  console.log("Decoded:", text);
  const PASS = str === text;
  console.log("Eq:", PASS);
  return !PASS;
}

if(simple() || withText()){
  throw new Error("\nExample Failed!");
}else{
  console.log("\nExample Passed.");
}
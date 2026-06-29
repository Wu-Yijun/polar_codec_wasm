/**
 * @module utils
 * @description Shared helper functions for the Polar Codec examples.
 */

/** Decode a `Uint8Array` into a UTF-8 string. */
export function to_str(data: Uint8Array): string {
  return new TextDecoder().decode(data);
}

/** Encode a UTF-8 string into a `Uint8Array`. */
export function from_str(str: string): Uint8Array {
  return new TextEncoder().encode(str);
}

/** Byte-wise equality check for two `Uint8Array`s. */
export function bytes_eq(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

/**
 * Convert a 0/1 bit array into log-likelihood ratios (LLR).
 *
 * Convention: bit `1` -> LLR `-1`, bit `0` -> LLR `+1`.
 */
export function to_llr(data: Uint8Array): Float32Array {
  const llr = new Float32Array(data.length);
  for (let i = 0; i < data.length; i++) {
    llr[i] = data[i] ? -1 : 1;
  }
  return llr;
}

/**
 * Sample from the standard normal distribution using the Box-Muller transform.
 */
export function normal_dist(): number {
  let u = 0, v = 0;
  while (u === 0) u = Math.random();
  while (v === 0) v = Math.random();
  return Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);
}

/**
 * Add Gaussian noise to an LLR array in-place.
 *
 * @param llr         - The LLR values to corrupt.
 * @param noise_level - Standard deviation of the additive noise.
 */
export function add_noise(llr: Float32Array, noise_level: number) {
  for (let i = 0; i < llr.length; i++) {
    llr[i] += normal_dist() * noise_level;
  }
}

/** Generate a random 0/1 bit array of the given length. */
export function bits_array(len: number): Uint8Array {
  const ret = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    ret[i] = Math.round(Math.random());
  }
  return ret;
}

/** Generate a random byte array of the given length (0-255). */
export function bytes_array(len: number): Uint8Array {
  const ret = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    ret[i] = Math.round(Math.random() * 256);
  }
  return ret;
}
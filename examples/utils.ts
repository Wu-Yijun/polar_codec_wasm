export function to_str(data: Uint8Array): string {
  return new TextDecoder().decode(data);
}

export function from_str(str: string): Uint8Array {
  return new TextEncoder().encode(str);
}

export function bytes_eq(a: Uint8Array, b: Uint8Array): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

export function to_llr(data: Uint8Array): Float32Array {
  const llr = new Float32Array(data.length);
  for (let i = 0; i < data.length; i++) {
    llr[i] = data[i] ? -1 : 1;
  }
  return llr;
}

export function normal_dist(): number {
  let u = 0, v = 0;
  while (u === 0) u = Math.random(); // Converting [0,1) to (0,1)
  while (v === 0) v = Math.random();
  return Math.sqrt(-2.0 * Math.log(u)) * Math.cos(2.0 * Math.PI * v);
}

export function add_noise(llr: Float32Array, noise_level: number) {
  for (let i = 0; i < llr.length; i++) {
    llr[i] += normal_dist() * noise_level;
  }
}

export function bits_array(len: number): Uint8Array {
  const ret = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    ret[i] = Math.round(Math.random());
  }
  return ret;
}
export function bytes_array(len: number): Uint8Array {
  const ret = new Uint8Array(len);
  for (let i = 0; i < len; i++) {
    ret[i] = Math.round(Math.random() * 256);
  }
  return ret;
}
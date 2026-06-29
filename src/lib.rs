//! # Polar Codec WASM
//!
//! WebAssembly bindings for the Rust `polar_codec` crate.
//!
//! This crate exposes a [`Codec`] struct to JavaScript/TypeScript via
//! `wasm-bindgen`.  It supports both **bit-level** and **byte-level**
//! encoding/decoding, multiple frozen-bits construction methods (5G, GA, RM),
//! and a wide range of CRC algorithms.
//!
//! ## Architecture
//!
//! ```text
//! TypeScript  ──►  wasm-bindgen  ──►  Codec (this crate)  ──►  polar_codec
//! ```

mod crc_map;

use polar_codec::{FrozenBitsMethod, PolarCodec, polar_crc::CrcEngine};
use wasm_bindgen::prelude::*;

pub use polar_codec::PolarCodeConfig;

use crate::crc_map::{Algorithm, Crc, to_crc};

// ---------------------------------------------------------------------------
// Codec
// ---------------------------------------------------------------------------

/// WebAssembly-exported Polar Codec.
///
/// Wraps the Rust `polar_codec::PolarCodec` and provides encode/decode
/// methods that accept and return flat arrays suitable for JavaScript.
#[wasm_bindgen]
pub struct Codec {
    core: PolarCodec,
}

// ---------------------------------------------------------------------------
// Frozen-bits enum
// ---------------------------------------------------------------------------

/// Frozen-bits construction method exposed to JavaScript.
#[wasm_bindgen]
pub enum FrozenBits {
    /// 5G NR frozen-bits pattern (default for N <= 1024).
    _5G = 0,
    /// Gaussian Approximation -- sigma can be tuned.
    GA,
    /// Reed-Muller based construction (default for N > 1024).
    RM,
}

// ---------------------------------------------------------------------------
// Bit / byte conversion helpers
// ---------------------------------------------------------------------------

/// Convert a bit array (each element 0 or 1) into packed bytes.
///
/// Bits are packed MSB-first: the first bit becomes the most-significant bit
/// of the first byte.  If the length is not a multiple of 8 the last byte is
/// zero-padded on the right.
fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            chunk.iter().enumerate().fold(0u8, |byte, (i, &bit)| {
                if bit == 0 {
                    byte
                } else {
                    byte | (1 << (7 - i)) // MSB first
                }
            })
        })
        .collect()
}

/// Convert packed bytes into a bit array (each element 0 or 1).
///
/// This is the inverse of [`bits_to_bytes`].  Each byte is expanded into 8
/// bits, MSB first.
fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .flat_map(|&byte| (0..8).map(move |i| (byte >> (7 - i)) & 1))
        .collect()
}

// ---------------------------------------------------------------------------
// Codec implementation
// ---------------------------------------------------------------------------

#[wasm_bindgen]
impl Codec {
    /// Create a new Polar Codec instance.
    ///
    /// # Arguments
    ///
    /// * `k`       - Number of information bits per codeword.
    /// * `n`       - Total codeword length (power of two, >= k).
    /// * `l`       - SCL list size (higher = better but slower).
    /// * `crc`     - CRC algorithm selector (see [`Crc`]).
    /// * `fb`      - Frozen-bits construction method (see [`FrozenBits`]).
    /// * `user_crc`- Optional custom CRC algorithm (used when `crc == Crc::UserDefined`).
    /// * `sigma`   - Optional sigma for the GA method (defaults to 0.5).
    #[wasm_bindgen(constructor)]
    pub fn new(
        k: usize,
        n: usize,
        l: usize,
        crc: Crc,
        fb: FrozenBits,
        user_crc: Option<Algorithm>,
        sigma: Option<f64>,
    ) -> Self {
        let crc = to_crc(crc, user_crc);
        let fb = match fb {
            FrozenBits::_5G => FrozenBitsMethod::_5G,
            FrozenBits::GA => FrozenBitsMethod::GA {
                sigma: sigma.unwrap_or(0.5),
            },
            FrozenBits::RM => FrozenBitsMethod::RM,
        };

        Self {
            core: PolarCodec::new(&PolarCodeConfig::<CrcEngine> { k, n, l, crc, fb }),
        }
    }

    /// Encode a sequence of **information bits** into a codeword of `n` bits.
    ///
    /// `src` must contain exactly `k` elements, each `0` or `1`.
    /// Returns a `Uint8Array` of length `n` with the encoded bits (0/1).
    pub fn encode_bit(&mut self, src: &[u8]) -> Vec<u8> {
        let mut dest = vec![0; self.core.n];
        self.core.encode(src, &mut dest);
        dest
    }

    /// Decode a codeword given as **log-likelihood ratios** back to information bits.
    ///
    /// `llr` must contain `n` floats.  A negative value represents bit 1,
    /// a positive value represents bit 0 (higher magnitude = higher confidence).
    /// Returns a `Uint8Array` of length `k` with the decoded information bits.
    pub fn decode_bit(&mut self, llr: &[f32]) -> Vec<u8> {
        let mut dest = vec![0; self.core.k_info];
        self.core.decode(llr, &mut dest);
        dest
    }

    /// Encode a byte array into a codeword.
    ///
    /// Internally converts bytes to bits, encodes, and returns the result as
    /// packed bytes.  `src.len() * 8` must equal `k`.
    pub fn encode(&mut self, src: &[u8]) -> Vec<u8> {
        self.encode_bit(&bytes_to_bits(src))
    }

    /// Decode LLRs back to a byte array.
    ///
    /// The inverse of [`encode`](Codec::encode): decodes the LLRs to bits and
    /// packs them into bytes.  The returned array has length `k / 8` (rounded up).
    pub fn decode(&mut self, llr: &[f32]) -> Vec<u8> {
        bits_to_bytes(&self.decode_bit(llr))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test() {
    let data = b"Hello, World";
    println!("{:?}", data);
    let bits = bytes_to_bits(data);
    let bytes = bits_to_bytes(&bits);
    println!("bits{:?}", bits);
    println!("bytes{:?}", bytes);
    assert_eq!(data.to_vec(), bytes);
}
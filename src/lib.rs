mod crc_map;

use polar_codec::{FrozenBitsMethod, PolarCodec, polar_crc::CrcEngine};
use wasm_bindgen::prelude::*;

pub use polar_codec::PolarCodeConfig;

use crate::crc_map::{Algorithm, Crc, to_crc};

#[wasm_bindgen]
pub struct Codec {
    core: PolarCodec,
}

#[wasm_bindgen]
pub enum FrozenBits {
    _5G = 0,
    GA,
    RM,
}

// 8 -> 1 (合并)
fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| {
            chunk.iter().enumerate().fold(0u8, |byte, (i, &bit)| {
                if bit == 0 {
                    byte
                } else {
                    byte | (1 << (7 - i)) // Most Significant Bit (MSB) first
                }
            })
        })
        .collect()
}

// 1 -> 8 (分离)
fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .flat_map(|&byte| (0..8).map(move |i| (byte >> (7 - i)) & 1))
        .collect()
}

#[wasm_bindgen]
impl Codec {
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

    pub fn encode_bit(&mut self, src: &[u8]) -> Vec<u8> {
        let mut dest = vec![0; self.core.n];
        self.core.encode(src, &mut dest);
        dest
    }
    pub fn decode_bit(&mut self, llr: &[f32]) -> Vec<u8> {
        let mut dest = vec![0; self.core.k_info];
        self.core.decode(llr, &mut dest);
        dest
    }

    pub fn encode(&mut self, src: &[u8]) -> Vec<u8> {
        self.encode_bit(&bytes_to_bits(src))
    }
    pub fn decode(&mut self, llr: &[f32]) -> Vec<u8> {
        bits_to_bytes(&self.decode_bit(llr))
    }
}

#[test]
fn test(){
    let data = b"Hello, World";
    println!("{:?}",data);
    let bits = bytes_to_bits(data);
    let bytes = bits_to_bytes(&bits);
    println!("bits{:?}", bits);
    println!("bytes{:?}", bytes);
    assert_eq!(data.to_vec(), bytes);
    // let c = Codec::new(96, 128, );
}
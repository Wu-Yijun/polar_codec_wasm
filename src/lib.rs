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

    pub fn encode(&mut self, src: &[u8], dest: &mut [u8]) {
        self.core.encode(src, dest);
    }
    pub fn decode(&mut self, llr: &[f32], dest: &mut [u8]) {
        self.core.decode(llr, dest);
    }
}

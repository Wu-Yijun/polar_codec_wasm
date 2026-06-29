//! # CRC Map
//!
//! This module bridges the `crc` crate's CRC algorithms with the WASM
//! boundary.  It provides:
//!
//! - A [`Crc`] enum listing every CRC variant supported by the `crc` crate,
//!   exported to JavaScript via `wasm-bindgen`.
//! - An [`Algorithm`] struct that lets users define a fully custom CRC
//!   algorithm from its canonical parameters.
//! - A [`to_crc`] helper that converts a `(Crc, Option<Algorithm>)` pair
//!   into a ready-to-use [`CrcEngine`].

use std::fmt::Debug;

use polar_codec::{IntoCrcEngine, polar_crc::CrcEngine};
use wasm_bindgen::prelude::wasm_bindgen;

// ---------------------------------------------------------------------------
// Crc enum -- one variant per CRC algorithm in the `crc` crate
// ---------------------------------------------------------------------------

/// Enumeration of all CRC algorithms available in the `crc` crate.
///
/// Each variant maps 1-to-1 with a constant in the `crc` crate (e.g.
/// `crc::CRC_16_UMTS`).  The numeric values are explicit to keep the
/// JavaScript enum stable.
#[allow(non_camel_case_types)]
#[wasm_bindgen]
pub enum Crc {
    /// No CRC -- pass-through.
    None = 0,
    /// Marker: use the user-supplied [`Algorithm`] instead of a built-in.
    UserDefined = 1,
    // -- 3-bit CRCs --
    CRC_3_GSM,
    CRC_3_ROHC,
    // -- 4-bit CRCs --
    CRC_4_G_704,
    CRC_4_INTERLAKEN,
    // -- 5-bit CRCs --
    CRC_5_EPC_C1G2,
    CRC_5_G_704,
    CRC_5_USB,
    // -- 6-bit CRCs --
    CRC_6_CDMA2000_A,
    CRC_6_CDMA2000_B,
    CRC_6_DARC,
    CRC_6_G_704,
    CRC_6_GSM,
    // -- 7-bit CRCs --
    CRC_7_MMC,
    CRC_7_ROHC,
    CRC_7_UMTS,
    // -- 8-bit CRCs --
    CRC_8_AUTOSAR,
    CRC_8_BLUETOOTH,
    CRC_8_CDMA2000,
    CRC_8_DARC,
    CRC_8_DVB_S2,
    CRC_8_GSM_A,
    CRC_8_GSM_B,
    CRC_8_HITAG,
    CRC_8_I_432_1,
    CRC_8_I_CODE,
    CRC_8_LTE,
    CRC_8_MAXIM_DOW,
    CRC_8_MIFARE_MAD,
    CRC_8_NRSC_5,
    CRC_8_OPENSAFETY,
    CRC_8_ROHC,
    CRC_8_SAE_J1850,
    CRC_8_SMBUS,
    CRC_8_TECH_3250,
    CRC_8_WCDMA,
    // -- 10-bit CRCs --
    CRC_10_ATM,
    CRC_10_CDMA2000,
    CRC_10_GSM,
    // -- 11-bit CRCs --
    CRC_11_FLEXRAY,
    CRC_11_UMTS,
    // -- 12-bit CRCs --
    CRC_12_CDMA2000,
    CRC_12_DECT,
    CRC_12_GSM,
    CRC_12_UMTS,
    // -- 13-bit CRCs --
    CRC_13_BBC,
    // -- 14-bit CRCs --
    CRC_14_DARC,
    CRC_14_GSM,
    // -- 15-bit CRCs --
    CRC_15_CAN,
    CRC_15_MPT1327,
    // -- 16-bit CRCs --
    CRC_16_ARC,
    CRC_16_CDMA2000,
    CRC_16_CMS,
    CRC_16_DDS_110,
    CRC_16_DECT_R,
    CRC_16_DECT_X,
    CRC_16_DNP,
    CRC_16_EN_13757,
    CRC_16_GENIBUS,
    CRC_16_GSM,
    CRC_16_IBM_3740,
    CRC_16_IBM_SDLC,
    CRC_16_ISO_IEC_14443_3_A,
    CRC_16_KERMIT,
    CRC_16_LJ1200,
    CRC_16_M17,
    CRC_16_MAXIM_DOW,
    CRC_16_MCRF4XX,
    CRC_16_MODBUS,
    CRC_16_NRSC_5,
    CRC_16_OPENSAFETY_A,
    CRC_16_OPENSAFETY_B,
    CRC_16_PROFIBUS,
    CRC_16_RIELLO,
    CRC_16_SPI_FUJITSU,
    CRC_16_T10_DIF,
    CRC_16_TELEDISK,
    CRC_16_TMS37157,
    CRC_16_UMTS,
    CRC_16_USB,
    CRC_16_XMODEM,
    // -- 17-bit CRCs --
    CRC_17_CAN_FD,
    // -- 21-bit CRCs --
    CRC_21_CAN_FD,
    // -- 24-bit CRCs --
    CRC_24_BLE,
    CRC_24_FLEXRAY_A,
    CRC_24_FLEXRAY_B,
    CRC_24_INTERLAKEN,
    CRC_24_LTE_A,
    CRC_24_LTE_B,
    CRC_24_OPENPGP,
    CRC_24_OS_9,
    // -- 30-bit CRCs --
    CRC_30_CDMA,
    // -- 31-bit CRCs --
    CRC_31_PHILIPS,
    // -- 32-bit CRCs --
    CRC_32_AIXM,
    CRC_32_AUTOSAR,
    CRC_32_BASE91_D,
    CRC_32_BZIP2,
    CRC_32_CD_ROM_EDC,
    CRC_32_CKSUM,
    CRC_32_ISCSI,
    CRC_32_ISO_HDLC,
    CRC_32_JAMCRC,
    CRC_32_MEF,
    CRC_32_MPEG_2,
    CRC_32_XFER,
    // -- 40-bit CRCs --
    CRC_40_GSM,
    // -- 64-bit CRCs --
    CRC_64_ECMA_182,
    CRC_64_GO_ISO,
    CRC_64_MS,
    CRC_64_NVME,
    CRC_64_REDIS,
    CRC_64_WE,
    CRC_64_XZ,
    // -- 82-bit CRCs --
    CRC_82_DARC,
}

// ---------------------------------------------------------------------------
// Algorithm -- user-defined CRC parameters
// ---------------------------------------------------------------------------

/// Description of a user-defined CRC algorithm.
///
/// Fields follow the [Catalogue of parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm).
/// Instances are created from JavaScript via the `new Algorithm(...)` constructor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Algorithm {
    /// The number of bit cells in the linear feedback shift register; the
    /// degree of the generator polynomial, minus one.
    pub width: u8,
    /// The generator polynomial that sets the feedback tap positions of the
    /// shift register.  The least significant bit corresponds to the inward
    /// end of the shift register and is always set.  The highest-order term
    /// is omitted.
    pub poly: u128,
    /// The settings of the bit cells at the start of each calculation,
    /// before reading the first message bit.
    pub init: u128,
    /// If `true`, the characters of the message are read bit-by-bit,
    /// **least** significant bit first.  If `false`, most significant bit
    /// first.
    pub refin: bool,
    /// If `true`, the register contents are reflected before the final XOR.
    pub refout: bool,
    /// The XOR value applied to the register after the last message bit has
    /// been read (and after the optional reflection).
    pub xorout: u128,
}

#[wasm_bindgen]
impl Algorithm {
    /// Construct a new CRC algorithm descriptor.
    ///
    /// This is the JavaScript-callable constructor.
    #[wasm_bindgen(constructor)]
    pub fn new(width: u8, poly: u128, init: u128, refin: bool, refout: bool, xorout: u128) -> Self {
        Self {
            width,
            poly,
            init,
            refin,
            refout,
            xorout,
        }
    }
}

impl Algorithm {
    /// Convert this descriptor into a `crc::Algorithm<W>` of the appropriate
    /// width.
    ///
    /// # Panics
    ///
    /// Panics if any field value does not fit into the target width `W`.
    fn to_crc_algorithm<W: crc::Width + TryFrom<u128>>(&self) -> &'static crc::Algorithm<W>
    where
        <W as TryFrom<u128>>::Error: Debug,
    {
        // Leak the boxed algorithm so it lives for 'static -- acceptable
        // because the number of distinct CRC algorithms is tiny.
        Box::leak(Box::new(crc::Algorithm::<W> {
            width: self.width,
            poly: W::try_from(self.poly).unwrap(),
            init: W::try_from(self.init).unwrap(),
            refin: self.refin,
            refout: self.refout,
            xorout: W::try_from(self.xorout).unwrap(),
            check: W::try_from(0).unwrap(),
            residue: W::try_from(0).unwrap(),
        }))
    }
}

// ---------------------------------------------------------------------------
// IntoCrcEngine -- bridge Algorithm -> CrcEngine
// ---------------------------------------------------------------------------

impl IntoCrcEngine for Algorithm {
    /// Build a [`CrcEngine`] from this algorithm, selecting the smallest
    /// native unsigned integer type that can hold the CRC width.
    fn into_engine(self) -> CrcEngine {
        match self.width {
            1..8   => CrcEngine::W8(crc::Crc::<u8>::new(self.to_crc_algorithm::<u8>())),
            8..16  => CrcEngine::W16(crc::Crc::<u16>::new(self.to_crc_algorithm::<u16>())),
            16..32 => CrcEngine::W32(crc::Crc::<u32>::new(self.to_crc_algorithm::<u32>())),
            32..64 => CrcEngine::W64(crc::Crc::<u64>::new(self.to_crc_algorithm::<u64>())),
            64..128 => CrcEngine::W128(crc::Crc::<u128>::new(self.to_crc_algorithm::<u128>())),
            _ => panic!("Unsupported CRC width: {}", self.width),
        }
    }

    fn width(&self) -> usize {
        self.width as usize
    }
}

// ---------------------------------------------------------------------------
// to_crc -- convert (Crc, Option<Algorithm>) into an optional CrcEngine
// ---------------------------------------------------------------------------

/// Resolve a [`Crc`] enum variant (and optional [`Algorithm`]) into a usable
/// [`CrcEngine`].
///
/// Returns `None` when `crc == Crc::None`.
pub fn to_crc(crc: Crc, algo: Option<Algorithm>) -> Option<CrcEngine> {
    match crc {
        Crc::None => None,
        Crc::UserDefined => algo.map(|a| a.into_engine()),
        // The long arm of match -- one arm per built-in CRC variant.
        Crc::CRC_3_GSM => Some(crc::CRC_3_GSM.into_engine()),
        Crc::CRC_3_ROHC => Some(crc::CRC_3_ROHC.into_engine()),
        Crc::CRC_4_G_704 => Some(crc::CRC_4_G_704.into_engine()),
        Crc::CRC_4_INTERLAKEN => Some(crc::CRC_4_INTERLAKEN.into_engine()),
        Crc::CRC_5_EPC_C1G2 => Some(crc::CRC_5_EPC_C1G2.into_engine()),
        Crc::CRC_5_G_704 => Some(crc::CRC_5_G_704.into_engine()),
        Crc::CRC_5_USB => Some(crc::CRC_5_USB.into_engine()),
        Crc::CRC_6_CDMA2000_A => Some(crc::CRC_6_CDMA2000_A.into_engine()),
        Crc::CRC_6_CDMA2000_B => Some(crc::CRC_6_CDMA2000_B.into_engine()),
        Crc::CRC_6_DARC => Some(crc::CRC_6_DARC.into_engine()),
        Crc::CRC_6_G_704 => Some(crc::CRC_6_G_704.into_engine()),
        Crc::CRC_6_GSM => Some(crc::CRC_6_GSM.into_engine()),
        Crc::CRC_7_MMC => Some(crc::CRC_7_MMC.into_engine()),
        Crc::CRC_7_ROHC => Some(crc::CRC_7_ROHC.into_engine()),
        Crc::CRC_7_UMTS => Some(crc::CRC_7_UMTS.into_engine()),
        Crc::CRC_8_AUTOSAR => Some(crc::CRC_8_AUTOSAR.into_engine()),
        Crc::CRC_8_BLUETOOTH => Some(crc::CRC_8_BLUETOOTH.into_engine()),
        Crc::CRC_8_CDMA2000 => Some(crc::CRC_8_CDMA2000.into_engine()),
        Crc::CRC_8_DARC => Some(crc::CRC_8_DARC.into_engine()),
        Crc::CRC_8_DVB_S2 => Some(crc::CRC_8_DVB_S2.into_engine()),
        Crc::CRC_8_GSM_A => Some(crc::CRC_8_GSM_A.into_engine()),
        Crc::CRC_8_GSM_B => Some(crc::CRC_8_GSM_B.into_engine()),
        Crc::CRC_8_HITAG => Some(crc::CRC_8_HITAG.into_engine()),
        Crc::CRC_8_I_432_1 => Some(crc::CRC_8_I_432_1.into_engine()),
        Crc::CRC_8_I_CODE => Some(crc::CRC_8_I_CODE.into_engine()),
        Crc::CRC_8_LTE => Some(crc::CRC_8_LTE.into_engine()),
        Crc::CRC_8_MAXIM_DOW => Some(crc::CRC_8_MAXIM_DOW.into_engine()),
        Crc::CRC_8_MIFARE_MAD => Some(crc::CRC_8_MIFARE_MAD.into_engine()),
        Crc::CRC_8_NRSC_5 => Some(crc::CRC_8_NRSC_5.into_engine()),
        Crc::CRC_8_OPENSAFETY => Some(crc::CRC_8_OPENSAFETY.into_engine()),
        Crc::CRC_8_ROHC => Some(crc::CRC_8_ROHC.into_engine()),
        Crc::CRC_8_SAE_J1850 => Some(crc::CRC_8_SAE_J1850.into_engine()),
        Crc::CRC_8_SMBUS => Some(crc::CRC_8_SMBUS.into_engine()),
        Crc::CRC_8_TECH_3250 => Some(crc::CRC_8_TECH_3250.into_engine()),
        Crc::CRC_8_WCDMA => Some(crc::CRC_8_WCDMA.into_engine()),
        Crc::CRC_10_ATM => Some(crc::CRC_10_ATM.into_engine()),
        Crc::CRC_10_CDMA2000 => Some(crc::CRC_10_CDMA2000.into_engine()),
        Crc::CRC_10_GSM => Some(crc::CRC_10_GSM.into_engine()),
        Crc::CRC_11_FLEXRAY => Some(crc::CRC_11_FLEXRAY.into_engine()),
        Crc::CRC_11_UMTS => Some(crc::CRC_11_UMTS.into_engine()),
        Crc::CRC_12_CDMA2000 => Some(crc::CRC_12_CDMA2000.into_engine()),
        Crc::CRC_12_DECT => Some(crc::CRC_12_DECT.into_engine()),
        Crc::CRC_12_GSM => Some(crc::CRC_12_GSM.into_engine()),
        Crc::CRC_12_UMTS => Some(crc::CRC_12_UMTS.into_engine()),
        Crc::CRC_13_BBC => Some(crc::CRC_13_BBC.into_engine()),
        Crc::CRC_14_DARC => Some(crc::CRC_14_DARC.into_engine()),
        Crc::CRC_14_GSM => Some(crc::CRC_14_GSM.into_engine()),
        Crc::CRC_15_CAN => Some(crc::CRC_15_CAN.into_engine()),
        Crc::CRC_15_MPT1327 => Some(crc::CRC_15_MPT1327.into_engine()),
        Crc::CRC_16_ARC => Some(crc::CRC_16_ARC.into_engine()),
        Crc::CRC_16_CDMA2000 => Some(crc::CRC_16_CDMA2000.into_engine()),
        Crc::CRC_16_CMS => Some(crc::CRC_16_CMS.into_engine()),
        Crc::CRC_16_DDS_110 => Some(crc::CRC_16_DDS_110.into_engine()),
        Crc::CRC_16_DECT_R => Some(crc::CRC_16_DECT_R.into_engine()),
        Crc::CRC_16_DECT_X => Some(crc::CRC_16_DECT_X.into_engine()),
        Crc::CRC_16_DNP => Some(crc::CRC_16_DNP.into_engine()),
        Crc::CRC_16_EN_13757 => Some(crc::CRC_16_EN_13757.into_engine()),
        Crc::CRC_16_GENIBUS => Some(crc::CRC_16_GENIBUS.into_engine()),
        Crc::CRC_16_GSM => Some(crc::CRC_16_GSM.into_engine()),
        Crc::CRC_16_IBM_3740 => Some(crc::CRC_16_IBM_3740.into_engine()),
        Crc::CRC_16_IBM_SDLC => Some(crc::CRC_16_IBM_SDLC.into_engine()),
        Crc::CRC_16_ISO_IEC_14443_3_A => Some(crc::CRC_16_ISO_IEC_14443_3_A.into_engine()),
        Crc::CRC_16_KERMIT => Some(crc::CRC_16_KERMIT.into_engine()),
        Crc::CRC_16_LJ1200 => Some(crc::CRC_16_LJ1200.into_engine()),
        Crc::CRC_16_M17 => Some(crc::CRC_16_M17.into_engine()),
        Crc::CRC_16_MAXIM_DOW => Some(crc::CRC_16_MAXIM_DOW.into_engine()),
        Crc::CRC_16_MCRF4XX => Some(crc::CRC_16_MCRF4XX.into_engine()),
        Crc::CRC_16_MODBUS => Some(crc::CRC_16_MODBUS.into_engine()),
        Crc::CRC_16_NRSC_5 => Some(crc::CRC_16_NRSC_5.into_engine()),
        Crc::CRC_16_OPENSAFETY_A => Some(crc::CRC_16_OPENSAFETY_A.into_engine()),
        Crc::CRC_16_OPENSAFETY_B => Some(crc::CRC_16_OPENSAFETY_B.into_engine()),
        Crc::CRC_16_PROFIBUS => Some(crc::CRC_16_PROFIBUS.into_engine()),
        Crc::CRC_16_RIELLO => Some(crc::CRC_16_RIELLO.into_engine()),
        Crc::CRC_16_SPI_FUJITSU => Some(crc::CRC_16_SPI_FUJITSU.into_engine()),
        Crc::CRC_16_T10_DIF => Some(crc::CRC_16_T10_DIF.into_engine()),
        Crc::CRC_16_TELEDISK => Some(crc::CRC_16_TELEDISK.into_engine()),
        Crc::CRC_16_TMS37157 => Some(crc::CRC_16_TMS37157.into_engine()),
        Crc::CRC_16_UMTS => Some(crc::CRC_16_UMTS.into_engine()),
        Crc::CRC_16_USB => Some(crc::CRC_16_USB.into_engine()),
        Crc::CRC_16_XMODEM => Some(crc::CRC_16_XMODEM.into_engine()),
        Crc::CRC_17_CAN_FD => Some(crc::CRC_17_CAN_FD.into_engine()),
        Crc::CRC_21_CAN_FD => Some(crc::CRC_21_CAN_FD.into_engine()),
        Crc::CRC_24_BLE => Some(crc::CRC_24_BLE.into_engine()),
        Crc::CRC_24_FLEXRAY_A => Some(crc::CRC_24_FLEXRAY_A.into_engine()),
        Crc::CRC_24_FLEXRAY_B => Some(crc::CRC_24_FLEXRAY_B.into_engine()),
        Crc::CRC_24_INTERLAKEN => Some(crc::CRC_24_INTERLAKEN.into_engine()),
        Crc::CRC_24_LTE_A => Some(crc::CRC_24_LTE_A.into_engine()),
        Crc::CRC_24_LTE_B => Some(crc::CRC_24_LTE_B.into_engine()),
        Crc::CRC_24_OPENPGP => Some(crc::CRC_24_OPENPGP.into_engine()),
        Crc::CRC_24_OS_9 => Some(crc::CRC_24_OS_9.into_engine()),
        Crc::CRC_30_CDMA => Some(crc::CRC_30_CDMA.into_engine()),
        Crc::CRC_31_PHILIPS => Some(crc::CRC_31_PHILIPS.into_engine()),
        Crc::CRC_32_AIXM => Some(crc::CRC_32_AIXM.into_engine()),
        Crc::CRC_32_AUTOSAR => Some(crc::CRC_32_AUTOSAR.into_engine()),
        Crc::CRC_32_BASE91_D => Some(crc::CRC_32_BASE91_D.into_engine()),
        Crc::CRC_32_BZIP2 => Some(crc::CRC_32_BZIP2.into_engine()),
        Crc::CRC_32_CD_ROM_EDC => Some(crc::CRC_32_CD_ROM_EDC.into_engine()),
        Crc::CRC_32_CKSUM => Some(crc::CRC_32_CKSUM.into_engine()),
        Crc::CRC_32_ISCSI => Some(crc::CRC_32_ISCSI.into_engine()),
        Crc::CRC_32_ISO_HDLC => Some(crc::CRC_32_ISO_HDLC.into_engine()),
        Crc::CRC_32_JAMCRC => Some(crc::CRC_32_JAMCRC.into_engine()),
        Crc::CRC_32_MEF => Some(crc::CRC_32_MEF.into_engine()),
        Crc::CRC_32_MPEG_2 => Some(crc::CRC_32_MPEG_2.into_engine()),
        Crc::CRC_32_XFER => Some(crc::CRC_32_XFER.into_engine()),
        Crc::CRC_40_GSM => Some(crc::CRC_40_GSM.into_engine()),
        Crc::CRC_64_ECMA_182 => Some(crc::CRC_64_ECMA_182.into_engine()),
        Crc::CRC_64_GO_ISO => Some(crc::CRC_64_GO_ISO.into_engine()),
        Crc::CRC_64_MS => Some(crc::CRC_64_MS.into_engine()),
        Crc::CRC_64_NVME => Some(crc::CRC_64_NVME.into_engine()),
        Crc::CRC_64_REDIS => Some(crc::CRC_64_REDIS.into_engine()),
        Crc::CRC_64_WE => Some(crc::CRC_64_WE.into_engine()),
        Crc::CRC_64_XZ => Some(crc::CRC_64_XZ.into_engine()),
        Crc::CRC_82_DARC => Some(crc::CRC_82_DARC.into_engine()),
    }
}

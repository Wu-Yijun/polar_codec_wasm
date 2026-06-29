use std::fmt::Debug;

use polar_codec::{IntoCrcEngine, polar_crc::CrcEngine};
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(non_camel_case_types)]
#[wasm_bindgen]
pub enum Crc {
    None = 0,
    UserDefined = 1,
    CRC_3_GSM,
    CRC_3_ROHC,
    CRC_4_G_704,
    CRC_4_INTERLAKEN,
    CRC_5_EPC_C1G2,
    CRC_5_G_704,
    CRC_5_USB,
    CRC_6_CDMA2000_A,
    CRC_6_CDMA2000_B,
    CRC_6_DARC,
    CRC_6_G_704,
    CRC_6_GSM,
    CRC_7_MMC,
    CRC_7_ROHC,
    CRC_7_UMTS,
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
    CRC_10_ATM,
    CRC_10_CDMA2000,
    CRC_10_GSM,
    CRC_11_FLEXRAY,
    CRC_11_UMTS,
    CRC_12_CDMA2000,
    CRC_12_DECT,
    CRC_12_GSM,
    CRC_12_UMTS,
    CRC_13_BBC,
    CRC_14_DARC,
    CRC_14_GSM,
    CRC_15_CAN,
    CRC_15_MPT1327,
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
    CRC_17_CAN_FD,
    CRC_21_CAN_FD,
    CRC_24_BLE,
    CRC_24_FLEXRAY_A,
    CRC_24_FLEXRAY_B,
    CRC_24_INTERLAKEN,
    CRC_24_LTE_A,
    CRC_24_LTE_B,
    CRC_24_OPENPGP,
    CRC_24_OS_9,
    CRC_30_CDMA,
    CRC_31_PHILIPS,
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
    CRC_40_GSM,
    CRC_64_ECMA_182,
    CRC_64_GO_ISO,
    CRC_64_MS,
    CRC_64_NVME,
    CRC_64_REDIS,
    CRC_64_WE,
    CRC_64_XZ,
    CRC_82_DARC,
}

/// This struct describes a CRC algorithm using the fields specified by the [Catalogue of
/// parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Algorithm {
    /// The number of bit cells in the linear feedback shift register; the degree of the generator
    /// polynomial, minus one.
    pub width: u8,
    /// The generator polynomial that sets the feedback tap positions of the shift register. The
    /// least significant bit corresponds to the inward end of the shift register, and is always
    /// set. The highest-order term is omitted.
    pub poly: u128,
    /// The settings of the bit cells at the start of each calculation, before reading the first
    /// message bit. The least significant bit corresponds to the inward end of the shift register.
    pub init: u128,
    /// If equal to `false`, specifies that the characters of the message are read bit-by-bit, most
    /// significant bit (MSB) first; if equal to `true`, the characters are read bit-by-bit, least
    /// significant bit (LSB) first. Each sampled message bit is then XORed with the bit being
    /// simultaneously shifted out of the register at the most significant end, and the result is
    /// passed to the feedback taps.
    pub refin: bool,
    /// If equal to `false`, specifies that the contents of the register after reading the last
    /// message bit are unreflected before presentation; if equal to `true`, it specifies that they
    /// are reflected, character-by-character, before presentation. For the purpose of this
    /// definition, the reflection is performed by swapping the content of each cell with that of
    /// the cell an equal distance from the opposite end of the register; the characters of the CRC
    /// are then true images of parts of the reflected register, the character containing the
    /// original MSB always appearing first.
    pub refout: bool,
    /// The XOR value applied to the contents of the register after the last message bit has been
    /// read and after the optional reflection. It has the same endianness as the CRC such that its
    /// true image appears in the characters of the CRC.
    pub xorout: u128,
    // /// The contents of the register after initialising, reading the UTF-8 string `"123456789"` (as
    // /// 8-bit characters), optionally reflecting, and applying the final XOR.
    // pub check: u128,
    // /// The contents of the register after initialising, reading an error-free codeword and
    // /// optionally reflecting the register (if [`refout`](Algorithm::refout)=`true`), but not
    // /// applying the final XOR. This is mathematically equivalent to initialising the register with
    // /// the xorout parameter, reflecting it as described (if [`refout`](Algorithm::refout)=`true`),
    // /// reading as many zero bits as there are cells in the register, and reflecting the result (if
    // /// [`refin`](Algorithm::refin)=`true`). The residue of a crossed-endian model is calculated
    // /// assuming that the characters of the received CRC are specially reflected before submitting
    // /// the codeword.
    // pub residue: u128,
}

#[wasm_bindgen]
impl Algorithm {
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
    fn to_crc_algorithm<W: crc::Width + TryFrom<u128>>(&self) -> &'static crc::Algorithm<W>
    where
        <W as TryFrom<u128>>::Error: Debug,
    {
        Box::leak(Box::new(crc::Algorithm::<W> {
            width: self.width,
            poly: W::try_from(self.poly).unwrap(),
            init: W::try_from(self.init).unwrap(),
            refin: self.refin,
            refout: self.refout,
            xorout: W::try_from(self.xorout).unwrap(),
            check: W::try_from(0).unwrap(),
            residue: W::try_from(0).unwrap(), // check: W::try_from(self.check).unwrap(),
                                              // residue: W::try_from(self.residue).unwrap(),
        }))
    }
}

impl IntoCrcEngine for Algorithm {
    fn into_engine(self) -> CrcEngine {
        match self.width {
            1..8 => CrcEngine::W8(crc::Crc::<u8>::new(self.to_crc_algorithm::<u8>())),
            8..16 => CrcEngine::W16(crc::Crc::<u16>::new(self.to_crc_algorithm::<u16>())),
            16..32 => CrcEngine::W32(crc::Crc::<u32>::new(self.to_crc_algorithm::<u32>())),
            32..64 => CrcEngine::W64(crc::Crc::<u64>::new(self.to_crc_algorithm::<u64>())),
            64..128 => CrcEngine::W128(crc::Crc::<u128>::new(self.to_crc_algorithm::<u128>())),
            _ => panic!("Unsupport width {}", self.width),
        }
    }
    fn width(&self) -> usize {
        self.width as usize
    }
}

pub fn to_crc(crc: Crc, algo: Option<Algorithm>) -> Option<CrcEngine> {
    match crc {
        Crc::None => None,
        Crc::UserDefined => algo.map(|a| a.into_engine()),
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

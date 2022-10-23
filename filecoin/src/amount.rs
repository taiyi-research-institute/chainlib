use chainlib_core::{Amount, AmountError, to_basic_unit as to_atto_fil};

use core::fmt;
use chainlib_core::ethereum_types::U256;

/// Represents the amount of Ethereum in wei
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FilecoinAmount(pub U256);

pub enum Denomination {
    AttoFIL,
    FemtoFIL,
    PicoFIL,
    NanoFIL,
    MicroFIL,
    MilliFIL,
    FIL,
}

impl Denomination {
    /// The number of decimal places more than a wei.
    fn precision(self) -> u32 {
        match self {
            Denomination::AttoFIL => 0,
            Denomination::FemtoFIL => 3,
            Denomination::PicoFIL => 6,
            Denomination::NanoFIL => 9,
            Denomination::MicroFIL => 12,
            Denomination::MilliFIL => 15,
            Denomination::FIL => 18,
        }
    }
}

impl fmt::Display for Denomination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Denomination::AttoFIL => "attoFIL",
                Denomination::FemtoFIL => "femtoFIL",
                Denomination::PicoFIL => "picoFIL",
                Denomination::NanoFIL => "nanoFIL",
                Denomination::MicroFIL => "microFIL",
                Denomination::MilliFIL => "milliFIL",
                Denomination::FIL => "FIL",
            }
        )
    }
}

impl Amount for FilecoinAmount {}

impl FilecoinAmount {
    pub fn u256_from_str(val: &str) -> Result<U256, AmountError> {
        match U256::from_dec_str(val) {
            Ok(atto_fil) => Ok(atto_fil),
            Err(error) => return Err(AmountError::Crate("uint", format!("{:?}", error))),
        }
    }

    pub fn from_u256(atto_fil: U256) -> Self {
        Self(atto_fil)
    }

    pub fn from_atto_fil(atto_fil_value: &str) -> Result<Self, AmountError> {
        let atto_fil = Self::u256_from_str(atto_fil_value)?;

        Ok(Self::from_u256(atto_fil))
    }

    pub fn from_femto_fil(femto_fil_value: &str) -> Result<Self, AmountError> {
        let atto_fil_value = to_atto_fil(femto_fil_value, Denomination::FemtoFIL.precision());
        let atto_fil = Self::u256_from_str(&atto_fil_value)?;
        
        Ok(Self::from_u256(atto_fil))
    }

    pub fn from_pico_fil(pico_fil_value: &str) -> Result<Self, AmountError> {
        let atto_fil_value = to_atto_fil(pico_fil_value, Denomination::PicoFIL.precision());
        let atto_fil = Self::u256_from_str(&atto_fil_value)?;

        Ok(Self::from_u256(atto_fil))
    }

    pub fn from_nano_fil(nano_fil_value: &str) -> Result<Self, AmountError> {
        let atto_fil_value = to_atto_fil(nano_fil_value, Denomination::NanoFIL.precision());
        let atto_fil = Self::u256_from_str(&atto_fil_value)?;
        Ok(Self::from_u256(atto_fil))
    }

    pub fn from_micro_fil(micro_fil_value: &str) -> Result<Self, AmountError> {
        let atto_fil_value = to_atto_fil(micro_fil_value, Denomination::MilliFIL.precision());
        let atto_fil = Self::u256_from_str(&atto_fil_value)?;

        Ok(Self::from_u256(atto_fil))
    }

    pub fn from_milli_fil(milli_fil_value: &str) -> Result<Self, AmountError> {
        let atto_fil_value = to_atto_fil(milli_fil_value, Denomination::MilliFIL.precision());
        let atto_fil = Self::u256_from_str(&atto_fil_value)?;

        Ok(Self::from_u256(atto_fil))
    }

    pub fn from_fil(fil_value: &str) -> Result<Self, AmountError> {
        let atto_fil_value = to_atto_fil(fil_value, Denomination::FIL.precision());
        let atto_fil = Self::u256_from_str(&atto_fil_value)?;
        
        Ok(Self::from_u256(atto_fil))
    }

    pub fn add(self, b: Self) -> Self {
        Self::from_u256(self.0 + b.0)
    }

    pub fn sub(self, b: Self) -> Self {
        Self::from_u256(self.0 - b.0)
    }
}

impl fmt::Display for FilecoinAmount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

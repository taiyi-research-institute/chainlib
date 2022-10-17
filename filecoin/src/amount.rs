use chainlib_core::{Amount, AmountError};

use core::fmt;
use chainlib_core::ethereum_types::U256;

/// Represents the amount of Ethereum in wei
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FilecoinAmount(pub U256);

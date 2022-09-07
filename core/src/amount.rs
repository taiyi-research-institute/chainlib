use crate::no_std::*;
use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

/// The interface for a generic amount.
pub trait Amount: Copy + Clone + Debug + Display + Send + Sync + 'static + Eq + Ord + Sized + Hash {}

#[derive(Debug, Error)]
pub enum AmountError {
    #[error("{0}: {1}")]
    Crate(&'static str, String),
    
    #[error("the amount: {0:} exceeds the supply bounds of {1:}")]
    AmountOutOfBounds(String, String),

    #[error("invalid amount: {0:}")]
    InvalidAmount(String),
}

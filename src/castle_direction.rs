
use core::marker::ConstParamTy;

/// The direction to castle in for either side
#[repr(u8)]
#[derive(Copy, Clone, ConstParamTy, Debug, PartialEq, Eq)]
pub enum  CastleDirection {
    KingSide,
    QueenSide,
}


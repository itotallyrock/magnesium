use core::marker::ConstParamTy;

#[repr(u8)]
#[derive(ConstParamTy, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[repr(u8)]
#[derive(ConstParamTy, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum NonKingPieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
}

#[repr(u8)]
#[derive(ConstParamTy, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum PromotionPieceType {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PieceType {
    pub const COUNT: usize = 6;
}

impl NonKingPieceType {
    pub const COUNT: usize = 5;
    pub const fn to_piece_type(self) -> PieceType {
        match self {
            NonKingPieceType::Pawn => PieceType::Pawn,
            NonKingPieceType::Knight => PieceType::Knight,
            NonKingPieceType::Bishop => PieceType::Bishop,
            NonKingPieceType::Rook => PieceType::Rook,
            NonKingPieceType::Queen => PieceType::Queen,
        }
    }
}

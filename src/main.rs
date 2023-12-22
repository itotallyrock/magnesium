#![feature(const_trait_impl, structural_match)]

use std::marker::StructuralEq;

pub type Player = bool;
pub const WHITE: Player = false;
pub const BLACK: Player = true;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub const fn to_mask(self) -> Bitboard {
        1u64 << (self as u8)
    }
}

pub type Bitboard = u64;

/// The direction to castle in for either side
pub type CastleDirection = bool;
/// Castle with the rook on the same side as the king
const KING_SIDE: CastleDirection = false;
/// Castle with the rook on the same side as the queen
const QUEEN_SIDE: CastleDirection = true;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct BoardStatus {
    pub side_to_move: Player,
    pub has_ep_pawn: bool,
    pub can_white_king_castle: bool,
    pub can_white_queen_castle: bool,
    pub can_black_king_castle: bool,
    pub can_black_queen_castle: bool,
}

impl BoardStatus {
    const WHITE_NOT_OCCUPIED_QUEEN: Bitboard = 0b01110000u64;
    const WHITE_NOT_ATTACKED_QUEEN: Bitboard = 0b00111000u64;

    const WHITE_NOT_OCCUPIED_KING: Bitboard = 0b00000110u64;
    const WHITE_NOT_ATTACKED_KING: Bitboard = 0b00001110u64;

    const BLACK_NOT_OCCUPIED_KING: Bitboard = 0b01110000u64 << 56u64;
    const BLACK_NOT_ATTACKED_KING: Bitboard = 0b00111000u64 << 56u64;

    const BLACK_NOT_OCCUPIED_QUEEN: Bitboard = 0b00000110u64 << 56u64;
    const BLACK_NOT_ATTACKED_QUEEN: Bitboard = 0b00001110u64 << 56u64;

    const WHITE_ROOK_QUEEN_CHANGE: Bitboard = 0b11111000u64;
    const BLACK_ROOK_QUEEN_CHANGE: Bitboard = 0b11111000u64 << 56u64;
    const WHITE_ROOK_KING_CHANGE: Bitboard = 0b00001111u64;
    const BLACK_ROOK_KING_CHANGE: Bitboard = 0b00001111u64 << 56u64;

    const WHITE_ROOK_QUEEN: Bitboard = 0b10000000u64;
    const BLACK_ROOK_KING: Bitboard = 0b10000000u64 << 56u64;
    const WHITE_ROOK_KING: Bitboard = 0b00000001u64;
    const BLACK_ROOK_QUEEN: Bitboard = 0b00000001u64 << 56u64;
    const fn can_castle(self, castle_direction: CastleDirection, attacked: Bitboard, occupied: Bitboard, rook: Bitboard) -> bool {
        (castle_direction == KING_SIDE && (self.side_to_move == WHITE && self.can_white_king_castle && occupied & Self::WHITE_NOT_OCCUPIED_KING == 0u64 && attacked & Self::WHITE_NOT_ATTACKED_KING == 0u64 && rook & Self::WHITE_ROOK_KING != 0u64)
        || (self.can_black_king_castle && occupied & Self::BLACK_NOT_OCCUPIED_KING == 0u64 && attacked & Self::BLACK_NOT_ATTACKED_KING == 0u64 && rook & Self::BLACK_ROOK_KING != 0u64))
        || (self.side_to_move == WHITE && self.can_white_queen_castle && occupied & Self::WHITE_NOT_OCCUPIED_QUEEN == 0u64 && attacked & Self::WHITE_NOT_ATTACKED_QUEEN == 0u64 && rook & Self::WHITE_ROOK_QUEEN != 0u64)
            || (self.can_black_queen_castle && occupied & Self::BLACK_NOT_OCCUPIED_QUEEN == 0u64 && attacked & Self::BLACK_NOT_ATTACKED_QUEEN == 0u64 && rook & Self::BLACK_ROOK_QUEEN != 0u64)
    }

    const fn pawn_push(self) -> Self {
        Self {
            side_to_move: !self.side_to_move,
            has_ep_pawn: true,
            ..self
        }
    }

    const fn king_move(self) -> Self {
        Self {
            side_to_move: !self.side_to_move,
            can_white_king_castle: self.side_to_move != WHITE && self.can_white_king_castle,
            can_white_queen_castle: self.side_to_move != WHITE && self.can_white_queen_castle,
            can_black_king_castle: self.side_to_move != BLACK && self.can_black_king_castle,
            can_black_queen_castle: self.side_to_move != BLACK && self.can_black_queen_castle,
            ..self
        }
    }

    const fn quiet_move(self) -> Self {
        Self {
            side_to_move: !self.side_to_move,
            has_ep_pawn: false,
            ..self
        }
    }

    const fn rook_move(self, castle_direction: CastleDirection) -> Self {
        Self {
            side_to_move: !self.side_to_move,
            can_white_king_castle: self.side_to_move != WHITE && castle_direction != KING_SIDE && self.can_white_king_castle,
            can_white_queen_castle: self.side_to_move != WHITE && castle_direction != QUEEN_SIDE && self.can_white_queen_castle,
            can_black_king_castle: self.side_to_move != BLACK && castle_direction != KING_SIDE && self.can_black_king_castle,
            can_black_queen_castle: self.side_to_move != BLACK && castle_direction != QUEEN_SIDE && self.can_black_queen_castle,
            ..self
        }
    }
}

fn main() {
    println!("Hello, world!");
}

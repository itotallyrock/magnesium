#![feature(const_trait_impl, structural_match)]

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
    King,
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

pub const EMPTY_BITBOARD: Bitboard = 0u64;

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
    pub white_king_castle_rights: bool,
    pub white_queen_castle_rights: bool,
    pub black_king_castle_rights: bool,
    pub black_queen_castle_rights: bool,
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
    const fn can_castle(
        self,
        castle_direction: CastleDirection,
        attacked: Bitboard,
        occupied: Bitboard,
        rooks: Bitboard,
    ) -> bool {
        (castle_direction == KING_SIDE
            // King side white
            && (self.side_to_move == WHITE
                && self.white_king_castle_rights
                && occupied & Self::WHITE_NOT_OCCUPIED_KING == EMPTY_BITBOARD
                && attacked & Self::WHITE_NOT_ATTACKED_KING == EMPTY_BITBOARD
                && rooks & Self::WHITE_ROOK_KING != EMPTY_BITBOARD)
            // King side black
            || (self.black_king_castle_rights
                && occupied & Self::BLACK_NOT_OCCUPIED_KING == EMPTY_BITBOARD
                && attacked & Self::BLACK_NOT_ATTACKED_KING == EMPTY_BITBOARD
                && rooks & Self::BLACK_ROOK_KING != EMPTY_BITBOARD))
            // Queen side white
            || (self.side_to_move == WHITE
                && self.white_queen_castle_rights
                && occupied & Self::WHITE_NOT_OCCUPIED_QUEEN == EMPTY_BITBOARD
                && attacked & Self::WHITE_NOT_ATTACKED_QUEEN == EMPTY_BITBOARD
                && rooks & Self::WHITE_ROOK_QUEEN != EMPTY_BITBOARD)
            // Queen side black
            || (self.black_queen_castle_rights
                && occupied & Self::BLACK_NOT_OCCUPIED_QUEEN == EMPTY_BITBOARD
                && attacked & Self::BLACK_NOT_ATTACKED_QUEEN == EMPTY_BITBOARD
                && rooks & Self::BLACK_ROOK_QUEEN != EMPTY_BITBOARD)
    }

    const fn double_pawn_push(self) -> Self {
        Self {
            side_to_move: !self.side_to_move,
            has_ep_pawn: true,
            ..self
        }
    }

    const fn king_move(self) -> Self {
        Self {
            side_to_move: !self.side_to_move,
            has_ep_pawn: false,
            white_king_castle_rights: self.side_to_move != WHITE && self.white_king_castle_rights,
            white_queen_castle_rights: self.side_to_move != WHITE && self.white_queen_castle_rights,
            black_king_castle_rights: self.side_to_move != BLACK && self.black_king_castle_rights,
            black_queen_castle_rights: self.side_to_move != BLACK && self.black_queen_castle_rights,
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

    const fn rook_move<const CASTLE_DIRECTION: CastleDirection>(self) -> Self {
        let white_king_castle_rights = (self.side_to_move != WHITE
            || CASTLE_DIRECTION == KING_SIDE)
            && self.white_king_castle_rights;
        let white_queen_castle_rights = (self.side_to_move != WHITE
            || CASTLE_DIRECTION == QUEEN_SIDE)
            && self.white_queen_castle_rights;
        let black_king_castle_rights = (self.side_to_move != BLACK
            || CASTLE_DIRECTION == KING_SIDE)
            && self.black_king_castle_rights;
        let black_queen_castle_rights = (self.side_to_move != BLACK
            || CASTLE_DIRECTION == QUEEN_SIDE)
            && self.black_queen_castle_rights;
        Self {
            // Switch sides
            side_to_move: !self.side_to_move,
            has_ep_pawn: false,
            white_king_castle_rights,
            white_queen_castle_rights,
            black_king_castle_rights,
            black_queen_castle_rights,
            ..self
        }
    }
}

pub fn main() {
    let board_status = BoardStatus {
        side_to_move: WHITE,
        has_ep_pawn: false,
        white_king_castle_rights: true,
        white_queen_castle_rights: true,
        black_king_castle_rights: true,
        black_queen_castle_rights: true,
    };
    let attacked: Bitboard = 0;
    let occupied: Bitboard = 0b1111111110010001u64;
    let rooks: Bitboard = 0b10000001u64;
    println!(
        "can white king castle {}",
        board_status.can_castle(KING_SIDE, attacked, occupied, rooks)
    );
}

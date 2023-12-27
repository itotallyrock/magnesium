use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::piece_type::{NonKingPieceType, PieceType};
use crate::player::Player;
use crate::square::Square;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct PieceArrangement {
    king_squares: [Square; Player::COUNT],
    occupied_by_player: [Bitboard; Player::COUNT],
    occupied_by_piece: [Bitboard; NonKingPieceType::COUNT],
    piece_by_square: [Option<PieceType>; Square::COUNT],
}

impl PieceArrangement {
    pub const fn king_square<const IS_WHITE: bool>(&self) -> Square {
        self.king_squares[IS_WHITE as usize]
    }

    pub const fn piece_type_on(&self, square: Square) -> Option<PieceType> {
        self.piece_by_square[square as usize]
    }

    pub const fn player_on(&self, square: Square) -> Option<Player> {
        let square = square.to_bit();
        if self.occupied_by_player[true as usize] & square != EMPTY_BITBOARD {
            Some(Player::White)
        } else if self.occupied_by_player[false as usize] & square != EMPTY_BITBOARD {
            Some(Player::Black)
        } else {
            None
        }
    }

    pub const fn occupied(&self) -> Bitboard {
        self.occupied_by_player[Player::White as usize]
            | self.occupied_by_player[Player::Black as usize]
    }

    pub const fn mask_for_piece<const PIECE_TYPE: NonKingPieceType>(&self) -> Bitboard {
        self.occupied_by_piece[PIECE_TYPE as usize]
    }

    pub const fn mask_for_player<const IS_WHITE: bool>(&self) -> Bitboard {
        self.occupied_by_player[IS_WHITE as usize]
    }

    pub const fn mask_for_player_and_piece<
        const IS_WHITE: bool,
        const PIECE_TYPE: NonKingPieceType,
    >(
        &self,
    ) -> Bitboard {
        self.mask_for_player::<{ IS_WHITE }>() & self.mask_for_piece::<PIECE_TYPE>()
    }

    pub const fn new(white_king: Square, black_king: Square) -> Self {
        let king_squares = [black_king, white_king];
        let occupied_by_player = [black_king.to_bit(), white_king.to_bit()];
        let piece_by_square = {
            let mut piece_by_square = [None; Square::COUNT];
            piece_by_square[white_king as usize] = Some(PieceType::King);
            piece_by_square[black_king as usize] = Some(PieceType::King);
            piece_by_square
        };

        Self {
            king_squares,
            occupied_by_player,
            piece_by_square,
            occupied_by_piece: [EMPTY_BITBOARD; NonKingPieceType::COUNT],
        }
    }

    pub const fn add_piece<const IS_WHITE: bool, const PIECE: NonKingPieceType>(
        self,
        square: Square,
    ) -> Self {
        let Self {
            mut occupied_by_player,
            mut occupied_by_piece,
            mut piece_by_square,
            king_squares: _,
        } = self;
        debug_assert!(self.occupied() & square.to_bit() == EMPTY_BITBOARD);
        occupied_by_player[IS_WHITE as usize] |= square.to_bit();
        occupied_by_piece[PIECE as usize] |= square.to_bit();
        piece_by_square[square as usize] = Some(PIECE.to_piece_type());
        Self {
            occupied_by_player,
            occupied_by_piece,
            piece_by_square,
            ..self
        }
    }

    pub const fn remove_piece<const IS_WHITE: bool, const PIECE: NonKingPieceType>(
        self,
        square: Square,
    ) -> Self {
        let Self {
            mut occupied_by_player,
            mut occupied_by_piece,
            mut piece_by_square,
            king_squares: _,
        } = self;
        debug_assert!(occupied_by_piece[PIECE as usize] & square.to_bit() != EMPTY_BITBOARD);
        debug_assert!(occupied_by_player[IS_WHITE as usize] & square.to_bit() != EMPTY_BITBOARD);
        occupied_by_player[IS_WHITE as usize] &= !square.to_bit();
        occupied_by_piece[PIECE as usize] &= !square.to_bit();
        piece_by_square[square as usize] = None;

        Self {
            occupied_by_player,
            occupied_by_piece,
            piece_by_square,
            ..self
        }
    }

    pub const fn move_piece<const IS_WHITE: bool, const PIECE: PieceType>(
        self,
        from: Square,
        to: Square,
    ) -> Self {
        let Self {
            mut occupied_by_player,
            mut occupied_by_piece,
            mut piece_by_square,
            mut king_squares,
        } = self;
        debug_assert!(self.occupied() & to.to_bit() == EMPTY_BITBOARD);
        let from_to = from.to_bit() | to.to_bit();
        if PIECE as u8 == PieceType::King as u8 {
            debug_assert!(king_squares[IS_WHITE as usize] as u8 == from as u8);
            king_squares[IS_WHITE as usize] = to;
        } else {
            debug_assert!(occupied_by_piece[PIECE as usize] & from.to_bit() != EMPTY_BITBOARD);
            occupied_by_piece[PIECE as usize] ^= from_to;
        }
        debug_assert!(occupied_by_player[IS_WHITE as usize] & from.to_bit() != EMPTY_BITBOARD);
        occupied_by_player[IS_WHITE as usize] ^= from_to;
        piece_by_square[from as usize] = None;
        piece_by_square[to as usize] = Some(PIECE);

        Self {
            occupied_by_player,
            occupied_by_piece,
            piece_by_square,
            king_squares,
            ..self
        }
    }

    pub const fn move_by_squares<const IS_WHITE: bool>(self, from: Square, to: Square) -> Self {
        match self.piece_type_on(from).unwrap() {
            PieceType::Pawn => self.move_piece::<{ IS_WHITE }, { PieceType::Pawn }>(from, to),
            PieceType::Knight => self.move_piece::<{ IS_WHITE }, { PieceType::Knight }>(from, to),
            PieceType::Bishop => self.move_piece::<{ IS_WHITE }, { PieceType::Bishop }>(from, to),
            PieceType::Rook => self.move_piece::<{ IS_WHITE }, { PieceType::Rook }>(from, to),
            PieceType::Queen => self.move_piece::<{ IS_WHITE }, { PieceType::Queen }>(from, to),
            PieceType::King => self.move_piece::<{ IS_WHITE }, { PieceType::King }>(from, to),
        }
    }

    pub const fn remove_by_square<const IS_WHITE: bool>(self, from: Square) -> Self {
        match self.piece_type_on(from).unwrap() {
            PieceType::Pawn => self.remove_piece::<{ IS_WHITE }, { NonKingPieceType::Pawn }>(from),
            PieceType::Knight => {
                self.remove_piece::<{ IS_WHITE }, { NonKingPieceType::Knight }>(from)
            }
            PieceType::Bishop => {
                self.remove_piece::<{ IS_WHITE }, { NonKingPieceType::Bishop }>(from)
            }
            PieceType::Rook => self.remove_piece::<{ IS_WHITE }, { NonKingPieceType::Rook }>(from),
            PieceType::Queen => {
                self.remove_piece::<{ IS_WHITE }, { NonKingPieceType::Queen }>(from)
            }
            PieceType::King => panic!("attempting to remove king"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO
}

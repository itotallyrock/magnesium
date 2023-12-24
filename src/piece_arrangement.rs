use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::piece_type::{NonKingPieceType, PieceType};
use crate::player::Player;
use crate::square::Square;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct PieceArrangement {
    king_squares: [Square; Player::COUNT],
    occupied_by_player: [Bitboard; Player::COUNT],
    occupied_by_piece: [Bitboard; NonKingPieceType::COUNT],
    piece_by_square: [Option<PieceType>; Square::COUNT],
}

impl PieceArrangement {
    pub const fn king_square<const PLAYER: Player>(&self) -> Square {
        self.king_squares[PLAYER as usize]
    }

    pub const fn piece_type_on(&self, square: Square) -> Option<PieceType> {
        self.piece_by_square[square as usize]
    }

    pub const fn player_on(&self, square: Square) -> Option<Player> {
        let square = square.to_bit();
        if self.occupied_by_player[Player::White as usize] & square != EMPTY_BITBOARD {
            Some(Player::White)
        } else if self.occupied_by_player[Player::Black as usize] & square != EMPTY_BITBOARD {
            Some(Player::Black)
        } else {
            None
        }
    }

    pub const fn mask_for_piece<const PIECE_TYPE: NonKingPieceType>(&self) -> Bitboard {
        self.occupied_by_piece[PIECE_TYPE as usize]
    }

    pub const fn mask_for_player<const PLAYER: Player>(&self) -> Bitboard {
        self.occupied_by_player[PLAYER as usize]
    }

    pub const fn mask_for_player_and_piece<
        const PLAYER: Player,
        const PIECE_TYPE: NonKingPieceType,
    >(
        &self,
    ) -> Bitboard {
        self.mask_for_player::<PLAYER>() & self.mask_for_piece::<PIECE_TYPE>()
    }

    pub const fn new(white_king: Square, black_king: Square) -> Self {
        let king_squares = [white_king, black_king];
        let occupied_by_player = [white_king.to_bit(), black_king.to_bit()];
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

    pub const fn add_piece<const PLAYER: Player, const PIECE: NonKingPieceType>(
        self,
        square: Square,
    ) -> Self {
        let Self {
            mut occupied_by_player,
            mut occupied_by_piece,
            mut piece_by_square,
            king_squares: _,
        } = self;
        occupied_by_player[PLAYER as usize] |= square.to_bit();
        occupied_by_piece[PIECE as usize] |= square.to_bit();
        piece_by_square[square as usize] = Some(PIECE.to_piece_type());
        Self {
            occupied_by_player,
            occupied_by_piece,
            piece_by_square,
            ..self
        }
    }

    pub const fn remove_piece<const PLAYER: Player, const PIECE: NonKingPieceType>(
        self,
        square: Square,
    ) -> Self {
        let Self {
            mut occupied_by_player,
            mut occupied_by_piece,
            mut piece_by_square,
            king_squares: _,
        } = self;
        occupied_by_player[PLAYER as usize] &= !square.to_bit();
        occupied_by_piece[PIECE as usize] &= !square.to_bit();
        piece_by_square[square as usize] = None;

        Self {
            occupied_by_player,
            occupied_by_piece,
            piece_by_square,
            ..self
        }
    }

    pub const fn move_piece<const PLAYER: Player, const PIECE: PieceType>(
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
        let from_to = from.to_bit() | to.to_bit();
        if PIECE as u8 == PieceType::King as u8 {
            king_squares[PLAYER as usize] = to;
        } else {
            occupied_by_piece[PIECE as usize] ^= from_to;
        }
        occupied_by_player[PLAYER as usize] ^= from_to;
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
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO
}

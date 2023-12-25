use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::board_status::BoardStatus;
use crate::castle_direction::CastleDirection;
use crate::piece_arrangement::PieceArrangement;
use crate::piece_type::{NonKingPieceType, PieceType};
use crate::player::Player;
use crate::square::Square::{A1, A8, H1, H8};

pub struct Board<
    const IS_WHITE_TO_MOVE: bool,
    const HAS_EP_PAWN: bool,
    const WHITE_HAS_KING_CASTLE_RIGHTS: bool,
    const WHITE_HAS_QUEEN_CASTLE_RIGHTS: bool,
    const BLACK_HAS_KING_CASTLE_RIGHTS: bool,
    const BLACK_HAS_QUEEN_CASTLE_RIGHTS: bool,
> {
    pieces: PieceArrangement,
}

impl<
        const IS_WHITE_TO_MOVE: bool,
        const HAS_EP_PAWN: bool,
        const WHITE_HAS_KING_CASTLE_RIGHTS: bool,
        const WHITE_HAS_QUEEN_CASTLE_RIGHTS: bool,
        const BLACK_HAS_KING_CASTLE_RIGHTS: bool,
        const BLACK_HAS_QUEEN_CASTLE_RIGHTS: bool,
    >
    Board<
        { IS_WHITE_TO_MOVE },
        { HAS_EP_PAWN },
        WHITE_HAS_KING_CASTLE_RIGHTS,
        WHITE_HAS_QUEEN_CASTLE_RIGHTS,
        BLACK_HAS_KING_CASTLE_RIGHTS,
        BLACK_HAS_QUEEN_CASTLE_RIGHTS,
    >
{
    pub const fn attacked(&self) -> Bitboard {
        // TODO
        EMPTY_BITBOARD
    }

    pub const fn can_castle<const CASTLE_DIRECTION: CastleDirection>(self) -> bool {
        let attacked = self.attacked();
        let occupied = self.pieces.occupied();
        let rooks = self.pieces.mask_for_piece::<{ NonKingPieceType::Rook }>();

        match (IS_WHITE_TO_MOVE, CASTLE_DIRECTION) {
            (true, CastleDirection::KingSide) => {
                WHITE_HAS_KING_CASTLE_RIGHTS
                    && rooks & H1.to_bit() != EMPTY_BITBOARD
                    && occupied & 0x60 == EMPTY_BITBOARD
                    && attacked & 0x60 == EMPTY_BITBOARD
            }
            (true, CastleDirection::QueenSide) => {
                WHITE_HAS_QUEEN_CASTLE_RIGHTS
                    && rooks & H1.to_bit() != EMPTY_BITBOARD
                    && occupied & 0xE == EMPTY_BITBOARD
                    && attacked & 0xC == EMPTY_BITBOARD
            }
            (false, CastleDirection::KingSide) => {
                BLACK_HAS_KING_CASTLE_RIGHTS
                    && rooks & H1.to_bit() != EMPTY_BITBOARD
                    && occupied & 0x6000_0000_0000_0000 == EMPTY_BITBOARD
                    && attacked & 0x6000_0000_0000_0000 == EMPTY_BITBOARD
            }
            (false, CastleDirection::QueenSide) => {
                BLACK_HAS_QUEEN_CASTLE_RIGHTS
                    && rooks & H1.to_bit() != EMPTY_BITBOARD
                    && occupied & 0x0E00_0000_0000_0000 == EMPTY_BITBOARD
                    && attacked & 0x0C00_0000_0000_0000 == EMPTY_BITBOARD
            }
        }
    }

    const fn white_king_castle_after() -> bool {
        !IS_WHITE_TO_MOVE && WHITE_HAS_KING_CASTLE_RIGHTS
    }
    const fn white_queen_castle_after() -> bool {
        !IS_WHITE_TO_MOVE && WHITE_HAS_QUEEN_CASTLE_RIGHTS
    }
    const fn black_king_castle_after() -> bool {
        IS_WHITE_TO_MOVE && BLACK_HAS_KING_CASTLE_RIGHTS
    }
    const fn black_queen_castle_after() -> bool {
        IS_WHITE_TO_MOVE && BLACK_HAS_QUEEN_CASTLE_RIGHTS
    }
    pub const fn castle<const CASTLE_DIRECTION: CastleDirection>(
        self,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        { false },
        { Self::white_king_castle_after() },
        { Self::white_queen_castle_after() },
        { Self::black_king_castle_after() },
        { Self::black_queen_castle_after() },
    > {
        Board {
            pieces: self.pieces,// TODO: Move rook and king pieces
        }
    }
}

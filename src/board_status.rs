use crate::bitboard::{Bit, Bitboard, EMPTY_BITBOARD};
use crate::castle_direction::{CastleDirection, KING_SIDE, QUEEN_SIDE};
use crate::player::{BLACK, Player, WHITE};
use crate::square::Square::{A1, A8, H1, H8};

use core::marker::ConstParamTy;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, ConstParamTy)]
pub struct BoardStatus {
    pub side_to_move: Player,
    pub has_ep_pawn: bool,
    pub white_king_castle_rights: bool,
    pub white_queen_castle_rights: bool,
    pub black_king_castle_rights: bool,
    pub black_queen_castle_rights: bool,
}

impl BoardStatus {
    const WHITE_QUEEN_SIDE_CASTLE_EMPTY: Bitboard = 0xE;
    const WHITE_QUEEN_SIDE_CASTLE_UNATTACKED: Bitboard = 0xC;

    const WHITE_KING_SIDE_CASTLE_EMPTY: Bitboard = 0x60;
    const WHITE_KING_SIDE_CASTLE_UNATTACKED: Bitboard = 0x60;

    const BLACK_KING_SIDE_CASTLE_EMPTY: Bitboard = 0x6000_0000_0000_0000;
    const BLACK_KING_SIDE_CASTLE_UNATTACKED: Bitboard = 0x6000_0000_0000_0000;

    const BLACK_QUEEN_SIDE_CASTLE_EMPTY: Bitboard = 0x0E00_0000_0000_0000;
    const BLACK_QUEEN_SIDE_CASTLE_UNATTACKED: Bitboard = 0x0C00_0000_0000_0000;

    // const WHITE_ROOK_QUEEN_CHANGE: Bitboard = 0b11111000u64;
    // const BLACK_ROOK_QUEEN_CHANGE: Bitboard = 0b11111000u64 << 56u64;
    // const WHITE_ROOK_KING_CHANGE: Bitboard = 0b00001111u64;
    // const BLACK_ROOK_KING_CHANGE: Bitboard = 0b00001111u64 << 56u64;

    const WHITE_QUEEN_SIDE_ROOK: Bit = A1.to_bit();
    const BLACK_KING_SIDE_ROOK: Bit = H8.to_bit();
    const WHITE_KING_SIDE_ROOK: Bit = H1.to_bit();
    const BLACK_QUEEN_SIDE_ROOK: Bit = A8.to_bit();
    const fn can_castle<const CASTLE_DIRECTION: CastleDirection>(
        self,
        attacked: Bitboard,
        occupied: Bitboard,
        rooks: Bitboard,
    ) -> bool {
        match (CASTLE_DIRECTION, self.side_to_move) {
            (KING_SIDE, WHITE) => {
                self.white_king_castle_rights
                    && occupied & Self::WHITE_KING_SIDE_CASTLE_EMPTY == EMPTY_BITBOARD
                    && attacked & Self::WHITE_KING_SIDE_CASTLE_UNATTACKED == EMPTY_BITBOARD
                    && rooks & Self::WHITE_KING_SIDE_ROOK != EMPTY_BITBOARD
            }
            (QUEEN_SIDE, WHITE) => {
                self.white_queen_castle_rights
                    && occupied & Self::WHITE_QUEEN_SIDE_CASTLE_EMPTY == EMPTY_BITBOARD
                    && attacked & Self::WHITE_QUEEN_SIDE_CASTLE_UNATTACKED == EMPTY_BITBOARD
                    && rooks & Self::WHITE_QUEEN_SIDE_ROOK != EMPTY_BITBOARD
            }
            (KING_SIDE, BLACK) => {
                self.black_king_castle_rights
                    && occupied & Self::BLACK_KING_SIDE_CASTLE_EMPTY == EMPTY_BITBOARD
                    && attacked & Self::BLACK_KING_SIDE_CASTLE_UNATTACKED == EMPTY_BITBOARD
                    && rooks & Self::BLACK_KING_SIDE_ROOK != EMPTY_BITBOARD
            }
            (QUEEN_SIDE, BLACK) => {
                self.black_queen_castle_rights
                    && occupied & Self::BLACK_QUEEN_SIDE_CASTLE_EMPTY == EMPTY_BITBOARD
                    && attacked & Self::BLACK_QUEEN_SIDE_CASTLE_UNATTACKED == EMPTY_BITBOARD
                    && rooks & Self::BLACK_QUEEN_SIDE_ROOK != EMPTY_BITBOARD
            }
        }
    }

    const fn switch_sides(self) -> Self {
        Self {
            side_to_move: !self.side_to_move,
            ..self
        }
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
            || CASTLE_DIRECTION != KING_SIDE)
            && self.white_king_castle_rights;
        let white_queen_castle_rights = (self.side_to_move != WHITE
            || CASTLE_DIRECTION != QUEEN_SIDE)
            && self.white_queen_castle_rights;
        let black_king_castle_rights = (self.side_to_move != BLACK
            || CASTLE_DIRECTION != KING_SIDE)
            && self.black_king_castle_rights;
        let black_queen_castle_rights = (self.side_to_move != BLACK
            || CASTLE_DIRECTION != QUEEN_SIDE)
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

#[cfg(test)]
mod test {
    use test_case::test_case;
    use crate::board_status::BoardStatus;

    const ALL_RIGHTS_WHITE_TO_MOVE: BoardStatus = BoardStatus {
        side_to_move: WHITE,
        has_ep_pawn: false,
        white_king_castle_rights: true,
        white_queen_castle_rights: true,
        black_king_castle_rights: true,
        black_queen_castle_rights: true,
    };
    const WHITE_MISSING_KING_WHITE_TO_MOVE: BoardStatus = BoardStatus {
        white_king_castle_rights: false,
        ..ALL_RIGHTS_WHITE_TO_MOVE
    };
    const WHITE_MISSING_QUEEN_WHITE_TO_MOVE: BoardStatus = BoardStatus {
        white_queen_castle_rights: false,
        ..ALL_RIGHTS_WHITE_TO_MOVE
    };
    const WHITE_MISSING_BOTH_WHITE_TO_MOVE: BoardStatus = BoardStatus {
        white_king_castle_rights: false,
        white_queen_castle_rights: false,
        ..ALL_RIGHTS_WHITE_TO_MOVE
    };
    const ALL_RIGHTS_BLACK_TO_MOVE: BoardStatus = BoardStatus {
        side_to_move: BLACK,
        ..ALL_RIGHTS_WHITE_TO_MOVE
    };
    const BLACK_MISSING_KING_BLACK_TO_MOVE: BoardStatus = BoardStatus {
        black_king_castle_rights: false,
        ..ALL_RIGHTS_BLACK_TO_MOVE
    };
    const BLACK_MISSING_QUEEN_BLACK_TO_MOVE: BoardStatus = BoardStatus {
        black_queen_castle_rights: false,
        ..ALL_RIGHTS_BLACK_TO_MOVE
    };
    const BLACK_MISSING_BOTH_BLACK_TO_MOVE: BoardStatus = BoardStatus {
        black_king_castle_rights: false,
        black_queen_castle_rights: false,
        ..ALL_RIGHTS_BLACK_TO_MOVE
    };

    // White to move unobstructed unattacked
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0xFF91, 0x81, true; "white to move with rights and unobstructed unattacked king side")]
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0xFF91, 0x81, true; "white to move with rights and unobstructed unattacked queen side")]
    #[test_case(WHITE_MISSING_KING_WHITE_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0xFF51, 0x41, false; "white to move without king rights and unobstructed unattacked king side")]
    #[test_case(WHITE_MISSING_KING_WHITE_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0xFF51, 0x41, true; "white to move without king rights and unobstructed unattacked queen side")]
    #[test_case(WHITE_MISSING_QUEEN_WHITE_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0xFF92, 0x82, true; "white to move without queen rights and unobstructed unattacked king side")]
    #[test_case(WHITE_MISSING_QUEEN_WHITE_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0xFF92, 0x82, false; "white to move without queen rights and unobstructed unattacked queen side")]
    #[test_case(WHITE_MISSING_BOTH_WHITE_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0xFF52, 0x42, false; "white to move without rights and unobstructed unattacked king side")]
    #[test_case(WHITE_MISSING_BOTH_WHITE_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0xFF52, 0x42, false; "white to move without rights and unobstructed unattacked queen side")]
    // Black to move unobstructed unattacked
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0x91FF000000000000, 0x8100000000000000, true; "black to move with rights and unobstructed unattacked king side")]
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0x91FF000000000000, 0x8100000000000000, true; "black to move with rights and unobstructed unattacked queen side")]
    #[test_case(BLACK_MISSING_KING_BLACK_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0x51ff000000000000, 0x4100000000000000, false; "black to move without king rights and unobstructed unattacked king side")]
    #[test_case(BLACK_MISSING_KING_BLACK_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0x51ff000000000000, 0x4100000000000000, true; "black to move without king rights and unobstructed unattacked queen side")]
    #[test_case(BLACK_MISSING_QUEEN_BLACK_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0x92ff000000000000, 0x8200000000000000, true; "black to move without queen rights and unobstructed unattacked king side")]
    #[test_case(BLACK_MISSING_QUEEN_BLACK_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0x92ff000000000000, 0x8200000000000000, false; "black to move without queen rights and unobstructed unattacked queen side")]
    #[test_case(BLACK_MISSING_BOTH_BLACK_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0x52ff000000000000, 0x4200000000000000, false; "black to move without rights and unobstructed unattacked king side")]
    #[test_case(BLACK_MISSING_BOTH_BLACK_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0x52ff000000000000, 0x4200000000000000, false; "black to move without rights and unobstructed unattacked queen side")]
    // White to move obstructed unattacked
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0xB1, 0x81, false; "white to move with rights and obstructed unattacked king side")]
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0x99, 0x81, false; "white to move with rights and obstructed unattacked queen side")]
    // Black to move obstructed unattacked
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, KING_SIDE, EMPTY_BITBOARD, 0xb100000000000000, 0x8100000000000000, false; "black to move with rights and obstructed unattacked king side")]
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, QUEEN_SIDE, EMPTY_BITBOARD, 0x9900000000000000, 0x8100000000000000, false; "black to move with rights and obstructed unattacked queen side")]
    // White to move unobstructed attacked
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, KING_SIDE, 0x80412214001422, 0x80091, 0x81, false; "white to move with rights and unobstructed attacked king side")]
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, QUEEN_SIDE, 0x404040404bb0404, 0x40091, 0x81, false; "white to move with rights and unobstructed attacked queen side")]
    // Black to move unobstructed attacked
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, KING_SIDE, 0x4428002844820100, 0x9100100000000000, 0x8100000000000000, false; "black to move with rights and unobstructed attacked king side")]
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, QUEEN_SIDE, 0x4428002844820100, 0x9100100000000000, 0x8100000000000000, false; "black to move with rights and unobstructed attacked queen side")]
    // Can castle, but missing rights for opposite player
    #[test_case(BLACK_MISSING_BOTH_BLACK_TO_MOVE.switch_sides(), KING_SIDE, EMPTY_BITBOARD, 0xff91, 0x81, true; "white to move with black missing king rights unobstructed unattacked king side")]
    #[test_case(WHITE_MISSING_BOTH_WHITE_TO_MOVE.switch_sides(), KING_SIDE, EMPTY_BITBOARD, 0x91FF000000000000, 0x8100000000000000, true; "black to move with white missing king rights unobstructed unattacked king side")]
    #[test_case(BLACK_MISSING_BOTH_BLACK_TO_MOVE.switch_sides(), QUEEN_SIDE, EMPTY_BITBOARD, 0xff91, 0x81, true; "white to move with black missing king rights unobstructed unattacked queen side")]
    #[test_case(WHITE_MISSING_BOTH_WHITE_TO_MOVE.switch_sides(), QUEEN_SIDE, EMPTY_BITBOARD, 0x91FF000000000000, 0x8100000000000000, true; "black to move with white missing king rights unobstructed unattacked queen side")]
    fn can_castle_scenarios(
        board_status: BoardStatus,
        castle_direction: CastleDirection,
        attacked: Bitboard,
        occupied: Bitboard,
        rooks: Bitboard,
        expected: bool,
    ) {
        let actual = if castle_direction == KING_SIDE {
            board_status.can_castle::<KING_SIDE>(attacked, occupied, rooks)
        } else {
            board_status.can_castle::<QUEEN_SIDE>(attacked, occupied, rooks)
        };
        assert_eq!(
            actual, expected,
            "attacked = {attacked:X} occupied = {occupied:X} rooks = {rooks:X} {board_status:?}"
        );
    }
}

use crate::bitboard::{Bit, Bitboard, EMPTY_BITBOARD};
use crate::castle_direction::CastleDirection;
use crate::player::Player;
use crate::square::Square::{A1, A8, H1, H8};

use core::marker::ConstParamTy;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd, ConstParamTy)]
pub struct BoardStatus {
    pub side_to_move: Player,
    pub has_ep_pawn: bool,
    pub has_rights: [[bool; CastleDirection::COUNT]; Player::COUNT],
}

impl BoardStatus {
    const CASTLE_EMPTY: [[Bitboard; CastleDirection::COUNT]; Player::COUNT] =
        [[0x60, 0xE], [0x6000_0000_0000_0000, 0x0E00_0000_0000_0000]];
    const CASTLE_UNATTACKED: [[Bitboard; CastleDirection::COUNT]; Player::COUNT] =
        [[0x60, 0xC], [0x6000_0000_0000_0000, 0x0C00_0000_0000_0000]];
    const ROOK_MASKS: [[Bitboard; CastleDirection::COUNT]; Player::COUNT] =
        [[H1.to_bit(), A1.to_bit()], [H8.to_bit(), A8.to_bit()]];
    const fn can_castle<const CASTLE_DIRECTION: CastleDirection>(
        self,
        attacked: Bitboard,
        occupied: Bitboard,
        rooks: Bitboard,
    ) -> bool {
        self.has_rights[self.side_to_move as usize][CASTLE_DIRECTION as usize]
            && occupied & Self::CASTLE_EMPTY[self.side_to_move as usize][CASTLE_DIRECTION as usize]
                == EMPTY_BITBOARD
            && attacked
                & Self::CASTLE_UNATTACKED[self.side_to_move as usize][CASTLE_DIRECTION as usize]
                == EMPTY_BITBOARD
            && rooks & Self::ROOK_MASKS[self.side_to_move as usize][CASTLE_DIRECTION as usize]
                != EMPTY_BITBOARD
    }

    const fn switch_sides(self) -> Self {
        Self {
            side_to_move: self.side_to_move.switch(),
            ..self
        }
    }

    const fn double_pawn_push(self) -> Self {
        Self {
            side_to_move: self.side_to_move.switch(),
            has_ep_pawn: true,
            ..self
        }
    }

    const fn king_move(self) -> Self {
        Self {
            side_to_move: self.side_to_move.switch(),
            has_ep_pawn: false,
            has_rights: [
                [
                    self.side_to_move.is_black()
                        && self.has_rights[Player::White as usize]
                            [CastleDirection::KingSide as usize],
                    self.side_to_move.is_white()
                        && self.has_rights[Player::White as usize]
                            [CastleDirection::QueenSide as usize],
                ],
                [
                    self.side_to_move.is_black()
                        && self.has_rights[Player::Black as usize]
                            [CastleDirection::KingSide as usize],
                    self.side_to_move.is_white()
                        && self.has_rights[Player::Black as usize]
                            [CastleDirection::QueenSide as usize],
                ],
            ],
            ..self
        }
    }

    const fn quiet_move(self) -> Self {
        Self {
            side_to_move: self.side_to_move.switch(),
            has_ep_pawn: false,
            ..self
        }
    }

    const fn rook_move<const CASTLE_DIRECTION: CastleDirection>(self) -> Self {
        Self {
            // Switch sides
            side_to_move: self.side_to_move.switch(),
            has_ep_pawn: false,
            has_rights: [
                [
                    (self.side_to_move.is_black()
                        || CASTLE_DIRECTION as u8 != CastleDirection::KingSide as u8)
                        && self.has_rights[Player::White as usize]
                            [CastleDirection::KingSide as usize],
                    (self.side_to_move.is_black()
                        || CASTLE_DIRECTION as u8 != CastleDirection::QueenSide as u8)
                        && self.has_rights[Player::White as usize]
                            [CastleDirection::QueenSide as usize],
                ],
                [
                    (self.side_to_move.is_white()
                        || CASTLE_DIRECTION as u8 != CastleDirection::KingSide as u8)
                        && self.has_rights[Player::Black as usize]
                            [CastleDirection::KingSide as usize],
                    (self.side_to_move.is_white()
                        || CASTLE_DIRECTION as u8 != CastleDirection::QueenSide as u8)
                        && self.has_rights[Player::Black as usize]
                            [CastleDirection::QueenSide as usize],
                ],
            ],
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
    use crate::board_status::BoardStatus;
    use crate::castle_direction::CastleDirection;
    use crate::player::Player;
    use test_case::test_case;

    const ALL_RIGHTS_WHITE_TO_MOVE: BoardStatus = BoardStatus {
        side_to_move: Player::White,
        has_ep_pawn: false,
        has_rights: [[true; CastleDirection::COUNT]; Player::COUNT],
    };
    const WHITE_MISSING_KING_WHITE_TO_MOVE: BoardStatus = BoardStatus {
        has_rights: [[false, true], [true, true]],
        ..ALL_RIGHTS_WHITE_TO_MOVE
    };
    const WHITE_MISSING_QUEEN_WHITE_TO_MOVE: BoardStatus = BoardStatus {
        has_rights: [[true, false], [true, true]],
        ..ALL_RIGHTS_WHITE_TO_MOVE
    };
    const WHITE_MISSING_BOTH_WHITE_TO_MOVE: BoardStatus = BoardStatus {
        has_rights: [[false, false], [true, true]],
        ..ALL_RIGHTS_WHITE_TO_MOVE
    };
    const ALL_RIGHTS_BLACK_TO_MOVE: BoardStatus = BoardStatus {
        side_to_move: Player::Black,
        ..ALL_RIGHTS_WHITE_TO_MOVE
    };
    const BLACK_MISSING_KING_BLACK_TO_MOVE: BoardStatus = BoardStatus {
        has_rights: [[true, true], [false, true]],
        ..ALL_RIGHTS_BLACK_TO_MOVE
    };
    const BLACK_MISSING_QUEEN_BLACK_TO_MOVE: BoardStatus = BoardStatus {
        has_rights: [[true, true], [true, false]],
        ..ALL_RIGHTS_BLACK_TO_MOVE
    };
    const BLACK_MISSING_BOTH_BLACK_TO_MOVE: BoardStatus = BoardStatus {
        has_rights: [[true, true], [false, false]],
        ..ALL_RIGHTS_BLACK_TO_MOVE
    };

    // White to move unobstructed unattacked
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0xFF91, 0x81, true; "white to move with rights and unobstructed unattacked king side")]
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0xFF91, 0x81, true; "white to move with rights and unobstructed unattacked queen side")]
    #[test_case(WHITE_MISSING_KING_WHITE_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0xFF51, 0x41, false; "white to move without king rights and unobstructed unattacked king side")]
    #[test_case(WHITE_MISSING_KING_WHITE_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0xFF51, 0x41, true; "white to move without king rights and unobstructed unattacked queen side")] // FIXME
    #[test_case(WHITE_MISSING_QUEEN_WHITE_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0xFF92, 0x82, true; "white to move without queen rights and unobstructed unattacked king side")] // FIXME
    #[test_case(WHITE_MISSING_QUEEN_WHITE_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0xFF92, 0x82, false; "white to move without queen rights and unobstructed unattacked queen side")]
    #[test_case(WHITE_MISSING_BOTH_WHITE_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0xFF52, 0x42, false; "white to move without rights and unobstructed unattacked king side")]
    #[test_case(WHITE_MISSING_BOTH_WHITE_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0xFF52, 0x42, false; "white to move without rights and unobstructed unattacked queen side")]
    // Black to move unobstructed unattacked
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0x91FF000000000000, 0x8100000000000000, true; "black to move with rights and unobstructed unattacked king side")]
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0x91FF000000000000, 0x8100000000000000, true; "black to move with rights and unobstructed unattacked queen side")]
    #[test_case(BLACK_MISSING_KING_BLACK_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0x51ff000000000000, 0x4100000000000000, false; "black to move without king rights and unobstructed unattacked king side")]
    #[test_case(BLACK_MISSING_KING_BLACK_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0x51ff000000000000, 0x4100000000000000, true; "black to move without king rights and unobstructed unattacked queen side")] // FIXME
    #[test_case(BLACK_MISSING_QUEEN_BLACK_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0x92ff000000000000, 0x8200000000000000, true; "black to move without queen rights and unobstructed unattacked king side")] // FIXME
    #[test_case(BLACK_MISSING_QUEEN_BLACK_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0x92ff000000000000, 0x8200000000000000, false; "black to move without queen rights and unobstructed unattacked queen side")]
    #[test_case(BLACK_MISSING_BOTH_BLACK_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0x52ff000000000000, 0x4200000000000000, false; "black to move without rights and unobstructed unattacked king side")]
    #[test_case(BLACK_MISSING_BOTH_BLACK_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0x52ff000000000000, 0x4200000000000000, false; "black to move without rights and unobstructed unattacked queen side")]
    // White to move obstructed unattacked
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0xB1, 0x81, false; "white to move with rights and obstructed unattacked king side")]
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0x99, 0x81, false; "white to move with rights and obstructed unattacked queen side")]
    // Black to move obstructed unattacked
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, CastleDirection::KingSide, EMPTY_BITBOARD, 0xb100000000000000, 0x8100000000000000, false; "black to move with rights and obstructed unattacked king side")]
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, CastleDirection::QueenSide, EMPTY_BITBOARD, 0x9900000000000000, 0x8100000000000000, false; "black to move with rights and obstructed unattacked queen side")]
    // White to move unobstructed attacked
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, CastleDirection::KingSide, 0x80412214001422, 0x80091, 0x81, false; "white to move with rights and unobstructed attacked king side")]
    #[test_case(ALL_RIGHTS_WHITE_TO_MOVE, CastleDirection::QueenSide, 0x404040404bb0404, 0x40091, 0x81, false; "white to move with rights and unobstructed attacked queen side")]
    // Black to move unobstructed attacked
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, CastleDirection::KingSide, 0x4428002844820100, 0x9100100000000000, 0x8100000000000000, false; "black to move with rights and unobstructed attacked king side")]
    #[test_case(ALL_RIGHTS_BLACK_TO_MOVE, CastleDirection::QueenSide, 0x4428002844820100, 0x9100100000000000, 0x8100000000000000, false; "black to move with rights and unobstructed attacked queen side")]
    // Can castle, but missing rights for opposite player
    #[test_case(BLACK_MISSING_BOTH_BLACK_TO_MOVE.switch_sides(), CastleDirection::KingSide, EMPTY_BITBOARD, 0xff91, 0x81, true; "white to move with black missing king rights unobstructed unattacked king side")]
    #[test_case(WHITE_MISSING_BOTH_WHITE_TO_MOVE.switch_sides(), CastleDirection::KingSide, EMPTY_BITBOARD, 0x91FF000000000000, 0x8100000000000000, true; "black to move with white missing king rights unobstructed unattacked king side")]
    #[test_case(BLACK_MISSING_BOTH_BLACK_TO_MOVE.switch_sides(), CastleDirection::QueenSide, EMPTY_BITBOARD, 0xff91, 0x81, true; "white to move with black missing king rights unobstructed unattacked queen side")]
    #[test_case(WHITE_MISSING_BOTH_WHITE_TO_MOVE.switch_sides(), CastleDirection::QueenSide, EMPTY_BITBOARD, 0x91FF000000000000, 0x8100000000000000, true; "black to move with white missing king rights unobstructed unattacked queen side")]
    fn can_castle_scenarios(
        board_status: BoardStatus,
        castle_direction: CastleDirection,
        attacked: Bitboard,
        occupied: Bitboard,
        rooks: Bitboard,
        expected: bool,
    ) {
        let actual = if castle_direction == CastleDirection::KingSide {
            board_status.can_castle::<{ CastleDirection::KingSide }>(attacked, occupied, rooks)
        } else {
            board_status.can_castle::<{ CastleDirection::QueenSide }>(attacked, occupied, rooks)
        };
        assert_eq!(
            actual, expected,
            "attacked = {attacked:X} occupied = {occupied:X} rooks = {rooks:X} {board_status:?}"
        );
    }
}

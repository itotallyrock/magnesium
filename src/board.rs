use crate::bitboard::{Bitboard, EMPTY_BITBOARD};
use crate::castle_direction::CastleDirection;
use crate::piece_arrangement::PieceArrangement;
use crate::piece_type::{NonKingPieceType, PieceType};
use crate::player::Player;
use crate::square::{Square, Square::*, SQUARES};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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

pub const DEFAULT_BOARD: Board<true, false, true, true, true, true> =
    Board {
        pieces: PieceArrangement::new(E1, E8)
            // White back rank
            .add_piece::<true, { NonKingPieceType::Rook }>(A1)
            .add_piece::<true, { NonKingPieceType::Knight }>(B1)
            .add_piece::<true, { NonKingPieceType::Bishop }>(C1)
            .add_piece::<true, { NonKingPieceType::Queen }>(D1)
            .add_piece::<true, { NonKingPieceType::Bishop }>(F1)
            .add_piece::<true, { NonKingPieceType::Knight }>(G1)
            .add_piece::<true, { NonKingPieceType::Rook }>(H1)
            // White pawns
            .add_piece::<true, { NonKingPieceType::Pawn }>(A2)
            .add_piece::<true, { NonKingPieceType::Pawn }>(B2)
            .add_piece::<true, { NonKingPieceType::Pawn }>(C2)
            .add_piece::<true, { NonKingPieceType::Pawn }>(D2)
            .add_piece::<true, { NonKingPieceType::Pawn }>(E2)
            .add_piece::<true, { NonKingPieceType::Pawn }>(F2)
            .add_piece::<true, { NonKingPieceType::Pawn }>(G2)
            .add_piece::<true, { NonKingPieceType::Pawn }>(H2)
            // Black back rank
            .add_piece::<false, { NonKingPieceType::Rook }>(A8)
            .add_piece::<false, { NonKingPieceType::Knight }>(B8)
            .add_piece::<false, { NonKingPieceType::Bishop }>(C8)
            .add_piece::<false, { NonKingPieceType::Queen }>(D8)
            .add_piece::<false, { NonKingPieceType::Bishop }>(F8)
            .add_piece::<false, { NonKingPieceType::Knight }>(G8)
            .add_piece::<false, { NonKingPieceType::Rook }>(H8)
            // Black pawns
            .add_piece::<false, { NonKingPieceType::Pawn }>(A7)
            .add_piece::<false, { NonKingPieceType::Pawn }>(B7)
            .add_piece::<false, { NonKingPieceType::Pawn }>(C7)
            .add_piece::<false, { NonKingPieceType::Pawn }>(D7)
            .add_piece::<false, { NonKingPieceType::Pawn }>(E7)
            .add_piece::<false, { NonKingPieceType::Pawn }>(F7)
            .add_piece::<false, { NonKingPieceType::Pawn }>(G7)
            .add_piece::<false, { NonKingPieceType::Pawn }>(H7),
    };

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
    pub fn fen(self) -> String {
        let mut fen = String::with_capacity(128);

        for (row_index, row) in SQUARES.chunks(8).rev().enumerate() {
            let mut offset = 0;
            for &sq in row {
                if let Some(piece) = self.pieces.piece_type_on(sq) {
                    if offset > 0 {
                        fen.push_str(&offset.to_string());
                        offset = 0;
                    }
                    let c = match (piece, self.pieces.player_on(sq).unwrap()) {
                        (PieceType::Pawn, Player::Black) => 'p',
                        (PieceType::Knight, Player::Black) => 'n',
                        (PieceType::Bishop, Player::Black) => 'b',
                        (PieceType::Rook, Player::Black) => 'r',
                        (PieceType::Queen, Player::Black) => 'q',
                        (PieceType::King, Player::Black) => 'k',
                        (PieceType::Pawn, Player::White) => 'P',
                        (PieceType::Knight, Player::White) => 'N',
                        (PieceType::Bishop, Player::White) => 'B',
                        (PieceType::Rook, Player::White) => 'R',
                        (PieceType::Queen, Player::White) => 'Q',
                        (PieceType::King, Player::White) => 'K',
                    };
                    fen.push(c);
                } else {
                    offset += 1;
                }
            }
            if offset > 0 {
                fen.push_str(&offset.to_string());
            }
            if row_index < 7 {
                fen.push('/');
            }
        }

        fen.push(' ');
        if IS_WHITE_TO_MOVE {
            fen.push('w');
        } else {
            fen.push('b');
        }

        fen.push(' ');
        if WHITE_HAS_KING_CASTLE_RIGHTS {
            fen.push('K');
        }
        if WHITE_HAS_QUEEN_CASTLE_RIGHTS {
            fen.push('Q');
        }
        if BLACK_HAS_KING_CASTLE_RIGHTS {
            fen.push('k');
        }
        if BLACK_HAS_KING_CASTLE_RIGHTS {
            fen.push('q');
        }

        if !WHITE_HAS_KING_CASTLE_RIGHTS
            && !WHITE_HAS_QUEEN_CASTLE_RIGHTS
            && !BLACK_HAS_KING_CASTLE_RIGHTS
            && !BLACK_HAS_KING_CASTLE_RIGHTS
        {
            fen.push('-');
        }

        fen.push(' ');
        if HAS_EP_PAWN {
            fen.push_str(todo!());
        } else {
            fen.push('-');
        }

        fen.push(' ');
        fen.push('0');
        fen.push(' ');
        fen.push('1');

        fen
    }

    pub const fn attacked(&self) -> Bitboard {
        // TODO
        EMPTY_BITBOARD
    }

    pub const fn is_white_to_move(&self) -> bool {
        IS_WHITE_TO_MOVE
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

    pub const fn white_king_castle_after() -> bool {
        !IS_WHITE_TO_MOVE && WHITE_HAS_KING_CASTLE_RIGHTS
    }
    pub const fn white_queen_castle_after() -> bool {
        !IS_WHITE_TO_MOVE && WHITE_HAS_QUEEN_CASTLE_RIGHTS
    }
    pub const fn black_king_castle_after() -> bool {
        IS_WHITE_TO_MOVE && BLACK_HAS_KING_CASTLE_RIGHTS
    }
    pub const fn black_queen_castle_after() -> bool {
        IS_WHITE_TO_MOVE && BLACK_HAS_QUEEN_CASTLE_RIGHTS
    }

    pub const fn castle<const CASTLE_DIRECTION: CastleDirection>(
        self,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { Self::white_king_castle_after() },
        { Self::white_queen_castle_after() },
        { Self::black_king_castle_after() },
        { Self::black_queen_castle_after() },
    > {
        const ROOK_FROM_SQUARES: [[Square; 2]; 2] = [[H1, A1], [H8, A8]];
        const ROOK_TO_SQUARES: [[Square; 2]; 2] = [[F1, D1], [F8, D8]];
        const KING_FROM_SQUARES: [Square; 2] = [E1, E8];
        const KING_TO_SQUARE: [[Square; 2]; 2] = [[G1, C1], [G8, C8]];

        Board {
            pieces: self
                .pieces
                .move_piece::<{ IS_WHITE_TO_MOVE }, { PieceType::Rook }>(
                    ROOK_FROM_SQUARES[!IS_WHITE_TO_MOVE as usize][CASTLE_DIRECTION as usize],
                    ROOK_TO_SQUARES[!IS_WHITE_TO_MOVE as usize][CASTLE_DIRECTION as usize],
                )
                .move_piece::<{ IS_WHITE_TO_MOVE }, { PieceType::King }>(
                    KING_FROM_SQUARES[!IS_WHITE_TO_MOVE as usize],
                    KING_TO_SQUARE[!IS_WHITE_TO_MOVE as usize][CASTLE_DIRECTION as usize],
                ),
        }
    }

    pub const fn quiet_move(
        self,
        from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { WHITE_HAS_KING_CASTLE_RIGHTS },
        { WHITE_HAS_QUEEN_CASTLE_RIGHTS },
        { BLACK_HAS_KING_CASTLE_RIGHTS },
        { BLACK_HAS_QUEEN_CASTLE_RIGHTS },
    > {
        Board {
            pieces: self
                .pieces
                .move_by_squares::<{ IS_WHITE_TO_MOVE }>(from, to),
        }
    }

    pub const fn capture(
        self,
        from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { WHITE_HAS_KING_CASTLE_RIGHTS },
        { WHITE_HAS_QUEEN_CASTLE_RIGHTS },
        { BLACK_HAS_KING_CASTLE_RIGHTS },
        { BLACK_HAS_QUEEN_CASTLE_RIGHTS },
    > {
        Board {
            pieces: self
                .pieces
                .remove_by_square::<{ !IS_WHITE_TO_MOVE }>(to)
                .move_by_squares::<{ IS_WHITE_TO_MOVE }>(from, to),
        }
    }

    pub const fn white_capture_king_rook_after<const CASTLE_DIRECTION: CastleDirection>() -> bool {
        (IS_WHITE_TO_MOVE || !matches!(CASTLE_DIRECTION, CastleDirection::QueenSide))
            && WHITE_HAS_KING_CASTLE_RIGHTS
    }
    pub const fn white_capture_queen_rook_after<const CASTLE_DIRECTION: CastleDirection>() -> bool {
        (IS_WHITE_TO_MOVE || !matches!(CASTLE_DIRECTION, CastleDirection::KingSide))
            && WHITE_HAS_QUEEN_CASTLE_RIGHTS
    }
    pub const fn black_capture_king_rook_after<const CASTLE_DIRECTION: CastleDirection>() -> bool {
        (!IS_WHITE_TO_MOVE || !matches!(CASTLE_DIRECTION, CastleDirection::QueenSide))
            && BLACK_HAS_KING_CASTLE_RIGHTS
    }
    pub const fn black_capture_queen_rook_after<const CASTLE_DIRECTION: CastleDirection>() -> bool {
        (!IS_WHITE_TO_MOVE || !matches!(CASTLE_DIRECTION, CastleDirection::KingSide))
            && BLACK_HAS_QUEEN_CASTLE_RIGHTS
    }

    pub const fn capture_rook<const CASTLE_DIRECTION: CastleDirection>(
        self,
        from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { Self::white_king_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::white_queen_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::black_king_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::black_queen_rook_move_after::<CASTLE_DIRECTION>() },
    > {
        Board {
            pieces: self
                .pieces
                .remove_piece::<{ !IS_WHITE_TO_MOVE }, { NonKingPieceType::Rook }>(to)
                .move_by_squares::<{ IS_WHITE_TO_MOVE }>(from, to),
        }
    }

    pub const fn promote<const PROMOTION: NonKingPieceType>(
        self,
        from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { WHITE_HAS_KING_CASTLE_RIGHTS },
        { WHITE_HAS_QUEEN_CASTLE_RIGHTS },
        { BLACK_HAS_KING_CASTLE_RIGHTS },
        { BLACK_HAS_QUEEN_CASTLE_RIGHTS },
    > {
        Board {
            pieces: self
                .pieces
                .remove_piece::<{ IS_WHITE_TO_MOVE }, { NonKingPieceType::Pawn }>(from)
                .add_piece::<{ IS_WHITE_TO_MOVE }, { PROMOTION }>(to),
        }
    }

    pub const fn promote_capture<const PROMOTION: NonKingPieceType>(
        self,
        _from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { WHITE_HAS_KING_CASTLE_RIGHTS },
        { WHITE_HAS_QUEEN_CASTLE_RIGHTS },
        { BLACK_HAS_KING_CASTLE_RIGHTS },
        { BLACK_HAS_QUEEN_CASTLE_RIGHTS },
    > {
        Board {
            pieces: self
                .pieces
                .remove_by_square::<{ !IS_WHITE_TO_MOVE }>(to)
                .add_piece::<{ IS_WHITE_TO_MOVE }, { PROMOTION }>(to),
        }
    }

    pub const fn promote_capture_rook<
        const PROMOTION: NonKingPieceType,
        const CASTLE_DIRECTION: CastleDirection,
    >(
        self,
        _from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { Self::white_king_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::white_queen_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::black_king_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::black_queen_rook_move_after::<CASTLE_DIRECTION>() },
    > {
        Board {
            pieces: self
                .pieces
                .remove_piece::<{ !IS_WHITE_TO_MOVE }, { NonKingPieceType::Rook }>(to)
                .add_piece::<{ IS_WHITE_TO_MOVE }, { PROMOTION }>(to),
        }
    }

    pub const fn king_move(
        self,
        from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { Self::white_king_castle_after() },
        { Self::white_queen_castle_after() },
        { Self::black_king_castle_after() },
        { Self::black_queen_castle_after() },
    > {
        Board {
            pieces: self
                .pieces
                .move_piece::<{ IS_WHITE_TO_MOVE }, { PieceType::King }>(from, to),
        }
    }

    pub const fn white_king_rook_move_after<const CASTLE_DIRECTION: CastleDirection>() -> bool {
        !IS_WHITE_TO_MOVE
            && !matches!(CASTLE_DIRECTION, CastleDirection::KingSide)
            && WHITE_HAS_KING_CASTLE_RIGHTS
    }
    pub const fn white_queen_rook_move_after<const CASTLE_DIRECTION: CastleDirection>() -> bool {
        !IS_WHITE_TO_MOVE
            && !matches!(CASTLE_DIRECTION, CastleDirection::QueenSide)
            && WHITE_HAS_QUEEN_CASTLE_RIGHTS
    }
    pub const fn black_king_rook_move_after<const CASTLE_DIRECTION: CastleDirection>() -> bool {
        IS_WHITE_TO_MOVE
            && !matches!(CASTLE_DIRECTION, CastleDirection::KingSide)
            && BLACK_HAS_KING_CASTLE_RIGHTS
    }
    pub const fn black_queen_rook_move_after<const CASTLE_DIRECTION: CastleDirection>() -> bool {
        IS_WHITE_TO_MOVE
            && !matches!(CASTLE_DIRECTION, CastleDirection::QueenSide)
            && BLACK_HAS_QUEEN_CASTLE_RIGHTS
    }
    pub const fn rook_move<const CASTLE_DIRECTION: CastleDirection>(
        self,
        from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        false,
        { Self::white_king_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::white_queen_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::black_king_rook_move_after::<CASTLE_DIRECTION>() },
        { Self::black_queen_rook_move_after::<CASTLE_DIRECTION>() },
    > {
        Board {
            pieces: self
                .pieces
                .move_piece::<{ IS_WHITE_TO_MOVE }, { PieceType::Rook }>(from, to),
        }
    }

    pub const fn double_pawn_push(
        self,
        from: Square,
        to: Square,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        true,
        { WHITE_HAS_KING_CASTLE_RIGHTS },
        { WHITE_HAS_QUEEN_CASTLE_RIGHTS },
        { BLACK_HAS_KING_CASTLE_RIGHTS },
        { BLACK_HAS_QUEEN_CASTLE_RIGHTS },
    > {
        // TODO: track en passant square somehow
        Board {
            pieces: self
                .pieces
                .move_piece::<{ IS_WHITE_TO_MOVE }, { PieceType::Pawn }>(from, to),
        }
    }

    pub const fn switch_sides(
        self,
    ) -> Board<
        { !IS_WHITE_TO_MOVE },
        { HAS_EP_PAWN },
        { WHITE_HAS_KING_CASTLE_RIGHTS },
        { WHITE_HAS_QUEEN_CASTLE_RIGHTS },
        { BLACK_HAS_KING_CASTLE_RIGHTS },
        { BLACK_HAS_QUEEN_CASTLE_RIGHTS },
    > {
        Board {
            pieces: self.pieces,
        }
    }
}

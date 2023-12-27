#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use crate::board::{Board, DEFAULT_BOARD};

use crate::castle_direction::CastleDirection;
use crate::square::Square::*;

mod bitboard;
mod board;
mod board_status;
mod castle_direction;
mod piece_arrangement;
mod piece_type;
mod player;
mod square;

pub fn main() {
    println!("TODO: Implement main");
    const BOARD: Board<false, false, false, false, true, true> = DEFAULT_BOARD
        .quiet_move(G1, F3)
        .quiet_move(G8, F6)
        .double_pawn_push(E2, E4)
        .double_pawn_push(E7, E5)
        .quiet_move(F1, C4)
        .quiet_move(F8, C5)
        .castle::<{ CastleDirection::KingSide }>();
    println!("fen after castle: {}", BOARD.fen());
}

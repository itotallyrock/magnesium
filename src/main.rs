#![feature(
    const_trait_impl,
    const_for,
    const_option,
    const_mut_refs,
    generic_const_exprs,
    structural_match,
    adt_const_params
)]

use crate::board::{Board, DEFAULT_BOARD};
use crate::board_status::DEFAULT_BOARD_STATUS;
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
    let board = DEFAULT_BOARD
        .quiet_move(G1, F3)
        .quiet_move(G8, F6)
        .double_pawn_push(E2, E4)
        .double_pawn_push(E7, E5)
        .quiet_move(F1, C4)
        .quiet_move(F8, C5)
        .castle::<{ CastleDirection::KingSide }>();
    println!("fen after castle: {}", board.fen());
}

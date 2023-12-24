use crate::bitboard::Bitboard;
use crate::square::Square;

pub struct PieceArrangement {
    white_king: Square,
    black_king: Square,
    queens: Bitboard,
    rooks: Bitboard,
    bishops: Bitboard,
    knights: Bitboard,
    pawns: Bitboard,
    white: Bitboard,
    black: Bitboard,
    // TODO: Consider a square -> piece map
}

use crate::board_status::BoardStatus;
use crate::piece_arrangement::PieceArrangement;

pub struct Board<const STATUS: BoardStatus> {
    pieces: PieceArrangement,
}

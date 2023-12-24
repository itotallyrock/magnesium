/// The direction to castle in for either side
pub type CastleDirection = bool;

/// Castle towards the rook on the same side as the king
pub const KING_SIDE: CastleDirection = false;
/// Castle towards the rook on the same side as the queen
pub const QUEEN_SIDE: CastleDirection = true;

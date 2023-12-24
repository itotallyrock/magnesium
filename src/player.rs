use core::marker::ConstParamTy;

#[repr(u8)]
#[derive(ConstParamTy, Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub const COUNT: usize = 2;
    pub const fn switch(self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
    pub const fn is_white(self) -> bool {
        match self {
            Player::White => true,
            Player::Black => false,
        }
    }
    pub const fn is_black(self) -> bool {
        !self.is_white()
    }
}

pub const WHITE: Player = Player::White;
pub const BLACK: Player = Player::Black;

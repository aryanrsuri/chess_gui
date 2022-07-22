use crate::piece::{Piece, Piece::*, Color::*};
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Square { pub piece: Option<Piece> }

impl From<Piece> for Square {
    fn from(piece: Piece) -> Self {
        Self { piece: Some(piece) }
    }
}

impl Square {
    pub fn new() -> Square { Square { piece: None} }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.piece == None
    }
    #[inline]
    pub fn get_piece(&self) -> Option<Piece> { self.piece }
    // todo!(instead of hard coding the color, add the get_color() method somehow);
    pub(crate) fn get_sym(&self) -> &str {
        match self.piece {
            Some(King(White)) => " ♔ ",
            Some(Queen(White)) => " ♕ ",
            Some(Bishop(White)) => " ♗ ",
            Some(Knight(White)) => " ♘ ",
            Some(Rook(White)) => " ♔ ",
            Some(Pawn(White)) => " ♙ ",

            Some(King(Black)) => " ♚ ",
            Some(Queen(Black)) => " ♛ ",
            Some(Rook(Black)) => " ♜ ",
            Some(Knight(Black)) => " ♞ ",
            Some(Bishop(Black)) => " ♝ ",
            Some(Pawn(Black)) => " ♟ ",

            None => "___",
            _ => unreachable!(),
        }
    }
}


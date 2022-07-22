use std::fmt;
use std::fmt::Formatter;
use crate::piece::*;
use crate::piece::{Position, Piece, Piece::*, Color::*};
use crate::square::Square;
pub const WHITE: Color = White;
pub const BLACK: Color = Black;

#[derive(Debug)]
pub struct Board {
  squares: [[Square; 8]; 8],
  turn: Color,
}

impl Board {
  pub fn new() -> Board {
    let mut squares = [[Square {piece: None };8]; 8];
    for x in 0..8 {
      squares[1][x]
          .piece = Some(Pawn(WHITE));
      squares[6][x]
          .piece = Some(Pawn(BLACK));
      squares[0][x].piece = match x {
        0 => Some(Rook(WHITE)),
        1 => Some(Knight(WHITE)),
        2 => Some(Bishop(WHITE)),
        3 => Some(King(WHITE)),
        4 => Some(Queen(WHITE)),
        5 => Some(Bishop(WHITE)),
        6 => Some(Knight(WHITE)),
        7 => Some(Rook(WHITE)),
        _ => unreachable!(),
      };
      squares[7][x].piece = match x {
        0 => Some(Rook(BLACK)),
        1 => Some(Knight(BLACK)),
        2 => Some(Bishop(BLACK)),
        3 => Some(King(BLACK)),
        4 => Some(Queen(BLACK)),
        5 => Some(Bishop(BLACK)),
        6 => Some(Knight(BLACK)),
        7 => Some(Rook(BLACK)),
        _ => unreachable!(),
      };

    }
    Board {
      squares,
      turn: White
    }
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let mut res = String::new();
    for row in 0..8 {
      for col in 0..8 {
        res.push_str("|");
        res.push_str(self.squares[row][col].get_sym());
        res.push_str("|")
      }
      res.push_str("\n");
    }
    write!(f,"{}", res)
  }
}
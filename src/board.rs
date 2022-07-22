use std::fmt;
use crate::board::Piece::*;

#[derive(Copy, Clone)]
pub enum Piece {
  King,
  Queen,
  Rook,
  Knight,
  Bishop,
  Pawn,
}

#[derive(Copy, Clone)]
struct Square { piece: Option<Piece> }

impl Square {
  pub fn new() -> Square {
    Square { piece: None}
  }

  fn symbol(&self) -> &str {
    match self.piece {
      Some(King) => " K ",
      Some(Queen) => " Q ",
      Some(Rook) => " R ",
      Some(Bishop) => " B ",
      Some(Knight) => " N ",
      Some(Pawn) => " P ",
      None => "___"
    }
  }
}

impl fmt::Display for Square {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.symbol())
  }
}
pub struct Board {
  squares: [[Square; 8]; 8]
}

impl Board {
  pub fn new() -> Board {
    let mut squares = [[Square { piece: None }; 8]; 8];
    for x in 0..8 {
      squares[1][x].piece = Some(Pawn);
      squares[6][x].piece = Some(Pawn);
      let piece = match x {
        0 | 7 => Some(Rook),
        1 | 6 => Some(Knight),
        2 | 5 => Some(Bishop),
        3 => Some(King),
        4 => Some(Queen),
        _ => unreachable!(),
      };
      squares[0][x].piece = piece;
      squares[7][x].piece = piece;
    }
    Board {
      squares
    }
  }

  pub fn valid() {

  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut res = String::new();
    for row in 0..8 {
      for col in 0..8 {
        res.push_str("|");
        res.push_str(self.squares[row][col].symbol());
        res.push_str("|");
      }
      res.push_str("\n");
    }
    write!(f, "{}", res)
  }
}

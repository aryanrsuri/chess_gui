#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub const fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    pub fn get_row(&self) -> i32 {
        self.row
    }
    pub fn get_col(&self) -> i32 {
        self.col
    }
    pub fn add_row(&self, rowd: i32) -> Self {
        let mut res = *self;
        res.row += rowd;
        res
    }
    pub fn add_col(&self, cold: i32) -> Self {
        let mut res = *self;
        res.col += cold;
        res
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Knight(Color),
    Bishop(Color),
    Pawn(Color),
}

impl Piece {
    #[inline]
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::King(_) => "K",
            Self::Queen(_) => "Q",
            Self::Rook(_) => "R",
            Self::Bishop(_) => "B",
            Self::Knight(_) => "N",
            Self::Pawn(_) => "P",
            _ => unreachable!(),
        }
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        match self {
            Self::King(c)
            | Self::Queen(c)
            | Self::Rook(c)
            | Self::Bishop(c)
            | Self::Knight(c)
            | Self::Pawn(c) => *c,
            _ => unreachable!(),
        }
    }
    
}



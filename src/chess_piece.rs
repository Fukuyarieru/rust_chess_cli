#[derive(Clone, Copy)]
pub enum ChessPiece {
    None,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

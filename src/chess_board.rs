use core::num;

use crate::chess_piece::ChessPiece;
use crate::chess_piece::ChessPiece::*;
use crate::chess_piece::Color::*;

pub struct ChessBoard {
    board: [[ChessPiece; 8]; 8],
}
impl ChessBoard {
    pub fn create() -> ChessBoard {
        ChessBoard {
            board: [
                [
                    Rook(Black),
                    Knight(Black),
                    Bishop(Black),
                    Queen(Black),
                    King(Black),
                    Bishop(Black),
                    Knight(Black),
                    Rook(Black),
                ],
                [Pawn(Black); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Pawn(White); 8],
                [
                    Rook(White),
                    Knight(White),
                    Bishop(White),
                    Queen(White),
                    King(White),
                    Bishop(White),
                    Knight(White),
                    Rook(White),
                ],
            ],
        }
    }
    fn index(&self, ch: char, num: usize) -> Result<&ChessPiece, (char, usize)> {
        if num == 0 || num > 8 {
            return Err((ch, num));
        }
        let board = &self.board;
        match ch.to_ascii_uppercase() {
            'A' => Ok(&board[0][num]),
            'B' => Ok(&board[1][num]),
            'C' => Ok(&board[2][num]),
            'D' => Ok(&board[3][num]),
            'E' => Ok(&board[4][num]),
            'F' => Ok(&board[5][num]),
            'G' => Ok(&board[6][num]),
            'H' => Ok(&board[7][num]),
            _ => Err((ch, num)),
        }
    }
    pub fn r#move(&mut self, from: (char, usize), to: (char, usize)) -> Result<(), ()> {
        // function does not care whose turn it is, it just make sure that the logic given between two square is correct
        // steps
        // - check if there is a piece at "from"
        if let Ok(ChessPiece::None) = self.index(from.0, from.1) {
            return Err(());
        }
        // - check if it can move to "to" (PIECE TYPE)
        // - check if it has a piece that isnt of the same color
        todo!()
    }
}
impl std::fmt::Display for ChessBoard {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

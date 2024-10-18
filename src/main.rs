mod chess_board;
use chess_board::ChessBoard;
mod chess_piece;
fn main() {
    println!("Hello, world!");

    let mut chess_board = ChessBoard::create();
    chess_board.r#move(('a', 1), ('b', 2));
}

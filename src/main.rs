mod pieces;
mod board;

use board::ChessBoard;

fn main() {
    let playing_board=ChessBoard::new();
    let board=&playing_board.board;
    // playing_board.find_piece_on(("C",1));
    let pc=&playing_board.board[0][0].piece.as_ref().unwrap();
    // println!("{:?}",&playing_board);
    pc.legal_moves(&board);
}

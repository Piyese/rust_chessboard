
use crate::pieces::{Square, generate_board};

#[derive(Debug)]
pub struct ChessBoard{
    pub board: Vec<Vec<Square>>
}
impl ChessBoard{
    pub fn new()->Self{
        ChessBoard{
            board: generate_board(),
        }
    }
    pub fn find_piece_on(&self, sq: (&'static str, usize)){
        if sq.1>8{
            eprintln!("invalid square");
            return;
        }
        let sq_string=format!("{}{}",sq.0,sq.1);
        for square in &self.board[8-sq.1]{
            if sq_string==square.name{
                if square.piece.is_none(){
                    eprintln!("Empty Square");
                    return;
                }
                let piece=square.piece.as_ref().unwrap();
                println!("A {:?} {:?} on {:?}",piece.color, piece.name,square.name);
            }
        }
    }
}

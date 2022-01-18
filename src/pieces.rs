
#[derive(Debug)]
pub enum Color{
    Black,
    White
}

#[derive(Debug)]
pub enum PieceType{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug)]
pub struct Piece{
    pub name:PieceType,
    pub color:Color,
    pub position:(String,String)
}
impl Piece{
    pub fn legal_moves(&self, cb: &Vec<Vec<Square>>)->&str{
        // step 1: get the piece' actual board position. ie (3,5)=>col,row.. starting from 1...
        let col_index :[&str;8] = [ "A", "B", "C", "D", "E", "F", "G", "H" ];
        let mut pos_cordinates = (0,0);
        
        for ( index, col_letter ) in col_index.iter().enumerate() {
            if col_letter.to_string() == self.position.0 {
                pos_cordinates = ( index + 1, self.position.1.clone().parse().unwrap() );
                break;
            }
        }

        let r_index = 8-pos_cordinates.1; //ie 5
        let c_index = pos_cordinates.0-1; //ie 3
        println!("{:?}",cb[r_index][c_index]); //mic testing.. one two.. one two..
        // step 2: go to individual piece types

        match &self.name{
            PieceType::Bishop=>{
                "sasasas"
            },
            PieceType::Pawn=>{
                match &self.color{
                    Color::White=>println!("{:?}",cb[r_index-1][c_index]),
                    Color::Black=>println!("{:?}",cb[r_index+1][c_index]),
                }
                "chsk chsk"
            },
            PieceType::Rook=>{
                "ajajaja"
            },
            PieceType::Knight=>{
                "knight"
            },
            PieceType::Queen=>{
                "hghgh"
            },
            PieceType::King=>{
                "hghgh"
            },
        }
    }
}

#[derive(Debug)]
pub struct Square{
    pub name:String,
    pub piece:Option<Piece>
}

pub fn generate_board()->Vec<Vec<Square>>{
    let mut board:Vec<Vec<Square>> = Vec::new();
    for row in 1..9{
        let mut single_row: Vec<Square> = Vec::new();
        for col in "ABCDEFGH".chars(){
            let sq = (col,row);
            let pcs = match sq {
                ('A',2)|('B',2)|('C',2)|('D',2)|('E',2)|('F',2)|('G',2)|('H',2) => Some(Piece{name: PieceType::Pawn, color:Color::White, position:(col.to_string(),row.to_string())}),
                ('A',7)|('B',7)|('C',7)|('D',7)|('E',7)|('F',7)|('G',7)|('H',7) => Some(Piece{name: PieceType::Pawn, color:Color::Black, position:(col.to_string(),row.to_string())}),
                ('A',1)|('H',1) => Some( Piece{name: PieceType::Rook, color:Color::White, position:(col.to_string(),row.to_string())} ),
                ('A',8)|('H',8) => Some( Piece{name: PieceType::Rook, color:Color::Black, position:(col.to_string(),row.to_string())} ),
                ('B',1)|('G',1) => Some( Piece{name: PieceType::Knight, color:Color::White, position:(col.to_string(),row.to_string())} ),
                ('B',8)|('G',8) => Some( Piece{name: PieceType::Knight, color:Color::Black, position:(col.to_string(),row.to_string())} ),
                ('C',1)|('F',1) => Some( Piece{name: PieceType::Bishop, color:Color::White, position:(col.to_string(),row.to_string())} ),
                ('C',8)|('F',8) => Some( Piece{name: PieceType::Bishop, color:Color::Black, position:(col.to_string(),row.to_string())} ),
                ('D',1) => Some(Piece{name: PieceType::Queen, color:Color::White, position:(col.to_string(),row.to_string())}),
                ('D',8) => Some(Piece{name: PieceType::Queen, color:Color::Black, position:(col.to_string(),row.to_string())}),
                ('E',1) => Some(Piece{name: PieceType::King, color:Color::White, position:(col.to_string(),row.to_string())}),
                ('E',8) => Some(Piece{name: PieceType::King, color:Color::Black, position:(col.to_string(),row.to_string())}),
                _ => None
            };

            let temp=Square{
                name:format!("{}{}",col,row),
                piece:pcs
            };
            single_row.push(temp);
        }
        board.push(single_row);
    }
    board.reverse();
    board
}

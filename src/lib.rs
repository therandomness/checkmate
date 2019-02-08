///Defines the possible Chess Pieces.
#[derive(PartialEq, Debug, Copy, Clone)]
enum ChessPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl ChessPiece {
    ///Assign values to the chess pieces.
    fn value(&self) -> u8 {
        match *self {
            ChessPiece::King => std::u8::MAX,
            ChessPiece::Queen => 9,
            ChessPiece::Rook => 5,
            ChessPiece::Bishop => 3,
            ChessPiece::Knight => 3,
            ChessPiece::Pawn => 1,
        }
    }
}

enum ChessBoardSetupType {
    Empty,
    Standard,
}

struct ChessBoard {
   board: [[Option<ChessPiece>; 8]; 8],
}

impl ChessBoard {
    ///Create a ChessBoard
    fn create(setup_type: ChessBoardSetupType) -> ChessBoard {
        match setup_type {
            ChessBoardSetupType::Empty => ChessBoard{ board: [[None; 8]; 8] },
            ChessBoardSetupType::Standard => ChessBoard{ board: [
                [Some(ChessPiece::Rook), Some(ChessPiece::Knight), Some(ChessPiece::Bishop), Some(ChessPiece::King), Some(ChessPiece::Queen), Some(ChessPiece::Bishop), Some(ChessPiece::Knight), Some(ChessPiece::Rook)],
                [Some(ChessPiece::Pawn); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(ChessPiece::Pawn); 8],
                [Some(ChessPiece::Rook), Some(ChessPiece::Knight), Some(ChessPiece::Bishop), Some(ChessPiece::Queen), Some(ChessPiece::King), Some(ChessPiece::Bishop), Some(ChessPiece::Knight), Some(ChessPiece::Rook)],
                ] 
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_value_check() {
        assert_eq!(ChessPiece::Bishop.value(), 3);
    }

    #[test]
    fn create_board() {
        let mut board = ChessBoard::create(ChessBoardSetupType::Empty);
        assert_eq!(board.board[0][0], None);
        
        board = ChessBoard::create(ChessBoardSetupType::Standard);
        assert_eq!(board.board[0][0], Some(ChessPiece::Rook));
    }
}

use crate::{
    chess::Chess,
    chessboard::{
        get_black_king, get_squares_with_white_pieces, get_white_king, rank::Rank, square::Square,
    },
    piece::{Piece, PieceColor},
};

const WHITE: PieceColor = PieceColor::White;
const BLACK: PieceColor = PieceColor::Black;

type SquareCoordinates = (usize, usize);
pub type MoveFromCoordinates = (SquareCoordinates, SquareCoordinates);

pub fn position_is_checkmate(chess: &Chess, king_color: PieceColor) -> bool {
    let king_sq = if king_color == PieceColor::White {
        get_white_king(&chess.board)
    } else {
        get_black_king(&chess.board)
    };

    false
}

pub fn _white_legal_moves(chess: &Chess) -> Vec<MoveFromCoordinates> {
    let chessboard = chess.board;
    let mut legal_moves: Vec<MoveFromCoordinates> = Vec::new();
    let white_pieces = get_squares_with_white_pieces(&chessboard);

    for i in white_pieces.iter() {
        match i.piece {
            Piece::None => {}
            Piece::Pawn(_) => todo!(),
            Piece::Knight(_) => todo!(),
            Piece::Bishop(_) => todo!(),
            Piece::Rook(_) => todo!(),
            Piece::Queen(_) => todo!(),
            Piece::King(_) => todo!(),
        }
    }
    legal_moves
}

fn pawn_legal_moves(sq: &Square) -> Vec<MoveFromCoordinates> {
    let mut legal_moves: Vec<MoveFromCoordinates> = Vec::new();
    if sq.piece.color() == &PieceColor::White {
        if sq.rank == Rank::Second {
            legal_moves.push((
                (sq.file as usize, Rank::Second as usize),
                (sq.file as usize, Rank::Fourth as usize),
            ));
            legal_moves.push((
                (sq.file as usize, Rank::Second as usize),
                (sq.file as usize, Rank::Third as usize),
            ));
        } else {
            legal_moves.push((
                (sq.file as usize, sq.rank as usize),
                (sq.file as usize, sq.rank as usize + 1),
            ));
        }
    }
    legal_moves
}

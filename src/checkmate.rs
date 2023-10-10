use crate::{
    chess::{Chess, Move},
    chessboard::{get_black_king, get_squares_with_white_pieces, get_white_king},
    piece::PieceColor,
};

pub fn position_is_checkmate(chess: &Chess, king_color: PieceColor) -> bool {
    let king_sq = if king_color == PieceColor::White {
        get_white_king(&chess.board)
    } else {
        get_black_king(&chess.board)
    };

    false
}

pub fn white_legal_moves(chess: &Chess) -> Vec<Move> {
    let chessboard = chess.board;
    let mut legal_moves: Vec<Move> = Vec::new();
    let white_pieces = get_squares_with_white_pieces(&chessboard);

    for i in white_pieces.iter() {}
    legal_moves
}

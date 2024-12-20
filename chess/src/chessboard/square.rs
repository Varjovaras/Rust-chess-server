use serde::{Deserialize, Serialize};

use crate::{
    check::is_king_in_check_state,
    checkmate::{
        bishop_possible_moves, king_possible_moves, knight_possible_moves, pawn_possible_moves,
        rook_possible_moves, MoveFromCoordinates,
    },
    chess::Chess,
    moves::move_helpers::helpers::{move_is_black_en_passant, move_is_white_en_passant},
    piece::{Piece, PieceColor},
};

use super::{file::File, rank::Rank};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[allow(clippy::module_name_repetitions)]
pub enum SquareColor {
    Black,
    #[default]
    White,
}
impl SquareColor {
    const fn _as_str(&self) -> &str {
        match self {
            Self::Black => "black",
            Self::White => "white",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
    pub color: SquareColor,
    pub piece: Piece,
    pub possible_moves: Vec<MoveFromCoordinates>,
}

impl Square {
    #[must_use]
    pub const fn new(file: File, rank: Rank, color: SquareColor, piece: Piece) -> Self {
        Self {
            file,
            rank,
            color,
            piece,
            possible_moves: vec![],
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new_from_u8(file: u8, rank: u8, color: SquareColor, piece: Piece) -> Self {
        let file = File::try_from(file).expect("Invalid file");
        let rank = Rank::try_from(rank).expect("Invalid rank");
        Self {
            file,
            rank,
            color,
            piece,
            possible_moves: vec![],
        }
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new_without_piece(file: u8, rank: u8) -> Self {
        let file = File::try_from(file).expect("Invalid file");
        let rank = Rank::try_from(rank).expect("Invalid rank");
        Self {
            file,
            rank,
            color: SquareColor::default(),
            piece: Piece::default(),
            possible_moves: vec![],
        }
    }

    #[must_use]
    pub fn square_name(&self) -> String {
        self.file.as_str().to_owned() + self.rank.as_str()
    }

    #[must_use]
    pub const fn square_color(&self) -> SquareColor {
        self.color
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.piece == Piece::default()
    }

    #[must_use]
    pub fn has_piece(&self) -> bool {
        self.piece != Piece::default()
    }

    #[must_use]
    pub const fn piece_name(&self) -> &'static str {
        match self.piece {
            Piece::None => " ",
            Piece::Pawn(_) => match self.piece.color() {
                PieceColor::White => "p",
                PieceColor::Black => "P",
                PieceColor::None => " ",
            },
            Piece::Knight(_) => match self.piece.color() {
                PieceColor::White => "n",
                PieceColor::Black => "N",
                PieceColor::None => " ",
            },
            Piece::Bishop(_) => match self.piece.color() {
                PieceColor::White => "b",
                PieceColor::Black => "B",
                PieceColor::None => " ",
            },
            Piece::Rook(_) => match self.piece.color() {
                PieceColor::White => "r",
                PieceColor::Black => "R",
                PieceColor::None => " ",
            },
            Piece::Queen(_) => match self.piece.color() {
                PieceColor::White => "q",
                PieceColor::Black => "Q",
                PieceColor::None => " ",
            },
            Piece::King(_) => match self.piece.color() {
                PieceColor::White => "k",
                PieceColor::Black => "K",
                PieceColor::None => " ",
            },
        }
    }

    #[must_use]
    pub fn possible_legal_moves(&self, chess: &Chess) -> Vec<MoveFromCoordinates> {
        let mut moves = vec![];
        match self.piece {
            Piece::None => return moves,
            Piece::Pawn(_) => moves = pawn_possible_moves(self),
            Piece::Knight(_) => moves = knight_possible_moves(self),
            Piece::Bishop(_) => moves = bishop_possible_moves(self),
            Piece::Rook(_) => moves = rook_possible_moves(self),
            Piece::Queen(_) => {
                moves = {
                    moves = bishop_possible_moves(self);
                    moves.append(&mut rook_possible_moves(self));
                    moves
                }
            }
            Piece::King(_) => moves = king_possible_moves(self),
        }
        // chess.print_white_board_to_terminal();
        moves
            .iter()
            .filter(|possible_move| {
                let start_sq = chess.board[possible_move.0 .0][possible_move.0 .1].clone();
                let end_sq = chess.board[possible_move.1 .0][possible_move.1 .1].clone();
                check_if_move_is_legal(chess, &start_sq, &end_sq)
            })
            .copied()
            .collect::<Vec<MoveFromCoordinates>>()
    }
}

#[must_use]
pub fn check_if_move_is_legal(chess: &Chess, start_sq: &Square, end_sq: &Square) -> bool {
    let mut temp_board = chess.board.clone();
    if end_sq.has_piece() && end_sq.piece.color() == start_sq.piece.color() {
        return false;
    }

    if !start_sq.piece.piece_move(start_sq, end_sq, chess) {
        return false;
    };

    if move_is_white_en_passant(start_sq, end_sq, chess)
        || move_is_black_en_passant(start_sq, end_sq, chess)
    {
        temp_board[end_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
    }

    temp_board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
    temp_board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
    !is_king_in_check_state(&temp_board, start_sq.piece.color())
}

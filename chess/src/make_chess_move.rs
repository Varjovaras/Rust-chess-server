use crate::{
    check::is_king_in_check_state,
    checkmate::{self},
    chess::Chess,
    chessboard::{file::File, rank::Rank, square::Square},
    game_state::{insufficient_material, stalemate, GameState},
    moves::{
        king::move_is_castling,
        move_helpers::helpers::{move_is_black_en_passant, move_is_white_en_passant},
        pawn::promote,
    },
    piece::{Piece, PieceColor},
};

pub fn make_chess_move(chess: &mut Chess, start_sq: &Square, end_sq: &Square) {
    let moving_piece_color = start_sq.piece.color();
    let opposite_color = moving_piece_color.opposite();

    if !is_move_valid(chess, start_sq, end_sq, moving_piece_color) {
        return;
    }

    handle_special_moves(chess, start_sq, end_sq);
    update_board(chess, start_sq, end_sq);
    handle_game_state(chess, opposite_color);
}

fn is_move_valid(
    chess: &Chess,
    start_sq: &Square,
    end_sq: &Square,
    moving_piece_color: PieceColor,
) -> bool {
    if !is_game_active(chess) || !is_correct_turn(chess, moving_piece_color) {
        return false;
    }

    if end_sq.piece == Piece::King(PieceColor::White)
        || end_sq.piece == Piece::King(PieceColor::Black)
    {
        return false;
    }

    if end_sq.has_piece() && end_sq.piece.color() == moving_piece_color {
        return false;
    }

    if !start_sq.piece.piece_move(start_sq, end_sq, chess) {
        return false;
    }

    if is_king_in_check(chess, moving_piece_color)
        && !king_is_not_in_check_after_move(chess, start_sq, end_sq)
    {
        return false;
    }

    king_is_not_in_check_after_move(chess, start_sq, end_sq)
}

fn is_game_active(chess: &Chess) -> bool {
    chess.gamestate == GameState::InProgress
        && !chess.white_player.victory
        && !chess.black_player.victory
        && chess.fifty_move_rule < 50
        && !insufficient_material(chess)
}

fn is_correct_turn(chess: &Chess, moving_piece_color: PieceColor) -> bool {
    (moving_piece_color == PieceColor::White && chess.turn_number % 2 == 0)
        || (moving_piece_color == PieceColor::Black && chess.turn_number % 2 == 1)
}

const fn is_king_in_check(chess: &Chess, color: PieceColor) -> bool {
    match color {
        PieceColor::White => chess.white_player.in_check(),
        PieceColor::Black => chess.black_player.in_check(),
        PieceColor::None => false,
    }
}

fn handle_special_moves(chess: &mut Chess, start_sq: &Square, end_sq: &Square) {
    if let Some(promoted_piece) = handle_promotion(chess, start_sq, end_sq) {
        chess.board[end_sq.file as usize][end_sq.rank as usize].piece = promoted_piece;
        chess.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
        return;
    }

    handle_en_passant(chess, start_sq, end_sq);
    handle_castling(chess, start_sq, end_sq);
}

fn handle_promotion(chess: &Chess, start_sq: &Square, end_sq: &Square) -> Option<Piece> {
    if (start_sq.piece == Piece::Pawn(PieceColor::White) && end_sq.rank == Rank::Eighth)
        || (start_sq.piece == Piece::Pawn(PieceColor::Black) && end_sq.rank == Rank::First)
    {
        match promote(start_sq, end_sq, chess) {
            Some(Piece::King(_) | Piece::Pawn(_)) | None => None,
            Some(promoted_piece) => Some(promoted_piece),
        }
    } else {
        None
    }
}

fn handle_en_passant(chess: &mut Chess, start_sq: &Square, end_sq: &Square) {
    if move_is_white_en_passant(start_sq, end_sq, chess)
        || move_is_black_en_passant(start_sq, end_sq, chess)
    {
        chess.board[end_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
    }
}

fn handle_castling(chess: &mut Chess, start_sq: &Square, end_sq: &Square) {
    if move_is_castling(start_sq, end_sq, chess) {
        // Implement castling logic here
    } else if start_sq.piece.is_king() || start_sq.piece.is_rook() {
        remove_castling(chess, start_sq);
    }
}

fn update_board(chess: &mut Chess, start_sq: &Square, end_sq: &Square) {
    // If the end square has a piece, it's being captured
    if end_sq.has_piece() {
        chess.pieces_eaten.add_piece(end_sq.piece);
        chess.fifty_move_rule = 0;
    } else if start_sq.piece.is_pawn() {
        chess.fifty_move_rule = 0;
    } else {
        chess.fifty_move_rule += 1;
    }

    // Handle en passant capture
    if (start_sq.piece == Piece::Pawn(PieceColor::White)
        && move_is_white_en_passant(start_sq, end_sq, chess))
        || (start_sq.piece == Piece::Pawn(PieceColor::Black)
            && move_is_black_en_passant(start_sq, end_sq, chess))
    {
        let captured_pawn = chess.board[end_sq.file as usize][start_sq.rank as usize].piece;
        chess.pieces_eaten.add_piece(captured_pawn);
        chess.board[end_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
    }

    chess.board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
    chess.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;

    chess.latest_move = Some((start_sq.clone(), end_sq.clone(), start_sq.piece.color()));
    chess.turn_number += 1;
    chess.list_of_moves.push((
        (start_sq.file, start_sq.rank.as_usize()),
        (end_sq.file, end_sq.rank.as_usize()),
    ));
}

fn handle_game_state(chess: &mut Chess, opposite_color: PieceColor) {
    update_check_status(chess);
    check_for_victory(chess);
    check_for_stalemate(chess, opposite_color);
}

fn check_for_victory(chess: &mut Chess) {
    if chess.white_player.in_check && checkmate::is_checkmate_position(chess) {
        chess.black_player.victory = true;
        chess.gamestate = GameState::BlackVictory;
    } else if chess.black_player.in_check && checkmate::is_checkmate_position(chess) {
        chess.white_player.victory = true;
        chess.gamestate = GameState::WhiteVictory;
    }
}

fn check_for_stalemate(chess: &mut Chess, opposite_color: PieceColor) {
    if chess.fifty_move_rule >= 50 || stalemate(chess, opposite_color) {
        chess.gamestate = GameState::Stalemate;
    }
}

fn update_check_status(chess: &mut Chess) {
    chess.white_player.in_check = is_king_in_check_state(&chess.board, PieceColor::White);
    chess.black_player.in_check = is_king_in_check_state(&chess.board, PieceColor::Black);
}

#[must_use]
pub fn king_is_not_in_check_after_move(chess: &Chess, start_sq: &Square, end_sq: &Square) -> bool {
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

fn remove_castling(chess: &mut Chess, start_sq: &Square) {
    match start_sq.piece {
        Piece::King(PieceColor::White) => {
            chess.castling.white.king = false;
            chess.castling.white.queen = false;
        }
        Piece::King(PieceColor::Black) => {
            chess.castling.black.king = false;
            chess.castling.black.queen = false;
        }
        Piece::Rook(PieceColor::White) => {
            if start_sq.file == File::A && start_sq.rank == Rank::First {
                chess.castling.white.queen = false;
            } else if start_sq.file == File::H && start_sq.rank == Rank::First {
                chess.castling.white.king = false;
            }
        }
        Piece::Rook(PieceColor::Black) => {
            if start_sq.file == File::A && start_sq.rank == Rank::Eighth {
                chess.castling.black.queen = false;
            } else if start_sq.file == File::H && start_sq.rank == Rank::Eighth {
                chess.castling.black.king = false;
            }
        }
        _ => panic!("Castling without starting piece shouldn't happen"),
    }
}

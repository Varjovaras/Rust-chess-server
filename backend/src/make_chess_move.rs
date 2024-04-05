use crate::{
    check::is_king_in_check_state,
    checkmate,
    chess::Chess,
    chessboard::{file::File, rank::Rank, square::Square},
    game_state::{insufficient_material, GameState},
    moves::{
        king::move_is_castling,
        move_helpers::helpers::{move_is_black_en_passant, move_is_white_en_passant},
        pawn::promote,
    },
    piece::{Piece, PieceColor},
};

pub fn make_chess_move(chess: &mut Chess, start_sq: &Square, end_sq: &Square) {
    let moving_piece_color = start_sq.piece.color();

    if !move_is_allowed(chess, moving_piece_color) {
        return;
    }

    if end_sq.piece == Piece::King(PieceColor::White)
        || end_sq.piece == Piece::King(PieceColor::Black)
    {
        return;
    }

    //cannot capture own piece
    if end_sq.has_piece() && end_sq.piece.color() == moving_piece_color {
        return;
    }

    //check if piece can make the move
    if !start_sq.piece.piece_move(start_sq, end_sq, chess) {
        return;
    };

    //king is in check and the move doesnt remove check return
    if (moving_piece_color == PieceColor::White && chess.white_player.in_check()
        || moving_piece_color == PieceColor::Black && chess.black_player.in_check())
        && !king_is_not_in_check_after_move(chess, start_sq, end_sq)
    {
        return;
    }

    //check if move puts you into check
    if !king_is_not_in_check_after_move(chess, start_sq, end_sq) {
        return;
    }

    if start_sq.piece == Piece::Pawn(PieceColor::White)
        || start_sq.piece == Piece::Pawn(PieceColor::Black)
    {
        chess.fifty_move_rule = 0;
    }

    if start_sq.piece == Piece::Pawn(PieceColor::White) && end_sq.rank == Rank::Eighth
        || start_sq.piece == Piece::Pawn(PieceColor::Black) && end_sq.rank == Rank::First
    {
        match promote(start_sq, end_sq, chess) {
            Some(Piece::King(_) | Piece::Pawn(_)) | None => return,
            Some(promoted_piece) => {
                chess.board[end_sq.file as usize][end_sq.rank as usize].piece = promoted_piece;
                chess.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
                handle_check_after_move(chess);
                chess.latest_move = Some((start_sq, end_sq, start_sq.piece.color()));
                chess.turn_number += 1;
                chess.list_of_moves.push((
                    (start_sq.file, start_sq.rank.as_usize()),
                    (end_sq.file, end_sq.rank.as_usize()),
                ));

                if end_sq.has_piece()
                    || start_sq.piece == Piece::Pawn(PieceColor::White)
                    || start_sq.piece == Piece::Pawn(PieceColor::Black)
                {
                    chess.fifty_move_rule = 0;
                } else {
                    chess.fifty_move_rule += 1;
                }

                if chess.fifty_move_rule >= 50 {
                    chess.gamestate = GameState::Stalemate;
                }
                return;
            }
        }
    }

    //remove en-passanted piece
    if move_is_white_en_passant(start_sq, end_sq, chess)
        || move_is_black_en_passant(start_sq, end_sq, chess)
    {
        chess.board[end_sq.file as usize][start_sq.rank as usize].piece = Piece::None;
    }

    if start_sq.piece == Piece::King(PieceColor::White)
        || start_sq.piece == Piece::King(PieceColor::Black)
        || start_sq.piece == Piece::Rook(PieceColor::White)
        || start_sq.piece == Piece::Rook(PieceColor::Black)
    {
        handle_rook_and_king_move(chess, start_sq, end_sq);
    }
    chess.board.iter_mut().for_each(|file| {
        file.iter()
            .for_each(|sq| sq.possible_moves = sq.possible_legal_moves(chess));
    });
    update_board(chess, start_sq, end_sq);
}

fn handle_rook_and_king_move(chess: &mut Chess, start_sq: Square, end_sq: Square) {
    //remove castling if king or rook moves
    if move_is_castling(start_sq, end_sq, chess) {
        handle_castling(chess, start_sq, end_sq);
        handle_check_after_move(chess);
        return;
    }
    remove_castling(chess, start_sq);
}

fn move_is_allowed(chess: &mut Chess, start_sq_piece_color: PieceColor) -> bool {
    if chess.gamestate != GameState::InProgress {
        return false;
    }

    if chess.white_player.victory || chess.black_player.victory {
        return false;
    }

    if chess.fifty_move_rule >= 50 {
        chess.gamestate = GameState::Stalemate;
        return false;
    }

    if insufficient_material(chess) {
        chess.gamestate = GameState::Stalemate;
        return false;
    }

    //wrong players turn
    if start_sq_piece_color == PieceColor::White && chess.turn_number % 2 == 1
        || start_sq_piece_color == PieceColor::Black && chess.turn_number % 2 == 0
    {
        return false;
    }

    true
}

fn update_board(chess: &mut Chess, start_sq: Square, end_sq: Square) {
    if end_sq.has_piece()
        || start_sq.piece == Piece::Pawn(PieceColor::White)
        || start_sq.piece == Piece::Pawn(PieceColor::Black)
    {
        chess.fifty_move_rule = 0;
    } else {
        chess.fifty_move_rule += 1;
    }

    if chess.fifty_move_rule >= 50 {
        chess.gamestate = GameState::Stalemate;
    }

    //move start_sq piece to end_sq
    chess.board[end_sq.file as usize][end_sq.rank as usize].piece = start_sq.piece;
    chess.board[start_sq.file as usize][start_sq.rank as usize].piece = Piece::None;

    chess.latest_move = Some((start_sq, end_sq, start_sq.piece.color()));
    chess.turn_number += 1;
    handle_check_after_move(chess);
    chess.list_of_moves.push((
        (start_sq.file, start_sq.rank.as_usize()),
        (end_sq.file, end_sq.rank.as_usize()),
    ));
}

fn handle_check_after_move(chess: &mut Chess) {
    chess.white_player.in_check = is_king_in_check_state(&chess.board, PieceColor::White);
    chess.black_player.in_check = is_king_in_check_state(&chess.board, PieceColor::Black);

    if chess.white_player.in_check {
        chess.black_player.victory = checkmate::is_checkmate_position(chess);
    } else if chess.black_player.in_check {
        chess.white_player.victory = checkmate::is_checkmate_position(chess);
    }

    if chess.white_player.victory {
        chess.gamestate = GameState::WhiteVictory;
    } else if chess.black_player.victory {
        chess.gamestate = GameState::BlackVictory;
    }
}

pub fn king_is_not_in_check_after_move(chess: &Chess, start_sq: Square, end_sq: Square) -> bool {
    let mut temp_board = chess.board;
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

fn handle_castling(chess: &mut Chess, start_sq: Square, end_sq: Square) {
    match (start_sq.rank, end_sq.file) {
        (Rank::First, File::G) => {
            chess.board[File::H as usize][Rank::First as usize].piece = Piece::None;
            chess.board[File::F as usize][Rank::First as usize].piece =
                Piece::Rook(PieceColor::White);
            chess.castling.white.king = false;
            chess.castling.white.queen = false;
        }
        (Rank::First, File::C) => {
            chess.board[File::A as usize][Rank::First as usize].piece = Piece::None;
            chess.board[File::D as usize][Rank::First as usize].piece =
                Piece::Rook(PieceColor::White);
            chess.castling.white.king = false;
            chess.castling.white.queen = false;
        }
        (Rank::Eighth, File::G) => {
            chess.board[File::H as usize][Rank::Eighth as usize].piece = Piece::None;
            chess.board[File::F as usize][Rank::Eighth as usize].piece =
                Piece::Rook(PieceColor::Black);
            chess.castling.black.king = false;
            chess.castling.black.queen = false;
        }
        (Rank::Eighth, File::C) => {
            chess.board[File::A as usize][Rank::Eighth as usize].piece = Piece::None;
            chess.board[File::D as usize][Rank::Eighth as usize].piece =
                Piece::Rook(PieceColor::Black);
            chess.castling.black.king = false;
            chess.castling.black.queen = false;
        }
        _ => panic!("Trying to castle with wrong start and end square"),
    }
}

fn remove_castling(chess: &mut Chess, start_sq: Square) {
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

// fn move_is_allowed_without_mutating_chess(chess: &Chess, start_sq_piece_color: PieceColor) -> bool {
//     if chess.gamestate != GameState::InProgress {
//         return false;
//     }

//     if chess.white_player.victory || chess.black_player.victory {
//         return false;
//     }

//     if chess.fifty_move_rule >= 50 {
//         return false;
//     }

//     if insufficient_material(chess) {
//         return false;
//     }

//     //wrong players turn
//     if start_sq_piece_color == PieceColor::White && chess.turn_number % 2 == 1
//         || start_sq_piece_color == PieceColor::Black && chess.turn_number % 2 == 0
//     {
//         return false;
//     }

//     true
// }

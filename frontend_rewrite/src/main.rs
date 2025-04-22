use chess::{
    Chess,
    chessboard::{file::File, rank::Rank},
    piece::PieceColor,
};

use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (chess, set_chess) = signal(Chess::new_starting_position());

    view! {
        <div class="container">
            <h1>"Chess Game"</h1>
            <ChessBoard chess=chess/>
        </div>
    }
}

#[component]
fn ChessBoard(chess: ReadSignal<Chess>) -> impl IntoView {
    view! {
        <div class="chessboard">
            {move || {
                let current_chess = chess.get();
                let board = current_chess.board;

                (0..8).map(move |rank| {
                    let rank_display = 8 - rank;
                    view! {
                        <div class="rank">
                            <div class="rank-label">{rank_display.to_string()}</div>
                            {(0..8).map(move |file| {
                                let square = current_chess.get_square(File::try_from(file as u8).unwrap(), Rank::try_from(rank as u8).unwrap());
                                let is_white_square = (file + rank) % 2 == 0;
                                let square_class = if is_white_square { "square white" } else { "square black" };

                                let (class, content) = if piece != chess::piece::Piece::None {
                                    let color_class = match piece.color() {
                                        PieceColor::White => "piece white-piece",
                                        PieceColor::Black => "piece black-piece",
                                        PieceColor::None => ""
                                    };
                                    let piece_type = piece.symbol().to_string();
                                    (color_class, piece_type)
                                } else {
                                    ("empty-square", "".to_string())
                                };

                                let piece_view = view! { <span class={class}>{content}</span>};

                                view! {
                                    <div class={square_class}>
                                        {piece_view}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    }
                }).collect_view()
            }}
            <div class="file-labels">
                {['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].map(|file| {
                    view! { <div class="file-label">{file.to_string()}</div> }
                }).collect_view()}
            </div>
        </div>
    }
}

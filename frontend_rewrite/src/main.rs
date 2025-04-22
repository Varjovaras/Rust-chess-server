use chess::Chess;

use leptos::prelude::*;

fn main() {
    let chess = Chess::new_starting_position();
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}

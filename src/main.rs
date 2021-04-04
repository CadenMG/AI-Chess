use clap::{Arg, App};
use chess::{Color};
mod game_logic;
mod game_printer;

fn main() {
    let matches = App::new("AI Chess Project")
        .arg(Arg::with_name("color")
                 .short("c")
                 .long("color")
                 .takes_value(true)
                 .help("Color you want to play as"))
        .arg(Arg::with_name("type")
                 .short("t")
                 .long("type")
                 .takes_value(true)
                 .help("Type of Chess game you want to play"))
        .get_matches();

    let color = matches.value_of("color").unwrap_or("W").to_ascii_uppercase();
    let game_type = matches.value_of("type").unwrap_or("1p").to_ascii_uppercase();

    let engine_color = if game_type == "2P" {
        Option::None
    } else if color == "W" {
        Option::Some(Color::Black)
    } else {
        Option::Some(Color::White)
    };

    let mut game = game_logic::GameLogic::new(engine_color);
    game.start();
}

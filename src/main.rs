use clap::{Arg, App};
use chess::{Color};
mod game_logic;
mod game_printer;
mod engine;

const DEPTH: i32 = 3;

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
        .arg(Arg::with_name("depth")
                 .short("d")
                 .long("depth")
                 .takes_value(true)
                 .help("Depth you want to analyzeA"))
        .arg(Arg::with_name("fen")
                 .short("f")
                 .long("fen")
                 .takes_value(true)
                 .help("FEN of the position you want to analyze")
                 .multiple(true))
        .arg(Arg::with_name("algo")
                 .short("a")
                 .long("algo")
                 .takes_value(true)
                 .help("Algorithm for AI to use"))
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
    let depth:i32 = matches.value_of("depth").unwrap_or(&*(DEPTH.to_string())).parse::<i32>().unwrap();
    let mut game = game_logic::GameLogic::new(engine_color, depth, matches.values_of("fen").unwrap().collect::<Vec<&str>>().join(" ").to_string(), matches.value_of("algo").unwrap_or("").to_string());
    game.start();
}

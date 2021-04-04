use chess::{Game, ChessMove, Square, Color};
use ai_chess::game_printer;

fn main() {
    let m = ChessMove::new(Square::D2,
        Square::D4,
        None);

    let mut game = Game::new();
    game.make_move(m);
    print!("{}", game_printer::board_to_string(&game.current_position(), Color::White));
}

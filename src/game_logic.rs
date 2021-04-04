use chess::{Game, Color, ChessMove, GameResult};
use std::io::{self, BufRead};
use crate::game_printer::board_to_string;

pub struct GameLogic {
    game_state: Game,
    engine_color: Option<Color>,
}

impl GameLogic {
    pub fn new(engine_color: Option<Color>) -> GameLogic {
        GameLogic {
            game_state: Game::new(),
            engine_color: engine_color, 
        }
    }
    pub fn start(&mut self) {
        while self.game_state.result().is_none() {
            let to_move = self.game_state.side_to_move();
            let position = self.game_state.current_position();
            print!("{}", board_to_string(&position, to_move));
            
            if self.engine_color.unwrap_or(opp_color(to_move)) != to_move {
                print!("Please enter your next move in algebraic notation\n");
                let mut raw_move = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut raw_move).expect("Could not read move");
                let maybe_move = ChessMove::from_san(&position, &raw_move);
                if maybe_move.is_ok() {
                    self.game_state.make_move(maybe_move.unwrap());
                } else {
                    print!("Given invalid move\n");
                }
            } else {
                print!("Engine made the move: ...\n");
                break;
            }
        }

        print!("{}", board_to_string(&self.game_state.current_position(), self.game_state.side_to_move()));
        print!("{}", res_to_string(self.game_state.result().unwrap()));
    }
}

fn res_to_string(game_result: GameResult) -> String {
    let string =
    match game_result {
        GameResult::WhiteCheckmates => "White wins by checkmate",
        GameResult::BlackCheckmates => "Black wins by checkmate",
        GameResult::WhiteResigns => "White wins by resignation",
        GameResult::BlackResigns => "Black wins by resignation",
        GameResult::DrawAccepted => "The draw was accepted",
        GameResult::DrawDeclared => "A draw has been offered",
        GameResult::Stalemate => "The game is drawn by stalemate"
    };
    return String::from(string);
}

fn opp_color(color: Color) -> Color {
    return match color {
        Color::Black => Color::White,
        Color::White => Color::Black
    }
}
use chess::{Game, ChessMove, MoveGen};

pub struct Engine {
    game_state: Game,
}

impl Engine {
    pub fn new(game_state: Game) -> Engine {
        Engine {
            game_state,
        }
    }

    pub fn play_move(&mut self, chess_move: ChessMove) {
        self.game_state.make_move(chess_move);
    }

    pub fn get_best_move(&self) -> Option<ChessMove> {
        let mut move_gen = MoveGen::new_legal(&self.game_state.current_position());
        return move_gen.next();
    }
}
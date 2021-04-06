use chess::{Game, ChessMove, MoveGen, Board, BoardStatus};

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
        let move_and_value = MoveGen::new_legal(&self.game_state.current_position())
            .map(|a_move| (a_move, minimax(self.game_state.current_position().make_move_new(a_move), 2, true)));
        let mut best_move = (Option::None, -1);
        for (chess_move, value) in move_and_value {
            if value > best_move.1 { best_move = (Option::Some(chess_move), value) }
        }
        return best_move.0;
    }
}

fn minimax(position: Board, depth: i32, maximizing_player: bool) -> i32 {
    if depth == 0 || position.status() != BoardStatus::Ongoing {
        return heuristic(position);
    }
    if maximizing_player {
        let mut value = -1;
        let children = MoveGen::new_legal(&position);
        for child in children {
            let new_position = position.make_move_new(child);
            value = std::cmp::max(value, minimax(new_position, depth - 1, false));
        }
        return value;
    }
    else {
        let mut value = 101;
        let children = MoveGen::new_legal(&position);
        for child in children {
            let new_position = position.make_move_new(child);
            value = std::cmp::min(value, minimax(new_position, depth - 1, true));
        }
        return value;
    }
}

fn heuristic(position: Board) -> i32 {
    //return rand::thread_rng().gen_range(0..100);
    return 10;
}
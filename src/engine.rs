use chess::{Game, Piece, Color, ChessMove, MoveGen, Board, BoardStatus};
use crate::game_printer::to_square;
use rand::prelude::*;
use std::time::{Duration, Instant};

const MAX_VAL: i32 = 1001;
const MIN_VAL: i32 = -1001;

fn get_algo(algo: String) -> fn(Board, i32, bool) -> i32 {
    let stralgo = &*algo;
    return match stralgo {
        "m" => minimax,
        "ab" => alphabetahelper,
        "bf" => bestfirst,
        "bs" => bstar,
        _ => alphabetahelper
    }

}

fn get_piece_weight(piece: Piece) -> i32 {
    let weight = match piece {
        Piece::King => 0,
        Piece::Queen => 9,
        Piece::Rook => 5,
        Piece::Bishop => 3,
        Piece::Knight => 3,
        Piece::Pawn => 1
    };
    return weight;
}

pub struct Engine {
    game_state: Game,
    depth: i32,
    algo: String
}
impl Engine {
    pub fn new(game_state: Game, depth: i32, algo: String) -> Engine {
        Engine {
            game_state,
            depth,
            algo
        }
    }

    pub fn play_move(&mut self, chess_move: ChessMove) {
        self.game_state.make_move(chess_move);
    }

    pub fn get_best_move(&self) -> Option<ChessMove> {
        let now = Instant::now();
        let move_and_value = MoveGen::new_legal(&self.game_state.current_position())
            .map(|a_move| (a_move, get_algo(self.algo.clone())(self.game_state.current_position().make_move_new(a_move), self.depth, true)));
        let mut best_move = (Option::None, MIN_VAL);
        for (chess_move, value) in move_and_value {
            if value > best_move.1 { best_move = (Option::Some(chess_move), value) }
        }
        println!("Time for move {}", now.elapsed().as_millis());
        return best_move.0;
    }
}

fn minimax(position: Board, depth: i32, maximizing_player: bool) -> i32 {
    if depth == 0 || position.status() != BoardStatus::Ongoing {
        return heuristic(position);
    }
    if maximizing_player {
        let mut value = MIN_VAL;
        let children = MoveGen::new_legal(&position);
        for child in children {
            let new_position = position.make_move_new(child);
            value = std::cmp::max(value, minimax(new_position, depth - 1, false));
        }
        return value;
    }
    else {
        let mut value = MAX_VAL;
        let children = MoveGen::new_legal(&position);
        for child in children {
            let new_position = position.make_move_new(child);
            value = std::cmp::min(value, minimax(new_position, depth - 1, true));
        }
        return value;
    }
}


fn alphabetahelper(position: Board, depth: i32, maximizing_player: bool) -> i32 {
    return alphabeta(position, depth, MIN_VAL, MAX_VAL, maximizing_player);
}

fn alphabeta(position: Board, depth: i32, alpha: i32, beta: i32, maximizing_player: bool) -> i32 {
    if depth == 0 || position.status() != BoardStatus::Ongoing {
        return heuristic(position);
    }
    let mut new_alpha = alpha;
    let mut new_beta = beta;
    if maximizing_player {
        let mut value = MIN_VAL;
        let children = MoveGen::new_legal(&position);
        for child in children {
            let new_position = position.make_move_new(child);
            value = std::cmp::max(value, alphabeta(new_position, depth - 1, new_alpha, new_beta, false));
            new_alpha = std::cmp::max(new_alpha, value);
            if new_alpha >= beta { break; }
        }
        return value;
    }
    else {
        let mut value = MAX_VAL;
        let children = MoveGen::new_legal(&position);
        for child in children {
            let new_position = position.make_move_new(child);
            value = std::cmp::min(value, alphabeta(new_position, depth - 1, new_alpha, new_beta, true));
            new_beta = std::cmp::min(new_beta, value);
            if new_beta <= new_alpha { break; }
        }
        return value;
    }
}

fn bestfirst(position: Board, depth: i32, maximizing_player: bool) -> i32 {
    if depth == 0 || position.status() != BoardStatus::Ongoing {
        return heuristic(position);
    }
    let moves: Vec<ChessMove> = MoveGen::new_legal(&position).collect();
    let mut moves_and_values: Vec<(ChessMove, i32)> = Vec::new();
    for mv in moves{
        moves_and_values.push((mv, heuristic(position.make_move_new(mv))));
    }
    moves_and_values.sort_by(|a, b| a.1.cmp(&b.1));

    let mut new_alpha = MIN_VAL;
    let mut new_beta = MAX_VAL;
    if maximizing_player {
        let mut value = MIN_VAL;
        for (mv, val) in moves_and_values {
            let new_position = position.make_move_new(mv);
            value = std::cmp::max(value, alphabeta(new_position, depth - 1, new_alpha, new_beta, false));
            new_alpha = std::cmp::max(new_alpha, value);
            if new_alpha >= new_beta { break; }
        }
        return value;
    }
    else {
        let mut value = MAX_VAL;
        for (mv, val) in moves_and_values {
            let new_position = position.make_move_new(mv);
            value = std::cmp::min(value, alphabeta(new_position, depth - 1, new_alpha, new_beta, true));
            new_beta = std::cmp::min(new_beta, value);
            if new_beta <= new_alpha { break; }
        }
        return value;
    }
}

fn bstar(position: Board, depth: i32, maximizing_player: bool) -> i32 {
    let moves: Vec<ChessMove> = MoveGen::new_legal(&position).collect();
    let mut moves_and_values: Vec<(ChessMove, i32)> = Vec::new();
    for mv in moves {
        let pos_after_first = position.make_move_new(mv);
        let pos_after_null = pos_after_first.null_move().unwrap_or(pos_after_first);
        let next_moves: Vec<ChessMove> = MoveGen::new_legal(&pos_after_null).collect();
        let mut values: Vec<i32> = Vec::new();
        for next_mv in next_moves {
            values.push(heuristic(pos_after_null.make_move_new(next_mv)));
        }
        moves_and_values.push((mv, *values.iter().max().unwrap_or(&0)));
    }

    moves_and_values.sort_by(|a, b| a.1.cmp(&b.1));

    let mut new_alpha = MIN_VAL;
    let mut new_beta = MAX_VAL;
    if maximizing_player {
        let mut value = MIN_VAL;
        for (mv, val) in moves_and_values {
            let new_position = position.make_move_new(mv);
            value = std::cmp::max(value, alphabeta(new_position, depth - 1, new_alpha, new_beta, false));
            new_alpha = std::cmp::max(new_alpha, value);
            if new_alpha >= new_beta { break; }
        }
        return value;
    }
    else {
        let mut value = MAX_VAL;
        for (mv, val) in moves_and_values {
            let new_position = position.make_move_new(mv);
            value = std::cmp::min(value, alphabeta(new_position, depth - 1, new_alpha, new_beta, true));
            new_beta = std::cmp::min(new_beta, value);
            if new_beta <= new_alpha { break; }
        }
        return value;
    }
}


fn randomheuristic(position: Board) -> i32 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    return (y * MAX_VAL as f64) as i32;
}

fn heuristic(position: Board) -> i32 {
    let status = position.status();
    let to_move = position.side_to_move();
    if status == BoardStatus::Checkmate {
        if to_move == Color::White { return MAX_VAL } else { return MIN_VAL; }
    } else if status == BoardStatus::Stalemate {
        return 0;
    } else {
       return score_material(position);
    }
}

fn score_material(position: Board) -> i32 {
   let mut score = 0;
   for row in 1..=8 {
       for col in 1..=8 {
           let square = to_square(row, col);
           let color = position.color_on(square).unwrap_or(Color::White);
           let value = score_square(position.piece_on(square)); 
           let multiplier = if color == Color::White { -1 } else { 1 };
           score += multiplier * value;
       }
   }
   return score;
}

fn score_square(square: Option<Piece>) -> i32 {
    if square.is_none() {
        return 0;
    } else{
        let piece = square.unwrap();
        return get_piece_weight(piece);
    }
}

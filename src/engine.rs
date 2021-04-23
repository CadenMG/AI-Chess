use chess::{Game, Piece, Color, ChessMove, MoveGen, Board, BoardStatus};
use crate::game_printer::to_square;

const MAX_VAL: i32 = 1001;
const MIN_VAL: i32 = -1001;

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
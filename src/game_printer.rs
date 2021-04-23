use chess::{Board, Color, Square, Piece, Rank, File};
use std::fmt::Write;

const EMPTY_SQUARE: &str = " _ ";
const BLACK_MARKER: &str = "*";
const BOARD_HEADER: &str = "    A  B  C  D  E  F  G  H    ";

pub fn board_to_string(board: &Board, perspective: Color) -> String {
    let (iter_order, actual_header) = if perspective == Color::White {
        (vec![7, 6, 5, 4, 3, 2, 1, 0], String::from(BOARD_HEADER))
    } else {
        (vec![0, 1, 2, 3, 4, 5, 6, 7], BOARD_HEADER.chars().rev().collect())
    };
    let mut res = String::from(format!("{}\n\n", actual_header));

    for row in iter_order.iter() {
        write!(&mut res, "{}  ", (row + 1).to_string());
        for col in iter_order.iter().rev() {
            let square = to_square(*row, *col);
            let piece = board.piece_on(square);
            let color = board.color_on(square);
            if piece.is_some() && color.is_some() {
                write!(&mut res, "{}", piece_to_string(piece.unwrap(), color.unwrap()));
            } else {
                write!(&mut res, "{}", EMPTY_SQUARE);
            }
        }
        write!(&mut res, "\n");
    }
    return res;
}

pub fn piece_to_string(piece: Piece, color: Color) -> String {
    let marker = if color == Color::Black {
        BLACK_MARKER
    } else {
        " "
    };
    let base = match piece {
        Piece::Bishop => " B",
        Piece::King => " K",
        Piece::Knight => " N",
        Piece::Pawn => " P",
        Piece::Queen => " Q",
        Piece::Rook => " R"
    };
    return [base, marker].join("");
}

pub fn to_square(row: i32, col: i32) -> Square {
    return Square::make_square(make_rank(row), make_file(col));
}

fn make_rank(row: i32) -> Rank {
    return Rank::from_index(row as usize);
}

fn make_file(col: i32) -> File {
    return File::from_index(col as usize);
}
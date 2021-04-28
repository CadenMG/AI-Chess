## Requirements

...

## How to play

- Execute `cargo run -- -c=[COLOR] -t=[GAME_TYPE] -a=[ALGO] -d=[DEPTH] -f=[FEN]` from inside the root directory
- `COLOR` can be in `(W|w|B|b)`
- `GAME_TYPE` can be in `(1p|2p)`
- ALGO can be in `(ab|bf|bs|m)`
- DEPTH can be any positive integer
- FEN is a valid FEN encoding of a chess position
- Run `cargo run -- -help` for more information

- example playing as white against the alpha beta AI with depth of 4
cargo run -- -cw -t1p -d4 -aab

- example playing as white against te best-first AI with depth 3 in a simple rook endgame
cargo run -- -cw -t1p -d3 -abf -f8/4k3/8/8/7R/8/2K5/8 w - - 0 1

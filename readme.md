# Quiddler Puzzle Game Solver

Brute forces all possible solutions of the [Quiddler Puzzle](https://www.setgame.com/quiddler/puzzle).

## Usage

1. Download the quiddler puzzle html page. `get_latest_puzzle.sh` is provided to do this automatically.

2. Run `quiddler_solver` (or `cargo run --release` if running from source).

## Notes

- Brute forcing can take an incredibly long time in some cases due to an enormous amount of possible solutions. Anywhere from 50,000 to upwards of 50,000,000 games could have to be calculated.

- Some potential game states can be missed if there are duplicate letters on the top level of cards or duplicate letters on the top and bottom levels. Duplicates in the bottom level are handled fully. Double letters are also handled fully.

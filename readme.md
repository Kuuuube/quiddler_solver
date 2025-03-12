# Quiddler Puzzle Game Solver

Brute forces all possible solutions of the [Quiddler Puzzle](https://www.setgame.com/quiddler/puzzle).

## Usage

1. Download the quiddler puzzle html page. It must be named `puzzle` and placed next to `quiddler_solver`.

    `get_latest_puzzle.sh` is provided to do this automatically.

2. Run `quiddler_solver` (or `cargo run --release` if running from source).

## Notes

- Brute forcing can take an incredibly long time in some cases due to an enormous amount of possible solutions. Anywhere from 50,000 to upwards of 50,000,000 games could have to be calculated. Be prepared for calculation to take hours.

- In some cases, the number of games may be too big to fit in your computer's memory. This does not affect the calculation of the games (`quiddler_games` output file) but scoring and sorting the games (`quiddler_games_scored` output file) will fail to be calculated.

- Some potential game states can be missed if there are duplicate letters on the top level of cards or duplicate letters on the top and bottom level at the same time. Duplicates in the bottom level are handled fully. Double letters are also handled fully.

- In `quiddler_games` output, double letters are replaces with symbols.

    - `!` = `er`

    - `@` = `cl`

    - `#` = `in`

    - `$` = `th`

    - `%` = `qu`

- In the `quiddler_games_scored` output, double letters are denoted by being capital.

    For example: the word `begin` from `b` `e` `g` `in` will be `begIN` but `b` `e` `g` `i` `n` will be `begin`.

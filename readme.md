# Quiddler Puzzle Game Solver

Brute forces all possible solutions of the [Quiddler Puzzle](https://www.setgame.com/quiddler/puzzle).

## Usage

1. Download the quiddler puzzle html page. It must be named `puzzle` and placed next to `quiddler_solver`.

    `get_latest_puzzle.sh` is provided to do this automatically.

2. Run `quiddler_solver` (or `cargo run --release` if running from source).

## Args

- `--skip_solving`: Uses an existing `quiddler_games` file to generate scores from. Useful if you stop solving midway through and want to calculate scores for a partial set of games.

- `--skip_sorting`: Reads back, scores, and outputs scored games buffered line by line. Useful for avoiding memory spikes from reading all games into memory.

- `--no_moving`: Prevents uncovering hidden cards when empty spaces are present. Disallowing moving can drastically cut down on the number of possible games and speed up calculation.

## Output

### `quiddler_games`

- Data Format:

    Words (comma `,` delimited)`|`Remaining letters (comma `,` delimited)

    ```
    cog,ai,lo|g,w,r,p,v,q,m,z,x
    ```

- Double letters are replaced with symbols.

    - `!` = `er`

    - `@` = `cl`

    - `#` = `in`

    - `$` = `th`

    - `%` = `qu`

### `quiddler_games_scored`

- Header:

    Shows the initially visible and hidden letters included in the puzzle. This can be useful for inputting solutions where the solver has moved cards.

    Example header:

    ```
    Quiddler Puzzle Letters:
    Visible | Hidden
    g w r p | a l g x
    o o c z | v q i m

    ```

- Data Format:

    `Words: `Words (comma `,` delimited)` | Remaining Letters: `Remaining letters (comma `,` delimited)` | Score: `Score integer

    ```
    Words: gorp,gov,qi,cwm,zax | Remaining Letters: l | Score: 103
    ```

- A header at the top shows the initially visible and hidden letters included in the puzzle. This can be useful for inputting solutions where the solver has moved cards.

- Double letters are denoted by being capital.

    For example: the word `begin` from `b` `e` `g` `in` will be `begIN` but `b` `e` `g` `i` `n` will be `begin`.


## Notes

- Brute forcing can take an incredibly long time in some cases due to an enormous amount of possible solutions. Anywhere from 50,000 to upwards of 50,000,000 games could have to be calculated. Be prepared for calculation to take hours.

- In some cases, the number of games may be too big to fit in your computer's memory. This does not affect the calculation of the games (`quiddler_games` output file) but scoring and sorting the games (`quiddler_games_scored` output file) will fail to be calculated. Pass the `--skip_sorting` arg to avoid memory constraints when calculating scores.

- Some potential game states can be missed if there are duplicate letters on the top level of cards or duplicate letters on the top and bottom level at the same time. Duplicates in the bottom level are handled fully. Double letters are also handled fully.

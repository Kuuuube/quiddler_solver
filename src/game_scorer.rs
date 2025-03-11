use std::io::{BufRead, Write};

use crate::double_letters;

pub fn calculate_game_scores(
    letter_scores: std::collections::HashMap<String, i32>,
    games_output_file_path: &str,
    scored_games_output_file_path: &str,
) -> i32 {
    let quiddler_games_file = std::fs::File::open(games_output_file_path).unwrap();
    let line_reader = std::io::BufReader::new(quiddler_games_file);

    let mut scored_games_output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(scored_games_output_file_path)
        .expect("Couldn't open output file `quiddler_games_scored`.");

    let mut games_count = 0;

    for line in line_reader.lines() {
        let game = get_game(line.unwrap());
        let mut score = 0;
        for word in &game.words {
            for letter in word.chars() {
                score += letter_scores.get(&letter.to_string()).unwrap_or_else(|| &0);
            }
            match word.len() {
                5 => score += 2,
                6 => score += 5,
                7 => score += 10,
                8 => score += 20,
                9 => score += 30,
                10 => score += 40,
                _ => {}
            }
        }
        for remaining_letter in &game.remaining_letters {
            score -= letter_scores
                .get(&remaining_letter.to_string())
                .unwrap_or_else(|| &0);
        }
        let game_score = score;
        let _ = scored_games_output_file.write(
            double_letters::replace_all_double_letter_symbols(format!(
                "Words: {} | Remaining Letters: {} | Score: {}\n",
                game.words.join(","),
                game.remaining_letters.join(","),
                game_score
            ))
            .as_bytes(),
        );
        games_count += 1;
    }
    return games_count;
}

fn get_game(game_string: String) -> QuiddlerGame {
    let game_string_split = game_string.split_once("|").unwrap();
    let words = game_string_split.0.split(",").map(String::from).collect();
    let remaining_letters: Vec<String> =
        game_string_split.1.split(",").map(String::from).collect();
    return QuiddlerGame {
        words,
        remaining_letters,
        score: None,
    };
}

#[derive(Debug)]
pub struct QuiddlerGame {
    pub words: Vec<String>,
    pub remaining_letters: Vec<String>,
    pub score: Option<i32>,
}

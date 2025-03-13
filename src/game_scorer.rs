use crate::double_letters;

pub fn calculate_game_score(
    letter_scores: &std::collections::HashMap<String, i32>,
    game_string: &str,
) -> QuiddlerGame {
    let mut quiddler_game = get_game(game_string);
    let mut score = 0;
    for word in &quiddler_game.words {
        for letter in word.chars() {
            score += letter_scores.get(&letter.to_string()).unwrap_or_else(|| &0);
        }
        match double_letters::replace_all_double_letter_symbols(word.to_string()).len() {
            5 => score += 2,
            6 => score += 5,
            7 => score += 10,
            8 => score += 20,
            9 => score += 30,
            10 => score += 40,
            _ => {}
        }
    }
    for remaining_letter in &quiddler_game.remaining_letters {
        score -= letter_scores
            .get(&remaining_letter.to_string())
            .unwrap_or_else(|| &0);
    }
    quiddler_game.score = Some(score);

    return quiddler_game;
}

fn get_game(game_string: &str) -> QuiddlerGame {
    let game_string_split = game_string.split_once("|").unwrap();
    let words = game_string_split.0.split(",").map(String::from).collect();
    let remaining_letters: Vec<String> = game_string_split.1.split(",").map(String::from).collect();

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

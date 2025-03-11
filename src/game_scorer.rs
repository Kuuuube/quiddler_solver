pub fn calculate_game_scores(letter_scores: std::collections::HashMap<String, i32>, games_output_file_path: &str) -> Vec<QuiddlerGame> {
    let mut quiddler_games = get_games(games_output_file_path);
    for game in &mut quiddler_games {
        let mut score = 0;
        for word in &game.words {
            for letter in word.chars() {
                score += letter_scores.get(&letter.to_string()).unwrap_or_else(|| &0);
            }
            match word.len() {
                5 => { score += 2 },
                6 => { score += 5 },
                7 => { score += 10 },
                8 => { score += 20 },
                9 => { score += 30 },
                10 => { score += 40 },
                _ => {}
            }
        }
        for remaining_letter in &game.remaining_letters {
            score -= letter_scores.get(&remaining_letter.to_string()).unwrap_or_else(|| &0);
        }
        game.score = Some(score);
    }
    // Highest to lowest score
    quiddler_games.sort_by(|a, b| (&b.score).cmp(&a.score));
    return quiddler_games;
}

fn get_games(games_output_file_path: &str) -> Vec<QuiddlerGame> {
    let mut games_without_score: Vec<QuiddlerGame> = vec![];
    let games_file_lines: Vec<String> = std::fs::read_to_string(games_output_file_path).unwrap().lines().map(String::from).collect();
    for game_string in games_file_lines {
        let game_string_split = match game_string.split_once("|") {
            Some(some) => some,
            None => continue,
        };
        let words = game_string_split.0.split(",").map(String::from).collect();
        let remaining_letters: Vec<String> = game_string_split.1.chars().map(String::from).collect();
        games_without_score.push(QuiddlerGame { words, remaining_letters, score: None });
    }
    return games_without_score;
}

#[derive(Debug)]
pub struct QuiddlerGame {
    pub words: Vec<String>,
    pub remaining_letters: Vec<String>,
    pub score: Option<i32>,
}
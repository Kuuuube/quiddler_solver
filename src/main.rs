use std::io::Write;

mod double_letters;
mod game_scorer;
mod quiddler_parser;
mod quiddler_solver;

fn main() {
    let quiddler_game_html = std::fs::read_to_string("puzzle").unwrap();

    let quiddler_game_init_regex =
        regex::Regex::new(r"    var dictionary = new Dictionary\(\);dictionary\.init\(.*").unwrap();
    let quiddler_game_init_str = quiddler_game_init_regex
        .find(&quiddler_game_html)
        .unwrap()
        .as_str()
        .to_string();

    let quiddler_game_dictionary =
        quiddler_parser::get_quiddler_dictionary(&quiddler_game_init_str);
    println!(
        "Parsed Quiddler game dictionary. Found {} words.",
        quiddler_game_dictionary.len()
    );
    let quiddler_game_letters = quiddler_parser::get_quiddler_letters(&quiddler_game_init_str);
    println!(
        "Parsed Quiddler game Letters. Visible: {}. Hidden: {}.",
        quiddler_game_letters.visible.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","),
        quiddler_game_letters
            .hidden
            .values()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
    let quiddler_game_letter_scores =
        quiddler_parser::get_quiddler_letter_scores(&quiddler_game_init_str);

    let games_output_file_path = "quiddler_games";
    let mut games_output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(games_output_file_path)
        .expect("Couldn't open output file `quiddler_games`.");

    let calculate_solutions_start_time = std::time::Instant::now();

    quiddler_solver::calculate_solutions(
        &quiddler_game_letters,
        &quiddler_game_dictionary,
        0,
        vec![],
        &mut games_output_file,
    );

    let calculate_solutions_time_elapsed = calculate_solutions_start_time.elapsed();
    println!("Brute forced all solutions in: {calculate_solutions_time_elapsed:.6?}");

    let calculate_scores_start_time = std::time::Instant::now();
    let scored_games =
        game_scorer::calculate_game_scores(quiddler_game_letter_scores, games_output_file_path);
    let calculate_scores_time_elapsed = calculate_scores_start_time.elapsed();
    println!(
        "Calculated scores of {} games in {calculate_scores_time_elapsed:.6?}",
        scored_games.len()
    );

    let scored_games_output_file_path = "quiddler_games_scored";
    let mut scored_games_output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(scored_games_output_file_path)
        .expect("Couldn't open output file `quiddler_games_scored`.");

    // Header
    let _ = scored_games_output_file.write(
        format!(
            "Quiddler Puzzle Letters: \nVisible | Hidden\n{} | {}\n{} | {}\n\n",
            double_letters::replace_all_double_letter_symbols(
                quiddler_parser::get_visible_letters_row(&quiddler_game_letters.visible, 1)
            ),
            double_letters::replace_all_double_letter_symbols(
                quiddler_parser::get_hidden_letters_row(&quiddler_game_letters.hidden, 1)
            ),
            double_letters::replace_all_double_letter_symbols(
                quiddler_parser::get_visible_letters_row(&quiddler_game_letters.visible, 2)
            ),
            double_letters::replace_all_double_letter_symbols(
                quiddler_parser::get_hidden_letters_row(&quiddler_game_letters.hidden, 2)
            )
        )
        .as_bytes(),
    );

    for game in scored_games {
        let _ = scored_games_output_file.write(
            double_letters::replace_all_double_letter_symbols(format!(
                "Words: {} | Remaining Letters: {} | Score: {}\n",
                game.words.join(","),
                game.remaining_letters.join(","),
                game.score.unwrap_or_default()
            ))
            .as_bytes(),
        );
    }
}

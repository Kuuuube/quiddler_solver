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
        quiddler_game_letters.visible.join(","),
        quiddler_game_letters.hidden.join(",")
    );

    let games_output_file_path = "quiddler_games";
    let mut games_output_file = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(games_output_file_path).expect("Couldn't open output file `quiddler_games`.");

    let calculate_solutions_start_time = std::time::Instant::now();

    quiddler_solver::calculate_solutions(
        quiddler_game_letters,
        quiddler_game_dictionary,
        0,
        vec![],
        &mut games_output_file,
    );

    let calculate_solutions_time_elapsed = calculate_solutions_start_time.elapsed();
    println!("Brute forced all solutions in: {calculate_solutions_time_elapsed:.6?}");
}

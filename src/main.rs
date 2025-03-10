use regex::Regex;

fn main() {
    let quiddler_game_html = std::fs::read_to_string("puzzle").unwrap();

    let quiddler_game_init_regex =
        Regex::new(r"    var dictionary = new Dictionary\(\);dictionary\.init\(.*").unwrap();
    let quiddler_game_init_str = quiddler_game_init_regex
        .find(&quiddler_game_html)
        .unwrap()
        .as_str()
        .to_string();

    let quiddler_game_dictionary_str_regex = Regex::new(r"dictionary\.init\(.*?\)").unwrap();
    let quiddler_game_dictionary = parse_quiddler_dictionary(
        quiddler_game_dictionary_str_regex
            .find(&quiddler_game_init_str)
            .unwrap()
            .as_str(),
    );

    let quiddler_game_letters_str_regex = Regex::new(r"board\.loadCards\(.*?\)").unwrap();
    let quiddler_game_letters = parse_quiddler_letters(
        quiddler_game_letters_str_regex
            .find(&quiddler_game_init_str)
            .unwrap()
            .as_str(),
    );
}

fn parse_quiddler_dictionary(dictionary_string: &str) -> Vec<String> {
    return remove_all(dictionary_string, vec!["dictionary.init(", ")", "\\", "\""])
        .split(",")
        .map(|x| x.to_owned())
        .collect();
}

fn parse_quiddler_letters(letters_string: &str) -> QuiddlerLetters {
    let scores_removing_regex = Regex::new(r#"\",\d+,\""#).unwrap();
    let filtered_letters_string = scores_removing_regex
        .replace_all(letters_string, ",")
        .into_owned();

    let letters_vec: Vec<String> = remove_all(
        &filtered_letters_string,
        vec!["board.loadCards(", ")", "\\", "\""],
    )
    .split(",")
    .map(|x| x.to_owned())
    .collect();

    return QuiddlerLetters {
        visible: letters_vec[0..8].to_vec(),
        hidden: letters_vec[8..=15].to_vec(),
    };
}

fn remove_all(input_string: &str, strings_to_remove: Vec<&str>) -> String {
    let mut output_string: String = input_string.to_string();
    for string_to_remove in strings_to_remove {
        output_string = output_string.replace(&string_to_remove, "");
    }
    return output_string;
}

struct QuiddlerLetters {
    visible: Vec<String>,
    hidden: Vec<String>,
}

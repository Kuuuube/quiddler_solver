pub fn get_quiddler_dictionary(quiddler_string: &str) -> Vec<String> {
    let quiddler_dictionary_str_regex = regex::Regex::new(r"dictionary\.init\(.*?\)").unwrap();
    let quiddler_dictionary_string = quiddler_dictionary_str_regex
        .find(&quiddler_string)
        .unwrap()
        .as_str();

    return remove_all(
        quiddler_dictionary_string,
        vec!["dictionary.init(", ")", "\\", "\""],
    )
    .split(",")
    .map(|x| x.to_owned())
    .collect();
}

pub fn get_quiddler_letters(quiddler_string: &str) -> QuiddlerLetters {
    let quiddler_letters_str_regex = regex::Regex::new(r"board\.loadCards\(.*?\)").unwrap();
    let quiddler_letters = quiddler_letters_str_regex
        .find(&quiddler_string)
        .unwrap()
        .as_str();

    let scores_removing_regex = regex::Regex::new(r#"\",\d+,\""#).unwrap();
    let filtered_letters_string = scores_removing_regex
        .replace_all(quiddler_letters, ",")
        .into_owned();

    let letters_vec: Vec<String> = remove_all(
        &filtered_letters_string,
        vec!["board.loadCards(", ")", "\\", "\""],
    )
    .to_lowercase()
    .split(",")
    .map(|x| x.to_owned())
    .collect();

    return QuiddlerLetters {
        visible: letters_vec[0..8].to_vec(),
        hidden: letters_vec[8..=15].to_vec(),
    };
}

pub fn remove_all(input_string: &str, strings_to_remove: Vec<&str>) -> String {
    let mut output_string: String = input_string.to_string();
    for string_to_remove in strings_to_remove {
        output_string = output_string.replace(&string_to_remove, "");
    }
    return output_string;
}

pub struct QuiddlerLetters {
    pub visible: Vec<String>,
    pub hidden: Vec<String>,
}

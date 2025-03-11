pub fn get_quiddler_dictionary(quiddler_string: &str) -> Vec<String> {
    let quiddler_dictionary_str_regex = regex::Regex::new(r"dictionary\.init\(.*?\)").unwrap();
    let quiddler_dictionary_string = quiddler_dictionary_str_regex
        .find(&quiddler_string)
        .unwrap()
        .as_str();

    let mut quiddler_dictionary: Vec<String> = remove_all(
        quiddler_dictionary_string,
        vec!["dictionary.init(", ")", "\\", "\""],
    )
    .split(",")
    .map(String::from)
    .collect();

    quiddler_dictionary.sort();
    quiddler_dictionary.dedup();

    // Only single occurrances of double letters are handled here
    for word in quiddler_dictionary.clone() {
        for double_letter in crate::double_letters::POSSIBLE_DOUBLE_LETTERS {
            if word.contains(double_letter) {
                quiddler_dictionary.push(word.replace(
                    double_letter,
                    crate::double_letters::get_double_letter_symbol(double_letter),
                ));
            }
        }
    }

    return quiddler_dictionary;
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
    .map(|x| crate::double_letters::get_double_letter_symbol(x).to_string())
    .collect();

    return QuiddlerLetters {
        visible: letters_vec[0..8].to_vec(),
        hidden: letters_vec[8..=15].to_vec(),
    };
}

pub fn get_quiddler_letter_scores(quiddler_string: &str) -> std::collections::HashMap<String, i32> {
    let quiddler_letters_str_regex = regex::Regex::new(r"board\.loadCards\(.*?\)").unwrap();
    let quiddler_letters = quiddler_letters_str_regex
        .find(&quiddler_string)
        .unwrap()
        .as_str();

    let letters_hashmap: std::collections::HashMap<String, i32> = remove_all(
        &quiddler_letters,
        vec!["board.loadCards(", ")", "\\", ",\"\""],
    )
    .to_lowercase()
    .split(",\"")
    .map(|x| {
        let cleaned_string = x.replace("\"", "");
        let letter_and_score = cleaned_string.split_once(",").unwrap_or_default();
        return (
            crate::double_letters::get_double_letter_symbol(letter_and_score.0).to_string(),
            letter_and_score.1.parse::<i32>().unwrap_or_default(),
        );
    })
    .collect();

    return letters_hashmap;
}

pub fn remove_all(input_string: &str, strings_to_remove: Vec<&str>) -> String {
    let mut output_string: String = input_string.to_string();
    for string_to_remove in strings_to_remove {
        output_string = output_string.replace(&string_to_remove, "");
    }
    return output_string;
}

#[derive(Debug, Clone)]
pub struct QuiddlerLetters {
    pub visible: Vec<String>,
    pub hidden: Vec<String>,
}

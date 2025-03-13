pub const POSSIBLE_DOUBLE_LETTERS: [&str; 5] = ["er", "cl", "in", "th", "qu"];

pub fn get_double_letter_symbol(letter: &str) -> &str {
    return match letter {
        "er" => "!",
        "cl" => "@",
        "in" => "#",
        "th" => "$",
        "qu" => "%",
        _ => letter,
    };
}

pub fn get_double_letter_symbol_char(letter: &str) -> char {
    return match letter {
        "er" => '!',
        "cl" => '@',
        "in" => '#',
        "th" => '$',
        "qu" => '%',
        _ => letter.chars().next().unwrap_or_else(|| '\0'),
    };
}

// pub fn get_double_letter_from_symbol(letter: &str) -> &str {
//     return match letter {
//         "!" => "er",
//         "@" => "cl",
//         "#" => "in",
//         "$" => "th",
//         "%" => "qu",
//         _ => letter
//     }
// }

pub fn replace_all_double_letter_symbols(input_string: String) -> String {
    return input_string
        .replace("!", "ER")
        .replace("@", "CL")
        .replace("#", "IN")
        .replace("$", "TH")
        .replace("%", "QU");
}

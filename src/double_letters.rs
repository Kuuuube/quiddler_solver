pub const POSSIBLE_DOUBLE_LETTERS: [&str; 5] = ["er", "cl", "in", "th", "qu"];

pub fn get_double_letter_symbol(letter: &str) -> &str {
    return match letter {
        "er" => "!",
        "cl" => "@",
        "in" => "#",
        "th" => "$",
        "qu" => "%",
        _ => letter
    }
}

pub fn get_double_letter_from_symbol(letter: &str) -> Option<String> {
    return match letter {
        "!" => Some("er".to_string()),
        "@" => Some("cl".to_string()),
        "#" => Some("in".to_string()),
        "$" => Some("th".to_string()),
        "%" => Some("qu".to_string()),
        _ => None
    }
}

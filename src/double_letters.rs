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

pub fn get_double_letter_from_symbol(letter: &str) -> &str {
    return match letter {
        "!" => "er",
        "@" => "cl",
        "#" => "in",
        "$" => "th",
        "%" => "qu",
        _ => letter
    }
}

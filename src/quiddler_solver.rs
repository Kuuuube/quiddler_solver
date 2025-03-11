use crate::quiddler_parser::QuiddlerLetters;

const USED_LETTER_PLACEHOLDER: &str = "-";
const MAXIMUM_POSSIBLE_WORDCOUNT: i32 = 8;

pub fn calculate_solutions(
    letters: QuiddlerLetters,
    dictionary: Vec<String>,
    current_wordcount: i32,
    previous_words: Vec<String>,
) {
    let possible_words = get_possible_words(&letters.visible, &dictionary);
    if current_wordcount >= MAXIMUM_POSSIBLE_WORDCOUNT || possible_words.len() == 0 {
        if letters.visible.join("").replace("-", "").len() == 0 {
            println!("Solved! {}", previous_words.join(","));
        } else {
            let remaining_letters = format!(
                "{}{}",
                letters.visible.join("").replace("-", ""),
                letters.hidden.join("").replace("-", "")
            );
            println!(
                "Failed. {} | Remaining letters: {} | Remaining letters count: {}",
                previous_words.join(","),
                remaining_letters,
                remaining_letters.len()
            );
        }
        return;
    }

    let mut i = 1;
    for possible_word in &possible_words {
        if current_wordcount == 0 {
            println!("{}/{}", i, possible_words.len());
            i += 1;
        }
        let mut working_letters = letters.clone();
        for char in possible_word.chars() {
            let visible_letter_index = working_letters
                .visible
                .iter()
                .position(|x| *x == char.to_string())
                .unwrap();
            working_letters.visible[visible_letter_index] = USED_LETTER_PLACEHOLDER.to_string();
            working_letters = repopulate_visible_letters(working_letters);
        }
        calculate_solutions(
            working_letters,
            dictionary.clone(),
            current_wordcount + 1,
            [previous_words.clone(), vec![possible_word.to_string()]].concat(),
        );
    }
}

fn get_possible_words(visible_letters: &Vec<String>, dictionary: &Vec<String>) -> Vec<String> {
    let mut new_dictionary = vec![];
    for word in dictionary {
        let mut valid_word = true;
        let mut working_visible_letters = visible_letters.clone();
        for char in word.chars() {
            if !working_visible_letters.contains(&char.to_string()) {
                valid_word = false;
                break;
            }
            let letter_index = working_visible_letters
                .iter()
                .position(|x| *x == char.to_string())
                .unwrap();
            working_visible_letters[letter_index] = USED_LETTER_PLACEHOLDER.to_string();
        }
        if valid_word {
            new_dictionary.push(word.to_string());
        }
    }
    return new_dictionary;
}

fn repopulate_visible_letters(input_letters: QuiddlerLetters) -> QuiddlerLetters {
    let mut output_letters = input_letters.clone();
    let mut i: usize = 0;
    while i < input_letters.visible.len() {
        if input_letters.visible.get(i).unwrap() == USED_LETTER_PLACEHOLDER {
            output_letters.visible[i] = output_letters.hidden[i].clone();
            output_letters.hidden[i] = USED_LETTER_PLACEHOLDER.to_string();
        }
        i += 1;
    }
    return output_letters;
}

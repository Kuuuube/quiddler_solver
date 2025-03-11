use std::io::Write;

use crate::quiddler_parser::QuiddlerLetters;

const USED_LETTER_PLACEHOLDER: &str = "-";
const MAXIMUM_POSSIBLE_WORDCOUNT: i32 = 8;

pub fn calculate_solutions(
    letters: QuiddlerLetters,
    dictionary: Vec<String>,
    current_wordcount: i32,
    previous_words: Vec<String>,
    output_file: &mut std::fs::File,
) {
    let possible_words = get_possible_words(&letters.visible, &dictionary);
    if current_wordcount >= MAXIMUM_POSSIBLE_WORDCOUNT || possible_words.len() == 0 {
        if letters.visible.join("").replace("-", "").len() == 0 {
            let success_message = format!("{}|\n", previous_words.join(","));
            let _ = output_file.write(success_message.as_bytes());
        } else {
            let remaining_letters = [letters.visible, letters.hidden].concat().iter().filter(|x| *x != "-").map(String::from).collect::<Vec<String>>().join(",");
            let fail_message = format!(
                "{}|{}\n",
                previous_words.join(","),
                remaining_letters,
            );
            let _ = output_file.write(fail_message.as_bytes());
        }
        return;
    }

    let mut i = 1;
    for possible_word in &possible_words {
        if current_wordcount == 0 {
            println!("{}/{} possible game branches finished", i, possible_words.len());
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
            output_file,
        );
    }
}

fn get_possible_words(visible_letters: &Vec<String>, dictionary: &Vec<String>) -> Vec<String> {
    let mut new_dictionary = vec![];
    for word in dictionary {
        let mut working_word = word.clone();
        for letter in visible_letters {
            if working_word.contains(letter) {
                working_word = working_word.replace(letter, "");
            }
        }
        if working_word.len() == 0 {
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

use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lower_word = word.to_lowercase();
    let sorted_word = sorted_str(&lower_word);

    possible_anagrams
        .iter()
        .filter(|&&possible_anagram| {
            let lower_possible_anagram = possible_anagram.to_lowercase();
            lower_word != lower_possible_anagram
                && sorted_word == sorted_str(&lower_possible_anagram)
        })
        .cloned()
        .collect()
}

fn sorted_str(word: &str) -> String {
    let mut word: Vec<char> = word.to_lowercase().chars().collect();
    word.sort();
    word.into_iter().collect()
}

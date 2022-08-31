use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lowered = word.to_lowercase();
    let sorted = normalize(word);

    possible_anagrams
        .iter()
        .filter(|&candidate| candidate.to_lowercase() != lowered && normalize(candidate) == sorted)
        .copied()
        .collect()
}

fn normalize(word: &str) -> Vec<char> {
    let mut characters: Vec<_> = word.to_lowercase().chars().collect();
    characters.sort_unstable();
    characters
}

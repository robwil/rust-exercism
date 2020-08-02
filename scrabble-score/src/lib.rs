use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|c| match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        })
        .sum()
}

// How would I model double/triple word and letter? with an overloaded function method like this:

pub fn score_with_multipliers(
    word: &str,
    word_multiplier: u64,
    letter_multipliers: &HashMap<usize, u64>,
) -> u64 {
    word.chars()
        .map(|c| match c.to_ascii_uppercase() {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        })
        .enumerate()
        .map(|(i, score)| match letter_multipliers.get(&i) {
            Some(letter_multiplier) => score * letter_multiplier,
            None => score,
        })
        .sum::<u64>()
        * word_multiplier
}

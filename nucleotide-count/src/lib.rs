use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let is_valid = |c| match c {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    };
    if !is_valid(nucleotide) {
        return Err(nucleotide);
    }
    let mut sum = 0;
    for c in dna.chars() {
        if !is_valid(c) {
            return Err(c);
        }
        if c == nucleotide {
            sum += 1;
        }
    }
    Ok(sum)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    ['A', 'C', 'G', 'T']
        .iter()
        .map(|c| count(*c, dna).map(|count| (*c, count)))
        .collect()
}

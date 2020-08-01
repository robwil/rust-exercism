use itertools::Itertools;
use std::fmt::Write;

pub fn encode(source: &str) -> String {
    encode_clean_fast(source)
}

fn partial_encode_rle(result: &mut String, c: char, count: usize) {
    if count == 1 {
        result.push(c);
    } else {
        write!(result, "{}{}", count, c).unwrap();
    }
}

pub fn encode_original(source: &str) -> String {
    if source.is_empty() {
        return String::from("");
    }
    let mut result = String::new();
    let mut prev_char: char = source.chars().next().unwrap();
    let mut running_count: usize = 1;
    for (i, c) in source.chars().enumerate() {
        if i == 0 {
            continue;
        }
        if c != prev_char {
            partial_encode_rle(&mut result, prev_char, running_count);
            prev_char = c;
            running_count = 1;
        } else {
            running_count += 1;
        }
    }
    partial_encode_rle(&mut result, prev_char, running_count);
    result
}

// This is 2x slower than my original.
// I assume this is because peekable iterator uses a Vec<> under the hood which requires looping the whole string an extra time to convert/copy to Vec.
// (But I was wrong, see comments for next function)
pub fn encode_cleaner(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    let mut running_count: usize = 0;
    while let Some(current_char) = chars.next() {
        running_count += 1;
        // if next char is different (or None), write current batch now
        if chars.peek() != Some(&current_char) {
            if running_count > 1 {
                result.push_str(&running_count.to_string());
            }
            result.push(current_char);
            running_count = 0;
        }
    }
    result
}

// Attempt to get the clean-looking solution but without expensive peek iterator.
// My first attempt was to use a byte slice with same approach as "encode_cleaner", but it had no speed improvement.
// It turns out the peekable iterator wasn't the slow part but rather the pushing to the result string.
// The use of write! here turns out to be very performant, and I was able to simplify the conditional similar to "cleaner" for a speed gain.
pub fn encode_clean_fast(source: &str) -> String {
    let mut result = String::new();
    let source = source.as_bytes();
    let mut running_count: usize = 0;
    for i in 0..source.len() {
        running_count += 1;
        // if next char is different (or nonexistent), write current batch now
        if i + 1 >= source.len() || source[i] != source[i + 1] {
            if running_count > 1 {
                write!(result, "{}", running_count).unwrap();
            }
            result.push(source[i] as char);
            running_count = 0;
        }
    }
    result
}

// This functional version is really nice, but group_by adds way too much performance overhead.
pub fn encode_functional(source: &str) -> String {
    source
        .chars()
        .group_by(|&c| c)
        .into_iter()
        .map(|(c, group)| match group.count() {
            1 => c.to_string(),
            n => format!("{}{}", n, c),
        })
        .collect()
}

pub fn decode(source: &str) -> String {
    decode_clean_fast(source)
}

pub fn decode_original(source: &str) -> String {
    let mut result = String::new();
    let source: Vec<char> = source.chars().collect();
    let mut i: usize = 0;
    let mut count = String::new();
    while i < source.len() {
        while source[i].is_numeric() {
            count.push(source[i]);
            i += 1;
        }
        if !count.is_empty() {
            let mut parsed_count = count.parse::<usize>().unwrap();
            count.clear();
            while parsed_count > 0 {
                result.push(source[i]);
                parsed_count -= 1;
            }
        } else {
            result.push(source[i]);
        }
        i += 1;
    }
    result
}

// This code is cleaner, but it turns out that .to_string().repeat() is super slow.
// I assume .repeat() is pretty smart, so the problem is probably the conversion of a single char to string.
pub fn decode_cleaner(source: &str) -> String {
    let mut result = String::new();
    let mut numeric_chars = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            numeric_chars.push(c);
        } else {
            result.push_str(
                &c.to_string()
                    .repeat(numeric_chars.parse::<usize>().unwrap_or(1)),
            );
            numeric_chars.clear();
        }
    }
    result
}

// clean and fast solution
pub fn decode_clean_fast(source: &str) -> String {
    let mut result = String::new();
    let mut numeric_chars = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            numeric_chars.push(c);
        } else {
            let mut parsed_count = numeric_chars.parse::<usize>().unwrap_or(1);
            while parsed_count > 0 {
                result.push(c);
                parsed_count -= 1;
            }
            numeric_chars.clear();
        }
    }
    result
}

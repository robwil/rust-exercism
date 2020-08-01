use itertools::Itertools;
use std::fmt::Write;

pub fn encode(source: &str) -> String {
    encode_cleaner(source)
}

fn partial_encode_rle(result: &mut String, c: char, count: usize) {
    if count == 1 {
        result.push(c);
    } else {
        write!(result, "{}{}", count, c).unwrap();
    }
}

pub fn encode_original(source: &str) -> String {
    if source.len() <= 0 {
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
    return result;
}

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
    return result;
}

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
    let mut result = String::new();
    let source: Vec<char> = source.chars().collect();
    let mut i: usize = 0;
    while i < source.len() {
        let mut count = String::new();
        while source[i] >= '0' && source[i] <= '9' {
            count.push(source[i]);
            i += 1;
        }
        if count.len() > 0 {
            let mut count = count.parse::<usize>().unwrap();
            while count > 0 {
                result.push(source[i]);
                count -= 1;
            }
        } else {
            result.push(source[i]);
        }
        i += 1;
    }
    return result;
}

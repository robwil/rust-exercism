use scrabble_score::*;
use std::collections::HashMap;

#[test]
fn a_is_worth_one_point() {
    assert_eq!(score("a"), 1);
}

#[test]
fn scoring_is_case_insensitive() {
    assert_eq!(score("A"), 1);
}

#[test]
fn f_is_worth_four() {
    assert_eq!(score("f"), 4);
}

#[test]
fn two_one_point_letters_make_a_two_point_word() {
    assert_eq!(score("at"), 2);
}

#[test]
fn three_letter_word() {
    assert_eq!(score("zoo"), 12);
}

#[test]
fn medium_word() {
    assert_eq!(score("street"), 6);
}

#[test]
fn longer_words_with_valuable_letters() {
    assert_eq!(score("quirky"), 22);
}

#[test]
fn long_mixed_case_word() {
    assert_eq!(score("OxyphenButazone"), 41);
}

#[test]
fn non_english_scrabble_letters_do_not_score() {
    assert_eq!(score("pinata"), 8, "'n' should score 1");
    assert_eq!(score("piñata"), 7, "'ñ' should score 0");
}

#[test]
fn empty_words_are_worth_zero() {
    assert_eq!(score(""), 0);
}

#[test]
fn all_letters_work() {
    assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}

#[test]
fn german_letters_do_not_score() {
    assert_eq!(score("STRASSE"), 7, "\"SS\" should score 2");
    assert_eq!(score("STRAßE"), 5, "'ß' should score 0");
}

#[test]
fn score_with_word_multiplier() {
    assert_eq!(score("abc"), 7, "\"abc\" should score 7");
    let no_letter_multipliers = HashMap::new();
    assert_eq!(
        score_with_multipliers("abc", 2, &no_letter_multipliers),
        14,
        "\"abc\" with double word should score 14"
    );
    assert_eq!(
        score_with_multipliers("abc", 3, &no_letter_multipliers),
        21,
        "\"abc\" with triple word should score 21"
    );
}

#[test]
fn score_with_letter_multiplier() {
    assert_eq!(score("abc"), 7, "\"abc\" should score 7");
    let mut double_b = HashMap::new();
    double_b.insert(1, 2);
    assert_eq!(
        score_with_multipliers("abc", 1, &double_b),
        10,
        "\"abc\" with double B should score 10"
    );
    let mut double_a = HashMap::new();
    double_a.insert(0, 2);
    assert_eq!(
        score_with_multipliers("abc", 1, &double_a),
        8,
        "\"abc\" with double A should score 8"
    );
    let mut triple_c = HashMap::new();
    triple_c.insert(2, 3);
    assert_eq!(
        score_with_multipliers("abc", 1, &triple_c),
        13,
        "\"abc\" with double word should score 13"
    );
}

#[test]
fn score_with_word_and_letter_multiplier() {
    assert_eq!(score("abc"), 7, "\"abc\" should score 7");
    let mut double_b = HashMap::new();
    double_b.insert(1, 2);
    assert_eq!(
        score_with_multipliers("abc", 2, &double_b),
        20,
        "\"abc\" with double B and double word should score 20"
    );
}

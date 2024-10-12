use crate::rs_wordclass::Wordclass;


/// Function to check if the word at `current_index` has suffix `suffix` and is not yet tagged.
pub fn has_suffix(sentence: Vec<(&str, Wordclass)>, current_index: i32, suffix: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => word.ends_with(suffix),
        _ => false,
    }
}

/// Function to check if the word at `current_index` has suffix `suffix` and has been tagged.
pub fn f_has_suffix(sentence: Vec<(&str, Wordclass)>, current_index: i32, suffix: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => false,
        Some(&(word, _)) => word.ends_with(suffix),
        _ => false,
    }
}

/// Function to check if the word at `current_index` contains char `c` and is not yet tagged.
pub fn has_char(sentence: Vec<(&str, Wordclass)>, current_index: i32, c: char) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => word.contains(c),
        _ => false,
    }
}

/// Function to check if the word at `current_index` contains char `c` and is tagged.
pub fn f_has_char(sentence: Vec<(&str, Wordclass)>, current_index: i32, c: char) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => false,
        Some(&(word, _)) => word.contains(c),
        _ => false,
    }
}

#[test]
fn test_has_suffix_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(has_suffix(sentence.clone(), 1, "ick"));
    assert!(has_suffix(sentence.clone(), 2, "rown"));

}

#[test]
fn test_has_suffix_not_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(!has_suffix(sentence.clone(), 1, "ick"));
    assert!(!has_suffix(sentence.clone(), 2, "abcd"));

}


#[test]
fn test_has_f_suffix_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(f_has_suffix(sentence.clone(), 1, "ick"));
    assert!(f_has_suffix(sentence.clone(), 2, "rown"));

}

#[test]
fn test_has_f_suffix_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!f_has_suffix(sentence.clone(), 1, "ick"));
    assert!(!f_has_suffix(sentence.clone(), 2, "abcd"));

}

#[test]
fn test_has_char_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::ANY),
        ("fox", Wordclass::ANY),
    ];
    assert!(has_char(sentence.clone(), 1, 'q'));
    assert!(has_char(sentence.clone(), 2, 'n'));

}

#[test]
fn test_has_char_not_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::ANY),
        ("fox", Wordclass::ANY),
    ];
    assert!(!has_char(sentence.clone(), 1, 'q'));
    assert!(!has_char(sentence.clone(), 2, 'k'));

}


#[test]
fn test_f_has_char_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::ANY),
    ];
    assert!(f_has_char(sentence.clone(), 1, 'q'));
    assert!(f_has_char(sentence.clone(), 2, 'n'));

}

#[test]
fn test_f_has_char_not_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::ANY),
    ];
    assert!(!f_has_char(sentence.clone(), 1, 'q'));
    assert!(!f_has_char(sentence.clone(), 2, 'k'));

}

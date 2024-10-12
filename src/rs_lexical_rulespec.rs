use crate::rs_wordclass::Wordclass;


/// Function to check if the tag at `current_index` - 1 is equal to `tag` in a sentence.
pub fn previous_tag(sentence: Vec<(&str, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    match sentence.get((current_index - 1) as usize) {
        Some((_, ref _tag)) if _tag == &tag => true,
        _ => false,
    }
}

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

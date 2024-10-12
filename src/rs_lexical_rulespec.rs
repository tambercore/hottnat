use crate::rs_wordclass::Wordclass;
use crate::WordclassMap;
use crate::initialize_tagger;

/// Function to check if the word at `current_index` has suffix `suffix` and is not yet tagged.
pub fn has_suffix(sentence: Vec<(&str, Wordclass)>, current_index: i32, suffix: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => word.ends_with(suffix),
        _ => false,
    }
}

/// Function to check if the word at `current_index` has suffix `suffix` and is tagged as `target_tag`.
pub fn f_has_suffix(sentence: Vec<(&str, Wordclass)>, current_index: i32, suffix: &str, target_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(word, ref tag)) => word.ends_with(suffix) && tag == &target_tag,
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
pub fn f_has_char(sentence: Vec<(&str, Wordclass)>, current_index: i32, c: char, target_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(word, ref tag)) => word.contains(c) && tag == &target_tag,
        _ => false,
    }
}

/// Function to check if the word at `current_index` is still a word if `suffix` is added, and is not yet tagged.
pub fn add_suffix(sentence: Vec<(&str, Wordclass)>, current_index: i32, suffix: &str, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => {
            let modified_word = word.to_string() + suffix;
            is_word_in_lexicon(modified_word.as_str(), wc_mapping)
        },
        _ => false,
    }
}

/// Function to check if the word at `current_index` is still a word if `suffix` is added, and is tagged.
pub fn f_add_suffix(sentence: Vec<(&str, Wordclass)>, current_index: i32, suffix: &str, target_tag: Wordclass, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(word, ref tag)) => {
            let modified_word = word.to_string() + suffix;
            is_word_in_lexicon(modified_word.as_str(), wc_mapping) && tag == &target_tag
        },
        _ => false,
    }
}

/// Function to check if the word at `current_index` is still a word if `suffix` is deleted, and is not yet tagged.
pub fn delete_suffix(sentence: Vec<(&str, Wordclass)>, current_index: i32, suffix: &str, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => {
            match word.strip_suffix(suffix) {
                Some(modified_word) => is_word_in_lexicon(modified_word, wc_mapping),
                _ => false
            }
        },
        _ => false,
    }
}

/// Function to check if the word at `current_index` is still a word if `suffix` is deleted, and is tagged.
pub fn f_delete_suffix(sentence: Vec<(&str, Wordclass)>, current_index: i32, suffix: &str, target_tag: Wordclass, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(word, ref tag)) => {
            match word.strip_suffix(suffix) {
                Some(modified_word) => is_word_in_lexicon(modified_word, wc_mapping) && tag == &target_tag,
                _ => false
            }
        },
        _ => false,
    }
}


/// Function to check if the word at `current_index` is still a word if `prefix` is deleted, and is not yet tagged.
pub fn delete_prefix(sentence: Vec<(&str, Wordclass)>, current_index: i32, prefix: &str, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => {
            match word.strip_prefix(prefix) {
                Some(modified_word) => is_word_in_lexicon(modified_word, wc_mapping),
                _ => false
            }
        },
        _ => false,
    }
}

/// Function to check if the word at `current_index` is still a word if `prefix` is deleted, and is tagged.
pub fn f_delete_prefix(sentence: Vec<(&str, Wordclass)>, current_index: i32, prefix: &str, target_tag: Wordclass, wc_mapping: &WordclassMap) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(word, ref tag)) => {
            match word.strip_prefix(prefix) {
                Some(modified_word) => is_word_in_lexicon(modified_word, wc_mapping) && tag == &target_tag,
                _ => false
            }
        },
        _ => false,
    }
}

/// Function to check if the word to the left of the word at `current_index` is `word` and is not yet tagged.
pub fn appears_to_left(sentence: Vec<(&str, Wordclass)>, current_index: i32, expected_word: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => {
            match sentence.get((current_index - 1) as usize) {
                Some(&(word, _)) => word == expected_word,
                _ => false,
            }
        }
        _ => false
    }

}

/// Function to check if the word to the left of the word at `current_index` is `word` and is tagged.
pub fn f_appears_to_left(sentence: Vec<(&str, Wordclass)>, current_index: i32, expected_word: &str, target_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(_, ref tag)) => {
            match sentence.get((current_index - 1) as usize) {

                Some(&(word, _)) => word == expected_word && tag == &target_tag,
                _ => false,
            }
        }
        _ => false
    }

}

/// Function to check if the word to the right of the word at `current_index` is `word` and is not yet tagged.
pub fn appears_to_right(sentence: Vec<(&str, Wordclass)>, current_index: i32, expected_word: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => {
            match sentence.get((current_index + 1) as usize) {
                Some(&(word, _)) => word == expected_word,
                _ => false,
            }
        }
        _ => false
    }

}

/// Function to check if the word to the right of the word at `current_index` is `word` and is tagged.
pub fn f_appears_to_right(sentence: Vec<(&str, Wordclass)>, current_index: i32, expected_word: &str, target_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(_, ref tag)) => {
            match sentence.get((current_index + 1) as usize) {

                Some(&(word, _)) => word == expected_word && tag == &target_tag,
                _ => false,
            }
        }
        _ => false
    }

}


/// Function to check if `word` appears in the Wordclass mappings retrieved from the lexicon.
pub fn is_word_in_lexicon(word: &str, wc_mapping: &WordclassMap) -> bool {
    match wc_mapping.get(word) {
        Some(_) => true,
        _ => false
    }
}

/*
/// Checks a given lexical rule.
pub fn lexical_rule_holds(sentence: Vec<(&str, Wordclass)>, current_index: i32, rule: LexicalRulespec) -> Option<bool> {

    match rule.ruleset_id {
        /*RulespecID::PREVTAG => {
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Ok(_wordclass) => { Option::from(previous_tag(sentence, current_index, _wordclass)) }
                Err(_) => { Option::from(false) }
            }
        },*/
        LexicalRuleID::FHASSUF => {
            let suffix: &str = rule.parameters.get()
            has_suffix(sentence, current_index, suffix);

            /*
            let param_original = rule.parameters.get(0)?;
            let param_wordclass = map_pos_tag(param_original);
            match param_wordclass {
                Ok(_wordclass) => { Option::from(f_has_suffix(sentence, current_index, _wordclass)) }
                Err(_) => { Option::from(false) }
            }*/
        }
        LexicalRuleID::FCHAR => {}
        LexicalRuleID::HASSUF => {}
        LexicalRuleID::ADDSUF => {}
        LexicalRuleID::FGOODRIGHT => {}
        LexicalRuleID::DELETEPREF => {}
        LexicalRuleID::FGOODLEFT => {}
        LexicalRuleID::GOODLEFT => {}
        LexicalRuleID::GOODRIGHT => {}
        LexicalRuleID::FDELETESUF => {}
        LexicalRuleID::CHAR => {}
        LexicalRuleID::FDELETEPREF => {}
        LexicalRuleID::FADDSUF => {}
    }
}


/// Applies a given lexical rule.
pub fn lexical_rule_apply(sentence: &mut Vec<(&str, Wordclass)>, current_index: i32, rule: LexicalRulespec) -> Option<bool> {
    // Check if Contextual Rule can be run
    let uindex: usize = current_index as usize;
    let check_pair = sentence.get(uindex)?;
    if check_pair.1 != rule.source_tag {
        return Option::from(false);
    }

    // Run Contextual Rule
    match crate::rs_contextual_rulespec::contextual_rule_holds(sentence.to_owned(), current_index, rule.clone()) {
        Some(true) => {
            let new_tag = rule.clone().target_tag;
            sentence[uindex] = (sentence[uindex].0, new_tag);
            Option::from(true)
        }
        _ => Option::from(false),
    }
}
*/
#[test]
fn test_appears_to_left_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(appears_to_left(sentence.clone(), 1, "The"));
    assert!(appears_to_left(sentence.clone(), 2, "quick"));

}

#[test]
fn test_appears_to_left_not_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(!appears_to_left(sentence.clone(), 1, "The"));
    assert!(!appears_to_left(sentence.clone(), 2, "none"));

}


#[test]
fn test_f_appears_to_left_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(f_appears_to_left(sentence.clone(), 1, "The", Wordclass::JJ));
    assert!(f_appears_to_left(sentence.clone(), 2, "quick", Wordclass::JJ));

}

#[test]
fn test_f_appears_to_left_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quickest", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!f_appears_to_left(sentence.clone(), 1, "The", Wordclass::ANY));
    assert!(!f_appears_to_left(sentence.clone(), 2, "none", Wordclass::JJ));

}

#[test]
fn test_appears_to_right_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(appears_to_right(sentence.clone(), 1, "brown"));
    assert!(appears_to_right(sentence.clone(), 2, "lazy"));

}

#[test]
fn test_appears_to_right_not_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(!appears_to_right(sentence.clone(), 1, "brown"));
    assert!(!appears_to_right(sentence.clone(), 2, "none"));

}


#[test]
fn test_f_appears_to_right_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(f_appears_to_right(sentence.clone(), 1, "brown", Wordclass::JJ));
    assert!(f_appears_to_right(sentence.clone(), 2, "fox", Wordclass::JJ));

}

#[test]
fn test_f_appears_to_right_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!f_appears_to_right(sentence.clone(), 1, "quick", Wordclass::ANY));
    assert!(!f_appears_to_right(sentence.clone(), 2, "none", Wordclass::JJ));

}

#[test]
fn test_delete_suffix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quickest", Wordclass::ANY),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(delete_suffix(sentence.clone(), 1, "est", &wc_mapping));
    assert!(delete_suffix(sentence.clone(), 2, "n", &wc_mapping));

}

#[test]
fn test_delete_suffix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quickest", Wordclass::JJ),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(!delete_suffix(sentence.clone(), 1, "est", &wc_mapping));
    assert!(!delete_suffix(sentence.clone(), 2, "own", &wc_mapping));

}


#[test]
fn test_delete_f_suffix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quickest", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(f_delete_suffix(sentence.clone(), 1, "est", Wordclass::JJ, &wc_mapping));
    assert!(f_delete_suffix(sentence.clone(), 2, "n", Wordclass::JJ, &wc_mapping));

}

#[test]
fn test_delete_f_suffix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quickest", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!f_delete_suffix(sentence.clone(), 1, "est", Wordclass::ANY, &wc_mapping));
    assert!(!f_delete_suffix(sentence.clone(), 2, "own", Wordclass::JJ, &wc_mapping));

}

#[test]
fn test_delete_prefix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("unquick", Wordclass::ANY),
        ("unbrown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(delete_prefix(sentence.clone(), 1, "un", &wc_mapping));
    assert!(delete_prefix(sentence.clone(), 2, "un", &wc_mapping));

}

#[test]
fn test_delete_prefix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("unquick", Wordclass::JJ),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(!delete_prefix(sentence.clone(), 1, "un", &wc_mapping));
    assert!(!delete_prefix(sentence.clone(), 2, "aaa", &wc_mapping));
    assert!(!delete_prefix(sentence.clone(), 2, "bro", &wc_mapping));

}


#[test]
fn test_delete_f_prefix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::DT),
        ("unquick", Wordclass::JJ),
        ("unbrown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(f_delete_prefix(sentence.clone(), 1, "un", Wordclass::JJ, &wc_mapping));
    assert!(f_delete_prefix(sentence.clone(), 2, "un", Wordclass::JJ, &wc_mapping));

}

#[test]
fn test_delete_f_prefix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::DT),
        ("unquick", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!f_delete_prefix(sentence.clone(), 1, "un", Wordclass::ANY, &wc_mapping));
    assert!(!f_delete_prefix(sentence.clone(), 2, "zzz", Wordclass::JJ, &wc_mapping));
    assert!(!f_delete_prefix(sentence.clone(), 2, "bro", Wordclass::JJ, &wc_mapping));

}


#[test]
fn test_add_suffix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(add_suffix(sentence.clone(), 1, "est", &wc_mapping));
    assert!(add_suffix(sentence.clone(), 2, "ed", &wc_mapping));

}

#[test]
fn test_add_suffix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(!add_suffix(sentence.clone(), 1, "est", &wc_mapping));
    assert!(!add_suffix(sentence.clone(), 2, "zzz", &wc_mapping));

}


#[test]
fn test_add_f_suffix_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(f_add_suffix(sentence.clone(), 1, "est", Wordclass::JJ, &wc_mapping));
    assert!(f_add_suffix(sentence.clone(), 2, "ed", Wordclass::JJ, &wc_mapping));

}

#[test]
fn test_add_f_suffix_not_found() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!f_add_suffix(sentence.clone(), 1, "est", Wordclass::ANY, &wc_mapping));
    assert!(!f_add_suffix(sentence.clone(), 2, "zzz", Wordclass::JJ, &wc_mapping));

}

#[test]
fn test_word_in_lexicon() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    assert!(is_word_in_lexicon("apple", &wc_mapping));
    assert!(is_word_in_lexicon("banana", &wc_mapping));
}

#[test]
fn test_word_not_in_lexicon() {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();
    assert!(!is_word_in_lexicon("abcde", &wc_mapping));
    assert!(!is_word_in_lexicon("", &wc_mapping));
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
    assert!(f_has_suffix(sentence.clone(), 1, "ick", Wordclass::JJ));
    assert!(f_has_suffix(sentence.clone(), 2, "rown", Wordclass::JJ));

}

#[test]
fn test_has_f_suffix_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!f_has_suffix(sentence.clone(), 1, "ick", Wordclass::ANY));
    assert!(!f_has_suffix(sentence.clone(), 2, "abcd", Wordclass::JJ));

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
    assert!(f_has_char(sentence.clone(), 1, 'q', Wordclass::JJ));
    assert!(f_has_char(sentence.clone(), 2, 'n', Wordclass::JJ));

}

#[test]
fn test_f_has_char_not_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::ANY),
    ];
    assert!(!f_has_char(sentence.clone(), 1, 'q', Wordclass::ANY));
    assert!(!f_has_char(sentence.clone(), 2, 'k', Wordclass::JJ));
}

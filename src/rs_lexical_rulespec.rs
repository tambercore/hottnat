use crate::rs_wordclass::{map_pos_tag, Wordclass};
use crate::WordclassMap;
use crate::initialize_tagger;
use crate::rs_lex_rulespec_id::{LexicalRuleID, LexicalRulespec};


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


/// Function to check if the word at `current_index` has suffix `prefix` and is not yet tagged.
pub fn has_prefix(sentence: Vec<(&str, Wordclass)>, current_index: i32, prefix: &str) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(word, Wordclass::ANY)) => word.starts_with(prefix),
        _ => false,
    }
}


/// Function to check if the word at `current_index` has suffix `prefix` and is tagged as `target_tag`.
pub fn f_has_prefix(sentence: Vec<(&str, Wordclass)>, current_index: i32, prefix: &str, target_tag: Wordclass) -> bool {
    match sentence.get(current_index as usize) {
        Some(&(_, Wordclass::ANY)) => false,
        Some(&(word, ref tag)) => word.starts_with(prefix) && tag == &target_tag,
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


/// Checks a given lexical rule.
pub fn lexical_rule_holds(sentence: Vec<(&str, Wordclass)>, current_index: i32, rule: LexicalRulespec, wc_mapping: &WordclassMap) -> Option<bool> {

    match rule.ruleset_id {
        LexicalRuleID::HASSUF => {

            let suffix: &str = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(has_suffix(sentence, current_index, suffix)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FCHAR => {
            let c = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(f_has_char(sentence, current_index, c.parse().unwrap(), _wordclass)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::ADDSUF => {
            let suffix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(add_suffix(sentence, current_index, suffix, wc_mapping)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FGOODRIGHT => {
            let expected_word = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(f_appears_to_right(sentence, current_index, expected_word, _wordclass)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::DELETEPREF => {
            let prefix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(delete_prefix(sentence, current_index, prefix, wc_mapping)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FGOODLEFT => {
            let expected_word = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(f_appears_to_left(sentence, current_index, expected_word, _wordclass)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::GOODLEFT => {
            let expected_word = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(appears_to_left(sentence, current_index, expected_word)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::GOODRIGHT => {
            let expected_word = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(appears_to_right(sentence, current_index, expected_word)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FDELETESUF => {
            let suffix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(f_delete_suffix(sentence, current_index, suffix, _wordclass, wc_mapping)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::CHAR => {
            let c = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(has_char(sentence, current_index, c.parse().unwrap())) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FDELETEPREF => {
            let prefix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(f_delete_prefix(sentence, current_index, prefix, _wordclass, wc_mapping)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FADDSUF => {
            let suffix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(f_add_suffix(sentence, current_index, suffix, _wordclass, wc_mapping)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FHASSUF => {
            let suffix: &str = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(f_has_suffix(sentence, current_index, suffix, _wordclass)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::FHASPREF => {
            let suffix: &str = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(f_has_prefix(sentence, current_index, suffix, _wordclass)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
        LexicalRuleID::DELETESUF => {
            let suffix = rule.parameters.get(1)?;
            let source_tag= rule.parameters.get(0)?;
            let source_tag_wc = map_pos_tag(source_tag);

            match source_tag_wc {
                Ok(_wordclass) => { Option::from(delete_suffix(sentence, current_index, suffix, wc_mapping)) }
                Err(_) => {
                    Option::from(false)
                }
            }
        }
    }
}


/// Applies a given lexical rule.
pub fn lexical_rule_apply(sentence: &mut Vec<(&str, Wordclass)>, current_index: i32, rule: LexicalRulespec) -> Option<bool> {
    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();

    let uindex: usize = current_index as usize;

    // Run Lexical Rule
    match lexical_rule_holds(sentence.to_owned(), current_index, rule.clone(), &wc_mapping) {
        Some(true) => {
            let new_tag = rule.clone().target_tag;
            sentence[uindex] = (sentence[uindex].0, new_tag);
            Option::from(true)
        }
        _ => Option::from(false),
    }
}


#[test]
fn test_lexical_rule_apply() {

    let mut sentence_untagged = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];

    let rule_hassuf = LexicalRulespec {
        ruleset_id: LexicalRuleID::HASSUF,
        target_tag: Wordclass::JJ,
        parameters: vec![String::from("''"), "ick".parse().unwrap()],
    };

    assert!(lexical_rule_apply(&mut sentence_untagged, 1, rule_hassuf).unwrap());


}


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
fn test_has_prefix_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(has_prefix(sentence.clone(), 1, "qui"));
    assert!(has_prefix(sentence.clone(), 2, "bro"));

}


#[test]
fn test_has_prefix_not_found() {
    let sentence = vec![
        ("The", Wordclass::ANY),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::ANY),
        ("lazy", Wordclass::ANY),
        ("dog", Wordclass::ANY),
    ];
    assert!(!has_prefix(sentence.clone(), 1, "qui"));
    assert!(!has_prefix(sentence.clone(), 2, "abcd"));

}


#[test]
fn test_has_f_prefix_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(f_has_prefix(sentence.clone(), 1, "qui", Wordclass::JJ));
    assert!(f_has_prefix(sentence.clone(), 2, "bro", Wordclass::JJ));

}


#[test]
fn test_has_f_prefix_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::ANY),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!f_has_prefix(sentence.clone(), 1, "qui", Wordclass::ANY));
    assert!(!f_has_prefix(sentence.clone(), 2, "abcd", Wordclass::JJ));

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
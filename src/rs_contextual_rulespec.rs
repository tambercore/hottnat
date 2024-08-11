use std::fmt;
use crate::rs_rulespec_id::RulespecID;
use crate::rs_wordclass::Wordclass;



/// Function to check if the tag at index - 1 is equal to `tag` in a sentence.
pub fn previous_tag(sentence: Vec<(&str, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    match sentence.get((current_index - 1) as usize) {
        Some((_, ref _tag)) if _tag == &tag => true,
        _ => false,
    }
}



/// Function to check if the word at index - 1 is equal to `word` in a sentence.
pub fn previous_word(sentence: Vec<(&str, Wordclass)>, current_index: i32, word: &str) -> bool {
    match sentence.get((current_index - 1) as usize) {
        Some((_word, _)) if _word == &word => true,
        _ => false,
    }
}



/// Function to check if the tag at index - 1 or index - 2 is equal to `tag` in a sentence.
pub fn previous_one_or_two_tag(sentence: Vec<(&str, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    (1..=2).any(|offset| {
        sentence.get((current_index - offset) as usize).map_or(false, |&(_, ref _tag)| _tag == &tag)
    })
}



/// Function to check if the word at index - 1 or index - 2 or index - 3 is equal to `tag` in a sentence.
pub fn previous_one_or_two_or_three_tag(sentence: Vec<(&str, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    (1..=3).any(|offset| {
        sentence.get((current_index - offset) as usize).map_or(false, |&(_, ref _tag)| _tag == &tag)
    })
}



/// Function to check if the tag at index +1, +2 or +3 is equal to `tag` in a sentence.
pub fn next_one_or_two_or_three_tag(sentence: Vec<(&str, Wordclass)>, current_index: usize, tag: &Wordclass) -> bool {
    sentence.iter().skip(current_index + 1).take(3).any(|(_, t)| t == tag)
}



/// Function to check current word, and tag 2 words after.
pub fn word_and_tag_2_after(sentence: Vec<(&str, Wordclass)>, current_index: usize, word: &str, tag: &Wordclass) -> bool {
    if sentence.get(current_index).map_or(false, |(w, _)| w == &word) {
        sentence.get(current_index + 2).map_or(false, |(_, t)| t == tag)
    } else { false }
}



/// Function to check current word, and word 2 words after
pub fn word_and_2_after(sentence: Vec<(&str, Wordclass)>, current_index: usize, word_one: &str, word_two: &str) -> bool {
    if sentence.get(current_index).map_or(false, |(w1, _)| w1 == &word_one) {
        sentence.get(current_index + 2).map_or(false, |(w2, _)| w2 == &word_two)
    } else { false }
}



pub fn build_contextual_rule(rule: ContextualRulespec) {
    match rule.ruleset_id {
        RulespecID::PREVTAG => {}
        RulespecID::PREVWD => {}
        RulespecID::PREV1OR2TAG => {}
        RulespecID::PREV1OR2OR3TAG => {}
        RulespecID::NEXT1OR2OR3TAG => {}
        RulespecID::WDAND2TAGAFT => {}
        RulespecID::WDAND2AFT => {}
        RulespecID::PREV1OR2WD => {}
        RulespecID::NEXT1OR2TAG => {}
        RulespecID::NEXTTAG => {}
        RulespecID::PREV2TAG => {}
        RulespecID::NEXTWD => {}
        RulespecID::WDNEXTTAG => {}
        RulespecID::SURROUNDTAG => {}
        RulespecID::WDAND2TAGBFR => {}
        RulespecID::RBIGRAM => {}
        RulespecID::PREVBIGRAM => {}
        RulespecID::CURWD => {}
        RulespecID::WDPREVTAG => {}
        RulespecID::NEXTBIGRAM => {}
        RulespecID::NEXT2TAG => {}
        RulespecID::LBIGRAM => {}
    }
    
}



pub struct ContextualRulespec {
    pub source_tag: Wordclass,
    pub target_tag: Wordclass,
    pub ruleset_id: RulespecID,
    pub parameters: Vec<String>,
}



impl fmt::Display for ContextualRulespec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuleContextual {{{:?} -> {:?} if {} passes with parameters: [{}] }}",
               self.source_tag, self.target_tag, self.ruleset_id, self.parameters.join(", ")
        )
    }
}


#[test]
fn test_previous_one_or_two_tag_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(previous_one_or_two_tag(sentence.clone(), 3, Wordclass::JJ)); // at index -1
    assert!(previous_one_or_two_tag(sentence.clone(), 4, Wordclass::JJ)); // at index -2
}

#[test]
fn test_previous_one_or_two_tag_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_one_or_two_tag(sentence.clone(), 2, Wordclass::NN)); // Neither -1 nor -2
    assert!(!previous_one_or_two_tag(sentence.clone(), 1, Wordclass::NN)); // Neither -1 nor -2
}

#[test]
fn test_previous_one_or_two_tag_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
    ];
    assert!(!previous_one_or_two_tag(sentence.clone(), 1, Wordclass::NN)); // out of bounds at -2
    assert!(!previous_one_or_two_tag(sentence.clone(), 0, Wordclass::DT)); // out of bounds at -1
}

#[test]
fn test_previous_one_or_two_or_three_tag_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("lazy", Wordclass::JJ),
        ("dog", Wordclass::NN),
    ];
    assert!(previous_one_or_two_or_three_tag(sentence.clone(), 4, Wordclass::JJ)); // at index -1
    assert!(previous_one_or_two_or_three_tag(sentence.clone(), 5, Wordclass::JJ)); // at index -2 or -3
}

#[test]
fn test_previous_one_or_two_or_three_tag_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("lazy", Wordclass::JJ),
        ("dog", Wordclass::NN),
    ];
    assert!(!previous_one_or_two_or_three_tag(sentence.clone(), 3, Wordclass::NN)); // Neither -1, -2 nor -3
    assert!(!previous_one_or_two_or_three_tag(sentence.clone(), 2, Wordclass::NN)); // Neither -1, -2 nor -3
}

#[test]
fn test_previous_one_or_two_or_three_tag_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
    ];
    assert!(!previous_one_or_two_or_three_tag(sentence.clone(), 1, Wordclass::NN)); // out of bounds at -3
    assert!(!previous_one_or_two_or_three_tag(sentence.clone(), 0, Wordclass::DT)); // out of bounds at -2 and -3
}

#[test]
fn test_next_one_or_two_or_three_tag_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("lazy", Wordclass::JJ),
        ("dog", Wordclass::NN),
    ];
    assert!(next_one_or_two_or_three_tag(sentence.clone(), 1, &Wordclass::JJ)); // at index +1
    assert!(next_one_or_two_or_three_tag(sentence.clone(), 0, &Wordclass::JJ)); // at index +2 or +3
}

#[test]
fn test_next_one_or_two_or_three_tag_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("lazy", Wordclass::JJ),
        ("dog", Wordclass::JJ),
    ];
    assert!(!next_one_or_two_or_three_tag(sentence.clone(), 2, &Wordclass::NN)); // Neither +1, +2 nor +3
    assert!(!next_one_or_two_or_three_tag(sentence.clone(), 3, &Wordclass::NN)); // Neither +1, +2 nor +3
}

#[test]
fn test_next_one_or_two_or_three_tag_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
    ];
    assert!(!next_one_or_two_or_three_tag(sentence.clone(), 2, &Wordclass::NN)); // out of bounds at +3
    assert!(!next_one_or_two_or_three_tag(sentence.clone(), 1, &Wordclass::NN)); // out of bounds at +2 and +3
}

#[test]
fn test_word_and_tag_2_after_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
        ("jumps", Wordclass::VB),
    ];
    assert!(word_and_tag_2_after(sentence.clone(), 0, "The", &Wordclass::JJ)); // The with JJ 2 words after
    assert!(word_and_tag_2_after(sentence.clone(), 1, "quick", &Wordclass::NN)); // quick with NN 2 words after
}

#[test]
fn test_word_and_tag_2_after_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
        ("jumps", Wordclass::VB),
    ];
    assert!(!word_and_tag_2_after(sentence.clone(), 0, "The", &Wordclass::NN)); // The without NN 2 words after
    assert!(!word_and_tag_2_after(sentence.clone(), 1, "quick", &Wordclass::VB)); // quick without VB 2 words after
}

#[test]
fn test_word_and_tag_2_after_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
    ];
    assert!(!word_and_tag_2_after(sentence.clone(), 2, "brown", &Wordclass::NN)); // out of bounds at +2
    assert!(!word_and_tag_2_after(sentence.clone(), 1, "quick", &Wordclass::NN)); // out of bounds at +2
}

#[test]
fn test_word_and_2_after_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
        ("jumps", Wordclass::VB),
    ];
    assert!(word_and_2_after(sentence.clone(), 0, "The", "brown")); // The with "brown" 2 words after
    assert!(word_and_2_after(sentence.clone(), 1, "quick", "fox")); // quick with "fox" 2 words after
}

#[test]
fn test_word_and_2_after_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
        ("jumps", Wordclass::VB),
    ];
    assert!(!word_and_2_after(sentence.clone(), 0, "The", "fox")); // The without "fox" 2 words after
    assert!(!word_and_2_after(sentence.clone(), 1, "quick", "jumps")); // quick without "jumps" 2 words after
}

#[test]
fn test_word_and_2_after_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
    ];
    assert!(!word_and_2_after(sentence.clone(), 1, "quick", "brown")); // out of bounds at +2
    assert!(!word_and_2_after(sentence.clone(), 0, "The", "quick")); // out of bounds at +2
}

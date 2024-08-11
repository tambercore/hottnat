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
fn test_previous_tag_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(previous_tag(sentence.clone(), 2, Wordclass::JJ));
    assert!(previous_tag(sentence.clone(), 3, Wordclass::JJ));
}



#[test]
fn test_previous_tag_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_tag(sentence.clone(), 3, Wordclass::NN));
    assert!(!previous_tag(sentence.clone(), 1, Wordclass::NN));
}



#[test]
fn test_previous_word_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(previous_word(sentence.clone(), 1, "The"));
    assert!(previous_word(sentence.clone(), 3, "brown"));
}



#[test]
fn test_previous_word_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_word(sentence.clone(), 2, "fox"));
    assert!(!previous_word(sentence.clone(), 1, "fox"));
}



#[test]
fn test_previous_word_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_word(sentence.clone(), 0, "anything"));
}



#[test]
fn test_previous_tag_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_tag(sentence.clone(), 0, Wordclass::NN));
}
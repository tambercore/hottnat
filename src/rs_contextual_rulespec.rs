use std::fmt;
use crate::rs_rulespec_id::RulespecID;
use crate::rs_wordclass::Wordclass;

pub fn previous_tag(sentence: Vec<(&str, Wordclass)>, current_index: i32, tag: Wordclass) -> bool {
    match sentence.get((current_index - 1) as usize) {
        Some((_, ref _tag)) if _tag == &tag => true,
        _ => false,
    }
}

pub fn previous_word(sentence: Vec<(&str, Wordclass)>, current_index: i32, word: &str) -> bool {
    match sentence.get((current_index - 1) as usize) {
        Some((_word, _)) if _word == &word => true,
        _ => false,
    }
}

pub fn build_contextual_rule(rule: ContextualRulespec) -> () {
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
    return ()
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
    assert!(previous_tag(sentence.clone(), 2, Wordclass::JJ));  // "quick" has tag JJ
    assert!(previous_tag(sentence.clone(), 3, Wordclass::JJ));  // "brown" has tag JJ
}

#[test]
fn test_previous_tag_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_tag(sentence.clone(), 3, Wordclass::NN));  // "brown" doesn't have tag NN
    assert!(!previous_tag(sentence.clone(), 1, Wordclass::NN));  // "The" doesn't have a previous word
}

#[test]
fn test_previous_word_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(previous_word(sentence.clone(), 1, "The"));   // "quick" has previous word "The"
    assert!(previous_word(sentence.clone(), 3, "brown"));  // "fox" has previous word "brown"
}

#[test]
fn test_previous_word_not_found() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_word(sentence.clone(), 2, "fox"));  // "quick" doesn't have previous word "fox"
    assert!(!previous_word(sentence.clone(), 1, "fox"));  // "The" doesn't have a previous word
}

#[test]
fn test_previous_word_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_word(sentence.clone(), 0, "anything"));  // Out of bounds, no previous word
}

#[test]
fn test_previous_tag_out_of_bounds() {
    let sentence = vec![
        ("The", Wordclass::DT),
        ("quick", Wordclass::JJ),
        ("brown", Wordclass::JJ),
        ("fox", Wordclass::NN),
    ];
    assert!(!previous_tag(sentence.clone(), 0, Wordclass::NN));  // Out of bounds, no previous tag
}


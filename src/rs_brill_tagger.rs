use std::collections::HashMap;
use itertools::{enumerate, Itertools};
use crate::rs_contextual_ruleset::parse_contextual_ruleset;
use crate::rs_contextual_rulespec::{contextual_rule_apply, ContextualRulespec};
use crate::rs_wordclass::Wordclass;
use crate::{initialize_tagger, WordclassMap};

fn tag_sentence(sentence: &str) {
    //let lexical_ruleset: Vec<LexicalRulespec> = parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap();
    let contextual_ruleset: HashMap<Wordclass, Vec<ContextualRulespec>> = parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap();

    let wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();

    println!("sentence: {}\n", sentence);

    // Match each word with its list of possible tags
    let words_to_tags: Vec<(&str, Vec<Wordclass>)> = sentence
        .split_whitespace()
        .enumerate()
        .map(|(_, word)| (word, wc_mapping.get(word).ok_or(format!("Word not in lexicon: {}", word)).unwrap().to_owned()))
        .collect();

    println!("possible tags: {:?}\n", words_to_tags);

    // Create a vector of tuples: strings to the first element in the list
    let mut sentence_to_tag: Vec<(&str, Wordclass)> = words_to_tags
        .iter()
        .filter_map(|(word, tags)| tags.first().map(|first_tag| (*word, first_tag.clone())))
        .collect();

    println!("sentence to tag: {:?}", sentence_to_tag);


    println!("valid contextual rules:\n");

    loop {
        let mut rules_applied = 0;
        for (index, (_, tag)) in enumerate(sentence_to_tag.clone()) {
            let valid_rules = contextual_ruleset.get(&tag);
            match valid_rules {
                Some(_valid_rules) => {
                    for rule in _valid_rules {
                        match contextual_rule_apply(&mut sentence_to_tag, index as i32, rule.clone()) {
                            None => {}
                            Some(false) => {}
                            Some(true) => {
                                println!("RuleContextual (word {:?} -> {} if {} passes with parameters {:?})", &sentence_to_tag.get(index), rule.target_tag, rule.ruleset_id, rule.parameters);
                                rules_applied +=1;
                            }
                        }
                    }
                }
                None => continue // some Wordclasses have no associated rules (e.g. CC). In this case, the tag is kept.
            }

        }

        // check if each word in sentence_to_tag's corresponding tag is in its corresponding tag vector in words_to_tags
        let all_tags_valid = sentence_to_tag.iter().all(|(word, tag)| {
            if let Some(possible_tags) = words_to_tags.iter().find(|(w, _)| *w == *word) {
                possible_tags.1.contains(tag)
            } else {
                false
            }
        });
        if all_tags_valid || rules_applied == 0 {break;}


    }

    println!("final sentence: {:?}", sentence_to_tag);

}

#[test]
fn test_tag_sentence() {
    tag_sentence("It is now some years since I detected how many were the false
beliefs that I had from my earliest youth admitted as true");
}
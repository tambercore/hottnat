use std::collections::HashMap;
use itertools::{enumerate, Itertools};
use crate::rs_contextual_ruleset::parse_contextual_ruleset;
use crate::rs_contextual_rulespec::{contextual_rule_apply, ContextualRulespec};
use crate::rs_wordclass::Wordclass;
use crate::{initialize_tagger, WordclassMap};
use crate::rs_contractions::find_contractions;
use crate::rs_lex_rulespec_id::LexicalRulespec;
use crate::rs_lexical_ruleset::parse_lexical_ruleset;
//use crate::rs_lexical_ruleset::parse_lexical_ruleset;
use crate::rs_lexical_rulespec::lexical_rule_apply;

fn tag_sentence(sentence: &str) -> bool {
    let lexical_ruleset: Vec<LexicalRulespec> = parse_lexical_ruleset("data/rulefile_lexical.txt").unwrap();
    let contextual_ruleset: HashMap<Wordclass, Vec<ContextualRulespec>> = parse_contextual_ruleset("data/rulefile_contextual.txt").unwrap();
    let mut wc_mapping: WordclassMap = initialize_tagger("data/lexicon.txt").unwrap();

    println!("sentence: {}\n", sentence);

    let tokenised_sentence = tokenize_sentence(sentence);
    println!("tokenised sentence: {:?}", tokenised_sentence);

    // Match each word with its list of possible tags
    let words_to_tags: Vec<(String, Vec<Wordclass>)> = get_possible_tags(tokenised_sentence, &mut wc_mapping);
    println!("possible tags: {:?}\n", words_to_tags);

    // Create a vector of tuples: strings to the first element in the list
    let mut sentence_to_tag: Vec<(String, Wordclass)> = retrieve_sentence_to_tag(words_to_tags.clone());
    println!("sentence to tag: {:?}", sentence_to_tag);

    // Apply lexical rules.
    println!("applying lexical rules\n");
    apply_lexical_rules(&mut sentence_to_tag, &lexical_ruleset, &words_to_tags, &wc_mapping);

    // Apply contextual rules.
    println!("applying contextual rules:\n");
    let result = apply_contextual_rules(&mut sentence_to_tag, &words_to_tags, &contextual_ruleset, 100);

    match result {
        Ok(_) => {}
        Err(_) => {
            println!("max iterations reached");
            return false;}
    }

    println!("final sentence: {:?}", sentence_to_tag);

    return true;

}


/// Apply lexical rules to a sentence `sentence_to_tag`
fn apply_lexical_rules(sentence_to_tag: &mut Vec<(String, Wordclass)>, lexical_ruleset: &Vec<LexicalRulespec>, possible_tags: &Vec<(String, Vec<Wordclass>)>, wc_mapping: &WordclassMap) {
    for (index, (word, _)) in enumerate(sentence_to_tag.clone()) {
        for rule in lexical_ruleset {

            if !is_tag_contained_in_word_possible_tags(&possible_tags, &word, &rule.target_tag) {continue;}

            match lexical_rule_apply(sentence_to_tag, index as i32, rule, wc_mapping) {
                Some(true) => {
                    println!("RuleLexical (word {:?} -> {} if {} passes with parameters {:?})", &sentence_to_tag.get(index), rule.target_tag, rule.ruleset_id, rule.parameters);
                }
                _ => {}
            }
        }
    }
}


/// Continuously apply contextual rules to a sentence `sentence_to_tag` until each word's tag is in `possible_tags` or no rules were applied.
fn apply_contextual_rules(sentence_to_tag: &mut Vec<(String, Wordclass)>, possible_tags: &Vec<(String, Vec<Wordclass>)>, contextual_ruleset: &HashMap<Wordclass, Vec<ContextualRulespec>>, threshold:i32) -> Result<String, String> {

    let mut iterations = 0;
    loop {
        for (index, (word, tag)) in enumerate(sentence_to_tag.clone()) {
            let valid_rules = contextual_ruleset.get(&tag);
            match valid_rules {
                Some(_valid_rules) => {
                    for rule in _valid_rules {

                        if !is_tag_contained_in_word_possible_tags(possible_tags, &word, &rule.target_tag) {continue;}

                        match contextual_rule_apply(sentence_to_tag, index as i32, rule.clone()) {
                            Some(true) => {
                                println!("RuleContextual (word {:?} with tag {:?} -> {:?} if {} passes with parameters {:?})", &sentence_to_tag.get(index).unwrap().0, tag, rule.target_tag, rule.ruleset_id, rule.parameters);
                            }
                            _ => {}
                        }
                    }
                }
                None => continue // some Wordclasses have no associated rules (e.g. CC). In this case, the tag is kept.
            }



        }

        // check if each word in sentence_to_tag's corresponding tag is in its corresponding tag vector in words_to_tags
        let all_tags_valid = sentence_to_tag.iter().all(|(word, tag)| {
            if let Some(possible_tags) = possible_tags.iter().find(|(w, _)| w == word)  {
                // The assigned tag must be in the word's possible tags.
                // If the word can be ANY, then any tag is valid.
                // The tag CANNOT be ANY (this means it is unassigned).
                (possible_tags.1.contains(tag) || possible_tags.1.contains(&Wordclass::ANY)) && *tag != Wordclass::ANY
            } else {
                false
            }
        });
        if all_tags_valid {return Ok(String::from("Sentence is valid"))}

        if(iterations == threshold) {return Err(String::from("Number of iterations exceeded in contextual rules."));}

        iterations +=1;
    }

}

/// Function to take a `sentence` (&str), split whitespace and tokenize any contractions.
fn tokenize_sentence(sentence: &str) -> Vec<String> {
    sentence.split_whitespace().
        map(|word|find_contractions(String::from(word)).unwrap())
        .flatten()
        .collect()
}

/// Function to: given a tokenized `sentence` and mapping `wc_mapping`, retrieve the possible tags for each word.
fn get_possible_tags(sentence: Vec<String>, wc_mapping: &mut WordclassMap) -> Vec<(String, Vec<Wordclass>)> {
    sentence.iter()
        .map(|word| (word.as_str().to_owned(), wc_mapping.entry(word.as_str().parse().unwrap()).or_insert(vec![Wordclass::ANY]).to_owned()))
        .collect()
}

/// Function to alter the first tag of the word's possible tags. Retrieve this tag for each word.
fn retrieve_sentence_to_tag(sentence: Vec<(String, Vec<Wordclass>)>) -> Vec<(String, Wordclass)> {
    sentence
        .iter()
        .filter_map(|(word, tags)| tags.first().map(|first_tag| (word.to_owned(), first_tag.clone()))).collect()
}

/// Function to check if `possible_tag`s of a given `word` contain `target_tag`.
fn is_tag_contained_in_word_possible_tags(possible_tags: &Vec<(String, Vec<Wordclass>)>, word: &String, target_tag: &Wordclass) -> bool {
    let possible_tags_for_word =     possible_tags.iter()
        .find(|(first, _)| first == word) // Find the tuple where the first element matches `key`
        .map(|(_, second)| second).unwrap(); // Map to the second element

    possible_tags_for_word.contains(target_tag)
}
#[test]
fn test_tag_sentence() {
    assert!(tag_sentence("i want to rock and roll every night"));
    //assert!(tag_sentence("the quick fox jumped over the lazy dog"))
}
mod rs_wordclass;
mod rs_contextual_ruleset;
mod rs_rulespec_id;
mod rs_contextual_rulespec;
mod rs_lex_rulespec_id;
mod rs_lexical_ruleset;
mod rs_lexical_rulespec;
mod rs_contractions;
mod rs_benchmark;
mod rs_brill_tagger;
mod rs_conllu_parser;
mod rs_ccg;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use rs_wordclass::*;
use rs_contextual_rulespec::*;
use rs_contextual_ruleset::*;
use rs_benchmark::benchmark_pos_tagger;
use crate::rs_ccg::test_ccg;
use crate::rs_lex_rulespec_id::LexicalRulespec;
use crate::rs_lexical_ruleset::parse_lexical_ruleset;

type WordclassMap = HashMap<String, Vec<Wordclass>>;



/// ...
pub fn initialize_tagger(path: &str) -> Result<WordclassMap, io::Error>
{
    // This attempts to read the file, then creates an instance of the WordclassMap.
    let contents = fs::read_to_string(path)?;
    let mut tagger: WordclassMap = HashMap::new();

    // Defining a small function to map a vector of strings to a vector of wordclass enumerations.
    // This function invokes the above `map_pos_tag` function on each element of the original vector.
    // E.g. the vector of strings: {"WP$", "VBZ"} would map to {`Wordclass::WPO`, `Wordlass::VBZ`}
    fn process_tags(tags: Vec<String>) -> Vec<Option<Wordclass>> {
        tags.into_iter().map(|tag| map_pos_tag(&tag)).collect::<Vec<Option<Wordclass>>>()
    }

    // Here, a type `LineFunction` is declared, to process a row of the lexicon into the `WordclassMap`.
    // This function `process_line` splits the word literal (key) from its potential wordclasses (value).
    // E.g. the string 'beans NN' maps the word to its wordclasses ('beans NN' → 'beans': [Wordclass::NN]).
    type LineFunction = fn(&mut WordclassMap, Vec<&str>);
    let process_line: LineFunction = |tagger, parts| {
        let string_vector: Vec<Wordclass> = process_tags(parts[1..].iter().map(|&s| s.to_string()).collect())
            .into_iter()
            .filter_map(|opt| opt) // Filter out None values
            .collect(); // Collect only Some values into a new vector
        tagger.insert(parts[0].to_string(), string_vector);
    };

    // Lastly, each line is processed into the wordclass mapping using the `process_line` function.
    contents.lines().for_each(|ln| process_line(&mut tagger, ln.split_whitespace().collect()));
    Ok(tagger)
}



fn format_vec(wordclasses: &Vec<Wordclass>) -> String {
    let wordclass_str: Vec<String> = wordclasses.iter().map(|wc| wc.to_string()).collect();
    wordclass_str.join(", ")
}



fn main() -> io::Result<()> {

    test_ccg();

    Ok(())
}

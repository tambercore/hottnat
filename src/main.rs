mod rs_wordclass;
mod rs_contextual_ruleset;
mod rs_rulespec_id;
mod rs_contextual_rulespec;
mod rs_lex_rulespec_id;
mod rs_lexical_ruleset;
mod rs_lexical_rulespec;

use std::collections::HashMap;
use std::fs;
use std::io::{self, Error, Write};
use rs_wordclass::*;
use rs_contextual_rulespec::*;
use rs_contextual_ruleset::*;

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
    fn process_tags(tags: Vec<String>) -> Result<Vec<Wordclass>, Error> {
        tags.into_iter().map(|tag| map_pos_tag(&tag)).collect::<Result<Vec<Wordclass>, Error>>()
    }

    // Here, a type `LineFunction` is declared, to process a row of the lexicon into the `WordclassMap`.
    // This function `process_line` splits the word literal (key) from its potential wordclasses (value).
    // E.g. the string 'beans NN' maps the word to its wordclasses ('beans NN' → 'beans': [Wordclass::NN]).
    type LineFunction = fn(&mut WordclassMap, Vec<&str>);
    let process_line: LineFunction = |tagger, parts| {
        let string_vector: Result<Vec<Wordclass>, Error> = process_tags(parts[1..].iter().map(|&s| s.to_string()).collect::<Vec<String>>());
        match string_vector {
            Ok(wordclass_vector) => tagger.insert(parts[0].to_string(), wordclass_vector),
            Err(_) => tagger.insert(parts[0].to_string(), Vec::new())
        };
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

    let contextual_ruleset: HashMap<Wordclass, Vec<ContextualRulespec>> = parse_contextual_ruleset("data/rulefile_contextual.txt")?;
    //let lexical_ruleset: Vec<LexicalRulespec> = parse_lexical_ruleset("data/rulefile_lexical.txt")?;

    let tagger: WordclassMap = initialize_tagger("data/lexicon.txt")?;

    loop {
        print!("Enter a word: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let word = input.trim();
        if word.is_empty() {
            break;
        }

        match tagger.get(word) {
            Some(tags) => {
                println!("Associated list: {}", format_vec(tags));
                for t in tags {
                    let values_contextual = contextual_ruleset.get(t);

                    println!("CONTEXTUAL RULES");

                    match values_contextual{
                        None => {}
                        Some(_) => {for v in values_contextual {
                            for v2 in v {
                                println!("\t{}, ", v2);
                            }
                        }}
                    }

                }
            }
            None => {
                println!("Word not found in the lexicon.");
            }
        }
    }

    Ok(())
}

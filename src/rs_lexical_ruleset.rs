use std::collections::HashMap;
use std::io;
use crate::rs_lex_rulespec_id::LexicalRulespec;
use crate::rs_wordclass::{Wordclass};

/// ...
pub fn parse_lexical_ruleset(path: &str) -> Result<HashMap<Wordclass, Vec<LexicalRulespec>>, io::Error>
{
    println!("{0}", path);
    let mut result: HashMap<Wordclass, Vec<LexicalRulespec>> = HashMap::new();
    /* for line in read_to_string(path)?.lines() {
        // let parts: Vec<&str> = line.split_whitespace().collect();

        // Brill's original lexical rules come in a (somewhat weird) variety of forms, with each rule varying in syntactic structure.
        // The only common attributes are the `rulestring` and `target_tag`, as some rules are source-tag ambiguous. This processes it.

        // If the first token is a tag, then the rulestring is the 3rd token, otherwise it is the second.

        let maybe_rulestring: &str = parts.get()

        let source: &str = parts.first().ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing source tag"))?;
        let target: &str = parts.get(1).ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing target tag"))?;
        let rulestring: &str = parts.get(2).ok_or_else(|| Error::new(ErrorKind::InvalidData, "Missing ruleset ID"))?;

        // Since `source` and `target` should map to POS tags, the rulespec ID should also map.
        let source_tag: Wordclass = map_pos_tag(source)?;
        let target_tag: Wordclass = map_pos_tag(target)?;
        let ruleset_id: RulespecID = map_rulespec_id(rulestring)?;

        // Finally, any additional parameters are collected, before the structure is added to the vector.
        let parameters: Vec<String> = parts.iter().skip(3).map(|s| s.to_string()).collect();
        let new_rulespec = LexicalRulespec {
            source_tag: source_tag.clone(), target_tag, ruleset_id, parameters,
        };

        // Append the rule specification into the vector mapping of the source tag, meaning this rule applies to the source tag.
        result.entry(source_tag).or_default().push(new_rulespec);
    }
    */
    Ok(result)
}

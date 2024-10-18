use std::collections::HashMap;
//use std::fs::File;
use crate::rs_wordclass::Wordclass;
use crate::rs_conllu_parser::parse_conllu_file; // Import your custom parser
use crate::rs_brill_tagger::tag_sentence;
use crate::rs_contextual_rulespec::ContextualRulespec;
use crate::rs_lex_rulespec_id::LexicalRulespec;
use crate::WordclassMap;

/// Function to map a `Wordclass` POS tag to a `rs_conllu::UPOS` POS tag (sacrificing variety).
pub fn wordclass_to_upos(wordclass: &Wordclass) -> crate::rs_conllu_parser::UPOS {
    match wordclass {
        Wordclass::CC    => crate::rs_conllu_parser::UPOS::CCONJ,
        Wordclass::CD    => crate::rs_conllu_parser::UPOS::NUM,
        Wordclass::DT    => crate::rs_conllu_parser::UPOS::DET,
        Wordclass::EX    => crate::rs_conllu_parser::UPOS::PRON,
        Wordclass::FW    => crate::rs_conllu_parser::UPOS::X,
        Wordclass::IN    => crate::rs_conllu_parser::UPOS::ADP,
        Wordclass::JJ    => crate::rs_conllu_parser::UPOS::ADJ,
        Wordclass::JJR   => crate::rs_conllu_parser::UPOS::ADJ,
        Wordclass::JJS   => crate::rs_conllu_parser::UPOS::ADJ,
        Wordclass::LS    => crate::rs_conllu_parser::UPOS::X,
        Wordclass::MD    => crate::rs_conllu_parser::UPOS::AUX,
        Wordclass::NN    => crate::rs_conllu_parser::UPOS::NOUN,
        Wordclass::NNS   => crate::rs_conllu_parser::UPOS::NOUN,
        Wordclass::NNP   => crate::rs_conllu_parser::UPOS::PROPN,
        Wordclass::NNPS  => crate::rs_conllu_parser::UPOS::PROPN,
        Wordclass::PDT   => crate::rs_conllu_parser::UPOS::DET,
        Wordclass::POS   => crate::rs_conllu_parser::UPOS::PART,
        Wordclass::PRPE  => crate::rs_conllu_parser::UPOS::PRON,
        Wordclass::PRPO  => crate::rs_conllu_parser::UPOS::PRON,
        Wordclass::RB    => crate::rs_conllu_parser::UPOS::ADV,
        Wordclass::RBR   => crate::rs_conllu_parser::UPOS::ADV,
        Wordclass::RBS   => crate::rs_conllu_parser::UPOS::ADV,
        Wordclass::RP    => crate::rs_conllu_parser::UPOS::PART,
        Wordclass::SYM   => crate::rs_conllu_parser::UPOS::SYM,
        Wordclass::TO    => crate::rs_conllu_parser::UPOS::PART,
        Wordclass::UH    => crate::rs_conllu_parser::UPOS::INTJ,
        Wordclass::VB    => crate::rs_conllu_parser::UPOS::VERB,
        Wordclass::VBD   => crate::rs_conllu_parser::UPOS::VERB,
        Wordclass::VBG   => crate::rs_conllu_parser::UPOS::VERB,
        Wordclass::VBN   => crate::rs_conllu_parser::UPOS::VERB,
        Wordclass::VBP   => crate::rs_conllu_parser::UPOS::VERB,
        Wordclass::VBZ   => crate::rs_conllu_parser::UPOS::VERB,
        Wordclass::WDT   => crate::rs_conllu_parser::UPOS::DET,
        Wordclass::WPR   => crate::rs_conllu_parser::UPOS::PRON,
        Wordclass::WPO   => crate::rs_conllu_parser::UPOS::PRON,
        Wordclass::WRB   => crate::rs_conllu_parser::UPOS::ADV,
        Wordclass::PUNC => crate::rs_conllu_parser::UPOS::PUNCT,
        Wordclass::ANY   => crate::rs_conllu_parser::UPOS::PROPN,
    }
}

/// Function to benchmark the POS tagger using a `.conllu` file (give the path as a parameter).
pub fn benchmark_pos_tagger(conllu_filepath: &str, lexical_ruleset: &Vec<LexicalRulespec>, contextual_ruleset: &HashMap<Wordclass, Vec<ContextualRulespec>>, wc_mapping: &mut WordclassMap) -> f32 {
    // Open the file and create a buffered reader
    // let file = File::open(conllu_filepath).expect("Unable to open file");

    // Parse the CoNLL-U file using your custom parser
    let sentences = parse_conllu_file(conllu_filepath).expect("Failed to parse file");

    // Count the total number of sentences
    let total_sentences = sentences.len();
    println!("Total sentences: {}", total_sentences);

    let mut total_score = 0.0;  // To track the total score for all sentences
    let mut sentence_count = 0; // To track the number of sentences
    let max_sentences = 50; // Shorten the test base to speed up testing.

    // Iterate through the parsed sentences
    for (i, sentence) in sentences.iter().enumerate() {
        // Collect forms into a space-separated string
        let str_sentence: String = sentence.iter()   // Use iter() to avoid moving ownership
            .map(|token| token.form.as_str())         // Map each token to its form
            .collect::<Vec<&str>>()                   // Collect into a Vec<&str>
            .join(" ");                               // Join the words into a single string

        // Tag the sentence using the tagging function
        let tagged_sentence = tag_sentence(&str_sentence, &lexical_ruleset, &contextual_ruleset, wc_mapping);

        // Print sentence number and header
        println!("\nSentence {} score:", i + 1);
        println!("{:<20} | {:<20} | {:<15} | {:<10} | {}",
                 "Original Word", "Original UPOS", "Predicted Word", "Predicted Tag", "Match");
        println!("{}", "-".repeat(80));

        // Variables to calculate match score for this sentence
        let mut matches = 0;
        let total_tokens = sentence.len();

        // Zip the original tokens with the tagged tokens to print them side by side
        for (token, (word, tag)) in sentence.iter().zip(tagged_sentence.iter()) {
            let predicted_upos = wordclass_to_upos(tag).clone();  // Convert predicted Wordclass to UPOS

            // Compare the predicted UPOS with the actual UPOS
            let correct = token.upos == Some(predicted_upos.clone());  // Check if they match
            if correct {
                matches += 1;
            }

            // Format the token's actual UPOS for display
            let original_upos = match token.clone().upos {
                Some(upos) => format!("{:?}", upos),
                None => "None".to_string(),
            };

            // Print the token, predicted, and actual POS tags, along with a check or cross symbol
            println!("{:<20} | {:<20} | {:<15} | {:<10?} | {} |",
                     token.form,                               // Original word
                     original_upos,                            // Original UPOS
                     word,                                     // Predicted word
                     predicted_upos,                           // Predicted UPOS
                     if correct { "✔" } else { "✘" });         // Match indicator
        }

        // Calculate the match score for this sentence
        let sentence_score = matches as f32 / total_tokens as f32;
        total_score += sentence_score;
        sentence_count += 1;

        // Print the match score for the sentence
        println!("\nSentence {} (of {}) match score: {:.2}\n", i + 1, total_sentences, sentence_score);
        println!("{}", "=".repeat(80));  // Separate output for readability

        if sentence_count > max_sentences {break;} // Limit testing to 100 sentences.
    }

    // Calculate and return the average match score across all sentences
    let avg_score = if sentence_count > 0 {
        total_score / sentence_count as f32
    } else {
        0.0
    };

    println!("Average match score: {:.2}", avg_score);
    avg_score
}

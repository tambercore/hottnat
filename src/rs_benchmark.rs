use std::fs::File;
use crate::rs_wordclass::Wordclass;
use rs_conllu;
use crate::rs_brill_tagger::tag_sentence;

/// Function to map a `Wordclass` POS tag to a `rs_conllu::UPOS` POS tag (sacraficing of variety).
pub fn wordclass_to_upos(wordclass: &Wordclass) -> rs_conllu::UPOS {
    match wordclass {
        Wordclass::CC    => rs_conllu::UPOS::CCONJ,     // Coordinating conjunction
        Wordclass::CD    => rs_conllu::UPOS::NUM,       // Cardinal number
        Wordclass::DT    => rs_conllu::UPOS::DET,       // Determiner
        Wordclass::EX    => rs_conllu::UPOS::PRON,      // Existential there (treated as pronoun)
        Wordclass::FW    => rs_conllu::UPOS::X,         // Foreign word (other)
        Wordclass::IN    => rs_conllu::UPOS::ADP,       // Preposition or subordinating conjunction
        Wordclass::JJ    => rs_conllu::UPOS::ADJ,       // Adjective
        Wordclass::JJR   => rs_conllu::UPOS::ADJ,       // Adjective, comparative
        Wordclass::JJS   => rs_conllu::UPOS::ADJ,       // Adjective, superlative
        Wordclass::LS    => rs_conllu::UPOS::X,         // List item marker (other)
        Wordclass::MD    => rs_conllu::UPOS::AUX,       // Modal
        Wordclass::NN    => rs_conllu::UPOS::NOUN,      // Noun, singular or mass
        Wordclass::NNS   => rs_conllu::UPOS::NOUN,      // Noun, plural
        Wordclass::NNP   => rs_conllu::UPOS::PROPN,     // Proper noun, singular
        Wordclass::NNPS  => rs_conllu::UPOS::PROPN,     // Proper noun, plural
        Wordclass::PDT   => rs_conllu::UPOS::DET,       // Predeterminer
        Wordclass::POS   => rs_conllu::UPOS::PART,      // Possessive ending (treated as particle)
        Wordclass::PRPE  => rs_conllu::UPOS::PRON,      // Personal pronoun
        Wordclass::PRPO  => rs_conllu::UPOS::PRON,      // Possessive pronoun
        Wordclass::RB    => rs_conllu::UPOS::ADV,       // Adverb
        Wordclass::RBR   => rs_conllu::UPOS::ADV,       // Adverb, comparative
        Wordclass::RBS   => rs_conllu::UPOS::ADV,       // Adverb, superlative
        Wordclass::RP    => rs_conllu::UPOS::PART,      // Particle
        Wordclass::SYM   => rs_conllu::UPOS::SYM,       // Symbol
        Wordclass::TO    => rs_conllu::UPOS::PART,      // "to" (treated as particle)
        Wordclass::UH    => rs_conllu::UPOS::INTJ,      // Interjection
        Wordclass::VB    => rs_conllu::UPOS::VERB,      // Verb, base form
        Wordclass::VBD   => rs_conllu::UPOS::VERB,      // Verb, past tense
        Wordclass::VBG   => rs_conllu::UPOS::VERB,      // Verb, gerund or present participle
        Wordclass::VBN   => rs_conllu::UPOS::VERB,      // Verb, past participle
        Wordclass::VBP   => rs_conllu::UPOS::VERB,      // Verb, non-3rd person singular present
        Wordclass::VBZ   => rs_conllu::UPOS::VERB,      // Verb, 3rd person singular present
        Wordclass::WDT   => rs_conllu::UPOS::DET,       // Wh-determiner
        Wordclass::WPR   => rs_conllu::UPOS::PRON,      // Wh-pronoun
        Wordclass::WPO   => rs_conllu::UPOS::PRON,      // Possessive wh-pronoun
        Wordclass::WRB   => rs_conllu::UPOS::ADV,       // Wh-adverb
        Wordclass::OTHER => rs_conllu::UPOS::X,         // Other
        Wordclass::ANY   => rs_conllu::UPOS::X,         // Any (contextual, treated as other)
    }
}

/// Function to benchmark the POS tagger using a `.conllu` file (give the path as a parameter).
pub fn benchmark_pos_tagger(conllu_filepath: &str) -> f32 {

    // Open the file and create a buffered reader
    let mut file = File::open(conllu_filepath).unwrap();

    // First pass: Count the total number of sentences
    let total_sentences = rs_conllu::parse_file(file)
        .count();

    // Reopen the file for the second pass to process the sentences
    let file = File::open(conllu_filepath).unwrap();  // Reopen the file
    let doc = rs_conllu::parse_file(file);

    let mut total_score = 0.0;  // To track the total score for all sentences
    let mut sentence_count = 0; // To track the number of sentences

    for (i, sentence) in doc.enumerate() {
        let sentence = sentence.expect("REASON");  // Unwrap the sentence safely

        // Collect forms into a space-separated string
        let str_sentence: String = sentence.tokens.iter()   // Use iter() to avoid moving ownership
            .map(|token| token.form.as_str())               // Map each token to its form
            .collect::<Vec<&str>>()                         // Collect into a Vec<&str>
            .join(" ");                                     // Join the words into a single string

        // Tag the sentence using the tagging function
        let tagged_sentence = tag_sentence(&str_sentence);

        // Print sentence number and header
        println!("\nSentence {} score:", i + 1);
        println!("{:<20} | {:<20} | {:<15} | {:<10} | {}", "Original Word", "Original UPOS", "Predicted Word", "Predicted Tag", "Match");
        println!("{}", "-".repeat(80));

        // Variables to calculate match score for this sentence
        let mut matches = 0;
        let total_tokens = sentence.tokens.len();

        // Zip the original tokens with the tagged tokens to print them side by side
        for (token, (word, tag)) in sentence.tokens.iter().zip(tagged_sentence.iter()) {
            let predicted_upos = wordclass_to_upos(tag);  // Convert predicted Wordclass to UPOS

            // Compare the predicted UPOS with the actual UPOS
            let correct = token.upos == Some(predicted_upos);  // Check if they match
            if correct {
                matches += 1;
            }

            // Format the token's actual UPOS for display
            let original_upos = match token.upos {
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
        println!("\nSentence {} (of {}) match score: {:.2}\n", i, total_sentences, sentence_score);
        println!("{}", "=".repeat(80));  // Separate output for readability
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

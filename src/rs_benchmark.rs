use crate::rs_wordclass::Wordclass;
use rs_conllu;

pub fn wordclass_to_upos(wordclass: Wordclass) -> rs_conllu::UPOS {
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

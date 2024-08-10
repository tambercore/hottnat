use std::fmt;
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Wordclass {
    CC,     // Coordinating conjunction
    CD,     // Cardinal number
    DT,     // Determiner
    EX,     // Existential there
    FW,     // Foreign word
    IN,     // Preposition or subordinating conjunction
    JJ,     // Adjective
    JJR,    // Adjective, comparative
    JJS,    // Adjective, superlative
    LS,     // List item marker
    MD,     // Modal
    NN,     // Noun, singular or mass
    NNS,    // Noun, plural
    NNP,    // Proper noun, singular
    NNPS,   // Proper noun, plural
    PDT,    // Predeterminer
    POS,    // Possessive ending
    PRPE,   // Personal pronoun
    PRPO,   // Possessive pronoun
    RB,     // Adverb
    RBR,    // Adverb, comparative
    RBS,    // Adverb, superlative
    RP,     // Particle
    SYM,    // Symbol
    TO,     // to
    UH,     // Interjection
    VB,     // Verb, base form
    VBD,    // Verb, past tense
    VBG,    // Verb, gerund or present participle
    VBN,    // Verb, past participle
    VBP,    // Verb, non-3rd person singular present
    VBZ,    // Verb, 3rd person singular present
    WDT,    // Wh-determiner
    WPR,    // Wh-pronoun
    WPO,    // Possessive wh-pronoun
    WRB,    // Wh-adverb
    OTHER,  // Other!
    ANY,    // Any, used in contextual rules.
}

impl fmt::Display for Wordclass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let wordclass_str = match self {
            Wordclass::CC => "Coordinating conjunction",
            Wordclass::CD => "Cardinal number",
            Wordclass::DT => "Determiner",
            Wordclass::EX => "Existential there",
            Wordclass::FW => "Foreign word",
            Wordclass::IN => "Preposition or subordinating conjunction",
            Wordclass::JJ => "Adjective",
            Wordclass::JJR => "Adjective (comparative)",
            Wordclass::JJS => "Adjective (superlative)",
            Wordclass::LS => "List item marker",
            Wordclass::MD => "Modal",
            Wordclass::NN => "Noun (singular or mass)",
            Wordclass::NNS => "Noun (plural)",
            Wordclass::NNP => "Proper noun (singular)",
            Wordclass::NNPS => "Proper noun (plural)",
            Wordclass::PDT => "Predeterminer",
            Wordclass::POS => "Possessive ending",
            Wordclass::PRPE => "Personal pronoun",
            Wordclass::PRPO => "Possessive pronoun",
            Wordclass::RB => "Adverb",
            Wordclass::RBR => "Adverb (comparative)",
            Wordclass::RBS => "Adverb (superlative)",
            Wordclass::RP => "Particle",
            Wordclass::SYM => "Symbol",
            Wordclass::TO => "to",
            Wordclass::UH => "Interjection",
            Wordclass::VB => "Verb (base form)",
            Wordclass::VBD => "Verb (past tense)",
            Wordclass::VBG => "Verb (gerund or present participle)",
            Wordclass::VBN => "Verb (past participle)",
            Wordclass::VBP => "Verb (non-3rd person singular present)",
            Wordclass::VBZ => "Verb (3rd person singular present)",
            Wordclass::WDT => "Wh-determiner",
            Wordclass::WPR => "Wh-pronoun",
            Wordclass::WPO => "Possessive wh-pronoun",
            Wordclass::WRB => "Wh-adverb",
            Wordclass::OTHER => "Other!",
            Wordclass::ANY => "Any!",
        };
        write!(f, "{}", wordclass_str)
    }
}


pub fn map_pos_tag(tag: &str) -> Result<Wordclass, Error> {
    match tag {
        "CC" => Ok(Wordclass::CC),
        "CD" => Ok(Wordclass::CD),
        "DT" => Ok(Wordclass::DT),
        "EX" => Ok(Wordclass::EX),
        "FW" => Ok(Wordclass::FW),
        "IN" => Ok(Wordclass::IN),
        "JJ" => Ok(Wordclass::JJ),
        "JJR" => Ok(Wordclass::JJR),
        "JJS" => Ok(Wordclass::JJS),
        "LS" => Ok(Wordclass::LS),
        "MD" => Ok(Wordclass::MD),
        "NN" => Ok(Wordclass::NN),
        "NNS" => Ok(Wordclass::NNS),
        "NNP" => Ok(Wordclass::NNP),
        "NNPS" => Ok(Wordclass::NNPS),
        "PDT" => Ok(Wordclass::PDT),
        "POS" => Ok(Wordclass::POS),
        "PRP" => Ok(Wordclass::PRPE),
        "PRP$" => Ok(Wordclass::PRPO),
        "RB" => Ok(Wordclass::RB),
        "RBR" => Ok(Wordclass::RBR),
        "RBS" => Ok(Wordclass::RBS),
        "RP" => Ok(Wordclass::RP),
        "SYM" => Ok(Wordclass::SYM),
        "TO" => Ok(Wordclass::TO),
        "UH" => Ok(Wordclass::UH),
        "VB" => Ok(Wordclass::VB),
        "VBD" => Ok(Wordclass::VBD),
        "VBG" => Ok(Wordclass::VBG),
        "VBN" => Ok(Wordclass::VBN),
        "VBP" => Ok(Wordclass::VBP),
        "VBZ" => Ok(Wordclass::VBZ),
        "WDT" => Ok(Wordclass::WDT),
        "WP" => Ok(Wordclass::WPR),
        "WP$" => Ok(Wordclass::WPO),
        "WRB" => Ok(Wordclass::WRB),
        "''" => Ok(Wordclass::ANY),
        _ => Err(Error::new(ErrorKind::InvalidData, format!("Invalid POS Tag Identifier: {}", tag))),
    }
}

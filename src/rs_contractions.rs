use std::collections::HashMap;
use std::fs;
use serde::{Deserialize};
use serde_json;
use std::io;

#[derive(Deserialize, Debug)]
struct Contractions {
    #[serde(flatten)]
    contractions: HashMap<String, Vec<String>>,
}

/// Function to load `data/contractions.json` as a hashmap of contractions to their expansions.
fn load_contractions() -> Result<HashMap<String, Vec<String>>, io::Error> {
    let data = fs::read_to_string("data/contractions.json")?;
    let contractions: Contractions = serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    Ok(contractions.contractions)
}

/// Function to expand a contraction `input` according to the `contractions_map`.
fn expand_contraction(input: String, contractions_map: &HashMap<String, Vec<String>>) -> Option<Vec<String>> {
    contractions_map.get(&input).map(|expansion| expansion.clone())
}

/// Function to find contractions for a given `input`
fn find_contractions(input: String) -> Result<Vec<String>, String> {
    let contractions_map = load_contractions().map_err(|e| format!("Error loading contractions: {}", e))?;

    // Map the `input` to its corresponding contraction
    let mut result: Vec<String> = Vec::new();
    if let Some(expansion) = expand_contraction(input.clone(), &contractions_map) {
        match expansion.get(0) {
            Some(first_expansion) => result.push(first_expansion.to_string()),  // Convert &str to String
            None => return Err("Empty contraction vector.".to_string()),  // Error if empty
        }
    }

    // If no contractions are found, return the original input as a single-element vector
    if result.is_empty() { Ok(vec![input]) } else { Ok(result) }
}


pub fn test_contractions() {
    let input = "you're";
    match find_contractions(input.to_string()) {
        Ok(expansions) => {
            for contraction in expansions {
                println!("{}", contraction); // Print each contraction
            }
        }
        Err(e) => println!("Error: {}", e), // Print the error if any
    }
}

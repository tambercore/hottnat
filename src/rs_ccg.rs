
/// Enumeration to over atomic CCG categories like S, NP, N, etc.
#[derive(Debug, Clone)]
enum Category {
    S,  // Sentence
    NP, // Noun Phrase
    N,  // Noun
}

/// Enumeration to represent the functor application categories
#[derive(Debug, Clone)]
enum Functor {
    Forward(Box<CCG>, Box<CCG>),  // A/B (applies between any two CCG categories)
    Backward(Box<CCG>, Box<CCG>), // A\B (applies between any two CCG categories)
}

/// Enumeration to represent a category (either atomic or a functor)
#[derive(Debug, Clone)]
enum CCG {
    Atom(Category),         // Atomic category
    Func(Functor),          // Functor (applies between categories)
}

/// Helper functions for the CCG type.
impl CCG {
    // Function to create forward functors
    fn forward(left: CCG, right: CCG) -> CCG {
        CCG::Func(Functor::Forward(Box::new(left), Box::new(right)))
    }

    // Function to create backward functors
    fn backward(left: CCG, right: CCG) -> CCG {
        CCG::Func(Functor::Backward(Box::new(left), Box::new(right)))
    }
}

// Temporary test.
pub fn test_ccg() {
    // Example: (S\NP)\(S\NP)
    let s = CCG::Atom(Category::S);
    let np = CCG::Atom(Category::NP);

    let s_np = CCG::backward(s.clone(), np.clone()); // S\NP
    let complex_category = CCG::backward(s_np.clone(), s_np.clone()); // (S\NP)\(S\NP)

    // Print the complex category in a human-readable format
    println!("{:?}", complex_category);
}
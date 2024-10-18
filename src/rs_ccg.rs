

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
    Atomic(Category),          // Atomic category
    Functor(Functor),          // Functor (applies between categories)
}


// Function to create backward functors, as a CCG.
fn create_forward_functor(left: CCG, right: CCG) -> CCG {
    CCG::Functor(Functor::Forward(Box::new(left), Box::new(right)))
}


// Function to create forward functors, as a CCG.
fn create_backward_functor(left: CCG, right: CCG) -> CCG {
    CCG::Functor(Functor::Backward(Box::new(left), Box::new(right)))
}


// Temporary test.
pub fn test_ccg() {
    // Example: (S\NP)\(S\NP)
    let s = CCG::Atomic(Category::S);
    let np = CCG::Atomic(Category::NP);

    let s_np = create_backward_functor(s.clone(), np.clone()); // S\NP
    let s = CCG::Atomic(Category::S); // S\NP
    let complex_category = create_forward_functor(s_np.clone(), s_np.clone()); // (S\NP)\(S\NP)

    // Print the complex category in a human-readable format
    println!("{:?}", complex_category);
    println!("{:?}", s_np);
    println!("{:?}", s);

}
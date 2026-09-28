use rand::prelude::*;

// Array of quotes
const QUOTES: &[&str] = &[
    "All we have to decide is what to do with the time that is given us.\n- Gandalf",
    "Not all those who wander are lost.\n- Bilbo Baggins",
    "The road goes ever on and on.\n- Bilbo Baggins",
];

// Choosing a random quote
pub fn random() -> String {
    QUOTES.choose(&mut rand::rng()).unwrap().to_string()
}

pub mod jaccard;
pub mod jaro;
pub mod shared;
pub mod soundex;

// Clean trait handles string preprocessing before comparison can take place.
pub trait Clean {
    fn clean(&self, s1: &str) -> String {
        // Replaces all non-alphabetic with whitespace
        let mut s1 = s1.replace(|c: char| !c.is_alphabetic(), " ");
        s1.make_ascii_uppercase();
        s1
    }
}

// Is dependent on Clean trait
pub trait Compare: Clean {
    // compare method returns the similarity score between two strings
    // s1 and s2. Score is between 0.0 and 1.0.
    fn compare(&self, s1: &str, s2: &str) -> f64;
}

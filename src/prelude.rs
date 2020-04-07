// Clean trait handles string preprocessing before comparison can take place.
pub trait Clean {
    fn clean(&self, s1: &str) -> String {
        // Replaces all non-alphabetic with whitespace
        let mut cleaned = s1
            .replace(|c: char| !c.is_alphabetic(), " ")
            .trim()
            .to_owned();
        cleaned.make_ascii_uppercase();
        cleaned
    }
}

// Weighted trait exposes the weight attribute of concrete type
pub trait Weighted {
    fn get_weight(&self) -> f64;
}

// Named trait exposes name attribute of concrete type
pub trait Named {
    fn get_name(&self) -> &str;
}

// Is dependent on Clean trait
pub trait Matcher: Clean + Weighted + Named {
    // get_score method returns the similarity score between two strings
    // s1 and s2. Score is between 0.0 and 1.0.
    fn get_score(&self, s1: &str, s2: &str) -> f64;
}

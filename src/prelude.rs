use serde::{Deserialize, Serialize};

// MatcherRule represents the matching result
// in greated detail
#[derive(Serialize, Deserialize, Debug)]
pub struct MatchResult<'a> {
    name: String,
    #[serde(borrow)]
    input_strings: Vec<&'a str>,
    weight: f64,
    absolute_score: f64,
    weighted_score: f64,
}

impl MatchResult<'_> {
    pub fn new<'a>(
        name: &str,
        input_strings: Vec<&'a str>,
        weight: f64,
        score: f64,
    ) -> MatchResult<'a> {
        MatchResult {
            name: name.to_owned(),
            input_strings,
            weight,
            absolute_score: score,
            weighted_score: score * weight,
        }
    }
}

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

pub trait Matcher: Clean + Weighted + Named {
    // get_score method returns the similarity score between two strings
    // s1 and s2. Score is between 0.0 and 1.0.
    fn get_score(&self, s1: &str, s2: &str) -> f64;

    // get_weighted_score returns the weighted score
    // which is defined as weight x score.
    fn get_weighted_score(&self, s1: &str, s2: &str) -> f64 {
        self.get_weight() * self.get_score(s1, s2)
    }

    // get_match_result returns the MatchReult of the Comparison operation
    fn get_match_result<'a>(&self, s1: &'a str, s2: &'a str) -> MatchResult<'a> {
        let input_strings = vec![s1, s2];
        let score = self.get_score(s1, s2);
        MatchResult::new(self.get_name(), input_strings, self.get_weight(), score)
    }
}

use super::super::shared::compute;
use super::super::{Clean, Compare};
use std::collections::HashSet;

pub struct JaccardMatcher {
    weight: f64,
}

// JaccardMatcher calculates Jaccard Index or (Overlapping Coefficient)
// between two names.
// Tokenize two names first and determine the degree of similarity.
// Formula is (# of similar items)/ (Minimum Hashsize)
// Betwen 0.0 and 1.0
impl JaccardMatcher {
    pub fn new(weight: Option<f64>) -> JaccardMatcher {
        let weight = weight.unwrap_or(1.0);
        JaccardMatcher { weight }
    }

    pub fn default() -> JaccardMatcher {
        JaccardMatcher { weight: 1.0 }
    }

    fn as_tokenized_set(&self, string: &str) -> HashSet<String> {
        let mut token_set = HashSet::new();
        for token in string.split_whitespace() {
            let token = token.to_owned();
            token_set.insert(token);
        }
        token_set
    }
}

impl Clean for JaccardMatcher {}

impl Compare for JaccardMatcher {
    fn compare(&self, s1: &str, s2: &str) -> f64 {
        let cleaned_s1 = self.clean(s1);
        let tokenized_s1 = self.as_tokenized_set(&cleaned_s1[..]);
        let cleaned_s2 = self.clean(s2);
        let tokenized_s2 = self.as_tokenized_set(&cleaned_s2[..]);
        self.weight * compute::jaccard_index(&tokenized_s1, &tokenized_s2)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::*;
    #[test]
    fn test_case_sensitive_match() {
        let matcher = super::JaccardMatcher::default();
        let name1 = "john doe";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.compare(name1, name2), 1.0);
    }

    #[test]
    fn test_ignore_special_character_match() {
        let matcher = super::JaccardMatcher::default();
        let name1 = "joh^ doe";
        let name2 = "joh**doe";
        assert_eq!(matcher.compare(name1, name2), 1.0);
    }

    #[test]
    fn test_weighted_match() {
        let matcher = super::JaccardMatcher::new(Some(0.5));
        let name1 = "JOHN DOE";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.compare(name1, name2), 0.5);
    }
}

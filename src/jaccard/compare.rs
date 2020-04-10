use super::super::prelude::*;
use super::super::shared::compute;
use std::collections::HashSet;
use std::sync::RwLock;

pub struct JaccardMatcher {
    name: String,
    weight: RwLock<f64>,
}

// JaccardMatcher calculates Jaccard Index or (Overlapping Coefficient)
// between two names.
// Tokenize two names first and determine the degree of similarity.
// Formula is (# of similar items)/ (Minimum Hashsize)
// Betwen 0.0 and 1.0
impl JaccardMatcher {
    pub fn new(weight: Option<f64>) -> JaccardMatcher {
        let weight = weight.unwrap_or(1.0);
        let locked_weight = RwLock::new(weight);
        JaccardMatcher {
            name: "Jaccard".to_owned(),
            weight: locked_weight,
        }
    }

    pub fn default() -> JaccardMatcher {
        let weight = RwLock::new(1.0);
        JaccardMatcher {
            name: "Jaccard".to_owned(),
            weight: weight,
        }
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

impl Named for JaccardMatcher {
    fn get_name(&self) -> &str {
        &self.name[..]
    }
}

impl Clean for JaccardMatcher {}

impl Weighted for JaccardMatcher {
    fn get_weight(&self) -> f64 {
        let weight_ptr = self.weight.read().unwrap();
        *weight_ptr
    }

    fn set_weight(&mut self, weight: f64) {
        let mut weight_ptr = self.weight.write().unwrap();
        *weight_ptr = weight;
    }
}

impl Matcher for JaccardMatcher {
    fn get_score(&self, s1: &str, s2: &str) -> f64 {
        let cleaned_s1 = self.clean(s1);
        let tokenized_s1 = self.as_tokenized_set(&cleaned_s1[..]);
        let cleaned_s2 = self.clean(s2);
        let tokenized_s2 = self.as_tokenized_set(&cleaned_s2[..]);
        compute::jaccard_index(&tokenized_s1, &tokenized_s2)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::prelude::*;
    #[test]
    fn test_case_sensitive_match() {
        let matcher = super::JaccardMatcher::default();
        let name1 = "john doe";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.get_score(name1, name2), 1.0);
    }

    #[test]
    fn test_ignore_special_character_match() {
        let matcher = super::JaccardMatcher::default();
        let name1 = "joh^ doe";
        let name2 = "joh**doe";
        assert_eq!(matcher.get_score(name1, name2), 1.0);
    }

    #[test]
    fn test_whitespaces_match() {
        let matcher = super::JaccardMatcher::default();
        let name1 = "  john    doe   ";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.get_score(name1, name2), 1.0);
    }

    #[test]
    fn test_half_weight() {
        let matcher = super::JaccardMatcher::new(Some(0.5));
        let name1 = "JOHN DOE";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.get_weighted_score(name1, name2), 0.5);
    }
}

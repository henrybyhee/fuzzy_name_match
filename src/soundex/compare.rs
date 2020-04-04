use super::super::shared::compute;
use super::super::{Clean, Compare};
use super::encode;
use std::collections::HashSet;

// Soundex is a phonetic algorithm that encodes names in Soundex code.
// 1.0 -> Match
// 0.0 -> No Match
pub struct SoundexMatcher {
    weight: f64,
}

impl SoundexMatcher {
    pub fn new(weight: Option<f64>) -> SoundexMatcher {
        let weight = weight.unwrap_or(1.0);
        SoundexMatcher { weight }
    }

    pub fn default() -> SoundexMatcher {
        SoundexMatcher { weight: 1.0 }
    }
}

impl Clean for SoundexMatcher {}

impl Compare for SoundexMatcher {
    fn compare(&self, s1: &str, s2: &str) -> f64 {
        let cleaned_s1 = self.clean(s1);
        let soundex_s1 = encode::apply_soundex(&cleaned_s1[..]);
        let cleaned_s2 = self.clean(s2);
        let soundex_s2 = encode::apply_soundex(&cleaned_s2[..]);
        if soundex_s1 == soundex_s2 {
            self.weight * 1.0
        } else {
            0.0
        }
    }
}

// SoundexJaccardMatcher is an improvement over SoundexMatcher, handles token
// transposition. "James Black" vs "Black, James"
// SoundexJaccardMatcher tokenizes string by space delimiter, encodes each token
// in their Soundex form and then computes the Jaccard index between two strings.
// Eg:
// "James Bond" vs "Bane Jimmy"
// -> {"J520", "B530"} vs {"B530", "J520"}
// -> Jaccard Index = 2.0 / 2.0 = 1.0
// Measures the degree of intersection
pub struct SoundexJaccardMatcher {
    weight: f64,
}

impl SoundexJaccardMatcher {
    pub fn new(weight: Option<f64>) -> SoundexJaccardMatcher {
        let weight = weight.unwrap_or(1.0);
        SoundexJaccardMatcher { weight }
    }

    pub fn default() -> SoundexJaccardMatcher {
        SoundexJaccardMatcher { weight: 1.0 }
    }

    // tokenize turns every part of a name into its
    // Soundex code and move into a Vector.
    // Eg:
    //   "James" -> ["J520"]
    //   "James Bond" -> ["J520", "B530"]
    fn as_tokenized_set(&self, name: &str) -> HashSet<String> {
        let mut soundex_set: HashSet<String> = HashSet::new();
        for token in name.split_whitespace() {
            let soundex_code = encode::apply_soundex(token);
            soundex_set.insert(soundex_code);
        }
        soundex_set
    }
}

impl Clean for SoundexJaccardMatcher {}

impl Compare for SoundexJaccardMatcher {
    fn compare(&self, s1: &str, s2: &str) -> f64 {
        let cleaned_s1 = self.clean(s1);
        let name_1_soundex_set = self.as_tokenized_set(&cleaned_s1[..]);
        let cleaned_s2 = self.clean(s2);
        let name_2_soundex_set = self.as_tokenized_set(&cleaned_s2[..]);
        self.weight * compute::jaccard_index(&name_1_soundex_set, &name_2_soundex_set)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::Compare;
    use super::SoundexJaccardMatcher;
    use super::SoundexMatcher;
    #[test]
    fn test_soundex_matcher() {
        let matcher = SoundexMatcher::default();
        let name1 = "Jame";
        let name2 = "Jimmy";
        let score = matcher.compare(name1, name2);
        assert_eq!(score, 1.0);
    }
    #[test]
    fn test_soundex_matcher_half_weight() {
        let matcher = SoundexMatcher::new(Some(0.5));
        let name1 = "Jame";
        let name2 = "Jimmy";
        let score = matcher.compare(name1, name2);
        assert_eq!(score, 0.5);
    }

    #[test]
    fn test_soundex_matcher_no_match() {
        let matcher = SoundexMatcher::default();
        let name1 = "James";
        let name2 = "Jimmy";
        let score = matcher.compare(name1, name2);
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_soundex_case_insensitive_match() {
        let matcher = SoundexMatcher::default();
        let name1 = "james";
        let name2 = "JAMES";
        let score = matcher.compare(name1, name2);
        assert_eq!(score, 1.0);
    }

    #[test]
    fn test_soundex_jaccard_half_match() {
        let matcher = SoundexJaccardMatcher::default();
        let name1 = "Jame Bond";
        let name2 = "Bane Jimmy";
        let score = matcher.compare(name1, name2);
        assert_eq!(score, 0.5);
    }

    #[test]
    fn test_soundex_jaccard_longer_name() {
        let matcher = SoundexJaccardMatcher::default();
        let name1 = "Robert Downey Junior";
        let name2 = "Anthony Rupert";
        let score = matcher.compare(name1, name2);
        assert_eq!(score, 0.5);
    }

    #[test]
    fn test_soundex_jaccard_half_weight() {
        let matcher = SoundexJaccardMatcher::new(Some(0.5));
        let name1 = "Jame Bond";
        let name2 = "Bane Jimmy";
        let score = matcher.compare(name1, name2);
        assert_eq!(score, 0.25);
    }

    #[test]
    fn test_soundex_jaccard_case_insensitive_match() {
        let matcher = SoundexJaccardMatcher::default();
        let name1 = "james";
        let name2 = "JAMES";
        let score = matcher.compare(name1, name2);
        assert_eq!(score, 1.0);
    }
}

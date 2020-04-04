use std::cmp;
use std::collections::HashSet;

// Jaccard Index computes similarity score between
// two hash sets.
// Formula is (A ∩ B) / (A U B)
pub fn jaccard_index(s1: &HashSet<String>, s2: &HashSet<String>) -> f64 {
    let overlaps: f64 = s1.intersection(s2).count() as f64;
    let union = s1.union(s2).count() as f64;
    overlaps / union
}

#[cfg(test)]
mod test {
    use super::jaccard_index;
    use std::collections::HashSet;
    fn build_map_from_word_vec(words: Vec<&str>) -> HashSet<String> {
        let mut hash_map: HashSet<String> = HashSet::new();
        for word in words {
            hash_map.insert(word.to_owned());
        }
        hash_map
    }
    #[test]
    fn test_jaccard_index() {
        let name1: HashSet<String> = build_map_from_word_vec(vec!["AB", "BC"]);
        let name2: HashSet<String> = build_map_from_word_vec(vec!["BC", "AA"]);
        let coef = jaccard_index(&name1, &name2);
        assert!(coef - 0.333 < 0.01);
    }

    #[test]
    fn test_jaccard_index_zero() {
        let name1: HashSet<String> = build_map_from_word_vec(vec!["AB", "BC"]);
        let name2: HashSet<String> = build_map_from_word_vec(vec!["BB", "AA"]);

        let coef = jaccard_index(&name1, &name2);
        assert_eq!(coef, 0.);
    }

    #[test]
    fn test_jaccard_index_match() {
        let name1: HashSet<String> = build_map_from_word_vec(vec![""]);
        let name2: HashSet<String> = build_map_from_word_vec(vec!["BB", "AA"]);

        let coef = jaccard_index(&name1, &name2);
        assert_eq!(coef, 0.);
    }
}

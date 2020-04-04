use std::cmp;
use std::collections::HashSet;

// Jaccard Index computes similarity score between
// two hash sets.
// Formula is (# of similar items)/ (Minimum Hashsize)
pub fn jaccard_index(s1: &HashSet<String>, s2: &HashSet<String>) -> f64 {
    let mut overlaps: f64 = 0.;
    for word in s1.iter() {
        if s2.contains(&word[..]) {
            overlaps += 1.;
        }
    }
    let min = cmp::min(s1.len(), s2.len()) as f64;
    overlaps / min
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
        assert_eq!(coef, 0.5);
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

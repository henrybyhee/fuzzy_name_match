use super::config;
use std::cmp;

fn max_distance_allowed(len1: i32, len2: i32) -> i32 {
    let max = cmp::max::<i32>(len1, len2);
    (max / 2) - 1
}

fn common_prefix_length(s1: &str, s2: &str) -> i32 {
    let mut prefix = 0;
    let min_length = cmp::min(s1.len(), s2.len());

    for i in 0..min_length {
        if s1.as_bytes()[i] != s2.as_bytes()[i] {
            break;
        }
        prefix += 1;
    }

    prefix
}

// matches returns number of matches between string s1 and s2.
//
// Eg:
// "Hello" vs "Hlelo"
// "e" is considered a match as it is within [i - max_matching_dist, i + max_matching_dist]
//
// Args:
//   - s1: String 1
//   - s2: String 2
//   - hash_1: vector of {0, 1} to indicate a matching character at index i of string 1.
//   - hash_2: vector of {0, 1} to indicated a matching character at index j of string 2.
//   - max_matching_dist: Maximum distance allowed to lookup.
fn matches(
    s1: &str,
    s2: &str,
    hash_1: &mut Vec<i32>,
    hash_2: &mut Vec<i32>,
    max_matching_dist: i32,
) -> i32 {
    let mut matches = 0;
    let length_1 = s1.len() as i32;
    let length_2 = s2.len() as i32;

    for i in 0..length_1 {
        let i_pos = i as usize;
        let char_i = s1.as_bytes()[i_pos];
        let start = cmp::max(0, i - max_matching_dist);
        let end = cmp::min(i + max_matching_dist + 1, length_2);

        for j in start..end {
            let j_pos = j as usize;
            let char_j = s2.as_bytes()[j_pos];

            if char_i == char_j && hash_2[j_pos] == 0 {
                hash_1[i_pos] = 1;
                hash_2[j_pos] = 1;
                matches += 1;
                break;
            }
        }
    }
    matches
}

// transpositions returns the half the number of matching characters
// that are out of order.
fn transpositions(s1: &str, s2: &str, hash_1: &Vec<i32>, hash_2: &Vec<i32>) -> f64 {
    let mut ptr_j: usize = 0;
    let mut transpositions: f64 = 0.;
    let length_1 = s1.len();

    for ptr_i in 0..length_1 {
        if hash_1[ptr_i] == 1 {
            // There has to be a matching character somewhere in s2.
            while hash_2[ptr_j] == 0 {
                ptr_j += 1;
            }
            if s1.as_bytes()[ptr_i] != s2.as_bytes()[ptr_j] {
                transpositions += 1.;
            }
            ptr_j += 1;
        }
    }

    transpositions / 2.
}

// jaro_score returns the jaro distance between
// string s1 and string s2.
// Range between 0.0 and 1.0.
pub fn jaro_score(s1: &str, s2: &str) -> f64 {
    //Remove all white space

    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 || len2 == 0 {
        return 0.0 as f64;
    }

    // Exact Match
    if s1 == s2 {
        return 1.0 as f64;
    }

    let max_matching_dist = max_distance_allowed(len1 as i32, len2 as i32);
    let mut matching_hash_1 = vec![0; len1];
    let mut matching_hash_2 = vec![0; len2];
    let matches = matches(
        s1,
        s2,
        &mut matching_hash_1,
        &mut matching_hash_2,
        max_matching_dist,
    );

    if matches == 0 {
        return 0 as f64;
    }

    let transpositions = transpositions(s1, s2, &matching_hash_1, &matching_hash_2);

    let matches = matches as f64;
    let len1 = len1 as f64;
    let len2 = len2 as f64;
    (matches / len1 + matches / len2 + (matches - transpositions) / matches) / 3.0
}

// jaro_winkler_score returns Jaro Winkler score
// Between 0.0 and 1.0
pub fn jaro_winkler_score(s1: &str, s2: &str, config: &config::JaroWinklerConfigOptions) -> f64 {
    let mut jaro_distance = jaro_score(s1, s2);
    if jaro_distance > config.similarity_threshold {
        let mut prefix_length = common_prefix_length(s1, s2);
        prefix_length = cmp::min(config.max_prefix_length, prefix_length);
        let prefix_length = prefix_length as f64;
        jaro_distance += config.scaling_factor * prefix_length * (1. - jaro_distance);
    }
    return jaro_distance;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_common_prefix_length() {
        let s1 = "Hello";
        let s2 = "Hellish";
        let common_prefix = super::common_prefix_length(&s1, &s2);
        assert_eq!(common_prefix, 4);
    }

    #[test]
    fn test_max_distance_is_for_5() {
        let distance = super::max_distance_allowed(5, 2);
        assert_eq!(distance, 1);
    }

    #[test]
    fn test_max_distance_allowed_for_10() {
        let dist = super::max_distance_allowed(10, 9);
        assert_eq!(dist, 4);
    }
    #[test]
    fn test_max_distance_is_0() {
        let distance = super::max_distance_allowed(1, 1);
        assert_eq!(distance, -1);
    }

    // Test Cases for matches
    #[test]
    fn test_matched_dwayne_duayne() {
        let string1 = "Dwayne";
        let string2 = "Duane";
        let mut hash_1 = vec![0, 0, 0, 0, 0, 0];
        let mut hash_2 = vec![0, 0, 0, 0, 0];

        let matches_count = super::matches(string1, string2, &mut hash_1, &mut hash_2, 4);
        assert_eq!(matches_count, 4);
        assert_eq!(hash_1, [1, 0, 1, 0, 1, 1]);
        assert_eq!(hash_2, [1, 0, 1, 1, 1]);
    }

    #[test]
    fn test_no_matched_martha_marhta() {
        let string1 = "Martha";
        let string2 = "Marhta";
        let mut hash_1 = vec![0, 0, 0, 0, 0, 0];
        let mut hash_2 = vec![0, 0, 0, 0, 0, 0];

        let matches_count = super::matches(string1, string2, &mut hash_1, &mut hash_2, 4);
        assert_eq!(matches_count, 6);
        assert_eq!(hash_1, vec![1, 1, 1, 1, 1, 1]);
        assert_eq!(hash_2, vec![1, 1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_empty_string_matches() {
        let string1 = "a";
        let string2 = "";
        let mut hash_1 = vec![0];
        let mut hash_2 = vec![];
        let matches_count = super::matches(string1, string2, &mut hash_1, &mut hash_2, 4);
        assert_eq!(matches_count, 0);
        assert_eq!(hash_1, vec![0]);
        assert_eq!(hash_2, vec![]);
    }

    // Test Case for transpositions
    #[test]
    fn test_empty_string_transpositions() {
        let string1 = "a";
        let string2 = "";
        let hash_1 = vec![0];
        let hash_2 = vec![];

        let transposed = super::transpositions(string1, string2, &hash_1, &hash_2);
        assert_eq!(transposed, 0.);
    }

    #[test]
    fn test_transposition_duane_dwayne() {
        let string1 = "Duane";
        let string2 = "Dwyane";
        let hash_1 = vec![1, 0, 1, 1, 1];
        let hash_2 = vec![1, 0, 0, 1, 1, 1];

        let transposed = super::transpositions(string1, string2, &hash_1, &hash_2);
        assert_eq!(transposed, 0.);
    }

    #[test]
    fn test_transposition_martha_mahrta() {
        let string1 = "Martha";
        let string2 = "Mharta";
        let hash_1 = vec![1, 1, 1, 1, 1, 1];
        let hash_2 = vec![1, 1, 1, 1, 1, 1];

        let transposed = super::transpositions(string1, string2, &hash_1, &hash_2);
        assert_eq!(transposed, 2.);
    }

    // Test Cases for Jaro Score
    #[test]
    fn test_jaro_empty_string() {
        let string1 = "";
        let string2 = "";

        let score = super::jaro_score(string1, string2);
        assert_eq!(score, 0.);
    }

    #[test]
    fn test_jaro_matching_string() {
        let string1 = "Adam";
        let string2 = "Adam";

        let score = super::jaro_score(string1, string2);
        assert_eq!(score, 1.);
    }

    #[test]
    fn test_jaro_duane_dwayne() {
        let string1 = "Duane";
        let string2 = "Dwayne";
        let score = super::jaro_score(string1, string2);
        let expected = 0.82222;
        assert!((score - expected).abs() < 0.01);
    }

    #[test]
    fn test_jaro_martha_mharta() {
        let string1 = "Martha";
        let string2 = "Mharta";
        let score = super::jaro_score(string1, string2);
        let expected = 0.89;
        assert!((score - expected).abs() < 0.01);
    }

    #[test]
    fn test_jaro_crate_trace() {
        let string1 = "crate";
        let string2 = "trace";
        let score = super::jaro_score(string1, string2);
        let expected = 0.733;
        assert!((score - expected).abs() < 0.01);
    }

    // Test Case for Jaro Winkler
    #[test]
    fn test_jaro_winkler_exact_match() {
        use super::config;
        let string1 = "Martha";
        let string2 = "Martha";
        let jw_config = config::JaroWinklerConfigOptions::default();
        let score = super::jaro_winkler_score(string1, string2, &jw_config);
        let expected = 1.0;
        assert_eq!(score, expected);
    }
    #[test]
    fn test_jaro_winkler_duane_dwayne() {
        use super::config;
        let string1 = "Duane";
        let string2 = "Dwayne";
        let jw_config = config::JaroWinklerConfigOptions::default();
        let score = super::jaro_winkler_score(string1, string2, &jw_config);
        let expected = 0.838;
        assert!((score - expected).abs() < 0.01);
    }

    #[test]
    fn test_jaro_winkler_martha_mharta() {
        use super::config;
        let string1 = "Martha";
        let string2 = "Mharta";
        let jw_config = config::JaroWinklerConfigOptions::default();
        let score = super::jaro_winkler_score(string1, string2, &jw_config);
        let expected = 0.901;
        assert!((score - expected).abs() < 0.01);
    }
}

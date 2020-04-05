use super::super::*;
use super::compute;
use super::config;

// JaroWinklerMatcher implements Jaro-Winkler algorithm between
// two names. Between 0.0 and 1.0.
// Only works well for ASCII characters.
// Case Insensitive.
// Basic preprocessing:
//   - Replaces non-alphanumeric with whitespace
//   - Convert to uppercase.
pub struct JaroWinklerMatcher {
    config: config::JaroWinklerConfigOptions,
    weight: f64,
}

impl JaroWinklerMatcher {
    // Implements Jaro Winkler Algorithm
    pub fn new(
        user_config: Option<config::JaroWinklerConfigOptions>,
        weight: Option<f64>,
    ) -> JaroWinklerMatcher {
        let configuration = user_config.unwrap_or(config::JaroWinklerConfigOptions::default());
        let weight = weight.unwrap_or(1.0);
        JaroWinklerMatcher {
            config: configuration,
            weight,
        }
    }

    pub fn default() -> JaroWinklerMatcher {
        let config = config::JaroWinklerConfigOptions::default();
        JaroWinklerMatcher {
            config,
            weight: 1.0,
        }
    }
}

impl Clean for JaroWinklerMatcher {}

impl Compare for JaroWinklerMatcher {
    fn compare(&self, s1: &str, s2: &str) -> f64 {
        let s1 = self.clean(s1);
        let s2 = self.clean(s2);
        self.weight * compute::jaro_winkler_score(&s1[..], &s2[..], &self.config)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::*;
    #[test]
    fn test_case_sensitive_match() {
        let matcher = super::JaroWinklerMatcher::default();
        let name1 = "john doe";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.compare(name1, name2), 1.0);
    }

    #[test]
    fn test_ignore_special_character_match() {
        let matcher = super::JaroWinklerMatcher::default();
        let name1 = "joh^ doe";
        let name2 = "joh**doe";
        assert_eq!(matcher.compare(name1, name2), 1.0);
    }

    #[test]
    fn test_whitespaces_match() {
        let matcher = super::JaroWinklerMatcher::default();
        let name1 = "  john doe   ";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.compare(name1, name2), 1.0);
    }

    #[test]
    fn test_weighted_match() {
        let matcher = super::JaroWinklerMatcher::new(
            None::<jaro::config::JaroWinklerConfigOptions>,
            Some(0.5),
        );
        let name1 = "JOHN DOE";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.compare(name1, name2), 0.5);
    }
}

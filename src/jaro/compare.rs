use super::super::prelude::*;
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
    name: String,
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
            name: "Jaro-Winkler".to_owned(),
            config: configuration,
            weight,
        }
    }

    pub fn default() -> JaroWinklerMatcher {
        let config = config::JaroWinklerConfigOptions::default();
        JaroWinklerMatcher {
            name: "Jaro-Winkler".to_owned(),
            config,
            weight: 1.0,
        }
    }
}

impl Named for JaroWinklerMatcher {
    fn get_name(&self) -> &str {
        &self.name[..]
    }
}

impl Clean for JaroWinklerMatcher {}

impl Weighted for JaroWinklerMatcher {
    fn get_weight(&self) -> f64 {
        self.weight
    }
}

impl Matcher for JaroWinklerMatcher {
    fn get_score(&self, s1: &str, s2: &str) -> f64 {
        let s1 = self.clean(s1);
        let s2 = self.clean(s2);
        compute::jaro_winkler_score(&s1[..], &s2[..], &self.config)
    }
}

#[cfg(test)]
mod test {
    use super::super::super::prelude::*;
    use super::super::config;
    #[test]
    fn test_case_sensitive_match() {
        let matcher = super::JaroWinklerMatcher::default();
        let name1 = "john doe";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.get_score(name1, name2), 1.0);
    }

    #[test]
    fn test_ignore_special_character_match() {
        let matcher = super::JaroWinklerMatcher::default();
        let name1 = "joh^ doe";
        let name2 = "joh**doe";
        assert_eq!(matcher.get_score(name1, name2), 1.0);
    }

    #[test]
    fn test_whitespaces_match() {
        let matcher = super::JaroWinklerMatcher::default();
        let name1 = "  john doe   ";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.get_score(name1, name2), 1.0);
    }

    #[test]
    fn test_half_weight() {
        let matcher =
            super::JaroWinklerMatcher::new(None::<config::JaroWinklerConfigOptions>, Some(0.5));
        let name1 = "JOHN DOE";
        let name2 = "JOHN DOE";
        assert_eq!(matcher.get_weighted_score(name1, name2), 0.5);
    }
}

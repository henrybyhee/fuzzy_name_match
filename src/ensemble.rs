use super::prelude::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EnsembleResult {
    pub name1: String,
    pub name2: String,
    pub score: f64,
    pub results: Vec<MatchResult>,
}

pub struct Ensemble {
    pub matchers: Vec<Box<dyn Matcher>>,
}

impl Ensemble {
    pub fn new(matchers: Vec<Box<dyn Matcher>>) -> Ensemble {
        Ensemble { matchers }
    }

    pub fn set_equal_weight(&mut self) {
        let count = self.matchers.len();
        let weight = 1. / (count as f64);
        for i in 0..count {
            self.matchers[i].set_weight(weight);
        }
    }

    pub fn get_aggregated_score(&self, name1: &str, name2: &str) -> f64 {
        let mut sum = 0.0;
        for matcher in self.matchers.iter() {
            let score = matcher.get_weighted_score(name1, name2);
            sum += score
        }
        sum
    }

    pub fn get_match_results(&self, name1: &str, name2: &str) -> Vec<MatchResult> {
        let mut results: Vec<MatchResult> = Vec::new();
        for matcher in self.matchers.iter() {
            let match_result = matcher.get_match_result(name1, name2);
            results.push(match_result);
        }
        results
    }

    pub fn get_ensemble_result(&self, name1: &str, name2: &str) -> EnsembleResult {
        let results = self.get_match_results(name1, name2);
        let score = results.iter().map(|result| result.weighted_score).sum();

        EnsembleResult {
            name1: name1.to_owned(),
            name2: name2.to_owned(),
            score,
            results,
        }
    }

    pub fn get_ensemble_result_arr(
        &self,
        query_name: &str,
        name_list: Vec<&str>,
    ) -> Vec<EnsembleResult> {
        name_list
            .par_iter()
            .map(|name| self.get_ensemble_result(query_name, name))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::super::jaro::compare::JaroWinklerMatcher;
    use super::super::prelude::*;
    use super::super::soundex::compare::SoundexMatcher;

    #[test]
    fn test_ensemble_set_equal_weight() {
        let jw = JaroWinklerMatcher::default();
        let soundex = SoundexMatcher::default();

        let matchers: Vec<Box<dyn Matcher>> = vec![Box::new(jw), Box::new(soundex)];

        let mut ensemble = super::Ensemble::new(matchers);
        ensemble.set_equal_weight();

        for matcher in ensemble.matchers.iter() {
            assert_eq!(matcher.get_weight(), 0.5);
        }
    }

    #[test]
    fn test_ensemble_get_aggregated_score() {
        let name1 = "John Doe";
        let name2 = "John Doe";

        let jw = JaroWinklerMatcher::default();
        let soundex = SoundexMatcher::default();

        let matchers: Vec<Box<dyn Matcher>> = vec![Box::new(jw), Box::new(soundex)];

        let mut ensemble = super::Ensemble::new(matchers);
        ensemble.set_equal_weight();

        let score = ensemble.get_aggregated_score(name1, name2);

        assert_eq!(score, 1.0);
    }
}

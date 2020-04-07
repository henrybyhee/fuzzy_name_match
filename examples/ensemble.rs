extern crate name_match;

use name_match::jaro::compare::JaroWinklerMatcher;
use name_match::soundex::compare::SoundexJaccardMatcher;
use name_match::ensemble::Ensemble;
use name_match::prelude::*;

fn main(){
    let name_1 = "Bond Jmes";
    let name_2 = "James Sancho Bond";

    // Ensemble method works better.
    // High Precision: Jaro-Winkler captures similarity in terms of edit distance
    let jw_matcher = JaroWinklerMatcher::default();
    // High Recall: Soundex captures phonetic similarity between two names.
    let soundex_matcher = SoundexJaccardMatcher::default();


    let matchers: Vec<Box<dyn Matcher>> = vec![
        Box::new(jw_matcher),
        Box::new(soundex_matcher),
    ];

    let mut ensemble = Ensemble::new(matchers);
    ensemble.set_equal_weight();

    let score = ensemble.get_aggregated_score(name_1, name_2);
    println!("Score = {}", score);
}
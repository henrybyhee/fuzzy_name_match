extern crate name_match;

use name_match::jaro::compare;
use name_match::jaro::config;
use name_match::prelude::*;

fn main() {
    let name_1 = "James Bay";
    let name_2 = "Jason Sancho";

    // Default implementation using
    // Similarity Threshold = 0.7
    // Max Prefix Length = 4
    // Scaling Factor = 0.1
    let name_matcher =
        compare::JaroWinklerMatcher::default();
    let score = name_matcher.get_score(name_1, name_2);
    println!("Jaro-Winkler Similarity = {}", score);

    // Custom config
    let jw_config = config::JaroWinklerConfigOptions::new(0.6, 4, 0.1);
    let name_matcher = compare::JaroWinklerMatcher::new(Some(jw_config), None::<f64>);
    let score = name_matcher.get_score(name_1, name_2);
    println!("Jaro-Winkler Similarity = {}", score)
}

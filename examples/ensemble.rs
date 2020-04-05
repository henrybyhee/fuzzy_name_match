extern crate name_match;

use name_match::jaro::config;
use name_match::jaro::compare::JaroWinklerMatcher;
use name_match::soundex::compare::SoundexJaccardMatcher;
use name_match::jaccard::compare::JaccardMatcher;
use name_match::Compare;

fn main(){
    let name_1 = "Bond Jmes";
    let name_2 = "James Sancho Bond";

    let weight = 1. /2.;

    // Ensemble method works better.
    // Jaro-Winkler captures similarity in terms of edit distance
    let jw_matcher = JaroWinklerMatcher::new(None::<config::JaroWinklerConfigOptions>, Some(weight));
    let jw_score = jw_matcher.compare(name_1, name_2);
    println!("Jaro-Winkler Score (50%) = {}", jw_score);

    // Soundex captures phonetic similarity between two names.
    let soundex_matcher = SoundexJaccardMatcher::new(Some(weight));
    let soundex_score = soundex_matcher.compare(name_1, name_2);
    println!("Soundex Score (50%) = {}", soundex_score);

    let combined = jw_score + soundex_score;
    println!("Combined = {}", combined);
}
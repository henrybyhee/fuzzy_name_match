extern crate name_match;

use name_match::jaro::config;
use name_match::jaro::compare::JaroWinklerMatcher;
use name_match::soundex::compare::SoundexJaccardMatcher;
use name_match::jaccard::compare::JaccardMatcher;
use name_match::Compare;

fn main(){
    let name_1 = "Bond Jmes";
    let name_2 = "James Sancho Bond";

    let weight = 1. / 3.;

    // Hybrid method works better.
    // Jaro-Winkler captures similarity in terms of edit distance
    let jw_matcher = JaroWinklerMatcher::new(None::<config::JaroWinklerConfigOptions>, Some(weight));
    let jw_score = jw_matcher.compare(name_1, name_2);
    println!("Jaro-Winkler Score (33%) = {}", jw_score);

    // Soundex captures phonetic similarity between two names.
    let soundex_matcher = SoundexJaccardMatcher::new(Some(weight));
    let soundex_score = soundex_matcher.compare(name_1, name_2);
    println!("Soundex Score (33%) = {}", soundex_score);

    // Jaccard Index handles first name last name transposition and missing name component.
    let jaccard_matcher = JaccardMatcher::new(Some(weight));
    let jaccard_score = jaccard_matcher.compare(name_1, name_2);
    println!("Jaccard index (33%) = {}", jaccard_score);

    let combined = jw_score + soundex_score + jaccard_score;
    println!("Combined = {}", combined);
}
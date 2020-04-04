extern crate name_match;

use name_match::soundex::compare;
use name_match::Compare;

fn main(){
    let name_1 = "James Bay";
    let name_2 = "Jason Bee";

    // Soundex matcher uses Soundex algorithm to encode each name
    let soundex_matcher = compare::SoundexMatcher::default();
    let score = soundex_matcher.compare(name_1, name_2);
    println!("Soundex Similarity = {}", score);


    // Soundex(Jaccard) Matcher tokenizes names by whitespace first,
    // encode token as Soundex code,
    // and measure their similarity usin Jaccard Index.
    let soundex_jaccard_matcher = compare::SoundexJaccardMatcher::default();
    let score = soundex_jaccard_matcher.compare(name_1, name_2);
    println!("Soundex(Jaccard) Similarity = {}", score);
}
extern crate name_match;

use name_match::jaro::compare;
use name_match::Compare;

fn main() {
    let name_1 = "James Bay";
    let name_2 = "Jason Sancho";

    let name_matcher =
        compare::JaroWinklerMatcher::default();
    let score = name_matcher.compare(name_1, name_2);
    println!("Jaro-Winkler Similarity = {}", score);
}

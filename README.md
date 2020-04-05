# Name Match
Popular fuzzy name matching algorithms written in Rust. Name matching is not easy due to variation arising from spelling,
missing components and pronunciation. These algorithms are designed to
capture similarity between non-identical name sequences by assigning probability to a match between
0.0 and 1.0. Input names are preprocessed and transformed before comparison.

# Implementation Notes

You'll find the following fuzzy string matching algorithms in this library:

1. **[Jaro Winkler Distance](#1-jaro-winkler-distance-classic)**
2. **[Jaccard Index](#2-jaccard-index-classic)**
3. **[Soundex](#3-soundex-classic)**
4. **[Soundex-Jaccard](#4-soundex-jaccard-custom)**

Each of these algorithms excel at solving different challenges of name matching. You'll find that they are complementary in nature.
To better tackle the challenge of name matching, each algorithm comes with a preprocessing stage and a comparison stage.
Input names undergo a series of preprocessing first before similary score is calculated in the subsequent comparison stage.


## 1. Jaro Winkler Distance (Classic)

Measures edit distance between two strings. The higher the score, the more similar the strings are. See [wikipedia](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance).

### Use Case

**Strength:**

- Spelling mistakes and typos such as 'John' vs 'Jon', 'SmellyFish' vs 'JellyFish'.
- High Precision

**Weakness:**

Name transpositions such as 'John Doe' vs 'Doe, John'. Also lacking linguistic nuances

### Preprocessing

- Non alpha-numeric characters are converted to whitespace.
- Trim leading and ending whitespaces.
- All characters are converted to uppercase.

### Usage

Refer to [example](/examples/jaro_winkler.rs).

```rust
let name1 = "John Doe"
let name2 = "Jon Doe"
let name_matcher = compare::JaroWinklerMatcher::default();
let score = name_matcher.compare(name_1, name_2); //0.9666667
```

## 2. Jaccard Index (Classic)

Measures overlapping tokens between two strings, defined by intersection divided by the size of their union. The higher the score, the more similar the strings are.
Names are tokenized by whitespace before score is calculated. See [wikipedia](https://en.wikipedia.org/wiki/Jaccard_index).

### Use Case

**Strength:**

- Name transpositions. 'John Doe' vs 'Doe John'.
- Missing name components. 'John Adam Doe' vs 'John Doe'

**Weakness:**

Spelling mistakes. Slight character variation within a token can drastically reduce string similarity.

### Preprocessing

- Non alpha-numeric characters are converted to whitespace.
- Trim leading and ending whitespaces.
- All characters are converted to uppercase.

### Usage

Refer to [example](/examples/jaccard.rs).

```rust
let name1 = "John Doe"
let name2 = "Jon Doe"
let name_matcher = compare::JaccardMatcher::default();
let score = name_matcher.compare(name_1, name_2); // 0.33333
```

## 3. Soundex (Classic)

Measures phoentic similarity between strings. Names are encoded to their Soundex form before comparison. See [wikipedia](https://en.wikipedia.org/wiki/Soundex).
Score is binary [0.0, 1.0].

### Use Case

**Strength:**

- Similar sounding names "Robert" vs "Rupert"

**Weakness:**

- Low Precision.
- Last name is omitted using naive implementation. "Robert Doe" (R163) vs "Rupert John" (R163) (See [improvement](#4-soundexjaccard-custom))
- Name transposition

### Preprocessing

- Non alpha-numeric characters are converted to whitespace.
- Trim leading and ending whitespaces.
- All characters are converted to uppercase.
- Reduced to their Soundex form.

### Usage

Refer to [example](/examples/soundex.rs).

```rust
let name1 = "Roberto Doe"
let name2 = "Rupert Doe"
let name_matcher = compare::SoundexMatcher::default();
let score = name_matcher.compare(name_1, name_2); // 1.0
```

## 4. Soundex-Jaccard (Custom)

An improvement over [Naive Soundex](#3-soundex-classic). Each name is tokenized (splitting by first whitespace) first, before encoding every token to Soundex form. Jaccard Index of the two names
are calculated. The higher the score, the more similar the strings.

**Illustration**: "John Robert Doe" vs "Jim Rupert"

1. Tokenized:
   - {"John", "Robert", "Doe"} vs {"Jim", "Rupert"}
2. Apply soundex:
   - {"J500", "R163", "D00"} vs {"J500", "R163"}
3. Jaccard Index:
   - Ovelapping = {"J500", "R163"}
   - Union = {"J500", "R163", "D00"}  
   - 2/3 = 0.66667

### Use Case

**Strength:**

- Higher recall than Classic Soundex.
- Every components within a name are taken into consideration.
- Works on transposed name. "Rubert Jim" vs "John Rupert"

**Weakness:**

- Low Precision.

### Preprocessing

- Non alpha-numeric characters are converted to whitespace.
- Trim leading and ending whitespaces.
- All characters are converted to uppercase.
- Reduced to their Soundex form.

### Usage

Refer to [example](/examples/soundex.rs).

```rust
let name1 = "Roberto Doe"
let name2 = "Rupert Doe"
let name_matcher = compare::SoundexJaccardMatcher::default();
let score = name_matcher.compare(name_1, name_2); // 1.0
```

## General Notes

Each algorithm has its set of weaknesses in matching names. Hence, a better approach would be to construct an ensemble model by combining two or more algorithms.
It would be useful to have a balanced mix of algorithms with high recall and high precision, weighting their score equally. This can be achieved using this 
library by setting the `weight` of every `Matcher` instances upfront.

Refer to [example](/examples/ensemble.rs).


## Applications

- Financial Compliance (Sanction List, PEP, etc)
- Identity Verification

## Current Limitations

- Only works on ASCII characters.

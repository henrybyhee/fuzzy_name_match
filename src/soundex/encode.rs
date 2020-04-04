use std::collections::HashSet;

fn map_char_to_number(letter: &char) -> char {
    match letter {
        'B' | 'F' | 'P' | 'V' => '1',
        'C' | 'G' | 'J' | 'K' | 'Q' | 'S' | 'X' | 'Z' => '2',
        'D' | 'T' => '3',
        'L' => '4',
        'M' | 'N' => '5',
        'R' => '6',
        _ => '0',
    }
}

pub fn apply_soundex(s1: &str) -> String {
    if s1.len() == 0 {
        return String::new();
    }
    let mut s1_copy = s1.to_owned();
    let mut res = String::new();

    // Step 1: Remove all non characters
    s1_copy.retain(|c| c.is_alphabetic());
    s1_copy.make_ascii_uppercase();

    // Step 2: Keep the first letter
    let first_letter = s1_copy.remove(0);
    res.push(first_letter);

    // Step 3: Filter out H, W, Y
    let hwy_set: HashSet<char> = vec!['H', 'W', 'Y'].into_iter().collect();
    s1_copy.retain(|c| !hwy_set.contains(&c));

    // Step 4: Encode according to soundex rule
    let vowels: HashSet<char> = vec!['A', 'E', 'I', 'O', 'U'].into_iter().collect();
    let mut prev_letter: char = first_letter;
    let mut prev_code: char = map_char_to_number(&first_letter);
    let mut was_vowel = vowels.contains(&first_letter);
    for letter in s1_copy.chars() {
        if vowels.contains(&letter) {
            was_vowel = true;
            continue;
        }
        let code = map_char_to_number(&letter);
        // Side by side rule
        if code == prev_code || prev_letter == letter {
            if was_vowel {
                // Consonant to the right of vowel should be added.
                res.push(code);
            }
            was_vowel = false;
            prev_code = code;
            prev_letter = letter;
            continue;
        }
        was_vowel = false;
        prev_code = code;
        prev_letter = letter;
        res.push(code);
    }

    // Step 5: Transform result to length 4
    let res_length_offset: i32 = res.len() as i32 - 4;
    if res_length_offset < 0 {
        let res_length_offset = res_length_offset.abs();
        // Pad with 0
        for _ in 0..res_length_offset {
            res.push('0');
        }
        return res;
    } else if res_length_offset > 0 {
        // Needs slicing
        let sliced = &res[..4];
        return sliced.to_owned();
    }
    res
}

#[cfg(test)]
mod test {
    use super::apply_soundex;
    #[test]
    fn test_apply_soundex_robert() {
        let name = "Robert";
        let code = apply_soundex(name);
        assert_eq!(code, "R163");
    }

    #[test]
    fn test_apply_soundex_gutierrex() {
        let name = "Gutierrez";
        let code = apply_soundex(name);
        assert_eq!(code, "G362");
    }

    #[test]
    fn test_apply_soundex_pfister() {
        let name = "Pfister";
        let code = apply_soundex(name);
        assert_eq!(code, "P236");
    }

    #[test]
    fn test_apply_soundex_jackson() {
        let name = "Jackson";
        let code = apply_soundex(name);
        assert_eq!(code, "J250");
    }

    #[test]
    fn test_apply_soundex_tymczak() {
        let name = "Tymczak";
        let code = apply_soundex(name);
        assert_eq!(code, "T522");
    }

    #[test]
    fn test_apply_soundex_andy() {
        let name = "Andy";
        let code = apply_soundex(name);
        assert_eq!(code, "A530");
    }
}

pub fn longest_vowel_substring(value: &str) -> (String, usize) {
    let vowels = "aeiouAEIOU";
    let mut max_substr = String::new();
    let mut current_substr = String::new();
    for ch in value.chars() {
        if vowels.contains(ch) {
            current_substr.push(ch);
        } else {
            if current_substr.len() > max_substr.len() {
                max_substr = current_substr.clone();
            }
            current_substr.clear();
        }
    }
    if current_substr.len() > max_substr.len() {
        max_substr = current_substr;
    }
    let length = max_substr.len();
    (max_substr, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_vowel_substring() {
        let (substr, length) = longest_vowel_substring("earthproblem");
        assert_eq!(substr, "ea");
        assert_eq!(length, 2);

        let (substr, length) = longest_vowel_substring("letsgosomewhereeeee");
        assert_eq!(substr, "eeeee");
        assert_eq!(length, 5);

        let (substr, length) = longest_vowel_substring("beautiful");
        assert_eq!(substr, "eau");
        assert_eq!(length, 3);

        let (substr, length) = longest_vowel_substring("rhythm");
        assert_eq!(substr, "");
        assert_eq!(length, 0);

        let (substr, length) = longest_vowel_substring("AEIOUxyz");
        assert_eq!(substr, "AEIOU");
        assert_eq!(length, 5);
    }
}

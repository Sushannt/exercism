use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let mut matched = HashSet::new();
    let word_norm = normalize(word);

    for &candidate in possible_anagrams {
        if word.to_lowercase() == candidate.to_lowercase() {
            continue;
        }

        if normalize(candidate) == word_norm {
            matched.insert(candidate);
        }
    }
    
    matched

}

fn normalize(s: &str) -> Vec<char> {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars
}
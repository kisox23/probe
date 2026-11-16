use rand::prelude::*;

const SYLLABLES: &[&str] = &[
    "ka", "be", "ni", "wol", "sa", "shi", "om", "ap", "gen", "bil", "gor", "ra", "exe", "zo", "ur",
    "fi", "vel", "sai", "gon", "hen", "hyu", "the", "dir", "gan", "liu", "ai", "ei", "ui", "jo",
    "ek", "ed", "rok", "man", "to", "ny", "ze", "low", "lim", "ass", "fal", "stat", "oi", "xi",
    "wan", "wa", "wo", "ess", "eim", "ra",
];

const NAMES: &[&str] = &[
    "IP", "LO", "AK", "FEI", "MA", "GO", "NO", "KI", "TEI", "UU", "UL", "ROI", "KE", "A", "DO",
];

pub fn make_name(n_syllables: usize, name: bool) -> String {
    let mut rng = rand::rng();
    let mut choices: Vec<&str> = Vec::new();

    if name {
        for _ in 0..n_syllables {
            if let Some(syllable) = NAMES.choose(&mut rng) {
                choices.push(syllable);
            }
        }
        return choices.concat();
    }

    for _ in 0..n_syllables {
        if let Some(syllable) = SYLLABLES.choose(&mut rng) {
            choices.push(syllable);
        }
    }

    if n_syllables >= 5 {
        let half = choices.len() / 2;

        let (first, last) = choices.split_at(half);

        let a = capital_letter(&first.concat());
        let b = capital_letter(&last.concat());

        format!("{} {}", a, b)
    } else {
        capital_letter(&choices.concat())
    }
}

fn capital_letter(word: &str) -> String {
    let mut chars = word.chars(); // UTF-8, not single bytes!
    match chars.next() {
        None => String::new(),
        Some(before) => before.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

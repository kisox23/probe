use rand::prelude::*;

const SYLLABLES: &[&str] = &[
    "ka", "be", "ni", "wol", "sa", "shi", "om", "ap", "gen", "bil", "gor", "ra", "exe", "zo", "ur",
    "fi", "vel", "sai", "gon", "hen", "hyu", "the", "dir", "gan", "liu", "ai", "ei", "ui", "jo",
    "ek", "ed", "rok", "man", "to", "ny", "ze", "low", "lim", "ass", "fal", "stat", "oi", "xi",
    "wan", "wa", "wo", "ess", "eim", "ra", "tak", "gyo", "zai", "ber", "uli", "opa", "al", "tok",
    "coll", "ral", "vn", "iri",
];

const NAMES: &[&str] = &[
    "IP", "LO", "AK", "FEI", "MA", "GO", "NO", "KI", "TEI", "UU", "UL", "ROI", "KE", "A", "DO",
    "FA", "FEL", "UT", "JA", "CON", "R", "EI",
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

// implementing a Markov Model; make_name will be deprecated
use std::collections::HashMap;

const HOT_CORPUS: &[&str] = &[
    "ignis", "terrax", "vulkan", "roggen", "rockas", "montan", "magma", "surt", "drak", "trok",
    "pyra", "skor", "rax", "kill", "geo", "morfo", "vex", "heiss", "foyr", "khor", "kryx",
    "ferrum", "cinder", "pyre", "solis", "krag", "obsidian", "ember", "caldera", "sulfur",
    "therma", "basalt", "scorch", "ignar", "molten", "ashen",
];

const COLD_CORPUS: &[&str] = &[
    "kalt", "cold", "vern", "wint", "ymir", "frost", "cryo", "boreas", "glacis", "weiss", "fenrir",
    "hel", "heim", "bjorn", "skol", "fjord", "svart", "vinter", "isbjorn", "uppsal", "frost",
    "tundra", "cryos", "nimbus", "kald", "glacier", "rime", "svalbard", "hyperion", "arctic",
    "hail", "shiver", "winter", "frozen", "mist",
];

const MILD_CORPUS: &[&str] = &[
    "geas", "aralia", "floral", "sylva", "lunala", "solis", "velaris", "oceanis", "tever", "valen",
    "orelia", "elysia", "aeloria", "iovis", "mars", "jupiter", "venus", "proxim", "garden", "ulif",
    "verdant", "silva", "zephyr", "lyra", "nova", "gaia", "terran", "aurelia", "meadow", "cascade",
    "halcyon", "calypso", "arcadia", "eden", "flora",
];

const SUFFIXES: &[&str] = &[
    "I", "II", "III", "IV", "V", "VI", "VII", "IX", "X", "Prime", "Major", "Minor", "Alpha",
    "Beta", "Gamma", "Delta", "Ultima",
];

pub struct MarkovModel {
    transitions: HashMap<char, Vec<char>>,
    // given a two-char window, give me the third one
}

impl MarkovModel {
    pub fn train(corpus: &[&str]) -> Self {
        let mut transitions: HashMap<char, Vec<char>> = HashMap::new();

        for word in corpus {
            let padded = format!("^{}$", word.to_lowercase()); // add tags for initial-final letters 
            let chars: Vec<char> = padded.chars().collect(); // create iterable of chars for every word

            for window in chars.windows(2) {
                // take a window of 3 chars
                let state = window[0]; // use the first two as reference
                let next = window[1]; // use the last as info 

                transitions.entry(state).or_default().push(next); // store the data
            }
        }

        Self { transitions }
    }

    pub fn generate(&self, min_len: usize, max_len: usize) -> String {
        let mut rng = rand::rng();
        let mut word = String::new();
        let mut state = '^'; // initial state

        while let Some(choices) = self.transitions.get(&state) {
            let &next = choices.choose(&mut rng).unwrap();

            // if it chooses '$' word ends
            if next == '$' {
                // if it's not long enough given alternatives
                if word.chars().count() < min_len && choices.len() > 1 {
                    continue;
                }
                break;
            }

            word.push(next);
            state = next; // new state is previous with the letter chosen before 

            if word.chars().count() >= max_len {
                break;
            }
        }

        capital_letter(&word)
    }
}

pub struct NameGenerator {
    hot: MarkovModel,
    cold: MarkovModel,
    mild: MarkovModel,
}

impl NameGenerator {
    pub fn new() -> Self {
        Self {
            hot: MarkovModel::train(HOT_CORPUS),
            cold: MarkovModel::train(COLD_CORPUS),
            mild: MarkovModel::train(MILD_CORPUS),
        }
    }

    pub fn make_name(&self, climate_class: u8) -> String {
        let mut rng = rand::rng();
        let model = match climate_class {
            1 => &self.hot,
            2 => &self.cold,
            _ => &self.mild,
        };

        let base_name = model.generate(4, 7);

        match rand::random_range(0..10) {
            0..=2 => {
                let suffix = SUFFIXES.choose(&mut rng).unwrap();
                format!("{}-{}", base_name, suffix)
            }
            3..=5 => {
                let second = model.generate(4, 6);
                format!("{} {}", base_name, second)
            }
            6..=7 => {
                let letter = (rand::random_range(65..=90) as u8) as char;
                let code = rand::random_range(10..99);
                format!("{}-{} {}", letter, code, base_name)
            }
            _ => base_name,
        }
    }
}

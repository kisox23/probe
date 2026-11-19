use std::collections::{HashMap, HashSet};

const TRAINING_DATA: &[(Intent, &str)] = &[
    // --- GREET: Saluti, apertura pacifica, presentazioni ---
    (Intent::Greet, "hello traveler"),
    (Intent::Greet, "hi stranger"),
    (Intent::Greet, "greetings"),
    (Intent::Greet, "hello there"),
    (Intent::Greet, "hey friend"),
    (Intent::Greet, "good to see you"),
    (Intent::Greet, "nice to meet you"),
    (Intent::Greet, "i come in peace"),
    (Intent::Greet, "how are you doing"),
    (Intent::Greet, "what is up"),
    (Intent::Greet, "hail alien"),
    (Intent::Greet, "welcome"),
    // --- TRADE: Verbi e nomi legati a scambio, risorse, rifornimenti ---
    (Intent::Trade, "i want to trade"),
    (Intent::Trade, "let us barter"),
    (Intent::Trade, "can we make a deal"),
    (Intent::Trade, "i want to buy supplies"),
    (Intent::Trade, "i have resources to sell"),
    (Intent::Trade, "show me your wares"),
    (Intent::Trade, "what are your goods"),
    (Intent::Trade, "i need fuel and materials"),
    (Intent::Trade, "looking for raw elements"),
    (Intent::Trade, "do you sell equipment"),
    (Intent::Trade, "i offer an exchange"),
    (Intent::Trade, "open your inventory"),
    // --- ASK: Domande sul pianeta, pericoli, lore e indicazioni ---
    (Intent::Ask, "tell me about this planet"),
    (Intent::Ask, "is this place safe"),
    (Intent::Ask, "what dangers are here"),
    (Intent::Ask, "where can i find resources"),
    (Intent::Ask, "describe the climate"),
    (Intent::Ask, "who are you"),
    (Intent::Ask, "what is this world like"),
    (Intent::Ask, "any advice for exploring"),
    (Intent::Ask, "explain this sector to me"),
    (Intent::Ask, "what species live here"),
    (Intent::Ask, "i need information"),
    (Intent::Ask, "where should i go next"),
    // --- BYE: arrivederci a tutti
    (Intent::Bye, "bye bye"),
    (Intent::Bye, "goodbye"),
    (Intent::Bye, "i have to go"),
    (Intent::Bye, "see you"),
    (Intent::Bye, "bye now"),
    (Intent::Bye, "i must leave"),
    (Intent::Bye, "i am leaving"),
    (Intent::Bye, "farewell"),
];

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Intent {
    Greet,
    Trade,
    Ask,
    Bye,
}

pub struct Classifier {
    frequencies: HashMap<(Intent, String), usize>,
    words_per_intent: HashMap<Intent, usize>,
    sentences_per_intent: HashMap<Intent, usize>,
    word_set: HashSet<String>,
}

impl Classifier {
    pub fn predict(&self, sentence: &str) -> Intent {
        let tokens = tokenize(sentence);

        let total_sentences = TRAINING_DATA.len() as f64;
        let vocab_size = self.word_set.len() as f64;

        let mut best_score = f64::NEG_INFINITY;
        let mut best_intent = Intent::Greet;

        for (&intent, &n_sentences) in &self.sentences_per_intent {
            // calculate prior
            let mut score = (n_sentences as f64 / total_sentences).ln();

            let intent_word_total = *self.words_per_intent.get(&intent).unwrap_or(&0) as f64;

            // calculate the likelihood
            for token in &tokens {
                let count = *self.frequencies.get(&(intent, token.clone())).unwrap_or(&0) as f64;
                let prob = (count + 1.0) / (intent_word_total + vocab_size);
                score += prob.ln();
            }

            if score > best_score {
                best_score = score;
                best_intent = intent;
            }
        }

        best_intent
    }
}

pub fn tokenize(text: &str) -> Vec<String> {
    let lower = text.to_lowercase();

    let mut tokens: Vec<String> = Vec::new();
    for element in lower.split_whitespace() {
        let token: String = element.chars().filter(|c| c.is_alphabetic()).collect();
        if !token.is_empty() {
            tokens.push(token);
        }
    }

    tokens
}

pub fn train() -> Classifier {
    let mut frequencies: HashMap<(Intent, String), usize> = HashMap::new();
    let mut words_per_intent: HashMap<Intent, usize> = HashMap::new();
    let mut sentences_per_intent: HashMap<Intent, usize> = HashMap::new();
    let mut word_set: HashSet<String> = HashSet::new();

    // count how many occurences per Intent
    for &(intent, sentence) in TRAINING_DATA {
        *sentences_per_intent.entry(intent).or_insert(0) += 1;
        for token in tokenize(sentence) {
            *words_per_intent.entry(intent).or_insert(0) += 1;
            *frequencies.entry((intent, token.clone())).or_insert(0) += 1;
            word_set.insert(token);
        }
    }

    Classifier {
        frequencies,
        words_per_intent,
        sentences_per_intent,
        word_set,
    }
}

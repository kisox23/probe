use std::collections::HashMap;
use std::fmt;
use std::fmt::write;

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::models::Planet;
use crate::planet_generator::Element::*;
use crate::planet_generator::Material::*;
use crate::word_generator;

const HOT_CLIMATE: &[&str] = &[
    "Acid",
    "Burning",
    "Corrosive",
    "Extreme",
    "Insular",
    "Quaggy",
    "Subtropical",
];

const COLD_CLIMATE: &[&str] = &["Frigid", "Glacial", "Nordic", "Polar", "Unsettled", "Windy"];

const MILD_CLIMATE: &[&str] = &[
    "Dainty",
    "Humid",
    "Jungly",
    "Littoral",
    "Mild",
    "Oceanic",
    "Temperate",
    "Vernal",
    "Rainy",
];

const CLASSES: &[&str] = &[
    "Gas Giant",
    "Lush",
    "Rocky",
    "Satellite",
    "Lifeless",
    "Resonant",
    "Familiar",
    "Radioactive",
    "Abandoned",
    "Moon",
];

pub const ELEMENTS: &[Element] = &[
    Iron, Titanium, Silicon, Uranium, Hydrogen, Helium, Nitrogen, Argon,
];

pub const MATERIALS: &[Material] = &[Wood, Fiber, Oil, Stone, Dust, Plastic];

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Item {
    Mat(Material),
    Ele(Element),
    Misc(SpaceFinding),
    Uti(Util),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Mat(m) => write!(f, "{:?}", m),
            Item::Ele(e) => write!(f, "{:?}", e),
            Item::Misc(misc) => write!(f, "{:?}", misc),
            Item::Uti(u) => write!(f, "{:?}", u),
        }
    }
}

#[derive(Debug, EnumIter, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum Material {
    Wood,
    Fiber,
    Oil,
    Stone,
    Dust,
    Plastic,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum Util {
    Canister,
    Fruit,
}

impl fmt::Display for Util {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for SpaceFinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum Element {
    Iron,
    Titanium,
    Silicon,
    Uranium,

    Hydrogen,
    Helium,
    Nitrogen,
    Argon,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub enum SpaceFinding {
    Capsule,
    Wire,
    Letter,
}

pub fn generate(next_id: u64) -> Planet {
    let mut rng = rand::rng();
    let name = word_generator::make_name(rand::random_range(2..8), false);
    let class = CLASSES.choose(&mut rng).unwrap();
    let inhabitated = rand::random_bool(0.08);

    let materials = generate_materials();

    let climate_class = rand::random_range(0..3);
    match climate_class {
        0 => {
            let climate = MILD_CLIMATE.choose(&mut rng).unwrap();
            let temperature = rand::random_range(10..=35);

            let elements = generate_resources(temperature);

            Planet {
                id: next_id,
                name,
                class: class.to_string(),
                climate: climate.to_string(),
                temperature,
                elements,
                materials,
                inhabitated,
            }
        }
        1 => {
            let climate = HOT_CLIMATE.choose(&mut rng).unwrap();
            let temperature = rand::random_range(36..=100);

            let elements = generate_resources(temperature);

            Planet {
                id: next_id,
                name,
                class: class.to_string(),
                climate: climate.to_string(),
                temperature,
                elements,
                materials,
                inhabitated,
            }
        }
        2 => {
            let climate = COLD_CLIMATE.choose(&mut rng).unwrap();
            let temperature = rand::random_range(-100..=9);

            let elements = generate_resources(temperature);

            Planet {
                id: next_id,
                name,
                class: class.to_string(),
                climate: climate.to_string(),
                temperature,
                elements,
                materials,
                inhabitated,
            }
        }
        _ => Planet {
            id: next_id,
            name: String::from("Unknown"),
            class: String::from("Lifeless"),
            climate: String::from("Temperate"),
            temperature: 0,
            elements: HashMap::new(),
            materials,
            inhabitated,
        },
    }
}

fn generate_resources(temperature: i32) -> HashMap<Element, f32> {
    // function that associates a percentage of spawning to a material based on temperature
    let mut raw_weights: HashMap<Element, f32> = HashMap::new();

    let heat_factor = ((temperature + 200) as f32 / 400.0).clamp(0.0, 1.0);
    let cold_factor = 1.0 - heat_factor;

    // hot materials
    raw_weights.insert(
        Element::Iron,
        (10.0 + heat_factor * 60.0) * rand::random_range(0.7..1.3),
    );
    raw_weights.insert(
        Element::Silicon,
        (15.0 + heat_factor * 50.0) * rand::random_range(0.7..1.3),
    );
    raw_weights.insert(
        Element::Titanium,
        (5.0 + heat_factor * 25.0) * rand::random_range(0.5..1.5),
    );
    raw_weights.insert(
        Element::Uranium,
        (2.0 + heat_factor * 15.0) * rand::random_range(0.2..1.8),
    );

    // cold materials
    raw_weights.insert(
        Element::Hydrogen,
        (5.0 + cold_factor * 70.0) * rand::random_range(0.7..1.3),
    );
    raw_weights.insert(
        Element::Helium,
        (5.0 + cold_factor * 50.0) * rand::random_range(0.7..1.3),
    );
    raw_weights.insert(
        Element::Nitrogen,
        (10.0 + cold_factor * 30.0) * rand::random_range(0.5..1.5),
    );
    raw_weights.insert(
        Element::Argon,
        (2.0 + cold_factor * 15.0) * rand::random_range(0.2..1.8),
    );

    let total_weight: f32 = raw_weights.values().sum();

    let mut percentages: HashMap<Element, f32> = HashMap::new();
    for (elem, weight) in raw_weights {
        let pct = (weight / total_weight) * 100.0;
        percentages.insert(elem, pct);
    }

    percentages
}

fn generate_materials() -> Vec<Material> {
    let mut materials: Vec<Material> = Vec::new();

    for elem in Material::iter() {
        let prob = rand::random_bool(0.4);
        if prob {
            materials.push(elem);
        }
    }

    materials
}

use rand::prelude::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::item::{Element, Material};
use crate::models::Planet;
use crate::word_generator::NameGenerator;

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

pub fn generate(next_id: u64, name_gen: &NameGenerator) -> Planet {
    let mut rng = rand::rng();
    // deprecated -> let name = word_generator::make_name(rand::random_range(2..8), false);
    let name = name_gen.make_name(rand::random_range(1..=6));
    let class = CLASSES.choose(&mut rng).unwrap();
    let inhabitated = if *class == "Lifeless" {
        false
    } else {
        rand::random_bool(0.1)
    };

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

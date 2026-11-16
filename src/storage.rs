use crate::inventory::Inventory;
use crate::models::{GameState, Planet};
use crate::planet_generator::Material::*;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

const SAVE_PATH: &str = "saves/state.json";

pub fn add_planet(planet: &Planet) -> io::Result<()> {
    fs::create_dir_all("saves/planets")?;
    let json_string = serde_json::to_string_pretty(planet).expect("COULDN'T SAVE");

    let path = format!("saves/planets/planet_{}.json", planet.id);
    fs::write(path, json_string)
}

pub fn load() -> GameState {
    if !Path::new(SAVE_PATH).exists() {
        erase_all();

        let inventory = Inventory {
            elements: HashMap::new(),
            materials: HashMap::new(),
            misc: HashMap::new(),
            utils: HashMap::new(),
        };

        return GameState {
            current_id: 0,
            current_planet: 0,
            fuel: 100,
            vigor: 0,
            inventory,
        };
    }

    let content = fs::read_to_string(SAVE_PATH).expect("UNABLE TO READ SAVE FILE");

    let state: GameState = serde_json::from_str(&content).expect("UNABLE TO DESERIALIZE");

    state
}

pub fn save(gs: &GameState) -> io::Result<()> {
    fs::create_dir_all("saves")?;

    let json_string = serde_json::to_string_pretty(&gs).expect("UNABLE TO WRITE SAVE FILE");

    fs::write(SAVE_PATH, json_string)
}

pub fn erase_all() {
    let inventory = Inventory {
        elements: HashMap::new(),
        materials: HashMap::new(),
        misc: HashMap::new(),
        utils: HashMap::new(),
    };

    let state = GameState {
        current_id: 0,
        current_planet: 0,
        fuel: 100,
        vigor: 100,
        inventory,
    };

    let json_string = serde_json::to_string_pretty(&state).expect("ERROR");
    fs::write(SAVE_PATH, json_string).expect("COULDN'T WRITE NEW DATA");

    fs::remove_dir_all("saves/planets").expect("UNABLE TO ERASE DATA");
    fs::create_dir_all("saves/planets").expect("UNABLE TO RESTORE DATA");

    let earth = Planet {
        name: String::from("Earth"),
        id: 0,
        class: String::from("Lush"),
        climate: String::from("Unsettled"),
        temperature: 25,
        elements: HashMap::new(),
        materials: vec![Fiber, Oil, Wood, Stone, Dust],
        inhabitated: true,
    };

    let json_string = serde_json::to_string_pretty(&earth).expect("ERROR");
    fs::write("saves/planets/planet_0.json", json_string).expect("UNABLE TO WRITE DATA");
}

pub fn get_current_planet(gs: &GameState) -> Planet {
    let id = gs.current_planet;
    let path = format!("saves/planets/planet_{}.json", id);
    let json = fs::read_to_string(&path).expect("UNABLE TO READ PLANET");
    serde_json::from_str(&json).expect("UNABLE TO DESERIALIZE PLANET")
}

pub fn list_planets(gs: &GameState) -> Vec<Planet> {
    let mut list: Vec<Planet> = Vec::new();
    for i in 0..=gs.current_id {
        let path = format!("saves/planets/planet_{}.json", i);
        let json_string = fs::read_to_string(&path).expect("UNABLE TO READ JSON");
        let planet: Planet = serde_json::from_str(&json_string).expect("UNABLE TO LOAD LIST");

        list.push(planet);
    }

    list
}

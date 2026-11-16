use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::inventory::Inventory;
use crate::planet_generator::Element;
use crate::planet_generator::Material;
use crate::planet_generator::Util;

#[derive(Debug, Serialize, Deserialize)]
pub struct Planet {
    pub id: u64,
    pub name: String,
    pub class: String,
    pub climate: String,
    pub temperature: i32,
    pub elements: HashMap<Element, f32>,
    pub materials: Vec<Material>,
    pub inhabitated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub current_id: u64,
    pub current_planet: u64,
    pub fuel: i32,
    pub vigor: i32,
    pub inventory: Inventory,
}

impl GameState {
    pub fn consume(&mut self, item: Util) {
        let available = self.inventory.utils.get(&item).copied().unwrap_or(0) > 0;
        if !available {
            println!("No {} found in storage.", item.to_string());
            return;
        }

        match item {
            Util::Canister => {
                if self.fuel == 100 {
                    println!("Tank is already full.");
                    return;
                }
                *self.inventory.utils.get_mut(&item).unwrap() -= 1;
                self.fuel = 100;
            }
            Util::Fruit => {
                *self.inventory.utils.get_mut(&item).unwrap() -= 1;
                self.vigor += 20
            }
        }
    }
}

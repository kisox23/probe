use std::str::FromStr;

use rand::{random_bool, seq::IndexedRandom};

use colored::Colorize;

use crate::{
    ascii_art::{PLANET, PLANET_INHABITATED, clear_and_render},
    encounter::talk,
    input_command,
    item::{
        Element, Item,
        Util::{self, Fruit},
    },
    models::{GameState, Planet},
    storage::get_current_planet,
};

pub fn visit_planet(gs: &mut GameState) {
    let planet = get_current_planet(gs);

    if planet.inhabitated {
        clear_and_render(PLANET_INHABITATED);
    } else {
        clear_and_render(PLANET);
    }
    println!("=== WELCOME TO {} ===", planet.name);

    loop {
        println!("What do you feel up to?");
        let cmd = input_command("Planet >>> ");
        match cmd.as_str() {
            "r" | "retrieve" => retrieve_material(gs, &planet),
            "h" | "harvest" => harvest_element(gs, &planet),
            "t" | "talk" => talk(gs, &planet),
            "b" | "back" => return,
            _ => {
                println!("Not a command");
                continue;
            }
        }
    }
}

fn retrieve_material(gs: &mut GameState, planet: &Planet) {
    if planet.materials.is_empty() {
        println!("{}", "This planet has nothing to offer.".bright_yellow());
        return;
    }

    if !tire(gs, planet) {
        println!("{}", "You're too tired for this now.".red());
        return;
    }

    let mut rng = rand::rng();
    if let Some(loot) = planet.materials.choose(&mut rng) {
        gs.inventory.update_item(&Item::Mat(loot.clone()), 1);
        let mut add = String::new();
        if rand::random_bool(0.25) {
            gs.inventory.update_item(&Item::Uti(Fruit), 1);
            add = format!(" and {} too.", Util::Fruit.to_string());
        }
        println!("Found {}{}", loot.to_string(), add);
    }
}

fn harvest_element(gs: &mut GameState, planet: &Planet) {
    if !tire(gs, planet) {
        println!("{}", "You're too tired for this now.".red());
        return;
    }

    if gs.inventory.get_qty(&Item::Uti(Util::Harvester)) == 0 {
        println!("You need a Harvester.");
        return;
    }

    let mut rng = rand::rng();
    let entries: Vec<(&Element, &f32)> = planet.elements.iter().collect();

    if let Ok((loot, _)) = entries.choose_weighted(&mut rng, |&(_, pct)| *pct) {
        gs.inventory.update_item(&Item::Ele((*loot).clone()), 1);
        println!("You managed to get some {}.", loot.to_string());
    }

    if random_bool(0.05) {
        gs.inventory.update_item(&Item::Uti(Util::Harvester), -1);
        println!("{}", "Your harvester broke!".yellow());
    }
}

fn tire(gs: &mut GameState, planet: &Planet) -> bool {
    if planet.temperature > 40 || planet.temperature < 0 {
        if gs.vigor >= 20 {
            gs.vigor -= 20;
            return true;
        } else {
            return false;
        }
    } else {
        if gs.vigor >= 10 {
            gs.vigor -= 10;
            return true;
        } else {
            return false;
        }
    }
}

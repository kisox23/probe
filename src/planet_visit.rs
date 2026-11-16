use rand::seq::IndexedRandom;

use colored::Colorize;

use crate::{
    encounter::talk,
    input_command,
    models::{GameState, Planet},
    planet_generator::{
        Element, Item,
        Util::{self, Fruit},
    },
    storage::get_current_planet,
};

pub fn visit_planet(gs: &mut GameState) {
    let planet = get_current_planet(gs);
    println!("=== WELCOME TO {} ===", planet.name);
    loop {
        println!("What do you feel up to?");
        let cmd = input_command("Planet >>> ");
        match cmd.as_str() {
            "h" | "harvest" => harvest_material(gs, &planet),
            "r" | "retrieve" => retrieve_element(gs, &planet),
            "t" | "talk" => talk(gs, &planet),
            "b" | "back" => return,
            _ => {
                println!("Not a command");
                continue;
            }
        }
    }
}

fn harvest_material(gs: &mut GameState, planet: &Planet) {
    if planet.materials.is_empty() {
        println!("{}", "This planet has nothing to offer.".bright_yellow());
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
    tire(gs, planet);
}

fn retrieve_element(gs: &mut GameState, planet: &Planet) {
    if !tire(gs, planet) {
        return;
    }

    let mut rng = rand::rng();
    let entries: Vec<(&Element, &f32)> = planet.elements.iter().collect();

    if let Ok((loot, _)) = entries.choose_weighted(&mut rng, |&(_, pct)| *pct) {
        gs.inventory.update_item(&Item::Ele((*loot).clone()), 1);
    }
}

fn tire(gs: &mut GameState, planet: &Planet) -> bool {
    if planet.temperature > 40 || planet.temperature < 0 {
        if gs.vigor >= 20 {
            gs.vigor -= 20;
            return true;
        } else {
            println!("{}", "You're too tired for this now.".red());
            return false;
        }
    } else {
        if gs.vigor >= 10 {
            gs.vigor -= 10;
            return true;
        } else {
            println!("{}", "You have no energy now.".red());
            return false;
        }
    }
}

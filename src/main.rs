mod encounter;
mod inventory;
mod models;
mod planet_generator;
mod planet_visit;
mod storage;
mod word_generator;

use std::{
    io::{self, Write},
    process::exit,
};

use colored::Colorize;

use crate::{
    models::GameState,
    planet_generator::{Util, generate},
    planet_visit::visit_planet,
    storage::{add_planet, erase_all, list_planets, load, save},
};

fn main() {
    let mut gs: GameState = load();

    println!("Welcome to PROBE.");
    println!("Enter 'help' for a list of commands.");

    loop {
        let cmd = input_command("Ship > ");

        match cmd.as_str() {
            "e" | "explore" => {
                explore(&mut gs);
            }
            "i" | "inventory" => {
                inventory(&mut gs);
            }
            "p" | "planets" => {
                show_planets(&gs);
            }
            "s" | "status" => {
                status(&gs);
            }
            "q" | "quit" => {
                quit(&gs);
            }
            "erase-all-data" => {
                erase_data();
            }
            "" => {}
            _ => {
                if let Some(id_str) = cmd.strip_prefix("travel to ") {
                    match id_str.trim().parse::<u64>() {
                        Ok(planet_id) => {
                            if planet_id <= gs.current_id {
                                if gs.fuel < 20 && gs.current_planet != planet_id {
                                    println!("No fuel to travel to another planet.");
                                    continue;
                                }
                                if gs.fuel >= 20 && gs.current_planet != planet_id {
                                    gs.fuel -= 20;
                                }
                                gs.current_planet = planet_id;
                                save(&gs).expect("UNABLE TO SAVE");
                                actions(&mut gs);
                                continue;
                            } else {
                                println!("Planet index out of range.");
                                continue;
                            }
                        }
                        Err(_) => {
                            println!("Not a valid planet ID.");
                        }
                    }
                } else if let Some(act_string) = cmd.strip_prefix("consume ") {
                    match act_string.trim() {
                        "canister" | "Canister" => gs.consume(Util::Canister),
                        "fruit" | "Fruit" => gs.consume(Util::Fruit),
                        &_ => println!("{}", "Not an item".red()),
                    }
                    continue;
                }
                println!("Not a command.");
            }
        }
    }
}

fn input_command(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("STDOUT error");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("STDIN error");

    input.trim().to_lowercase()
}

fn explore(gs: &mut GameState) {
    if gs.fuel <= 0 {
        println!("Insufficient fuel! Please refuel the tank.");
        return;
    }

    gs.current_id += 1;
    gs.fuel -= 20;

    let next_id = &gs.current_id;
    let planet = generate(*next_id);
    add_planet(&planet).expect("SOMETHING WENT WRONG");
    println!("Discovered a new planet, {}!", planet.name);

    gs.current_planet = gs.current_id;

    actions(gs);
}

fn actions(gs: &mut GameState) {
    loop {
        println!("What are you going to do?");
        let action = input_command("Orbit >> ");

        match action.as_str() {
            "l" | "land" => visit_planet(gs),
            "b" | "back" => return,
            _ => {
                println!("Not a command!");
                continue;
            }
        }
    }
}

fn status(gs: &GameState) {
    println!("============ STATUS ============");
    println!("Ship fuel: {}%", gs.fuel);
    println!("Vigor: {}%", gs.vigor);
}

fn show_planets(gs: &GameState) {
    println!("======== DISCOVERY LIST ========");
    let planets = list_planets(gs);

    for elem in planets.iter() {
        let mut current = "";
        if elem.id == gs.current_planet {
            current = "(current)";
        }
        println!(
            "    Planet {} {} {}",
            elem.id.to_string().bold(),
            elem.name.bold().purple(),
            current
        );
        println!("-> Class: {}", elem.class.green());
        println!(
            "-> Climate: {}, {}°",
            elem.climate.yellow(),
            elem.temperature
        );
        println!(
            "-> Inhabitated: {}",
            if elem.inhabitated {
                "yes".bold()
            } else {
                "no".bold()
            }
        );
        println!("--------------------------------")
    }
}

fn inventory(gs: &mut GameState) {
    println!("========== INVENTORY ===========");
    gs.inventory.print_all_items();
}

fn quit(gs: &GameState) {
    save(gs).expect("SOMETHING WENT WRONG");
    println!("GOOD-BYE");
    exit(0);
}

fn help() {
    println!("Commands");
}

fn erase_data() {
    erase_all();
    exit(0);
}

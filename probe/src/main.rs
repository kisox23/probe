mod ascii_art;
mod crafting;
mod encounter;
mod inventory;
mod item;
mod language;
mod models;
mod planet_generator;
mod planet_visit;
mod storage;
mod word_generator;

use std::{
    io::{self, Write},
    process::exit,
    str::FromStr,
    sync::LazyLock,
};

use colored::Colorize;

use crate::{
    ascii_art::*,
    crafting::craft,
    item::*,
    language::{Classifier, Intent, train},
    models::GameState,
    planet_generator::generate,
    planet_visit::visit_planet,
    storage::{add_planet, erase_all, get_current_planet, list_planets, load, save},
    word_generator::NameGenerator,
};

static CLASSIFIER: LazyLock<Classifier> = LazyLock::new(train);

pub fn classify(sentence: &str) -> Intent {
    CLASSIFIER.predict(sentence)
}

fn main() {
    let mut gs: GameState = load();
    let name_gen = NameGenerator::new();

    let current_ship_view = update_ship_view(gs.fuel);

    println!("Welcome to PROBE.");
    println!("Enter 'help' for a list of commands.");

    clear_and_render(&current_ship_view);

    loop {
        let cmd = input_command("Ship > ");

        match cmd.as_str() {
            "e" | "explore" => explore(&mut gs, &name_gen),
            "i" | "inventory" => inventory(&mut gs),
            "p" | "planets" => show_planets(&gs),
            "o" | "orbit" => {
                let planet = get_current_planet(&gs);
                actions(&mut gs, false, planet.name.as_str());
            }
            "s" | "status" => status(&gs),
            "h" | "help" => {
                help();
                clear_and_render(&(update_ship_view(gs.fuel)));
                continue;
            }
            "q" | "quit" => quit(&gs),
            "erase-all-data" => erase_data(),
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
                                gs.current_name = get_current_planet(&gs).name;
                                save(&gs).expect("UNABLE TO SAVE");
                                actions(&mut gs, false, "");
                                clear_and_render(&(update_ship_view(gs.fuel)));
                                continue;
                            } else {
                                println!("{}", "Planet index out of range.".red());
                                continue;
                            }
                        }
                        Err(_) => {
                            println!("Not a valid planet ID.");
                        }
                    }
                } else if let Some(drop_string) = cmd.strip_prefix("drop ") {
                    let mut words = drop_string.split_whitespace();
                    let Some(item_str) = words.next() else {
                        println!("Syntax is: drop [item] [qty]");
                        continue;
                    };

                    let Ok(item) = Item::from_str(item_str) else {
                        println!("Not an item");
                        continue;
                    };

                    let qty = match words.next() {
                        Some(qty_str) => match qty_str.parse::<i32>() {
                            Ok(n) if n > 0 => n,
                            _ => {
                                println!("Invalid quantity.");
                                continue;
                            }
                        },
                        None => 1,
                    };

                    if gs.inventory.get_qty(&item) < qty {
                        println!("You want to lose more than you have?");
                        continue;
                    }

                    gs.inventory.update_item(&item, -qty);
                    println!("Dropped.");
                    continue;
                } else if let Some(item_string) = cmd.strip_prefix("use ") {
                    let item = Item::from_str(item_string);
                    match item {
                        Ok(Item::Uti(u)) => {
                            gs.consume(u);
                            continue;
                        }
                        Ok(Item::Mat(_)) | Ok(Item::Ele(_)) => {
                            println!("Can't use elements or materials.");
                            continue;
                        }
                        Ok(Item::Misc(m)) => {}
                        Err(_) => {
                            println!("Not an item.");
                            continue;
                        }
                    }

                    clear_and_render(&update_ship_view(gs.fuel));
                    continue;
                } else if let Some(craft_string) = cmd.strip_prefix("combine ") {
                    let words = craft_string.split_whitespace();
                    let mut ingredients: Vec<Item> = Vec::new();

                    for word in words {
                        let item = Item::from_str(word);
                        match item {
                            Ok(Item::Ele(_)) | Ok(Item::Mat(_)) => {
                                ingredients.push(item.unwrap());
                            }
                            Ok(Item::Uti(_)) | Ok(Item::Misc(_)) => {
                                println!("Can't combine anything but elements or materials.");
                                break;
                            }
                            Err(_) => {
                                println!("Not items.");
                                break;
                            }
                        }
                    }

                    if craft(&mut gs, ingredients) {
                        continue;
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

fn explore(gs: &mut GameState, name_gen: &NameGenerator) {
    if gs.fuel <= 0 {
        println!("Insufficient fuel! Please refuel the tank.");
        return;
    }

    gs.current_id += 1;
    gs.fuel -= 20;

    let next_id = &gs.current_id;
    let planet = generate(*next_id, name_gen);
    add_planet(&planet).expect("SOMETHING WENT WRONG");

    gs.current_planet = gs.current_id;
    gs.current_name = get_current_planet(gs).name;

    actions(gs, true, planet.name.as_str());

    let render = update_ship_view(gs.fuel);
    clear_and_render(&render);
}

fn actions(gs: &mut GameState, expl: bool, planet_name: &str) {
    let mut visited = expl;
    loop {
        if expl && visited {
            clear_and_render(DISCOVERY);
            visited = false;
            println!("Discovered a new planet, {}!", planet_name.blue());
        } else {
            clear_and_render(ORBIT);
            println!("Currently orbiting {}", planet_name.blue());
        }
        println!("What are you going to do?");
        let action = input_command("Orbit >> ");

        match action.as_str() {
            "l" | "land" => visit_planet(gs),
            "b" | "back" => {
                clear_and_render(&update_ship_view(gs.fuel));
                return;
            }
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
    clear_and_render(HELP_SCREEN);
    let _ = input_command("Press enter to go back.");
}

fn erase_data() {
    erase_all();
    exit(0);
}

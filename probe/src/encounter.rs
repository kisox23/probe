use colored::Colorize;
use rand::seq::IndexedRandom;

use crate::{
    ascii_art::{
        INHABITANT_STANDING, INHABITANT_TELLING, PLANET, PLANET_INHABITATED, clear_and_render,
    },
    classify, input_command,
    item::{ELEMENTS, Element, Item, MATERIALS, Material},
    language::Intent,
    models::{GameState, Planet},
    word_generator,
};

struct Trade {
    item_received: Item,
    item_given: Item,
    qty_received: i32,
    qty_given: i32,
}

pub fn talk(gs: &mut GameState, planet: &Planet) {
    if !planet.inhabitated {
        println!("Who are you talking to?");
        return;
    }

    clear_and_render(INHABITANT_STANDING);
    let name = word_generator::make_name(rand::random_range(2..4), true);
    println!("You met the inhabitant {}.", name.yellow());
    loop {
        println!("What do you have to say?");
        let sentence = input_command("? > ");
        if sentence.len() < 3 {
            println!("Can't you express yourself?");
            continue;
        }
        let intent = classify(sentence.as_str());
        match intent {
            Intent::Trade => {
                trade(gs);
                continue;
            }
            Intent::Ask => {
                ask(planet);
                continue;
            }
            Intent::Greet => {
                println!("Hello there, stranger. What do you need?");
                continue;
            }
            Intent::Bye => {
                break;
            }
        }
    }

    if planet.inhabitated {
        clear_and_render(PLANET_INHABITATED);
    } else {
        clear_and_render(PLANET);
    }
}

fn trade(gs: &mut GameState) {
    clear_and_render(INHABITANT_TELLING);

    let mut rng = rand::rng();
    let trade: Trade;
    let kind = rand::random_bool(0.5);

    if kind {
        let elements: Vec<&Element> = ELEMENTS.sample(&mut rng, 2).collect();
        trade = Trade {
            item_received: Item::Ele(elements[0].clone()),
            item_given: Item::Ele(elements[1].clone()),
            qty_received: rand::random_range(1..3),
            qty_given: rand::random_range(1..3),
        };
    } else {
        let materials: Vec<&Material> = MATERIALS.sample(&mut rng, 2).collect();
        trade = Trade {
            item_received: Item::Mat(materials[0].clone()),
            item_given: Item::Mat(materials[1].clone()),
            qty_received: rand::random_range(1..3),
            qty_given: rand::random_range(1..3),
        };
    }

    println!(
        "They ask you if you want to trade {} x{} for {} x{}",
        trade.item_given, trade.qty_given, trade.item_received, trade.qty_received
    );

    loop {
        println!("Do you accept it?");
        let act = input_command("? > ");

        match act.trim() {
            "y" | "yes" => {
                if gs.inventory.get_qty(&trade.item_given) < trade.qty_given {
                    println!("{}", "You don't have the goods requested.".red());
                    break;
                }
                gs.inventory
                    .update_item(&trade.item_given, -trade.qty_given);
                gs.inventory
                    .update_item(&trade.item_received, trade.qty_given);
                println!("{}", "You got a deal!".blue());
                break;
            }
            "n" | "no" => {
                return;
            }
            &_ => {
                println!("It's a yes or no question.");
            }
        }
    }
}

fn ask(planet: &Planet) {
    let mut result: Vec<String> = Vec::new();
    for (_i, elem) in planet.elements.iter().enumerate() {
        if *elem.1 >= 15.0 {
            result.push(elem.0.to_string());
        }
    }

    print!("I'm pretty sure you can find ");
    let cnt = result.len();
    for (i, x) in result.iter().enumerate() {
        if i == cnt - 1 {
            print!("{} ", x);
            continue;
        }
        print!("{}, ", x);
    }
    if result.is_empty() {
        print!("nothing ");
    }
    print!("here.\n");
}

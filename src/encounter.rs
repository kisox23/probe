use colored::Colorize;
use rand::{random_range, seq::IndexedRandom};

use crate::{
    input_command,
    models::{GameState, Planet},
    planet_generator::{ELEMENTS, Element, Item, MATERIALS, Material},
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

    let name = word_generator::make_name(rand::random_range(2..4), true);
    println!("You met the inhabitant {}.", name.yellow());
    loop {
        println!("What do you want do to?");
        let act = input_command("? > ");
        match act.as_str() {
            "t" | "trade" => {
                trade(gs);
                continue;
            }
            "a" | "ask" => {
                ask(planet);
                continue;
            }
            "b" | "back" => return,
            "" => {}
            _ => {}
        }
    }
}

fn trade(gs: &mut GameState) {
    let mut rng = rand::rng();
    let trade: Trade;
    let kind = rand::random_bool(0.5);

    if kind {
        trade = Trade {
            item_received: Item::Ele(ELEMENTS.choose(&mut rng).unwrap().clone()),
            item_given: Item::Ele(ELEMENTS.choose(&mut rng).unwrap().clone()),
            qty_received: rand::random_range(1..3),
            qty_given: rand::random_range(1..3),
        };
    } else {
        trade = Trade {
            item_received: Item::Mat(MATERIALS.choose(&mut rng).unwrap().clone()),
            item_given: Item::Mat(MATERIALS.choose(&mut rng).unwrap().clone()),
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
                if !gs.inventory.get_qty(&trade.item_given) >= trade.qty_given {
                    println!("{}", "You don't have the goods requested.".red());
                    continue;
                }
                gs.inventory
                    .update_item(&trade.item_given, -trade.qty_given);
                gs.inventory
                    .update_item(&trade.item_received, trade.qty_given);
                println!("{}", "You got a deal!".blue());
                continue;
            }
            "n" | "no" => {
                println!("Bye bye, then.");
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
    for (i, elem) in planet.elements.iter().enumerate() {
        if *elem.1 >= 15.0 {
            result.push(elem.0.to_string());
        }
    }

    print!("I'm pretty sure you can find ");
    for x in result {
        print!("{}, ", x);
    }
    print!("here.\n");
}

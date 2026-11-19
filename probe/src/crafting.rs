use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::LazyLock;

use crate::item::Item;
use crate::item::Util::Harvester;
use crate::models::GameState;

static RECIPES: LazyLock<Vec<Recipe>> = LazyLock::new(|| {
    let content = fs::read_to_string("meta/recipes.json").expect("UNABLE TO LOCATE RECIPE FILE");
    let mut recipes: Vec<Recipe> = serde_json::from_str(&content).expect("UNABLE TO DESERIALIZE");

    for r in &mut recipes {
        r.ingredients.sort();
    }

    recipes
});

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct Recipe {
    product: Item,
    ingredients: Vec<Item>,
    quantity: i32,
}

pub fn craft(gs: &mut GameState, mut ingredients: Vec<Item>) -> bool {
    ingredients.sort();

    let Some(found) = RECIPES.iter().find(|r| r.ingredients == ingredients) else {
        return false;
    };

    for item in &ingredients {
        if gs.inventory.get_qty(item) == 0 {
            println!("You don't have those ingredients.");
            return false;
        }
    }

    if found.product == Item::Uti(Harvester) && gs.inventory.get_qty(&Item::Uti(Harvester)) > 0 {
        println!("You can only have one Harvester in your inventory.");
        return false;
    }

    for item in &found.ingredients {
        match item {
            Item::Ele(_) | Item::Mat(_) => gs.inventory.update_item(item, -1),
            Item::Misc(_) | Item::Uti(_) => {}
        }
    }

    gs.inventory.update_item(&found.product, found.quantity);

    println!("You crafted a {}!", found.product.to_string().green());

    return true;
}

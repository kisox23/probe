use serde::{Deserialize, Serialize};

use crate::item::{Element, Item, Material, SpaceFinding, Util};

use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub materials: HashMap<Material, i32>,
    pub elements: HashMap<Element, i32>,
    pub misc: HashMap<SpaceFinding, i32>,
    pub utils: HashMap<Util, i32>,
}

impl Inventory {
    pub fn update_item(&mut self, item: &Item, qty: i32) {
        match item {
            Item::Mat(m) => *self.materials.entry(m.clone()).or_insert(0) += qty,
            Item::Ele(e) => *self.elements.entry(e.clone()).or_insert(0) += qty,
            Item::Misc(i) => *self.misc.entry(i.clone()).or_insert(0) += qty,
            Item::Uti(u) => *self.utils.entry(u.clone()).or_insert(0) += qty,
        }
    }

    pub fn print_all_items(&self) {
        let all_items = self
            .utils
            .iter()
            .map(|(k, v)| (k as &dyn Display, v))
            .chain(self.misc.iter().map(|(k, v)| (k as &dyn Display, v)))
            .chain(self.materials.iter().map(|(k, v)| (k as &dyn Display, v)))
            .chain(self.elements.iter().map(|(k, v)| (k as &dyn Display, v)));

        let mut c = 0;
        for (_i, elem) in all_items.enumerate() {
            if c % 4 == 0 && c > 0 {
                println!("");
            }
            if *elem.1 == 0 {
                continue;
            }
            c += 1;
            print!("<{} x{}>    ", elem.0, elem.1);
        }
        print!("\n");
    }

    pub fn get_qty(&self, item: &Item) -> i32 {
        match item {
            Item::Ele(e) => *self.elements.get(e).unwrap_or(&0),
            Item::Mat(m) => *self.materials.get(m).unwrap_or(&0),
            Item::Uti(u) => *self.utils.get(u).unwrap_or(&0),
            Item::Misc(misc) => *self.misc.get(misc).unwrap_or(&0),
        }
    }
}

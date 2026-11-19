use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use strum_macros::EnumIter;

use crate::item::{Element::*, Material::*};

pub const ELEMENTS: &[Element] = &[
    Iron, Titanium, Silicon, Uranium, Hydrogen, Helium, Nitrogen, Argon,
];

pub const MATERIALS: &[Material] = &[Wood, Fiber, Oil, Stone, Dust, Plastic];

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub enum Item {
    Mat(Material),
    Ele(Element),
    Misc(SpaceFinding),
    Uti(Util),
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Mat(m) => write!(f, "{:?}", m),
            Item::Ele(e) => write!(f, "{:?}", e),
            Item::Misc(misc) => write!(f, "{:?}", misc),
            Item::Uti(u) => write!(f, "{:?}", u),
        }
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "iron" => Ok(Item::Ele(Element::Iron)),
            "titanium" => Ok(Item::Ele(Element::Titanium)),
            "silicon" => Ok(Item::Ele(Element::Silicon)),
            "uranium" => Ok(Item::Ele(Element::Uranium)),

            "hydrogen" => Ok(Item::Ele(Element::Hydrogen)),
            "argon" => Ok(Item::Ele(Element::Argon)),
            "helium" => Ok(Item::Ele(Element::Helium)),
            "nitrogen" => Ok(Item::Ele(Element::Nitrogen)),

            "wood" => Ok(Item::Mat(Material::Wood)),
            "fiber" => Ok(Item::Mat(Material::Fiber)),
            "oil" => Ok(Item::Mat(Material::Oil)),
            "stone" => Ok(Item::Mat(Material::Stone)),
            "dust" => Ok(Item::Mat(Material::Dust)),
            "plastic" => Ok(Item::Mat(Material::Plastic)),

            "canister" => Ok(Item::Uti(Util::Canister)),
            "fruit" => Ok(Item::Uti(Util::Fruit)),
            "harvester" => Ok(Item::Uti(Util::Harvester)),

            _ => Err(format!("Unknown object: '{}'", s)),
        }
    }
}

#[derive(Debug, EnumIter, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Material {
    Wood,
    Fiber,
    Oil,
    Stone,
    Dust,
    Plastic,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Util {
    Canister,
    Fruit,
    Harvester,
}

impl fmt::Display for Util {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for SpaceFinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum Element {
    Iron,
    Titanium,
    Silicon,
    Uranium,

    Hydrogen,
    Helium,
    Nitrogen,
    Argon,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub enum SpaceFinding {
    Capsule,
    Wire,
    Envelope,
}

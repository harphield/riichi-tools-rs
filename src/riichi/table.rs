use crate::riichi::hand::Hand;
use crate::riichi::tile::Tile;
use crate::riichi::shapes::Shape;
use wasm_bindgen::__rt::std::collections::HashMap;
use serde_json::{Map, Value};

/// Representation of the game state
struct Table {
    my_hand: Hand,
    my_riichi: bool,
    // player to the right
    shimocha_discards: Vec<Tile>,
    shimocha_open_tiles: Vec<Shape>,
    shimocha_riichi: bool,
    // player to the left
    kamicha_discards: Vec<Tile>,
    kamicha_open_tiles: Vec<Shape>,
    kamicha_riichi: bool,
    // opposite player
    toimen_discards: Vec<Tile>,
    toimen_open_tiles: Vec<Shape>,
    toimen_riichi: bool,

    // 1 = east, 2 = south, 3 = west, 4 = north
    prevalent_wind: u8,
    my_seat_wind: u8,
    wind_round: u8,
    total_round: u8,

    dora_indicators: Vec<Tile>,

    riichi_sticks_in_pot: u8,
    tsumibo: u8,

}

impl Table {
    pub fn from_map(params: &Map<String, Value>) -> Table {
        let mut t = Table {
            my_hand: Default::default(),
            my_riichi: false,
            shimocha_discards: vec![],
            shimocha_open_tiles: vec![],
            shimocha_riichi: false,
            kamicha_discards: vec![],
            kamicha_open_tiles: vec![],
            kamicha_riichi: false,
            toimen_discards: vec![],
            toimen_open_tiles: vec![],
            toimen_riichi: false,
            prevalent_wind: 0,
            my_seat_wind: 0,
            wind_round: 0,
            total_round: 0,
            dora_indicators: vec![],
            riichi_sticks_in_pot: 0,
            tsumibo: 0
        };

        for (index, value) in params {
            if index.eq(&String::from("my_hand")) {
                match value {
                    Value::String(s) => {
                        t.my_hand = Hand::from_text(s, false).unwrap();
                    },
                    _ => ()
                }
            } else if index.eq(&String::from("my_riichi")) {
                match value {
                    Value::Bool(b) => {
                        t.my_riichi = *b;
                    },
                    _ => ()
                }
            }
        }

        t
    }
}

mod tests {
    use super::*;

    #[test]
    fn set_my_hand() {

    }
}

use crate::riichi::hand::Hand;
use crate::riichi::tile::Tile;
use crate::riichi::shapes::Shape;
use serde_json::{Map, Value};
use crate::riichi::riichi_error::RiichiError;
use crate::riichi::yaku::{YakuFinder, Yaku};
use crate::riichi::scores::Score;

/// Representation of the game state
pub struct Table {
    my_hand: Option<Hand>,
    my_discards: Option<Vec<Tile>>,
    my_riichi: Option<bool>,
    my_tsumo: Option<bool>,
    my_points: Option<i32>,
    // player to the right
    shimocha_discards: Option<Vec<Tile>>,
    shimocha_open_tiles: Option<Vec<Shape>>,
    shimocha_riichi: Option<bool>,
    shimocha_tsumo: Option<bool>,
    shimocha_points: Option<i32>,
    // player to the left
    kamicha_discards: Option<Vec<Tile>>,
    kamicha_open_tiles: Option<Vec<Shape>>,
    kamicha_riichi: Option<bool>,
    kamicha_tsumo: Option<bool>,
    kamicha_points: Option<i32>,
    // opposite player
    toimen_discards: Option<Vec<Tile>>,
    toimen_open_tiles: Option<Vec<Shape>>,
    toimen_riichi: Option<bool>,
    toimen_tsumo: Option<bool>,
    toimen_points: Option<i32>,

    // 1 = east, 2 = south, 3 = west, 4 = north
    prevalent_wind: Option<u8>,
    my_seat_wind: Option<u8>,
    wind_round: Option<u8>,
    total_round: Option<u8>,
    tiles_remaining: Option<u8>,

    riichi_sticks_in_pot: Option<u8>,
    tsumibo: Option<u8>,

    dora_indicators: Vec<Tile>,
    visible_tiles: Vec<Tile>,
}

impl Table {
    pub fn from_map(params: &Map<String, Value>) -> Result<Table, RiichiError> {
        let mut t = Table {
            my_hand: None,
            my_discards: None,
            my_riichi: None,
            my_tsumo: None,
            my_points: None,
            shimocha_discards: None,
            shimocha_open_tiles: None,
            shimocha_riichi: None,
            shimocha_tsumo: None,
            shimocha_points: None,
            kamicha_discards: None,
            kamicha_open_tiles: None,
            kamicha_riichi: None,
            kamicha_tsumo: None,
            kamicha_points: None,
            toimen_discards: None,
            toimen_open_tiles: None,
            toimen_riichi: None,
            toimen_tsumo: None,
            toimen_points: None,
            prevalent_wind: None,
            my_seat_wind: None,
            wind_round: None,
            total_round: None,
            tiles_remaining: None,
            riichi_sticks_in_pot: None,
            tsumibo: None,
            dora_indicators: vec![],
            visible_tiles: vec![],
        };

        for (index, value) in params {
            if index.eq(&String::from("my_hand")) {
                match value {
                    Value::String(s) => {
                        match Hand::from_text(s, false) {
                            Ok(hand) => t.my_hand = Some(hand),
                            Err(error) => return Err(error)
                        }

                    },
                    _ => ()
                }
            } else if index.eq(&String::from("my_riichi")) {
                match value {
                    Value::Bool(b) => {
                        t.my_riichi = Some(*b);
                    },
                    _ => ()
                }
            } else if index.eq(&String::from("my_tsumo")) {
                match value {
                    Value::Bool(b) => {
                        t.my_tsumo = Some(*b);
                    },
                    _ => ()
                }
            } else if index.eq(&String::from("prevalent_wind")) {
                match value {
                    Value::Number(v) => {
                        t.prevalent_wind = Some(v.as_u64().unwrap() as u8);
                    },
                    Value::String(v) => {
                        let number: u8 = v.parse().unwrap();
                        t.prevalent_wind = Some(number);
                    }
                    _ => ()
                }
            } else if index.eq(&String::from("my_seat_wind")) {
                match value {
                    Value::Number(v) => {
                        t.my_seat_wind = Some(v.as_u64().unwrap() as u8);
                    },
                    Value::String(v) => {
                        let number: u8 = v.parse().unwrap();
                        t.my_seat_wind = Some(number);
                    }
                    _ => ()
                }
            }
        }

        Ok(t)
    }

    pub fn set_seat(&mut self, seat: u8) {
        self.my_seat_wind = Some(seat);
    }

    pub fn am_i_oya(&self) -> bool {
        match self.my_seat_wind {
            None => false,
            Some(seat) => {
                seat == 1
            },
        }
    }

    pub fn set_my_tsumo(&mut self, value: bool) {
        self.my_tsumo = Some(value);
    }

    pub fn did_i_tsumo(&self) -> bool {
        match self.my_tsumo {
            None => false,
            Some(value) => value,
        }
    }

    pub fn set_my_riichi(&mut self, value: bool) {
        self.my_riichi = Some(value);
    }

    pub fn did_i_riichi(&self) -> bool {
        match self.my_riichi {
            None => false,
            Some(value) => value,
        }
    }

    pub fn set_my_hand(&mut self, hand: Hand) {
        self.my_hand = Some(hand);
    }

    pub fn get_my_hand(&mut self) -> &mut Hand {
        match &mut self.my_hand {
            None => panic!("No hand!"),
            Some(hand) => hand,
        }
    }

    pub fn get_my_winning_tile(&self) -> Tile {
        match &self.my_hand {
            None => panic!("No drawn tile in hand!"),
            Some(hand) => {
                hand.get_drawn_tile().unwrap().clone()
            },
        }
    }

    pub fn set_tiles_remaining(&mut self, value: u8) {
        self.tiles_remaining = Some(value);
    }

    pub fn get_tiles_remaining(&self) -> Option<u8> {
        self.tiles_remaining
    }

    pub fn set_my_seat_wind(&mut self, value: u8) {
        self.my_seat_wind = Some(value);
    }

    pub fn get_my_seat_wind(&self) -> Option<u8> {
        self.my_seat_wind
    }

    pub fn set_prevalent_wind(&mut self, value: u8) {
        self.prevalent_wind = Some(value);
    }

    pub fn get_prevalent_wind(&self) -> Option<u8> {
        self.prevalent_wind
    }

    pub fn set_dora_indicators(&mut self, indicators: Vec<Tile>) {
        self.dora_indicators = indicators;
    }

    pub fn add_dora_indicator(&mut self, indicator: Tile) {
        self.dora_indicators.push(indicator);
        self.visible_tiles.push(indicator);
    }

    pub fn get_dora_indicators(&self) -> &Vec<Tile> {
        &self.dora_indicators
    }

    pub fn add_tile_to_visible_tiles(&mut self, tile: Tile) {
        self.visible_tiles.push(tile);
    }

    pub fn add_vector_to_visible_tiles(&mut self, mut tiles: Vec<Tile>) {
        self.visible_tiles.append(&mut tiles);
    }

    pub fn reset_visible_tiles(&mut self) {
        self.visible_tiles = vec![];
    }

    pub fn get_visible_tiles(&self) -> &Vec<Tile> {
        &self.visible_tiles
    }

    pub fn get_visible_tiles_as_array_34(&self) -> [u8; 34] {
        let mut result = [0; 34];

        for tile in &self.visible_tiles {
            result[(tile.to_id() - 1) as usize] += 1;

            if result[(tile.to_id() - 1) as usize] > 4 {
                panic!("can not have more than 4 tiles!!!");
            }
        }

        result
    }

    pub fn yaku(&mut self) -> Option<(Vec<Yaku>, Score)> {
        let yf = YakuFinder::new();
        yf.find(self)
    }
}

mod tests {
    use super::*;
}

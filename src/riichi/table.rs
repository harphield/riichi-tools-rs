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

    dora_indicators: Option<Vec<Tile>>,

    riichi_sticks_in_pot: Option<u8>,
    tsumibo: Option<u8>,
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
            dora_indicators: None,
            riichi_sticks_in_pot: None,
            tsumibo: None
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
            }
        }

        Ok(t)
    }

    pub fn am_i_oya(&self) -> bool {
        return match self.prevalent_wind {
            None => false,
            Some(prevalent) => {
                match self.my_seat_wind {
                    None => false,
                    Some(seat) => {
                        prevalent > 0 && prevalent == seat
                    },
                }
            },
        }
    }

    pub fn did_i_tsumo(&self) -> bool {
        match self.my_tsumo {
            None => false,
            Some(value) => value,
        }
    }

    pub fn did_i_riichi(&self) -> bool {
        match self.my_riichi {
            None => false,
            Some(value) => value,
        }
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
                hand.get_drawn_tile().clone()
            },
        }
    }

    pub fn get_tiles_remaining(&self) -> Option<u8> {
        self.tiles_remaining
    }

    pub fn get_my_seat_wind(&self) -> Option<u8> {
        self.my_seat_wind
    }

    pub fn get_prevalent_wind(&self) -> Option<u8> {
        self.prevalent_wind
    }

    pub fn yaku(&mut self) -> Option<(Vec<Yaku>, Score)> {
        let yf = YakuFinder::new();
        yf.find(self)
    }
}

mod tests {
    use super::*;
}

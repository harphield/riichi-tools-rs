use crate::riichi::hand::Hand;
use crate::riichi::shape_finder::ShapeFinder;
use crate::riichi::shapes::{Shape, ShapeType, CompleteShape};
use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;
use crate::riichi::scores::Score;
use crate::riichi::table::Table;
use crate::riichi::tile::TileType;

#[derive(IntoEnumIterator, Debug, Clone)]
pub enum Yaku {
    // 1 han closed
    MenzenTsumo,
    Riichi,
    Ippatsu,
    Pinfu,
    Iipeikou,
    // 1 han
    Haitei,
    Houtei,
    Rinshan,
    Chankan,
    Tanyao,
    EastRound,
    EastSeat,
    SouthRound,
    SouthSeat,
    WestRound,
    WestSeat,
    NorthSeat,
    WhiteDragons,
    GreenDragons,
    RedDragons,
    // 2 han
    DoubleRiichi,
    Chanta,
    SanshokuDoujun,
    Ittsu,
    Toitoi,
    Sanankou,
    SanshokuDoukou,
    Sankantsu,
    Chiitoitsu,
    Honroutou,
    Shousangen,
    // 3 han
    Honitsu,
    Junchan,
    Ryanpeikou,
    // 6 han
    Chinitsu,
    // Yakuman
    Kazoe,
    Kokushi,
    Suuankou,
    Daisangen,
    Shousuushii,
    Daisuushii,
    Tsuuiisou,
    Chinroutou,
    Ryuuiisou,
    Chuuren,
    Suukantsu,
    // special
    Tenhou,
    Chiihou
}

pub struct YakuFinder {

}

impl YakuFinder {
    pub fn new() -> YakuFinder {
        YakuFinder {
            ..Default::default()
        }
    }

    pub fn find(&self, mut table: &mut Table) -> Option<(Vec<Yaku>, Score)> {
        // only complete hands
        if table.my_hand.shanten() != -1 {
            return None;
        }

        let mut sf = ShapeFinder::new();
        let variants = sf.find(&mut table.my_hand);
        let mut best_variant: (Vec<Yaku>, Score) = (vec![], Score::new(0, 0, false, false));

        for (i, variant) in variants.iter().enumerate() {
            let mut yakus: Vec<Yaku> = vec!();
            let mut han: u8 = 0;
            let mut fu: u8 = 0;
            for yaku_type in Yaku::into_enum_iter() {
                if yaku_type.is_in_hand(&mut table, variant) {
                    match yaku_type {
                        Yaku::Pinfu => {
                            if table.did_i_tsumo() {
                                fu = 20;
                            } else {
                                fu = 30;
                            }
                        },
                        Yaku::Chiitoitsu => {
                            fu = 25;
                        },
                        _ => ()
                    }
                    yakus.push(yaku_type.clone());
                    han += yaku_type.get_han(table);
                }
            }

            if han >= 5 {
                fu = 0;
            } else if fu == 0 {
                // if we did not set fu based on yaku, we search shapes for fu
                if table.did_i_tsumo() {
                    fu = 20;
                } else {
                    fu = 30;
                }

                // set this for tanki, shanpon, kanchan and penchan waits
                let mut has_value_wait = false;

                let winning_tile = table.get_my_winning_tile();

                for shape in variant {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => {
                            match cs {
                                CompleteShape::Shuntsu(tiles) => {
                                    if tiles[1].eq(&winning_tile) || // kanchan
                                       tiles[0].prev_id(false, 1) == 0 && tiles[2].eq(&winning_tile) ||
                                       tiles[2].next_id(false, 1) == 0 && tiles[0].eq(&winning_tile) { // penchans
                                        has_value_wait = true;
                                    }
                                },
                                CompleteShape::Koutsu(tiles) => {
                                    // TODO open vs closed ofc
                                    // TODO kans
                                    match tiles[0].tile_type {
                                        TileType::Number(value, _) => {
                                            if value == 1 || value == 9 {
                                                fu += 8
                                            } else {
                                                fu += 4;
                                            }
                                        },
                                        TileType::Wind(_) | TileType::Dragon(_) => {
                                            fu += 8;
                                        },
                                    }

                                    if tiles[0].eq(&winning_tile) {
                                        has_value_wait = true;
                                    }
                                },
                                CompleteShape::Toitsu(tiles) => {
                                    match tiles[0].tile_type {
                                        TileType::Number(_, _) => {},
                                        TileType::Wind(value) => {

                                        },
                                        TileType::Dragon(_) => {},
                                    }

                                    if tiles[0].eq(&winning_tile) {
                                        has_value_wait = true;
                                    }
                                },
                                CompleteShape::Single(tile) => {},
                            }
                        },
                        ShapeType::Incomplete(_) => {},
                    }
                }
            }

            let score= Score::new(han, fu, table.am_i_oya(), table.did_i_tsumo());
            if i == 0 {
                best_variant = (yakus, score);
            } else {
                if score.total_points() > best_variant.1.total_points() {
                    best_variant = (yakus, score);
                }
            }
        }

        println!("{:#?}", best_variant);
        Some(best_variant)
    }
}

impl Default for YakuFinder {
    fn default() -> YakuFinder {
        YakuFinder {

        }
    }
}

////////////////

impl Yaku {
    fn get_name(&self) -> &str {
        match self {
            Yaku::MenzenTsumo =>    "Menzen tsumo",
            Yaku::Riichi =>         "Riichi",
            Yaku::Ippatsu =>        "Ippatsu",
            Yaku::Pinfu =>          "Pinfu",
            Yaku::Iipeikou =>       "Iipeikou",
            Yaku::Haitei =>         "Haitei raoyue",
            Yaku::Houtei =>         "Houtei raoyui",
            Yaku::Rinshan =>        "Rinshan kaihou",
            Yaku::Chankan =>        "Chankan",
            Yaku::Tanyao =>         "Tanyao",
            Yaku::EastRound =>      "East round winds",
            Yaku::EastSeat =>       "East seat winds",
            Yaku::SouthRound =>     "South round winds",
            Yaku::SouthSeat =>      "South seat winds",
            Yaku::WestRound =>      "West round winds",
            Yaku::WestSeat =>       "West seat winds",
            Yaku::NorthSeat =>      "North seat winds",
            Yaku::WhiteDragons =>   "White dragons",
            Yaku::GreenDragons =>   "Green dragons",
            Yaku::RedDragons =>     "Red dragons",
            Yaku::DoubleRiichi =>   "Double riichi",
            Yaku::Chanta =>         "Chantaiyao",
            Yaku::SanshokuDoujun => "Sanshoku doujun",
            Yaku::Ittsu =>          "Ittsu",
            Yaku::Toitoi =>         "Toitoi",
            Yaku::Sanankou =>       "Sanankou",
            Yaku::SanshokuDoukou => "Sanshoku doukou",
            Yaku::Sankantsu =>      "Sankantsu",
            Yaku::Chiitoitsu =>     "Chiitoitsu",
            Yaku::Honroutou =>      "Honroutou",
            Yaku::Shousangen =>     "Shousangen",
            Yaku::Honitsu =>        "Honitsu",
            Yaku::Junchan =>        "Junchan taiyao",
            Yaku::Ryanpeikou =>     "Ryanpeikou",
            Yaku::Chinitsu =>       "Chinitsu",
            Yaku::Kazoe =>          "Kazoe yakuman",
            Yaku::Kokushi =>        "Kokushi musou",
            Yaku::Suuankou =>       "Suuankou",
            Yaku::Daisangen =>      "Daisangen",
            Yaku::Shousuushii =>    "Shousuushii",
            Yaku::Daisuushii =>     "Daisuushii",
            Yaku::Tsuuiisou =>      "Tsuuiisou",
            Yaku::Chinroutou =>     "Chinroutou",
            Yaku::Ryuuiisou =>      "Ryuuiisou",
            Yaku::Chuuren =>        "Chuuren poutou",
            Yaku::Suukantsu =>      "Suukantsu",
            Yaku::Tenhou =>         "Tenhou",
            Yaku::Chiihou =>        "Chiihou"
        }
    }

    fn get_han(&self, table: &Table) -> u8 {
        match self {
            Yaku::MenzenTsumo =>    1,
            Yaku::Riichi =>         1,
            Yaku::Ippatsu =>        1,
            Yaku::Pinfu =>          1,
            Yaku::Iipeikou =>       1,
            Yaku::Haitei =>         1,
            Yaku::Houtei =>         1,
            Yaku::Rinshan =>        1,
            Yaku::Chankan =>        1,
            Yaku::Tanyao =>         1,
            Yaku::EastRound =>      1,
            Yaku::EastSeat =>       1,
            Yaku::SouthRound =>     1,
            Yaku::SouthSeat =>      1,
            Yaku::WestRound =>      1,
            Yaku::WestSeat =>       1,
            Yaku::NorthSeat =>      1,
            Yaku::WhiteDragons =>   1,
            Yaku::GreenDragons =>   1,
            Yaku::RedDragons =>     1,
            Yaku::DoubleRiichi =>   2,
            Yaku::Chanta =>         {
                if table.my_hand.is_closed() {
                    return 2;
                }

                1
            },
            Yaku::SanshokuDoujun => {
                if table.my_hand.is_closed() {
                    return 2;
                }

                1
            },
            Yaku::Ittsu =>          {
                if table.my_hand.is_closed() {
                    return 2;
                }

                1
            },
            Yaku::Toitoi =>         2,
            Yaku::Sanankou =>       2,
            Yaku::SanshokuDoukou => 2,
            Yaku::Sankantsu =>      2,
            Yaku::Chiitoitsu =>     2,
            Yaku::Honroutou =>      2,
            Yaku::Shousangen =>     2,
            Yaku::Honitsu =>        {
                if table.my_hand.is_closed() {
                    return 3;
                }

                2
            },
            Yaku::Junchan =>        {
                if table.my_hand.is_closed() {
                    return 3;
                }

                2
            },
            Yaku::Ryanpeikou =>     {
                if table.my_hand.is_closed() {
                    return 3;
                }

                2
            },
            Yaku::Chinitsu =>       {
                if table.my_hand.is_closed() {
                    return 6;
                }

                5
            },
            Yaku::Kazoe =>          13,
            Yaku::Kokushi =>        13,
            Yaku::Suuankou =>       13,
            Yaku::Daisangen =>      13,
            Yaku::Shousuushii =>    13,
            Yaku::Daisuushii =>     13,
            Yaku::Tsuuiisou =>      13,
            Yaku::Chinroutou =>     13,
            Yaku::Ryuuiisou =>      13,
            Yaku::Chuuren =>        13,
            Yaku::Suukantsu =>      13,
            Yaku::Tenhou =>         13,
            Yaku::Chiihou =>        13
        }
    }

    /// Check if this Yaku exists in this shape variant
    fn is_in_hand(&self, table: &mut Table, variant: &Vec<Shape>) -> bool {
        match self {
            Yaku::MenzenTsumo => {
                return table.my_hand.is_closed() && table.did_i_tsumo()
            },
            Yaku::Riichi => {
                return table.my_hand.is_closed() && table.did_i_riichi()
            },
            Yaku::Ippatsu => {
                if !table.my_hand.is_closed() || !table.did_i_riichi() {
                    return false;
                }

                // TODO
            },
            Yaku::Pinfu => {
                if !table.my_hand.is_closed() {
                    return false;
                }

                let mut pairs: u8 = 0;
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => {
                            match cs {
                                CompleteShape::Koutsu(_) | CompleteShape::Single(_) => return false,
                                CompleteShape::Toitsu(tiles) => {
                                    pairs += 1;
                                    if pairs > 1 {
                                        return false;
                                    }

                                    match tiles[0].tile_type {
                                        TileType::Wind(value) => {
                                            if value == table.get_prevalent_wind() || value == table.get_my_seat_wind() {
                                                return false;
                                            }
                                        },
                                        TileType::Dragon(_) => return false,
                                        _ => ()
                                    }
                                },
                                _ => ()
                            }
                        },
                        ShapeType::Incomplete(_) => return false,
                    }
                }
            },
            Yaku::Iipeikou => {
                if !table.my_hand.is_closed() {
                    return false;
                }
            },
            Yaku::Haitei => {
                return table.did_i_tsumo() && table.get_tiles_remaining() == 0
            },
            Yaku::Houtei => {
                return !table.did_i_tsumo() && table.get_tiles_remaining() == 0
            },
            Yaku::Rinshan => {},    // TODO
            Yaku::Chankan => {},    // TODO
            Yaku::Tanyao => {
                let array_34 = table.my_hand.get_34_array();
                // can't contain any terminals or honors
                for (i, count) in array_34.iter().enumerate() {
                    if ([1, 9, 10, 18, 19, 27].contains(&(i + 1)) || (i + 1) >= 28) && *count > 0 {
                        return false;
                    }
                }

                return true;
            },
            Yaku::EastRound => {
                if table.get_prevalent_wind() != 1 {
                    return false;
                }

                self.find_yakuhai(variant, 28);
            }
            Yaku::EastSeat => {
                if table.get_my_seat_wind() != 1 {
                    return false;
                }

                self.find_yakuhai(variant, 28);
            }
            Yaku::SouthRound => {
                if table.get_prevalent_wind() != 2 {
                    return false;
                }

                self.find_yakuhai(variant, 29);
            }
            Yaku::SouthSeat => {
                if table.get_my_seat_wind() != 2 {
                    return false;
                }

                self.find_yakuhai(variant, 29);
            }
            Yaku::WestRound => {
                if table.get_prevalent_wind() != 3 {
                    return false;
                }

                self.find_yakuhai(variant, 30);
            }
            Yaku::WestSeat => {
                if table.get_my_seat_wind() != 3 {
                    return false;
                }

                self.find_yakuhai(variant, 30);
            }
            Yaku::NorthSeat => {
                if table.get_my_seat_wind() != 4 {
                    return false;
                }

                self.find_yakuhai(variant, 31);
            }
            Yaku::WhiteDragons => return self.find_yakuhai(variant, 33),
            Yaku::GreenDragons => return self.find_yakuhai(variant, 32),
            Yaku::RedDragons => return self.find_yakuhai(variant, 34),
            Yaku::DoubleRiichi => {},   // TODO
            Yaku::Chanta => {
                // a chanta has to have shuntsu, else it's honroutou
                let mut has_shuntsu = false;
                // a chanta has to have honors, else it's junchan
                let mut has_honors = false;
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => {
                            match cs {
                                CompleteShape::Shuntsu(tiles) => {
                                    if !tiles[0].is_terminal() && !tiles[2].is_terminal() {
                                        return false;
                                    }

                                    has_shuntsu = true;
                                },
                                CompleteShape::Koutsu(tiles) => {
                                    if !tiles[0].is_terminal_or_honor() {
                                        return false;
                                    }

                                    if tiles[0].is_honor() {
                                        has_honors = true;
                                    }
                                },
                                CompleteShape::Toitsu(tiles) => {
                                    if !tiles[0].is_terminal_or_honor() {
                                        return false;
                                    }

                                    if tiles[0].is_honor() {
                                        has_honors = true;
                                    }
                                },
                                CompleteShape::Single(_) => {
                                    return false;
                                },
                            }
                        },
                        ShapeType::Incomplete(_) => {
                            return false;
                        },
                    }
                }

                return has_shuntsu && has_honors;
            },
            Yaku::SanshokuDoujun => {
                let mut combos: HashMap<String, [bool; 3]> = HashMap::new();

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => {
                            match cs {
                                CompleteShape::Shuntsu(tiles) => {
                                    match tiles[0].tile_type {
                                        TileType::Number(value, suit) => {
                                            let key = format!("{}{}{}", value, value + 1, value + 2);
                                            let mut info;
                                            if combos.contains_key(&key[..]) {
                                                info = combos[&key];
                                                combos.remove(&key);
                                            } else {
                                                info = [false, false, false];
                                            }

                                            if suit.to_char() == 'm' {
                                                info[0] = true;
                                            } else if suit.to_char() == 'p' {
                                                info[1] = true;
                                            } else if suit.to_char() == 's' {
                                                info[2] = true;
                                            }

                                            if info[0] && info[1] && info[2] {
                                                return true;
                                            }

                                            combos.insert(key, info);
                                        },
                                        _ => ()
                                    }
                                },
                                CompleteShape::Single(_) => return false,
                                _ => ()
                            }
                        },
                        ShapeType::Incomplete(_) => return false,
                    }
                }
            },
            Yaku::Ittsu => {},
            Yaku::Toitoi => {},
            Yaku::Sanankou => {},
            Yaku::SanshokuDoukou => {},
            Yaku::Sankantsu => {},      // TODO
            Yaku::Chiitoitsu => {
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => {
                            match cs {
                                CompleteShape::Shuntsu(_) => return false,
                                CompleteShape::Koutsu(_) => return false,
                                CompleteShape::Toitsu(_) => (),
                                CompleteShape::Single(_) => return false,
                            }
                        },
                        ShapeType::Incomplete(_) => return false,
                    }
                }

                return true;
            },
            Yaku::Honroutou => {},
            Yaku::Shousangen => {},
            Yaku::Honitsu => {},
            Yaku::Junchan => {},
            Yaku::Ryanpeikou => {},
            Yaku::Chinitsu => {},
            Yaku::Kazoe => {},
            Yaku::Kokushi => {},
            Yaku::Suuankou => {},
            Yaku::Daisangen => {},
            Yaku::Shousuushii => {},
            Yaku::Daisuushii => {},
            Yaku::Tsuuiisou => {},
            Yaku::Chinroutou => {},
            Yaku::Ryuuiisou => {},
            Yaku::Chuuren => {},
            Yaku::Suukantsu => {},
            Yaku::Tenhou => {},
            Yaku::Chiihou => {}
        }

        false
    }

    pub fn is_yakuman(&self) -> bool {
        match self {
            Yaku::Kokushi | Yaku::Suuankou | Yaku::Daisangen | Yaku::Shousuushii |
            Yaku::Daisuushii | Yaku::Tsuuiisou | Yaku::Chinroutou | Yaku::Ryuuiisou |
            Yaku::Chuuren | Yaku::Suukantsu | Yaku::Tenhou | Yaku::Chiihou => true,
            _ => false
        }
    }

    fn find_yakuhai(&self, variant: &Vec<Shape>, tile_id: u8) -> bool {
        for shape in variant.iter() {
            match shape.get_shape_type() {
                ShapeType::Complete(cs) => {
                    match cs {
                        CompleteShape::Koutsu(tiles) => {
                            if tiles[0].to_id() == tile_id {
                                return true;
                            }
                        },
                        _ => ()
                    }
                },
                ShapeType::Incomplete(_) => (),
            }
        }

        return false;
    }
}

mod tests {
    use super::*;
    use serde_json::{Map, Value};

    #[test]
    fn find_tanyao() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("234567m234567s88p"));

        let mut table = Table::from_map(&map).unwrap();
        table.yaku();
    }

    #[test]
    fn find_tanyao_chiitoi() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("224466m4477s3388p"));

        let mut table = Table::from_map(&map).unwrap();
        table.yaku();
    }

    #[test]
    fn find_white_dragons() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("123m234s67888p666z"));

        let mut table = Table::from_map(&map).unwrap();
        table.yaku();
    }

    #[test]
    fn find_sanshoku_doujun() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("234m234567s23488p"));

        let mut table = Table::from_map(&map).unwrap();
        table.yaku();
    }
}

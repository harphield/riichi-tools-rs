use crate::riichi::scores::Score;
use crate::riichi::shape_finder::ShapeFinder;
use crate::riichi::shapes::{ClosedShape, CompleteShape, OpenShape, Shape, ShapeType};
use crate::riichi::table::Table;
use crate::riichi::tile::{Tile, TileType};
use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;
use wasm_bindgen::__rt::std::collections::hash_map::Entry;

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
    Chiihou,
}

pub struct YakuFinder {}

impl YakuFinder {
    pub fn new() -> YakuFinder {
        YakuFinder {}
    }

    /// Finds the best variant of the hand + its score
    pub fn find(&self, mut table: &mut Table) -> Option<(Vec<Yaku>, Score)> {
        // only complete hands
        let mut hand = &mut table.get_my_hand().clone();

        if hand.shanten() != -1 {
            return None;
        }

        let mut sf = ShapeFinder::new();
        let variants = sf.find(&mut hand);
        let mut best_variant: (Vec<Yaku>, Score) = (vec![], Score::new(0, 0, false, false));

        for (i, variant) in variants.iter().enumerate() {
            let mut yakus: Vec<Yaku> = vec![];
            let mut han: u8 = 0;
            let mut fu: u8 = 0;

            // first find potential yakumans
            for yaku_type in Yaku::into_enum_iter() {
                if !yaku_type.is_yakuman() {
                    continue;
                }

                if yaku_type.is_in_hand(&mut table, variant) {
                    yakus.push(yaku_type.clone());
                }
            }

            if !yakus.is_empty() {
                han = 13;
            } else {
                for yaku_type in Yaku::into_enum_iter() {
                    if yaku_type.is_yakuman() {
                        continue;
                    }

                    if yaku_type.is_in_hand(&mut table, variant) {
                        match yaku_type {
                            Yaku::Pinfu => {
                                if table.did_i_tsumo() {
                                    fu = 20;
                                } else {
                                    fu = 30;
                                }
                            }
                            Yaku::Chiitoitsu => {
                                fu = 25;
                            }
                            _ => (),
                        }

                        yakus.push(yaku_type.clone());
                        han += yaku_type.get_han(table);
                    }
                }
            }

            if han >= 5 {
                fu = 0;
            } else if han > 0 && fu == 0 {
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
                                CompleteShape::Closed(closed) => {
                                    match closed {
                                        ClosedShape::Shuntsu(tiles) => {
                                            if tiles[1].eq(&winning_tile) || // kanchan
                                                tiles[0].prev_id(false, 1) == 0 && tiles[2].eq(&winning_tile) ||
                                                tiles[2].next_id(false, 1) == 0 && tiles[0].eq(&winning_tile)
                                            {
                                                // penchans
                                                has_value_wait = true;
                                            }
                                        }
                                        ClosedShape::Koutsu(tiles) => {
                                            match tiles[0].tile_type {
                                                TileType::Number(value, _) => {
                                                    if value == 1 || value == 9 {
                                                        fu += 8;
                                                    } else {
                                                        fu += 4;
                                                    }
                                                }
                                                TileType::Wind(_) | TileType::Dragon(_) => {
                                                    fu += 8;
                                                }
                                            }

                                            if tiles[0].eq(&winning_tile) {
                                                has_value_wait = true;
                                            }
                                        }
                                        ClosedShape::Kantsu(tiles) => {
                                            match tiles[0].tile_type {
                                                TileType::Number(value, _) => {
                                                    if value == 1 || value == 9 {
                                                        fu += 32;
                                                    } else {
                                                        fu += 16;
                                                    }
                                                }
                                                TileType::Wind(_) | TileType::Dragon(_) => {
                                                    fu += 32;
                                                }
                                            }

                                            if tiles[0].eq(&winning_tile) {
                                                panic!("Can't win on a tile in kan");
                                            }
                                        }
                                        ClosedShape::Toitsu(tiles) => {
                                            match tiles[0].tile_type {
                                                TileType::Number(_, _) => {}
                                                TileType::Wind(value) => {
                                                    // prevalent wind: +2 fu
                                                    // my wind: another +2 fu
                                                    match table.get_prevalent_wind() {
                                                        None => {}
                                                        Some(pw) => {
                                                            if pw == value {
                                                                fu += 2;
                                                            }
                                                        }
                                                    }

                                                    match table.get_my_seat_wind() {
                                                        None => {}
                                                        Some(msw) => {
                                                            if msw == value {
                                                                fu += 2;
                                                            }
                                                        }
                                                    }
                                                }
                                                TileType::Dragon(_) => {
                                                    fu += 2;
                                                }
                                            }

                                            if tiles[0].eq(&winning_tile) {
                                                has_value_wait = true;
                                            }
                                        }
                                        ClosedShape::Single(_) => {}
                                    }
                                }

                                CompleteShape::Open(open) => {
                                    match open {
                                        OpenShape::Chi(_) => {}
                                        OpenShape::Pon(tiles) => match tiles[0].tile_type {
                                            TileType::Number(value, _) => {
                                                if value == 1 || value == 9 {
                                                    fu += 4;
                                                } else {
                                                    fu += 2;
                                                }
                                            }
                                            TileType::Wind(_) | TileType::Dragon(_) => {
                                                fu += 4;
                                            }
                                        },
                                        OpenShape::Kan(tiles) => match tiles[0].tile_type {
                                            TileType::Number(value, _) => {
                                                if value == 1 || value == 9 {
                                                    fu += 8;
                                                } else {
                                                    fu += 4;
                                                }
                                            }
                                            TileType::Wind(_) | TileType::Dragon(_) => {
                                                fu += 8;
                                            }
                                        },
                                    }
                                }
                            }
                        }
                        ShapeType::Incomplete(..) => {}
                    }
                }

                if has_value_wait {
                    fu += 2;
                }
            }

            let score = Score::new(han, fu, table.am_i_oya(), table.did_i_tsumo());
            if i == 0 || score.total_points() > best_variant.1.total_points() {
                best_variant = (yakus, score);
            }
        }

        println!("{:#?}", best_variant);
        Some(best_variant)
    }
}

impl Default for YakuFinder {
    fn default() -> YakuFinder {
        YakuFinder {}
    }
}

////////////////

impl Yaku {
    pub fn get_name(&self) -> &str {
        match self {
            Yaku::MenzenTsumo => "Menzen tsumo",
            Yaku::Riichi => "Riichi",
            Yaku::Ippatsu => "Ippatsu",
            Yaku::Pinfu => "Pinfu",
            Yaku::Iipeikou => "Iipeikou",
            Yaku::Haitei => "Haitei raoyue",
            Yaku::Houtei => "Houtei raoyui",
            Yaku::Rinshan => "Rinshan kaihou",
            Yaku::Chankan => "Chankan",
            Yaku::Tanyao => "Tanyao",
            Yaku::EastRound => "East round winds",
            Yaku::EastSeat => "East seat winds",
            Yaku::SouthRound => "South round winds",
            Yaku::SouthSeat => "South seat winds",
            Yaku::WestRound => "West round winds",
            Yaku::WestSeat => "West seat winds",
            Yaku::NorthSeat => "North seat winds",
            Yaku::WhiteDragons => "White dragons",
            Yaku::GreenDragons => "Green dragons",
            Yaku::RedDragons => "Red dragons",
            Yaku::DoubleRiichi => "Double riichi",
            Yaku::Chanta => "Chantaiyao",
            Yaku::SanshokuDoujun => "Sanshoku doujun",
            Yaku::Ittsu => "Ittsu",
            Yaku::Toitoi => "Toitoi",
            Yaku::Sanankou => "Sanankou",
            Yaku::SanshokuDoukou => "Sanshoku doukou",
            Yaku::Sankantsu => "Sankantsu",
            Yaku::Chiitoitsu => "Chiitoitsu",
            Yaku::Honroutou => "Honroutou",
            Yaku::Shousangen => "Shousangen",
            Yaku::Honitsu => "Honitsu",
            Yaku::Junchan => "Junchan taiyao",
            Yaku::Ryanpeikou => "Ryanpeikou",
            Yaku::Chinitsu => "Chinitsu",
            Yaku::Kazoe => "Kazoe yakuman",
            Yaku::Kokushi => "Kokushi musou",
            Yaku::Suuankou => "Suuankou",
            Yaku::Daisangen => "Daisangen",
            Yaku::Shousuushii => "Shousuushii",
            Yaku::Daisuushii => "Daisuushii",
            Yaku::Tsuuiisou => "Tsuuiisou",
            Yaku::Chinroutou => "Chinroutou",
            Yaku::Ryuuiisou => "Ryuuiisou",
            Yaku::Chuuren => "Chuuren poutou",
            Yaku::Suukantsu => "Suukantsu",
            Yaku::Tenhou => "Tenhou",
            Yaku::Chiihou => "Chiihou",
        }
    }

    pub fn get_han(&self, table: &mut Table) -> u8 {
        match self {
            Yaku::MenzenTsumo => 1,
            Yaku::Riichi => 1,
            Yaku::Ippatsu => 1,
            Yaku::Pinfu => 1,
            Yaku::Iipeikou => 1,
            Yaku::Haitei => 1,
            Yaku::Houtei => 1,
            Yaku::Rinshan => 1,
            Yaku::Chankan => 1,
            Yaku::Tanyao => 1,
            Yaku::EastRound => 1,
            Yaku::EastSeat => 1,
            Yaku::SouthRound => 1,
            Yaku::SouthSeat => 1,
            Yaku::WestRound => 1,
            Yaku::WestSeat => 1,
            Yaku::NorthSeat => 1,
            Yaku::WhiteDragons => 1,
            Yaku::GreenDragons => 1,
            Yaku::RedDragons => 1,
            Yaku::DoubleRiichi => 2,
            Yaku::Chanta => {
                if table.get_my_hand().is_closed() {
                    return 2;
                }

                1
            }
            Yaku::SanshokuDoujun => {
                if table.get_my_hand().is_closed() {
                    return 2;
                }

                1
            }
            Yaku::Ittsu => {
                if table.get_my_hand().is_closed() {
                    return 2;
                }

                1
            }
            Yaku::Toitoi => 2,
            Yaku::Sanankou => 2,
            Yaku::SanshokuDoukou => 2,
            Yaku::Sankantsu => 2,
            Yaku::Chiitoitsu => 2,
            Yaku::Honroutou => 2,
            Yaku::Shousangen => 2,
            Yaku::Honitsu => {
                if table.get_my_hand().is_closed() {
                    return 3;
                }

                2
            }
            Yaku::Junchan => {
                if table.get_my_hand().is_closed() {
                    return 3;
                }

                2
            }
            Yaku::Ryanpeikou => {
                if table.get_my_hand().is_closed() {
                    return 3;
                }

                2
            }
            Yaku::Chinitsu => {
                if table.get_my_hand().is_closed() {
                    return 6;
                }

                5
            }
            Yaku::Kazoe => 13,
            Yaku::Kokushi => 13,
            Yaku::Suuankou => 13,
            Yaku::Daisangen => 13,
            Yaku::Shousuushii => 13,
            Yaku::Daisuushii => 13,
            Yaku::Tsuuiisou => 13,
            Yaku::Chinroutou => 13,
            Yaku::Ryuuiisou => 13,
            Yaku::Chuuren => 13,
            Yaku::Suukantsu => 13,
            Yaku::Tenhou => 13,
            Yaku::Chiihou => 13,
        }
    }

    /// Check if this Yaku exists in this shape variant
    fn is_in_hand(&self, table: &mut Table, variant: &[Shape]) -> bool {
        match self {
            Yaku::MenzenTsumo => return table.get_my_hand().is_closed() && table.did_i_tsumo(),
            Yaku::Riichi => return table.get_my_hand().is_closed() && table.did_i_riichi(),
            Yaku::Ippatsu => {
                if !table.get_my_hand().is_closed() || !table.did_i_riichi() {
                    return false;
                }

                // TODO
            }
            Yaku::Pinfu => {
                if !table.get_my_hand().is_closed() {
                    return false;
                }

                let winning_tile = table.get_my_winning_tile();

                let mut has_ryanmen_wait = false;

                let mut pairs: u8 = 0;
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(tiles) => {
                                    if tiles[0].prev_id(false, 1) == 0 {
                                        if tiles[0].eq(&winning_tile) {
                                            has_ryanmen_wait = true;
                                        }
                                    } else if tiles[2].next_id(false, 1) == 0 {
                                        if tiles[2].eq(&winning_tile) {
                                            has_ryanmen_wait = true;
                                        }
                                    } else if tiles[0].eq(&winning_tile) || tiles[2].eq(&winning_tile) {
                                        has_ryanmen_wait = true;
                                    }
                                }
                                ClosedShape::Koutsu(_)
                                | ClosedShape::Single(_)
                                | ClosedShape::Kantsu(_) => return false,
                                ClosedShape::Toitsu(tiles) => {
                                    pairs += 1;
                                    if pairs > 1 {
                                        return false;
                                    }

                                    match tiles[0].tile_type {
                                        TileType::Wind(value) => match table.get_prevalent_wind() {
                                            None => return false,
                                            Some(prevalent) => match table.get_my_seat_wind() {
                                                None => return false,
                                                Some(seat) => {
                                                    if value == prevalent || value == seat {
                                                        return false;
                                                    }
                                                }
                                            },
                                        },
                                        TileType::Dragon(_) => return false,
                                        _ => (),
                                    }
                                }
                            },

                            CompleteShape::Open(_) => return false,
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return has_ryanmen_wait;
            }
            Yaku::Iipeikou => {
                if !table.get_my_hand().is_closed() {
                    return false;
                }

                return self.find_peikou(variant) == 1;
            }
            Yaku::Haitei => {
                return match table.get_tiles_remaining() {
                    None => false,
                    Some(remaining) => table.did_i_tsumo() && remaining == 0,
                }
            }
            Yaku::Houtei => {
                return match table.get_tiles_remaining() {
                    None => false,
                    Some(remaining) => !table.did_i_tsumo() && remaining == 0,
                }
            }
            Yaku::Rinshan => {} // TODO
            Yaku::Chankan => {} // TODO
            Yaku::Tanyao => {
                let array_34 = table.get_my_hand().get_34_array(false);
                // can't contain any terminals or honors
                for (i, count) in array_34.iter().enumerate() {
                    if ([1, 9, 10, 18, 19, 27].contains(&(i + 1)) || (i + 1) >= 28) && *count > 0 {
                        return false;
                    }
                }

                return true;
            }
            Yaku::EastRound => {
                match table.get_prevalent_wind() {
                    None => return false,
                    Some(prevalent) => {
                        if prevalent != 1 {
                            return false;
                        }
                    }
                }

                return self.find_yakuhai(variant, 28);
            }
            Yaku::EastSeat => {
                match table.get_my_seat_wind() {
                    None => return false,
                    Some(seat) => {
                        if seat != 1 {
                            return false;
                        }
                    }
                }

                return self.find_yakuhai(variant, 28);
            }
            Yaku::SouthRound => {
                match table.get_prevalent_wind() {
                    None => return false,
                    Some(prevalent) => {
                        if prevalent != 2 {
                            return false;
                        }
                    }
                }

                return self.find_yakuhai(variant, 29);
            }
            Yaku::SouthSeat => {
                match table.get_my_seat_wind() {
                    None => return false,
                    Some(seat) => {
                        if seat != 2 {
                            return false;
                        }
                    }
                }

                return self.find_yakuhai(variant, 29);
            }
            Yaku::WestRound => {
                match table.get_prevalent_wind() {
                    None => return false,
                    Some(prevalent) => {
                        if prevalent != 3 {
                            return false;
                        }
                    }
                }

                return self.find_yakuhai(variant, 30);
            }
            Yaku::WestSeat => {
                match table.get_my_seat_wind() {
                    None => return false,
                    Some(seat) => {
                        if seat != 3 {
                            return false;
                        }
                    }
                }

                return self.find_yakuhai(variant, 30);
            }
            Yaku::NorthSeat => {
                match table.get_my_seat_wind() {
                    None => return false,
                    Some(seat) => {
                        if seat != 4 {
                            return false;
                        }
                    }
                }

                return self.find_yakuhai(variant, 31);
            }
            Yaku::WhiteDragons => return self.find_yakuhai(variant, 32),
            Yaku::GreenDragons => return self.find_yakuhai(variant, 33),
            Yaku::RedDragons => return self.find_yakuhai(variant, 34),
            Yaku::DoubleRiichi => {} // TODO
            Yaku::Chanta => {
                // a chanta has to have shuntsu, else it's honroutou
                let mut has_shuntsu = false;
                // a chanta has to have honors, else it's junchan
                let mut has_honors = false;
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(tiles) => {
                                    if !tiles[0].is_terminal() && !tiles[2].is_terminal() {
                                        return false;
                                    }

                                    has_shuntsu = true;
                                }
                                ClosedShape::Koutsu(tiles) => {
                                    if !tiles[0].is_terminal_or_honor() {
                                        return false;
                                    }

                                    if tiles[0].is_honor() {
                                        has_honors = true;
                                    }
                                }
                                ClosedShape::Kantsu(tiles) => {
                                    if !tiles[0].is_terminal_or_honor() {
                                        return false;
                                    }

                                    if tiles[0].is_honor() {
                                        has_honors = true;
                                    }
                                }
                                ClosedShape::Toitsu(tiles) => {
                                    if !tiles[0].is_terminal_or_honor() {
                                        return false;
                                    }

                                    if tiles[0].is_honor() {
                                        has_honors = true;
                                    }
                                }
                                ClosedShape::Single(_) => {
                                    return false;
                                }
                            },

                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(tiles) => {
                                    if !tiles[0].is_terminal() && !tiles[2].is_terminal() {
                                        return false;
                                    }

                                    has_shuntsu = true;
                                }
                                OpenShape::Pon(tiles) => {
                                    if !tiles[0].is_terminal_or_honor() {
                                        return false;
                                    }

                                    if tiles[0].is_honor() {
                                        has_honors = true;
                                    }
                                }
                                OpenShape::Kan(tiles) => {
                                    if !tiles[0].is_terminal_or_honor() {
                                        return false;
                                    }

                                    if tiles[0].is_honor() {
                                        has_honors = true;
                                    }
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => {
                            return false;
                        }
                    }
                }

                return has_shuntsu && has_honors;
            }
            Yaku::SanshokuDoujun => {
                let mut combos: HashMap<String, [bool; 3]> = HashMap::new();

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(tiles) => if let TileType::Number(value, suit) = tiles[0].tile_type {
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
                                ClosedShape::Single(_) => return false,
                                _ => (),
                            },
                            CompleteShape::Open(open) => if let OpenShape::Chi(tiles) = open {
                                if let TileType::Number(value, suit) = tiles[0].tile_type {
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
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }
            }
            Yaku::Ittsu => {
                let mut parts: [u8; 3] = [0, 0, 0];

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(tiles) => if let TileType::Number(value, color) = tiles[0].tile_type {
                                    if value == 1 {
                                        if color.to_char() == 'm' && parts[0] == 0 {
                                            parts[0] = 1;
                                        } else if color.to_char() == 'p' && parts[1] == 0 {
                                            parts[1] = 1;
                                        } else if color.to_char() == 's' && parts[2] == 0 {
                                            parts[2] = 1;
                                        }
                                    } else if value == 4 {
                                        if color.to_char() == 'm' && parts[0] == 1 {
                                            parts[0] = 2;
                                        } else if color.to_char() == 'p' && parts[1] == 1 {
                                            parts[1] = 2;
                                        } else if color.to_char() == 's' && parts[2] == 1 {
                                            parts[2] = 2;
                                        }
                                    } else if value == 7 {
                                        if color.to_char() == 'm' && parts[0] == 2 {
                                            parts[0] = 3;
                                        } else if color.to_char() == 'p' && parts[1] == 2 {
                                            parts[1] = 3;
                                        } else if color.to_char() == 's' && parts[2] == 2 {
                                            parts[2] = 3;
                                        }
                                    }
                                },
                                ClosedShape::Single(_) => return false,
                                _ => (),
                            },
                            CompleteShape::Open(open) => if let OpenShape::Chi(tiles) = open {
                                 if let TileType::Number(value, color) = tiles[0].tile_type {
                                    if value == 1 {
                                        if color.to_char() == 'm' && parts[0] == 0 {
                                            parts[0] = 1;
                                        } else if color.to_char() == 'p' && parts[1] == 0 {
                                            parts[1] = 1;
                                        } else if color.to_char() == 's' && parts[2] == 0 {
                                            parts[2] = 1;
                                        }
                                    } else if value == 4 {
                                        if color.to_char() == 'm' && parts[0] == 1 {
                                            parts[0] = 2;
                                        } else if color.to_char() == 'p' && parts[1] == 1 {
                                            parts[1] = 2;
                                        } else if color.to_char() == 's' && parts[2] == 1 {
                                            parts[2] = 2;
                                        }
                                    } else if value == 7 {
                                        if color.to_char() == 'm' && parts[0] == 2 {
                                            parts[0] = 3;
                                        } else if color.to_char() == 'p' && parts[1] == 2 {
                                            parts[1] = 3;
                                        } else if color.to_char() == 's' && parts[2] == 2 {
                                            parts[2] = 3;
                                        }
                                    }
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return parts[0] == 3 || parts[1] == 3 || parts[2] == 3;
            }
            Yaku::Toitoi => {
                let winning_tile = table.get_my_winning_tile();

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => {
                            match cs {
                                CompleteShape::Closed(closed) => {
                                    match closed {
                                        ClosedShape::Shuntsu(_) | ClosedShape::Single(_) => {
                                            return false
                                        }
                                        ClosedShape::Koutsu(_) | ClosedShape::Kantsu(_) => {}
                                        ClosedShape::Toitsu(tiles) => {
                                            if table.get_my_hand().is_closed()
                                                && tiles[0].eq(&winning_tile)
                                            {
                                                return false; // this is suuankou
                                            }
                                        }
                                    }
                                }
                                CompleteShape::Open(open) => if let OpenShape::Chi(_) = open {
                                    return false;
                                }
                            }
                        }
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return true;
            }
            Yaku::Sanankou => {
                let winning_tile = table.get_my_winning_tile();
                let mut ankou: u8 = 0;
                let mut has_tanki_wait = false;
                let mut has_shuntsu_wait = false;

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(tiles) => {
                                    if tiles[0].eq(&winning_tile)
                                        || tiles[1].eq(&winning_tile)
                                        || tiles[2].eq(&winning_tile)
                                    {
                                        has_shuntsu_wait = true;
                                    }
                                }
                                ClosedShape::Koutsu(_) => {
                                    ankou += 1;
                                }
                                ClosedShape::Kantsu(_) => {
                                    ankou += 1;
                                }
                                ClosedShape::Toitsu(tiles) => {
                                    if table.get_my_hand().is_closed() && tiles[0].eq(&winning_tile)
                                    {
                                        has_tanki_wait = true;
                                    }
                                }
                                ClosedShape::Single(_) => return false,
                            },
                            CompleteShape::Open(_) => {}
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                if ankou < 3 {
                    return false;
                }

                if !table.get_my_hand().is_closed() || ankou == 3 {
                    // open hand
                    if table.did_i_tsumo() || has_tanki_wait || has_shuntsu_wait {
                        return true;
                    }
                } else if ankou == 4 {
                    // else it's suuankou
                    if !table.did_i_tsumo() && !has_tanki_wait {
                        return true;
                    }
                }
            }
            Yaku::SanshokuDoukou => {
                println!("{:#?}", variant);

                let mut combos: HashMap<String, [bool; 3]> = HashMap::new();

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Koutsu(tiles) => {
                                    if self.ssdk_eval(&tiles[0], &mut combos) {
                                        return true;
                                    }
                                }
                                ClosedShape::Kantsu(tiles) => {
                                    if self.ssdk_eval(&tiles[0], &mut combos) {
                                        return true;
                                    }
                                }
                                ClosedShape::Single(_) => return false,
                                _ => {}
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(_) => {}
                                OpenShape::Pon(tiles) => {
                                    if self.ssdk_eval(&tiles[0], &mut combos) {
                                        return true;
                                    }
                                }
                                OpenShape::Kan(tiles) => {
                                    if self.ssdk_eval(&tiles[0], &mut combos) {
                                        return true;
                                    }
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }
            }
            Yaku::Sankantsu => {} // TODO
            Yaku::Chiitoitsu => {
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(_)
                                | ClosedShape::Koutsu(_)
                                | ClosedShape::Kantsu(_)
                                | ClosedShape::Single(_) => return false,
                                ClosedShape::Toitsu(_) => (),
                            },
                            CompleteShape::Open(_) => return false,
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return true;
            }
            Yaku::Honroutou => {
                let mut has_terminals = false;
                let mut has_honors = false;

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(_) | ClosedShape::Single(_) => return false,
                                ClosedShape::Koutsu(tiles) => {
                                    if !self.honroutou_eval(
                                        &tiles[0],
                                        &mut has_terminals,
                                        &mut has_honors,
                                    ) {
                                        return false;
                                    }
                                }
                                ClosedShape::Kantsu(tiles) => {
                                    if !self.honroutou_eval(
                                        &tiles[0],
                                        &mut has_terminals,
                                        &mut has_honors,
                                    ) {
                                        return false;
                                    }
                                }
                                ClosedShape::Toitsu(tiles) => {
                                    if !self.honroutou_eval(
                                        &tiles[0],
                                        &mut has_terminals,
                                        &mut has_honors,
                                    ) {
                                        return false;
                                    }
                                }
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(_) => return false,
                                OpenShape::Pon(tiles) => {
                                    if !self.honroutou_eval(
                                        &tiles[0],
                                        &mut has_terminals,
                                        &mut has_honors,
                                    ) {
                                        return false;
                                    }
                                }
                                OpenShape::Kan(tiles) => {
                                    if !self.honroutou_eval(
                                        &tiles[0],
                                        &mut has_terminals,
                                        &mut has_honors,
                                    ) {
                                        return false;
                                    }
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return has_honors && has_terminals;
            }
            Yaku::Shousangen => {
                let mut dragon_pons: u8 = 0;
                let mut has_dragon_pair = false;

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Koutsu(tiles) => {
                                    if !self.shousangen_eval(&tiles[0], &mut dragon_pons) {
                                        return false;
                                    }
                                }
                                ClosedShape::Kantsu(tiles) => {
                                    if !self.shousangen_eval(&tiles[0], &mut dragon_pons) {
                                        return false;
                                    }
                                }
                                ClosedShape::Toitsu(tiles) => if let TileType::Dragon(_) = tiles[0].tile_type {
                                    has_dragon_pair = true;
                                }
                                ClosedShape::Single(_) => return false,
                                _ => (),
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Pon(tiles) => {
                                    if !self.shousangen_eval(&tiles[0], &mut dragon_pons) {
                                        return false;
                                    }
                                }
                                OpenShape::Kan(tiles) => {
                                    if !self.shousangen_eval(&tiles[0], &mut dragon_pons) {
                                        return false;
                                    }
                                }
                                _ => (),
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return dragon_pons == 2 && has_dragon_pair;
            }
            Yaku::Honitsu => {
                let mut has_honors = false;
                let mut suit = 'x';
                for o_t in table.get_my_hand().get_tiles().iter() {
                    match o_t {
                        None => {}
                        Some(tile) => {
                            if tile.get_type_char() == 'z' {
                                has_honors = true;
                            } else if suit == 'x' {
                                suit = tile.get_type_char();
                            } else if suit != tile.get_type_char() {
                                return false;
                            }
                        }
                    }
                }

                return has_honors;
            }
            Yaku::Junchan => {
                // Junchan has to have shuntsu, else it's chinroutou
                let mut has_shuntsu = false;
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(tiles) => {
                                    if !tiles[0].is_terminal() && !tiles[2].is_terminal() {
                                        return false;
                                    }

                                    has_shuntsu = true;
                                }
                                ClosedShape::Koutsu(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                                ClosedShape::Kantsu(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                                ClosedShape::Toitsu(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                                ClosedShape::Single(_) => return false,
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(tiles) => {
                                    if !tiles[0].is_terminal() && !tiles[2].is_terminal() {
                                        return false;
                                    }

                                    has_shuntsu = true;
                                }
                                OpenShape::Pon(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                                OpenShape::Kan(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => {
                            return false;
                        }
                    }
                }

                return has_shuntsu;
            }
            Yaku::Ryanpeikou => {
                if !table.get_my_hand().is_closed() {
                    return false;
                }

                return self.find_peikou(variant) == 2;
            }
            Yaku::Chinitsu => {
                let mut suit = 'x';
                for o_t in table.get_my_hand().get_tiles().iter() {
                    match o_t {
                        None => {}
                        Some(tile) => {
                            if tile.get_type_char() == 'z' {
                                return false;
                            } else if suit == 'x' {
                                suit = tile.get_type_char();
                            } else if suit != tile.get_type_char() {
                                return false;
                            }
                        }
                    }
                }

                return true;
            }

            Yaku::Kazoe => {} // TODO
            Yaku::Kokushi => {
                if !table.get_my_hand().is_closed() {
                    return false;
                }

                let mut has_pair = false;
                for (i, count) in table.get_my_hand().get_34_array(true).iter().enumerate() {
                    if *count == 0 {
                        continue;
                    }

                    if !([1, 9, 10, 18, 19, 27, 28, 29, 30, 31, 32, 33, 34].contains(&(i + 1))) {
                        return false;
                    }

                    if *count > 2 {
                        return false;
                    }

                    if *count == 2 {
                        if has_pair {
                            return false;
                        } else {
                            has_pair = true;
                        }
                    }
                }

                return has_pair;
            }
            Yaku::Suuankou => {
                if !table.get_my_hand().is_closed() {
                    return false;
                }

                let winning_tile = table.get_my_winning_tile();
                let mut has_tanki_wait = false;
                let mut pair_found = false;

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Koutsu(_) | ClosedShape::Kantsu(_) => (),
                                ClosedShape::Toitsu(tiles) => {
                                    if pair_found {
                                        return false;
                                    } else {
                                        pair_found = true;
                                    }

                                    if tiles[0].eq(&winning_tile) {
                                        has_tanki_wait = true;
                                    }
                                }
                                ClosedShape::Single(_) | ClosedShape::Shuntsu(_) => return false,
                            },
                            CompleteShape::Open(_) => return false,
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return table.did_i_tsumo() || has_tanki_wait;
            }
            Yaku::Daisangen => {
                let mut dragon_pons: u8 = 0;

                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Koutsu(tiles) => if let TileType::Dragon(_) = tiles[0].tile_type {
                                    dragon_pons += 1;
                                },
                                ClosedShape::Kantsu(tiles) => if let TileType::Dragon(_) = tiles[0].tile_type {
                                    dragon_pons += 1;
                                },
                                ClosedShape::Toitsu(tiles) => if let TileType::Dragon(_) = tiles[0].tile_type {
                                    return false;
                                },
                                ClosedShape::Single(_) => return false,
                                _ => (),
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(_) => {}
                                OpenShape::Pon(tiles) => if let TileType::Dragon(_) = tiles[0].tile_type {
                                    dragon_pons += 1;
                                },
                                OpenShape::Kan(tiles) => if let TileType::Dragon(_) = tiles[0].tile_type {
                                    dragon_pons += 1;
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return dragon_pons == 3;
            }
            Yaku::Shousuushii => {
                let mut has_wind_pair = false;
                let mut other: u8 = 0;
                let mut toitsu_count: u8 = 0;
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(_) => {
                                    other += 1;
                                }
                                ClosedShape::Koutsu(tiles) => match tiles[0].tile_type {
                                    TileType::Wind(_value) => {}
                                    _ => {
                                        other += 1;
                                    }
                                },
                                ClosedShape::Kantsu(tiles) => match tiles[0].tile_type {
                                    TileType::Wind(_value) => {}
                                    _ => {
                                        other += 1;
                                    }
                                },
                                ClosedShape::Toitsu(tiles) => {
                                    toitsu_count += 1;

                                    if toitsu_count > 1 {
                                        return false;
                                    }

                                    if let TileType::Wind(_) = tiles[0].tile_type {
                                        if has_wind_pair {
                                            return false;
                                        }

                                        has_wind_pair = true;
                                    }
                                }
                                ClosedShape::Single(_) => return false,
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(_) => {
                                    other += 1;
                                }
                                OpenShape::Pon(tiles) => match tiles[0].tile_type {
                                    TileType::Wind(_value) => {}
                                    _ => {
                                        other += 1;
                                    }
                                },
                                OpenShape::Kan(tiles) => match tiles[0].tile_type {
                                    TileType::Wind(_value) => {}
                                    _ => {
                                        other += 1;
                                    }
                                },
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return other < 2 && has_wind_pair;
            }
            Yaku::Daisuushii => {
                let mut other: u8 = 0;
                let mut toitsu_count: u8 = 0;
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Koutsu(tiles) => match tiles[0].tile_type {
                                    TileType::Wind(_value) => {}
                                    _ => {
                                        other += 1;
                                    }
                                },
                                ClosedShape::Kantsu(tiles) => match tiles[0].tile_type {
                                    TileType::Wind(_value) => {}
                                    _ => {
                                        other += 1;
                                    }
                                },
                                ClosedShape::Toitsu(tiles) => {
                                    toitsu_count += 1;

                                    if toitsu_count > 1 {
                                        return false;
                                    }

                                    if let TileType::Wind(_) = tiles[0].tile_type {
                                        return false;
                                    }
                                }
                                _ => return false,
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(_) => return false,
                                OpenShape::Pon(tiles) => match tiles[0].tile_type {
                                    TileType::Wind(_value) => {}
                                    _ => {
                                        other += 1;
                                    }
                                },
                                OpenShape::Kan(tiles) => match tiles[0].tile_type {
                                    TileType::Wind(_value) => {}
                                    _ => {
                                        other += 1;
                                    }
                                },
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return other == 0;
            }
            Yaku::Tsuuiisou => {
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(_) | ClosedShape::Single(_) => return false,
                                ClosedShape::Koutsu(tiles) => {
                                    if !tiles[0].is_honor() {
                                        return false;
                                    }
                                }
                                ClosedShape::Kantsu(tiles) => {
                                    if !tiles[0].is_honor() {
                                        return false;
                                    }
                                }
                                ClosedShape::Toitsu(tiles) => {
                                    if !tiles[0].is_honor() {
                                        return false;
                                    }
                                }
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(_) => return false,
                                OpenShape::Pon(tiles) => {
                                    if !tiles[0].is_honor() {
                                        return false;
                                    }
                                }
                                OpenShape::Kan(tiles) => {
                                    if !tiles[0].is_honor() {
                                        return false;
                                    }
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return true;
            }
            Yaku::Chinroutou => {
                for shape in variant.iter() {
                    match shape.get_shape_type() {
                        ShapeType::Complete(cs) => match cs {
                            CompleteShape::Closed(closed) => match closed {
                                ClosedShape::Shuntsu(_) | ClosedShape::Single(_) => return false,
                                ClosedShape::Koutsu(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                                ClosedShape::Kantsu(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                                ClosedShape::Toitsu(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                            },
                            CompleteShape::Open(open) => match open {
                                OpenShape::Chi(_) => return false,
                                OpenShape::Pon(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                                OpenShape::Kan(tiles) => {
                                    if !tiles[0].is_terminal() {
                                        return false;
                                    }
                                }
                            },
                        },
                        ShapeType::Incomplete(..) => return false,
                    }
                }

                return true;
            }
            Yaku::Ryuuiisou => {
                // 2,3,4,6,8s, 6z
                for o_t in table.get_my_hand().get_tiles() {
                    match o_t {
                        None => {}
                        Some(tile) => {
                            if !([20, 21, 22, 24, 26, 33].contains(&tile.to_id())) {
                                return false;
                            }
                        }
                    }
                }

                return true;
            }
            Yaku::Chuuren => {
                if !table.get_my_hand().is_closed() {
                    return false;
                }

                // 1112345678999 + 1 tile

                let array_34 = table.get_my_hand().get_34_array(true);
                let offset;

                if array_34[0] >= 3 {
                    offset = 0;
                } else if array_34[9] >= 3 {
                    offset = 9;
                } else if array_34[18] >= 3 {
                    offset = 18;
                } else {
                    return false;
                }

                if array_34[8 + offset] < 3 {
                    return false;
                }

                if array_34[1 + offset] == 0
                    || array_34[1 + offset] > 2
                    || array_34[2 + offset] == 0
                    || array_34[2 + offset] > 2
                    || array_34[3 + offset] == 0
                    || array_34[3 + offset] > 2
                    || array_34[4 + offset] == 0
                    || array_34[4 + offset] > 2
                    || array_34[5 + offset] == 0
                    || array_34[5 + offset] > 2
                    || array_34[6 + offset] == 0
                    || array_34[6 + offset] > 2
                    || array_34[7 + offset] == 0
                    || array_34[7 + offset] > 2
                {
                    return false;
                }

                return true;
            }
            Yaku::Suukantsu => {} // TODO
            Yaku::Tenhou => {
                if !table.get_my_hand().is_closed() {
                    return false;
                }

                // TODO
            }
            Yaku::Chiihou => {
                if !table.get_my_hand().is_closed() {
                    return false;
                }
                // TODO
            }
        }

        false
    }

    pub fn is_yakuman(&self) -> bool {
        if matches!(self, Yaku::Kokushi
            | Yaku::Suuankou
            | Yaku::Daisangen
            | Yaku::Shousuushii
            | Yaku::Daisuushii
            | Yaku::Tsuuiisou
            | Yaku::Chinroutou
            | Yaku::Ryuuiisou
            | Yaku::Chuuren
            | Yaku::Suukantsu
            | Yaku::Tenhou
            | Yaku::Chiihou) {
            return true;
        }

        false
    }

    fn find_yakuhai(&self, variant: &[Shape], tile_id: u8) -> bool {
        for shape in variant.iter() {
            match shape.get_shape_type() {
                ShapeType::Complete(cs) => match cs {
                    CompleteShape::Closed(closed) => match closed {
                        ClosedShape::Koutsu(tiles) => {
                            if tiles[0].to_id() == tile_id {
                                return true;
                            }
                        }
                        ClosedShape::Kantsu(tiles) => {
                            if tiles[0].to_id() == tile_id {
                                return true;
                            }
                        }
                        _ => (),
                    },
                    CompleteShape::Open(open) => match open {
                        OpenShape::Chi(_) => {}
                        OpenShape::Pon(tiles) => {
                            if tiles[0].to_id() == tile_id {
                                return true;
                            }
                        }
                        OpenShape::Kan(tiles) => {
                            if tiles[0].to_id() == tile_id {
                                return true;
                            }
                        }
                    },
                },
                ShapeType::Incomplete(..) => (),
            }
        }

        false
    }

    fn find_peikou(&self, variant: &[Shape]) -> u8 {
        let mut map: HashMap<String, u8> = HashMap::new();
        let mut count = 0;

        for shape in variant.iter() {
            match shape.get_shape_type() {
                ShapeType::Complete(cs) => match cs {
                    CompleteShape::Closed(closed) => match closed {
                        ClosedShape::Shuntsu(tiles) => {
                            let key = format!(
                                "{}{}{}",
                                tiles[0].to_string(),
                                tiles[1].to_string(),
                                tiles[2].to_string()
                            );

                            match map.entry(key.clone()) {
                                Entry::Occupied(occupied) => {
                                    let v = *occupied.get();
                                    map.remove(&key);
                                    map.insert(key, v + 1);

                                    if v == 1 {
                                        count += 1;
                                    }
                                }
                                Entry::Vacant(_) => {
                                    map.insert(key, 1);
                                }
                            };

                            // if map.contains_key(&key) {
                            //     let v = *map.get(&key).unwrap();
                            //     map.remove(&key);
                            //     map.insert(key, v + 1);
                            //
                            //     if v == 1 {
                            //         count += 1;
                            //     }
                            // } else {
                            //     map.insert(key, 1);
                            // }
                        }
                        ClosedShape::Single(_) => return 0,
                        _ => (),
                    },
                    CompleteShape::Open(_) => {}
                },
                ShapeType::Incomplete(..) => return 0,
            }
        }

        count
    }

    /// Sanshoku doukou evaluation function
    fn ssdk_eval(&self, tile: &Tile, combos: &mut HashMap<String, [bool; 3]>) -> bool {
        if let TileType::Number(value, suit) = tile.tile_type {
            let key = format!("{}", value);
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
        }

        false
    }

    fn honroutou_eval(&self, tile: &Tile, has_terminals: &mut bool, has_honors: &mut bool) -> bool {
        if !tile.is_terminal_or_honor() {
            return false;
        }

        if tile.is_terminal() {
            *has_terminals = true;
        } else if tile.is_honor() {
            *has_honors = true;
        }

        true
    }

    fn shousangen_eval(&self, tile: &Tile, dragon_pons: &mut u8) -> bool {
        if let TileType::Dragon(_) = tile.tile_type {
            *dragon_pons += 1;

            if *dragon_pons > 2u8 {
                // daisangen
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Map, Value};

    #[test]
    fn find_1_30_score() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("123m234s456789p11z")); // won on east tanki wait
        map.insert("my_tsumo".to_string(), Value::from(true));
        map.insert("prevalent_wind".to_string(), Value::from(1)); // east as prevalent wind = +2 fu

        let mut table = Table::from_map(&map).unwrap();
        let (_yakus, score) = table.yaku().unwrap();

        assert_eq!(score.han, 1);
        assert_eq!(score.fu, 30);
    }

    #[test]
    fn find_1_30_kanchan() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("123m24s456789p22z3s"));
        map.insert("my_tsumo".to_string(), Value::from(true));
        map.insert("prevalent_wind".to_string(), Value::from(1));

        let mut table = Table::from_map(&map).unwrap();
        let (_yakus, score) = table.yaku().unwrap();

        assert_eq!(score.han, 1);
        assert_eq!(score.fu, 30);
    }

    #[test]
    fn find_mentanpin() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("23467m234567s88p5m"));
        map.insert("my_tsumo".to_string(), Value::from(true));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::MenzenTsumo => true,
            _ => false,
        });
        assert!(match res.0.get(1).unwrap() {
            Yaku::Pinfu => true,
            _ => false,
        });
        assert!(match res.0.get(2).unwrap() {
            Yaku::Tanyao => true,
            _ => false,
        });
    }

    #[test]
    fn find_yakuhai_south() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("222z123789m444s33p"));
        map.insert("my_seat_wind".to_string(), Value::from(2));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::SouthSeat => true,
            _ => false,
        });
    }

    #[test]
    fn find_tanyao_chiitoi() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("224466m4477s3388p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Tanyao => true,
            _ => false,
        });
        assert!(match res.0.get(1).unwrap() {
            Yaku::Chiitoitsu => true,
            _ => false,
        });
    }

    #[test]
    fn find_white_dragons_closed() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("123m234s67888p555z"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::WhiteDragons => true,
            _ => false,
        });
    }

    #[test]
    fn find_white_dragons_open() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("123m234s67888p(p5z1)"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::WhiteDragons => true,
            _ => false,
        });
    }

    #[test]
    fn find_sanshoku_doujun_closed() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("234m234567s23499p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::SanshokuDoujun => true,
            _ => false,
        });
    }

    #[test]
    fn find_sanshoku_doujun_open() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("234m234567s99p(234p2)"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::SanshokuDoujun => true,
            _ => false,
        });
    }

    #[test]
    fn find_sanshoku_doukou() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("222m222567s99p(p2p1)"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::SanshokuDoukou => true,
            _ => false,
        });
    }

    #[test]
    fn find_ittsu_closed() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("11m123456789s444p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Ittsu => true,
            _ => false,
        });
    }

    #[test]
    fn find_ittsu_open() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("11m123456s444p(789s0)"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Ittsu => true,
            _ => false,
        });
    }

    #[test]
    fn find_toitoi_sanankou() {
        let mut map = Map::new();
        // wins on a shanpon wait
        map.insert("my_hand".to_string(), Value::from("111m22255p777s22z5p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Toitoi => true,
            _ => false,
        });
        assert!(match res.0.get(1).unwrap() {
            Yaku::Sanankou => true,
            _ => false,
        });
    }

    #[test]
    fn find_suuankou() {
        let mut map = Map::new();
        // wins on a shanpon wait
        map.insert("my_hand".to_string(), Value::from("111m222555p777s22z"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Suuankou => true,
            _ => false,
        });
    }

    #[test]
    fn find_toitoi_sanankou_sanshoku() {
        let mut map = Map::new();
        // wins on a shanpon wait
        map.insert("my_hand".to_string(), Value::from("111m11155p111s22z5p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Toitoi => true,
            _ => false,
        });
        assert!(match res.0.get(1).unwrap() {
            Yaku::Sanankou => true,
            _ => false,
        });
        assert!(match res.0.get(2).unwrap() {
            Yaku::SanshokuDoukou => true,
            _ => false,
        });
    }

    #[test]
    fn find_toitoi_sanshoku() {
        let mut map = Map::new();
        // wins on a shanpon wait
        map.insert("my_hand".to_string(), Value::from("111m11155p22z5p(p1s3)"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Toitoi => true,
            _ => false,
        });
        assert!(match res.0.get(1).unwrap() {
            Yaku::SanshokuDoukou => true,
            _ => false,
        });
    }

    #[test]
    fn find_iipeikou() {
        let mut map = Map::new();
        // wins on a shanpon wait
        map.insert("my_hand".to_string(), Value::from("112233789m45677p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Iipeikou => true,
            _ => false,
        });
    }

    #[test]
    fn find_ryanpeikou() {
        let mut map = Map::new();
        // wins on a shanpon wait
        map.insert("my_hand".to_string(), Value::from("112233m45645688p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Ryanpeikou => true,
            _ => false,
        });
    }

    #[test]
    fn find_chinitsu() {
        let mut map = Map::new();
        // wins on a shanpon wait
        map.insert("my_hand".to_string(), Value::from("12322244467899p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Chinitsu => true,
            _ => false,
        });
    }

    #[test]
    fn find_kokushi() {
        let mut map = Map::new();
        // wins on a shanpon wait
        map.insert("my_hand".to_string(), Value::from("119m19p19s1234567z"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Kokushi => true,
            _ => false,
        });
    }

    #[test]
    fn find_shousangen() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("234m789s55566677z"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::WhiteDragons => true,
            _ => false,
        });
        assert!(match res.0.get(1).unwrap() {
            Yaku::GreenDragons => true,
            _ => false,
        });
        assert!(match res.0.get(2).unwrap() {
            Yaku::Shousangen => true,
            _ => false,
        });
    }

    #[test]
    fn find_daisangen() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("234m88s555666777z"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Daisangen => true,
            _ => false,
        });
    }

    #[test]
    fn find_shousuushii() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("11222333444z789p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Shousuushii => true,
            _ => false,
        });
    }

    #[test]
    fn find_daisuushii() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("111222333444z77p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Suuankou => true,
            _ => false,
        });
        assert!(match res.0.get(1).unwrap() {
            Yaku::Daisuushii => true,
            _ => false,
        });
    }

    #[test]
    fn find_ryuuiisou() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("23433366688s666z"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Ryuuiisou => true,
            _ => false,
        });
    }

    #[test]
    fn find_chuuren() {
        let mut map = Map::new();
        map.insert("my_hand".to_string(), Value::from("11123455678999p"));

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Chuuren => true,
            _ => false,
        });
    }

    #[test]
    fn find_open_tanyao() {
        let mut map = Map::new();
        map.insert(
            "my_hand".to_string(),
            Value::from("234s22555p(p2m2)(345m1)"),
        );

        let mut table = Table::from_map(&map).unwrap();
        let res = table.yaku().unwrap();
        assert!(match res.0.get(0).unwrap() {
            Yaku::Tanyao => true,
            _ => false,
        });
    }
}

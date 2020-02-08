use crate::riichi::hand::Hand;
use crate::riichi::shape_finder::ShapeFinder;
use crate::riichi::shapes::Shape;
use enum_iterator::IntoEnumIterator;
use std::collections::HashMap;

#[derive(IntoEnumIterator)]
pub enum YakuType {
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
    Yakuhai,
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

    pub fn find(&self, hand: &mut Hand) {
        let mut sf = ShapeFinder::new();
        let variants = sf.find(hand);
        let mut variant_yaku: HashMap<usize, Vec<YakuType>> = HashMap::new();

        for (i, variant) in variants.iter().enumerate() {
            let mut yakus: Vec<YakuType> = vec!();
            for yaku_type in YakuType::into_enum_iter() {
                if yaku_type.is_in_hand(hand, variant) {
                    yakus.push(yaku_type);
                }
            }

            variant_yaku.insert(i, yakus);
        }
    }
}

impl Default for YakuFinder {
    fn default() -> YakuFinder {
        YakuFinder {

        }
    }
}

////////////////

pub trait Yaku {
    fn get_name(&self) -> &str;
    fn get_han(&self) -> u8;
    fn is_in_hand(&self, hand: &mut Hand, variant: &Vec<Shape>) -> bool;
}

////////////////

impl Yaku for YakuType {
    fn get_name(&self) -> &str {
        match self {
            YakuType::MenzenTsumo =>    "Menzen tsumo",
            YakuType::Riichi =>         "Riichi",
            YakuType::Ippatsu =>        "Ippatsu",
            YakuType::Pinfu =>          "Pinfu",
            YakuType::Iipeikou =>       "Iipeikou",
            YakuType::Haitei =>         "Haitei raoyue",
            YakuType::Houtei =>         "Houtei raoyui",
            YakuType::Rinshan =>        "Rinshan kaihou",
            YakuType::Chankan =>        "Chankan",
            YakuType::Tanyao =>         "Tanyao",
            YakuType::Yakuhai =>        "Yakuhai",
            YakuType::DoubleRiichi =>   "Double riichi",
            YakuType::Chanta =>         "Chantaiyao",
            YakuType::SanshokuDoujun => "Sanshoku doujun",
            YakuType::Ittsu =>          "Ittsu",
            YakuType::Toitoi =>         "Toitoi",
            YakuType::Sanankou =>       "Sanankou",
            YakuType::SanshokuDoukou => "Sanshoku doukou",
            YakuType::Sankantsu =>      "Sankantsu",
            YakuType::Chiitoitsu =>     "Chiitoitsu",
            YakuType::Honroutou =>      "Honroutou",
            YakuType::Shousangen =>     "Shousangen",
            YakuType::Honitsu =>        "Honitsu",
            YakuType::Junchan =>        "Junchan taiyao",
            YakuType::Ryanpeikou =>     "Ryanpeikou",
            YakuType::Chinitsu =>       "Chinitsu",
            YakuType::Kazoe =>          "Kazoe yakuman",
            YakuType::Kokushi =>        "Kokushi musou",
            YakuType::Suuankou =>       "Suuankou",
            YakuType::Daisangen =>      "Daisangen",
            YakuType::Shousuushii =>    "Shousuushii",
            YakuType::Daisuushii =>     "Daisuushii",
            YakuType::Tsuuiisou =>      "Tsuuiisou",
            YakuType::Chinroutou =>     "Chinroutou",
            YakuType::Ryuuiisou =>      "Ryuuiisou",
            YakuType::Chuuren =>        "Chuuren poutou",
            YakuType::Suukantsu =>      "Suukantsu",
            YakuType::Tenhou =>         "Tenhou",
            YakuType::Chiihou =>        "Chiihou"
        }
    }

    fn get_han(&self) -> u8 {
        match self {
            YakuType::MenzenTsumo =>    1,
            YakuType::Riichi =>         1,
            YakuType::Ippatsu =>        1,
            YakuType::Pinfu =>          1,
            YakuType::Iipeikou =>       1,
            YakuType::Haitei =>         1,
            YakuType::Houtei =>         1,
            YakuType::Rinshan =>        1,
            YakuType::Chankan =>        1,
            YakuType::Tanyao =>         1,
            YakuType::Yakuhai =>        1,
            YakuType::DoubleRiichi =>   2,
            YakuType::Chanta =>         {
                // TODO closed / open difference
                return 2;
            },
            YakuType::SanshokuDoujun => {
                // TODO closed / open difference
                return 2;
            },
            YakuType::Ittsu =>          {
                // TODO closed / open difference
                return 2;
            },
            YakuType::Toitoi =>         2,
            YakuType::Sanankou =>       2,
            YakuType::SanshokuDoukou => 2,
            YakuType::Sankantsu =>      2,
            YakuType::Chiitoitsu =>     2,
            YakuType::Honroutou =>      2,
            YakuType::Shousangen =>     2,
            YakuType::Honitsu =>        {
                // TODO closed / open difference
                return 3;
            },
            YakuType::Junchan =>        {
                // TODO closed / open difference
                return 3;
            },
            YakuType::Ryanpeikou =>     {
                // TODO closed / open difference
                return 3;
            },
            YakuType::Chinitsu =>       {
                // TODO closed / open difference
                return 6;
            },
            YakuType::Kazoe =>          13,
            YakuType::Kokushi =>        13,
            YakuType::Suuankou =>       13,
            YakuType::Daisangen =>      13,
            YakuType::Shousuushii =>    13,
            YakuType::Daisuushii =>     13,
            YakuType::Tsuuiisou =>      13,
            YakuType::Chinroutou =>     13,
            YakuType::Ryuuiisou =>      13,
            YakuType::Chuuren =>        13,
            YakuType::Suukantsu =>      13,
            YakuType::Tenhou =>         13,
            YakuType::Chiihou =>        13
        }
    }

    /// 2x the same shuntsu shape
    fn is_in_hand(&self, hand: &mut Hand, variant: &Vec<Shape>) -> bool {
        match self {
            YakuType::MenzenTsumo => {},
            YakuType::Riichi => {},
            YakuType::Ippatsu => {},
            YakuType::Pinfu => {},
            YakuType::Iipeikou => {},
            YakuType::Haitei => {},
            YakuType::Houtei => {},
            YakuType::Rinshan => {},
            YakuType::Chankan => {},
            YakuType::Tanyao => {
                let array_34 = hand.get_34_array();
                // can't contain any terminals or honors
                for (i, count) in array_34.iter().enumerate() {
                    if ([1, 9, 10, 18, 19, 27].contains(&(i + 1)) || (i + 1) >= 28) && *count > 0 {
                        return false;
                    }
                }

                return true;
            },
            YakuType::Yakuhai => {},
            YakuType::DoubleRiichi => {},
            YakuType::Chanta => {},
            YakuType::SanshokuDoujun => {},
            YakuType::Ittsu => {},
            YakuType::Toitoi => {},
            YakuType::Sanankou => {},
            YakuType::SanshokuDoukou => {},
            YakuType::Sankantsu => {},
            YakuType::Chiitoitsu => {},
            YakuType::Honroutou => {},
            YakuType::Shousangen => {},
            YakuType::Honitsu => {},
            YakuType::Junchan => {},
            YakuType::Ryanpeikou => {},
            YakuType::Chinitsu => {},
            YakuType::Kazoe => {},
            YakuType::Kokushi => {},
            YakuType::Suuankou => {},
            YakuType::Daisangen => {},
            YakuType::Shousuushii => {},
            YakuType::Daisuushii => {},
            YakuType::Tsuuiisou => {},
            YakuType::Chinroutou => {},
            YakuType::Ryuuiisou => {},
            YakuType::Chuuren => {},
            YakuType::Suukantsu => {},
            YakuType::Tenhou => {},
            YakuType::Chiihou => {},
        }

        false
    }
}

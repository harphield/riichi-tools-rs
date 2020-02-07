use crate::riichi::hand::Hand;
use crate::riichi::shapes::Shape;
use crate::riichi::shapes::ShapeType;
use crate::riichi::shapes::CompleteShape;
use crate::riichi::tile::Tile;

pub struct ShapeFinder {
    variants: Vec<Vec<Shape>>,
    current_variant: Vec<Shape>
}

impl ShapeFinder {
    pub fn new() -> ShapeFinder {
        ShapeFinder {
            ..Default::default()
        }
    }

    pub fn find(&mut self, hand: &mut Hand) {
        if hand.count_tiles() < 14 {
            return;
        }

        let mut array_34 = hand.get_34_array();

        self.search(&mut array_34, 0);
    }

    fn search(&mut self, array_34: &mut [u8; 34], depth: usize) {
        if depth > 33 {
            // check validity
            if self.is_current_variant_valid() {
                let mut repr = String::from("");
                for s in self.current_variant.iter() {
                    repr.push_str(&s.to_string()[..]);
                    repr.push_str(" ");
                }

                println!("{}", repr);

                let variant: Vec<Shape> = self.current_variant.to_vec();
                self.variants.push(variant);
            }

            return;
        }

        let current_tile = Tile::from_id((depth + 1) as u8).unwrap();

        if array_34[depth] >= 3 {
            // 3
            self.add_shape(vec![current_tile, current_tile, current_tile], array_34);
            if array_34[depth] > 0 {
                self.search(array_34, depth);
            } else {
                self.search(array_34, depth + 1);
            }
            self.remove_shape(vec![current_tile, current_tile, current_tile], array_34);
        }

        if array_34[depth] >= 2 {
            // 2
            self.add_shape(vec![current_tile, current_tile], array_34);
            if array_34[depth] > 0 {
                self.search(array_34, depth);
            } else {
                self.search(array_34, depth + 1);
            }
            self.remove_shape(vec![current_tile, current_tile], array_34);
        }

        if array_34[depth] >= 1 {
            // 1
            // can only be a single complete shape if it's kokushi
            if [1, 9, 10, 18, 19, 27].contains(&(depth + 1)) || (depth + 1) >= 28 {
                self.add_shape(vec![current_tile], array_34);
                if array_34[depth] > 0 {
                    self.search(array_34, depth);
                } else {
                    self.search(array_34, depth + 1);
                }
                self.remove_shape(vec![current_tile], array_34);
            }
        } else {
            self.search(array_34, depth + 1);
        }
    }

    fn add_shape(&mut self, tiles: Vec<Tile>, array_34: &mut [u8; 34]) {
        self.current_variant.push(Shape::from_tiles(&tiles, true).unwrap());
        for t in tiles.iter() {
            array_34[(t.to_id() - 1) as usize] -= 1;
        }
    }

    fn remove_shape(&mut self, tiles: Vec<Tile>, array_34: &mut [u8; 34]) {
        let shape = Shape::from_tiles(&tiles, true).unwrap();
        let hash = shape.to_string();

        for (i, s) in self.current_variant.iter().enumerate() {
            if hash == s.to_string() {
                self.current_variant.remove(i);
                for t in tiles.iter() {
                    array_34[(t.to_id() - 1) as usize] += 1;
                }

                return;
            }
        }

        panic!("Removing a shape that is not there!");
    }

    /// Check for:
    /// - composition
    ///     - kokushi (all singles of 1-9 and honors)
    ///     - chiitoitsu (only pairs)
    ///     - 4 groups and 1 pair
    fn is_current_variant_valid(&self) -> bool {
        let mut has_single = false;
        let mut has_koutsu = false;
        let mut has_shuntsu = false;
        let mut toitsu_count = 0;

        for shape in self.current_variant.iter() {
            match shape.get_shape_type() {
                ShapeType::Complete(ct) => {
                    match ct {
                        CompleteShape::Single(tile) => {
                            // we can only have single tiles in kokushi, so no melds and no more than 1 pair
                            if has_koutsu || has_shuntsu || toitsu_count > 1 {
                                return false;
                            }

                            has_single = true;
                        },
                        CompleteShape::Toitsu(tiles) => {
                            // we can have more than 1 pair only in chiitoitsu, so no melds and singles there
                            if toitsu_count > 1 && (has_koutsu || has_shuntsu || has_single) {
                                return false;
                            }

                            toitsu_count += 1;
                        },
                        CompleteShape::Koutsu(tiles) => {
                            // we can't have singles or more than 1 pair with melds
                            if toitsu_count > 1 || has_single {
                                return false;
                            }

                            has_koutsu = true;
                        },
                        CompleteShape::Shuntsu(tiles) => {
                            // we can't have singles or more than 1 pair with melds
                            if toitsu_count > 1 || has_single {
                                return false;
                            }

                            has_shuntsu = true;
                        }
                    }
                },
                ShapeType::Incomplete(it) => {
                    return false;
                }
            }
        }

        // we always have to have 1 pair
        if toitsu_count == 0 {
            return false;
        }

        true
    }
}

impl Default for ShapeFinder {
    fn default() -> ShapeFinder {
        ShapeFinder {
            variants: vec!(),
            current_variant: vec!(),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn find_singles() {
        let mut hand = Hand::from_text("119m19s19p1234567z", false).unwrap();
        let mut sf = ShapeFinder::new();
        sf.find(&mut hand);

        println!("{:#?}", sf.variants);

        assert_eq!(sf.variants.len(), 1);
    }

    #[test]
    fn find_pairs() {
        let mut hand = Hand::from_text("11m22s33p44556677z", false).unwrap();
        let mut sf = ShapeFinder::new();
        sf.find(&mut hand);

        println!("{:#?}", sf.variants);

        assert_eq!(sf.variants.len(), 1);
    }

}

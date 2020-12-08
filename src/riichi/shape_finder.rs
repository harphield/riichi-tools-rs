use crate::riichi::hand::Hand;
use crate::riichi::shapes::ShapeType;
use crate::riichi::shapes::{ClosedShape, Shape};
use crate::riichi::shapes::{CompleteShape, OpenShape};
use crate::riichi::tile::Tile;

pub struct ShapeFinder {
    variants: Vec<Vec<Shape>>,
    current_variant: Vec<Shape>,
}

impl ShapeFinder {
    pub fn new() -> ShapeFinder {
        ShapeFinder {
            ..Default::default()
        }
    }

    pub fn find(&mut self, hand: &mut Hand) -> Vec<Vec<Shape>> {
        if hand.count_tiles() < 14 {
            return vec![];
        }

        let mut array_34 = hand.get_34_array(true);

        let complete_shapes = hand.get_shapes();
        let mut shapes = vec![];
        for shape in complete_shapes.iter() {
            match shape {
                CompleteShape::Closed(closed) => match closed {
                    ClosedShape::Kantsu(tiles) => shapes.push(Shape::new(
                        ShapeType::Complete(CompleteShape::Closed(ClosedShape::Kantsu(*tiles))),
                        4,
                        false,
                    )),
                    _ => {}
                },
                CompleteShape::Open(open) => match open {
                    OpenShape::Chi(tiles) => shapes.push(Shape::new(
                        ShapeType::Complete(CompleteShape::Open(OpenShape::Chi(*tiles))),
                        3,
                        true,
                    )),
                    OpenShape::Pon(tiles) => shapes.push(Shape::new(
                        ShapeType::Complete(CompleteShape::Open(OpenShape::Pon(*tiles))),
                        3,
                        true,
                    )),
                    OpenShape::Kan(tiles) => shapes.push(Shape::new(
                        ShapeType::Complete(CompleteShape::Open(OpenShape::Kan(*tiles))),
                        4,
                        true,
                    )),
                },
            }
        }
        self.search(&mut array_34, 0, &mut shapes);

        // add open shapes and closed kans to all variants

        self.variants.to_owned()
    }

    fn search(&mut self, array_34: &mut [u8; 34], depth: usize, mut add_shapes: &mut Vec<Shape>) {
        if depth > 33 {
            // check validity
            if self.is_current_variant_valid() && self.current_variant_is_not_included_yet() {
                let mut repr = String::from("");
                for s in self.current_variant.iter() {
                    repr.push_str(&s.to_string()[..]);
                    repr.push_str(" ");
                }

                // println!("{}", repr);

                let mut variant: Vec<Shape> = self.current_variant.to_vec();
                variant.append(add_shapes);
                self.variants.push(variant);
            }

            return;
        }

        let current_tile = Tile::from_id((depth + 1) as u8).unwrap();

        if array_34[depth] >= 3 {
            // 3
            self.add_shape(vec![current_tile, current_tile, current_tile], array_34);
            if array_34[depth] > 0 {
                self.search(array_34, depth, &mut add_shapes);
            } else {
                self.search(array_34, depth + 1, &mut add_shapes);
            }
            self.remove_shape(vec![current_tile, current_tile, current_tile], array_34);
        }

        if array_34[depth] >= 2 {
            // 2
            self.add_shape(vec![current_tile, current_tile], array_34);
            if array_34[depth] > 0 {
                self.search(array_34, depth, &mut add_shapes);
            } else {
                self.search(array_34, depth + 1, &mut add_shapes);
            }
            self.remove_shape(vec![current_tile, current_tile], array_34);
        }

        if array_34[depth] >= 1 {
            // 1
            // can only be a single complete shape if it's kokushi
            if [1, 9, 10, 18, 19, 27].contains(&(depth + 1)) || (depth + 1) >= 28 {
                self.add_shape(vec![current_tile], array_34);
                if array_34[depth] > 0 {
                    self.search(array_34, depth, &mut add_shapes);
                } else {
                    self.search(array_34, depth + 1, &mut add_shapes);
                }
                self.remove_shape(vec![current_tile], array_34);
            }

            // shuntsu
            match current_tile.next(false) {
                Some(t) => {
                    if array_34[(t.to_id() - 1) as usize] > 0 {
                        match t.next(false) {
                            Some(t2) => {
                                if array_34[(t2.to_id() - 1) as usize] > 0 {
                                    // found it!
                                    self.add_shape(vec![current_tile, t, t2], array_34);
                                    if array_34[depth] > 0 {
                                        self.search(array_34, depth, &mut add_shapes);
                                    } else {
                                        self.search(array_34, depth + 1, &mut add_shapes);
                                    }
                                    self.remove_shape(vec![current_tile, t, t2], array_34);
                                }
                            }
                            None => (),
                        }
                    }
                }
                None => (),
            }
        } else {
            self.search(array_34, depth + 1, &mut add_shapes);
        }
    }

    fn add_shape(&mut self, tiles: Vec<Tile>, array_34: &mut [u8; 34]) {
        self.current_variant
            .push(Shape::from_tiles(&tiles, false, true).unwrap());
        for t in tiles.iter() {
            array_34[(t.to_id() - 1) as usize] -= 1;
        }
    }

    fn remove_shape(&mut self, tiles: Vec<Tile>, array_34: &mut [u8; 34]) {
        let shape = Shape::from_tiles(&tiles, false, true).unwrap();
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
                        CompleteShape::Closed(closed) => {
                            match closed {
                                ClosedShape::Single(_tile) => {
                                    // we can only have single tiles in kokushi, so no melds and no more than 1 pair
                                    if has_koutsu || has_shuntsu || toitsu_count > 1 {
                                        return false;
                                    }

                                    has_single = true;
                                }
                                ClosedShape::Toitsu(_tiles) => {
                                    // we can have more than 1 pair only in chiitoitsu, so no melds and singles there
                                    if toitsu_count > 1 && (has_koutsu || has_shuntsu || has_single)
                                    {
                                        return false;
                                    }

                                    toitsu_count += 1;
                                }
                                ClosedShape::Koutsu(_tiles) => {
                                    // we can't have singles or more than 1 pair with melds
                                    if toitsu_count > 1 || has_single {
                                        return false;
                                    }

                                    has_koutsu = true;
                                }
                                ClosedShape::Kantsu(_tiles) => {
                                    // we can't have singles or more than 1 pair with melds
                                    if toitsu_count > 1 || has_single {
                                        return false;
                                    }

                                    // kan = koutsu anyway
                                    has_koutsu = true;
                                }
                                ClosedShape::Shuntsu(_tiles) => {
                                    // we can't have singles or more than 1 pair with melds
                                    if toitsu_count > 1 || has_single {
                                        return false;
                                    }

                                    has_shuntsu = true;
                                }
                            }
                        }
                        _ => (),
                    }
                }
                ShapeType::Incomplete(..) => {
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

    fn current_variant_is_not_included_yet(&self) -> bool {
        if self.variants.is_empty() {
            return true;
        }

        let mut current_repr = String::from("");
        let mut current_strings = vec![];
        for s in self.current_variant.iter() {
            current_strings.push(s.to_string());
        }

        current_strings.sort();

        for s in current_strings.iter() {
            current_repr.push_str(&s.to_string()[..]);
            current_repr.push_str(" ");
        }

        for v in &self.variants {
            let mut v_repr = String::from("");
            let mut v_strings = vec![];
            for s in v.iter() {
                v_strings.push(s.to_string());
            }

            for s in v_strings.iter() {
                v_repr.push_str(&s.to_string()[..]);
                v_repr.push_str(" ");
            }

            if current_repr == v_repr {
                return false;
            }
        }

        true
    }
}

impl Default for ShapeFinder {
    fn default() -> ShapeFinder {
        ShapeFinder {
            variants: vec![],
            current_variant: vec![],
        }
    }
}

#[cfg(test)]
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

    #[test]
    fn find_ankous() {
        let mut hand = Hand::from_text("111m222p333s111z33z", false).unwrap();
        let mut sf = ShapeFinder::new();
        sf.find(&mut hand);

        println!("{:#?}", sf.variants);

        assert_eq!(sf.variants.len(), 1);
    }

    #[test]
    fn find_pinfu() {
        let mut hand = Hand::from_text("123m456p123789s55z", false).unwrap();
        let mut sf = ShapeFinder::new();
        sf.find(&mut hand);

        println!("{:#?}", sf.variants);

        assert_eq!(sf.variants.len(), 1);
    }

    #[test]
    fn find_chuuren() {
        let mut hand = Hand::from_text("1112345678999m1m", false).unwrap();
        let mut sf = ShapeFinder::new();
        sf.find(&mut hand);

        println!("{:#?}", sf.variants);

        assert_eq!(sf.variants.len(), 1);
    }

    #[test]
    fn find_ryanpeikou() {
        let mut hand = Hand::from_text("223344m223344p2s2s", false).unwrap();
        let mut sf = ShapeFinder::new();
        sf.find(&mut hand);

        //        println!("{:#?}", sf.variants);

        assert_eq!(sf.variants.len(), 2);
    }
}

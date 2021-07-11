use crate::riichi::fast_hand_calculator::resources::RESOURCES;

pub struct SuitClassifier {
    meld_count: u8,
    entry: u32,
}

impl SuitClassifier {
    pub fn new() -> SuitClassifier {
        SuitClassifier {
            ..Default::default()
        }
    }

    pub fn set_melds(&mut self, mut melds: u32) {
        let mut current = 0;
        self.meld_count = 0;
        for _i in 0..5 {
            let m = melds & 0b111111;
            if m != 0 {
                current = RESOURCES.get_suit_first_phase()[current + m as usize] as usize;
                melds >>= 6;
                self.meld_count += 1;
            } else {
                break;
            }
        }
        self.entry = RESOURCES.get_suit_first_phase()[current];
        // just do it based on meld count later
        // self.second_phase = SuitSecondPhases[_meldCount];
    }

    pub fn get_value(&self, tiles: &[u8; 34], suit: usize, base5hashes: &[u32; 3]) -> u32 {
        let offset = suit * 9;
        match self.meld_count {
            0 => {
                return *RESOURCES
                    .get_suit_base_5_lookup()
                    .get(base5hashes[suit] as usize)
                    .unwrap() as u32
            }
            1 => {
                let mut current = self.entry;
                let second_phase = RESOURCES.get_suit_second_phase(self.meld_count).unwrap();
                current = second_phase[(current + tiles[offset] as u32) as usize];
                current = second_phase[(current + tiles[offset + 1] as u32) as usize];
                current = second_phase[(current + tiles[offset + 2] as u32) as usize];
                current = second_phase[(current + tiles[offset + 3] as u32) as usize] + 11752;
                current = second_phase[(current + tiles[offset + 4] as u32) as usize] + 30650;
                current = second_phase[(current + tiles[offset + 5] as u32) as usize] + 55952;
                current = second_phase[(current + tiles[offset + 6] as u32) as usize] + 80078;
                current = second_phase[(current + tiles[offset + 7] as u32) as usize] + 99750;
                return second_phase[(current + tiles[offset + 8] as u32) as usize];
            }
            2 => {
                let mut current = self.entry;
                let second_phase = RESOURCES.get_suit_second_phase(self.meld_count).unwrap();
                current = second_phase[(current + tiles[offset] as u32) as usize];
                current = second_phase[(current + tiles[offset + 1] as u32) as usize];
                current = second_phase[(current + tiles[offset + 2] as u32) as usize] + 22358;
                current = second_phase[(current + tiles[offset + 3] as u32) as usize] + 54162;
                current = second_phase[(current + tiles[offset + 4] as u32) as usize] + 90481;
                current = second_phase[(current + tiles[offset + 5] as u32) as usize] + 120379;
                current = second_phase[(current + tiles[offset + 6] as u32) as usize] + 139662;
                current = second_phase[(current + tiles[offset + 7] as u32) as usize] + 150573;
                return second_phase[(current + tiles[offset + 8] as u32) as usize];
            }
            3 => {
                let mut current = self.entry;
                let second_phase = RESOURCES.get_suit_second_phase(self.meld_count).unwrap();
                current = second_phase[(current + tiles[offset] as u32) as usize];
                current = second_phase[(current + tiles[offset + 1] as u32) as usize] + 24641;
                current = second_phase[(current + tiles[offset + 2] as u32) as usize] + 50680;
                current = second_phase[(current + tiles[offset + 3] as u32) as usize] + 76245;
                current = second_phase[(current + tiles[offset + 4] as u32) as usize] + 93468;
                current = second_phase[(current + tiles[offset + 5] as u32) as usize] + 102953;
                current = second_phase[(current + tiles[offset + 6] as u32) as usize] + 107217;
                current = second_phase[(current + tiles[offset + 7] as u32) as usize] + 108982;
                return second_phase[(current + tiles[offset + 8] as u32) as usize];
            }
            4 => {
                let mut current = self.entry;
                let second_phase = RESOURCES.get_suit_second_phase(self.meld_count).unwrap();
                current = second_phase[(current + tiles[offset] as u32) as usize];
                current = second_phase[(current + tiles[offset + 1] as u32) as usize];
                current = second_phase[(current + tiles[offset + 2] as u32) as usize];
                current = second_phase[(current + tiles[offset + 3] as u32) as usize];
                current = second_phase[(current + tiles[offset + 4] as u32) as usize];
                current = second_phase[(current + tiles[offset + 5] as u32) as usize];
                current = second_phase[(current + tiles[offset + 6] as u32) as usize];
                current = second_phase[(current + tiles[offset + 7] as u32) as usize];
                return second_phase[(current + tiles[offset + 8] as u32) as usize];
            }
            _ => {}
        }

        0
    }
}

impl Default for SuitClassifier {
    fn default() -> SuitClassifier {
        SuitClassifier {
            meld_count: 0,
            entry: 0,
        }
    }
}

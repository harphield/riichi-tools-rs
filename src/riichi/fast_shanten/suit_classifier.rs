use crate::riichi::fast_shanten::resources::Resources;

pub struct SuitClassifier {
    meld_count: u8,
    entry: u32,
    resources: Option<&'static Resources>,
}

impl SuitClassifier {
    pub fn new() -> SuitClassifier {
        SuitClassifier {
            ..Default::default()
        }
    }

    pub fn set_melds(&mut self, mut melds: u8) {
        let mut current = 0;
        self.meld_count = 0;
        for i in 0..5 {
            let m = melds & 0b111111;
            if m != 0 {
                current =
                    self.resources.unwrap().get_suit_first_phase()[current + m as usize] as usize;
                melds >>= 6;
                self.meld_count += 1;
            } else {
                break;
            }
        }
        self.entry = self.resources.unwrap().get_suit_first_phase()[current];
        // just do it based on meld count later
        // self.second_phase = SuitSecondPhases[_meldCount];
    }
}

impl Default for SuitClassifier {
    fn default() -> SuitClassifier {
        SuitClassifier {
            meld_count: 0,
            entry: 0,
            resources: None,
        }
    }
}

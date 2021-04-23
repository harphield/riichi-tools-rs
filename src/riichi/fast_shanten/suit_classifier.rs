pub struct SuitClassifier {
    second_phase: Vec<u8>,
    meld_count: u8,
    entry: u8,
    suit_first_phase: Vec<u8>, // Resource.Transitions("suit_first_phase.txt");
    suit_second_phase0: Vec<u8>, // Resource.Transitions("suit_second_phase0.txt");
    suit_second_phase1: Vec<u8>, // Resource.Transitions("suit_second_phase1.txt");
    suit_second_phase2: Vec<u8>, // Resource.Transitions("suit_second_phase2.txt");
    suit_second_phase3: Vec<u8>, // Resource.Transitions("suit_second_phase3.txt");
    suit_second_phase4: Vec<u8>, // Resource.Transitions("suit_second_phase4.txt");
    suit_base5lookup: Vec<u8>, // Resource.Lookup("suitArrangementsBase5NoMelds.dat");
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
                current = self.suit_first_phase[current + m as usize] as usize;
                melds >>= 6;
                self.meld_count += 1;
            } else {
                break;
            }
        }
        self.entry = self.suit_first_phase[current];
        // just do it based on meld count later
        // self.second_phase = SuitSecondPhases[_meldCount];
    }
}

impl Default for SuitClassifier {
    fn default() -> SuitClassifier {
        SuitClassifier {
            second_phase: vec![],
            meld_count: 0,
            entry: 0,
            suit_first_phase: vec![],
            suit_second_phase0: vec![],
            suit_second_phase1: vec![],
            suit_second_phase2: vec![],
            suit_second_phase3: vec![],
            suit_second_phase4: vec![],
            suit_base5lookup: vec![],
        }
    }
}

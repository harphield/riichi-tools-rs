pub struct Score {
    han: u8,
    fu: u8,
    oya: bool,
    tsumo: bool,
}

impl Score {
    pub fn new(han: u8, fu: u8, oya: bool, tsumo: bool) -> Score {
        // TODO chiitoitsu 25 fu
        let new_fu = ((fu as f32 / 10f32).round() * 10f32) as u8;

        Score {
            han,
            fu: new_fu,
            oya,
            tsumo,
        }
    }

    fn base_points(&self) -> u32 {
        let mut points: u32 = 0;

        if self.han >= 5 ||
            (self.han == 3 && self.fu >= 70) ||
            (self.han == 4 && self.fu >= 40) {
            return 0;
        }

        self.fu as u32 * (2u32.pow(2u32 + self.han as u32))
    }

    pub fn total_points(&self) -> u32 {
        let mut points: u32 = 0;
        let base_points = self.base_points();

        if self.han == 5 ||
            (self.han == 3 && self.fu >= 70) ||
            (self.han == 4 && self.fu >= 40) {
            // mangan
            if self.oya {
                points = 12000;
            } else {
                points = 8000;
            }
        } else if self.han >= 6 && self.han <= 7 {
            // haneman
            if self.oya {
                points = 18000;
            } else {
                points = 12000;
            }
        } else if self.han >= 8 && self.han <= 10 {
            // baiman
            if self.oya {
                points = 24000;
            } else {
                points = 16000;
            }
        } else if self.han >= 11 && self.han <= 12 {
            // sanbaiman
            if self.oya {
                points = 36000;
            } else {
                points = 24000;
            }
        } else if self.han >= 13 {
            // yakuman
            if self.oya {
                points = 48000;
            } else {
                points = 32000;
            }
        } else {
            if self.oya {
                points = (((base_points * 6) as f32 / 100f32).ceil() * 100f32) as u32;
            } else if self.tsumo {
                points = ((((base_points as f32 / 100f32).ceil() * 100f32) * 2f32) + (((base_points * 2) as f32 / 100f32).ceil() * 100f32)) as u32;
            } else {
                points = (((base_points * 4) as f32 / 100f32).ceil() * 100f32) as u32;;
            }
        }

        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_1_30() {
        let score = Score::new(1, 30, false, false);
        assert_eq!(score.total_points(), 1000);
    }

    #[test]
    fn score_1_30_tsumo() {
        let score = Score::new(1, 30, false, true);
        assert_eq!(score.total_points(), 1100);
    }

    #[test]
    fn score_1_30_oya_tsumo() {
        let score = Score::new(1, 30, true, true);
        assert_eq!(score.total_points(), 1500);
    }

    #[test]
    fn score_1_40() {
        let score = Score::new(1, 40, false, false);
        assert_eq!(score.total_points(), 1300);
    }

    #[test]
    fn score_4_30() {
        let score = Score::new(4, 30, false, false);
        assert_eq!(score.total_points(), 7700);
    }
}

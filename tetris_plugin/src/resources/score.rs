use std::ops::AddAssign;

use crate::ScoreEvent;

pub struct Score(pub u64);

impl AddAssign<ScoreEvent> for Score {
    fn add_assign(&mut self, rhs: ScoreEvent) {
        self.0 += rhs.0;
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

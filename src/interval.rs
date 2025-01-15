
pub struct Interval {
    pub left: f64,
    pub right: f64,
}

impl Interval {

    pub fn new (left: f64, right: f64) -> Interval {
        Interval { left, right }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.left <= x && x <= self.right
    }

    pub fn surrounds(&self, x:f64) -> bool {
        self.left < x && x < self.right
    }

    pub fn size(&self) -> f64 {
        self.right - self.left
    }
}

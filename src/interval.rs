
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {

    pub fn default() -> Interval {
        Interval {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn new(min: f64, max: f64) -> Interval {
        Interval {
            min,
            max,
        }
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

}


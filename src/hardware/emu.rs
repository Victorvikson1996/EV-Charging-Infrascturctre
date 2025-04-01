#[allow(dead_code)]
pub struct EmulatedCharger {
    power_rating: f64,
}

#[allow(dead_code)]
impl EmulatedCharger {
    pub fn new(power_rating: f64) -> Self {
        EmulatedCharger { power_rating }
    }

    pub fn deliver_power(&self, time_hours: f64) -> f64 {
        self.power_rating * time_hours
    }
}



#[derive(Copy, Debug, Clone)]
pub struct Orbit {
    pub apoapsis: f64,
    pub periapsis: f64,
    pub offset: f64
}

impl Default for Orbit {
    fn default() -> Self {
        Orbit { apoapsis: 0.0, periapsis: 0.0, offset: 0.0 }
    }
}

impl Orbit {
    pub fn new(apoapsis: f64, periapsis: f64, offset: f64) -> Orbit {
        Orbit { apoapsis, periapsis, offset }
    }

    pub fn semi_major_axis(self: &Self) -> f64 {
        (self.apoapsis + self.periapsis) / 2.0
    }
    pub fn semi_minor_axis(self: &Self) -> f64 {
        (self.apoapsis * self.periapsis).sqrt()
    }
    pub fn semi_latus(self: &Self) -> f64 {
        ((self.apoapsis.powi(-1) + self.periapsis.powi(-1)) / 2.0).powi(-1)
    }
    pub fn eccentricity (self: &Self) -> f64 {
        (self.apoapsis - self.periapsis) / (self.apoapsis + self.periapsis)
    }
}
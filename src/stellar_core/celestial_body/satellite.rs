use super::orbit::Orbit;

pub trait Satellite {
    fn get_orbit(self: &Self) -> Orbit;
    fn set_orbit(self: &mut Self, orbit: Orbit);
}
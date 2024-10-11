/// Represents a distance in meters.
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Distance(pub f32);

impl Distance {
    pub fn as_meters(&self) -> f32 {
        self.0
    }
}

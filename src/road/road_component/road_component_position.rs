use std::ops::Sub;

use serde::{Deserialize, Serialize};

/// Holds x positions of a components left side, center, and right side.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RoadComponentPosition {
    pub left: f32,
    pub center: f32,
    pub right: f32,
}

impl RoadComponentPosition {
    pub fn new(left: f32, center: f32, right: f32) -> Self {
        Self {
            left,
            center,
            right,
        }
    }

    pub fn get_field(&self, field: RoadComponentPositionField) -> f32 {
        match field {
            RoadComponentPositionField::Left => self.left,
            RoadComponentPositionField::Center => self.center,
            RoadComponentPositionField::Right => self.right,
        }
    }
}

impl Sub for RoadComponentPosition {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.left - rhs.left,
            self.center - rhs.center,
            self.right - rhs.right,
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RoadComponentPositionField {
    Left,
    Center,
    Right,
}

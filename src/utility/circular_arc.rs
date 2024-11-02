use std::f32::consts::PI;

use bevy::prelude::*;

use super::line_intersection::calculate_line_line_intersection_3d;

/// Describes a Circular arc, stores the position of the center of the circle and the shape of the arc.
///
/// - Can go either clockwise or counter-clockwise
/// - The delta angle will be positive for counter-clockwise (along the Y axis), which is CurveDirection::Right (when looking from above).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircularArc {
    /// The position of the center of the circle.
    pub position: Vec3,
    /// The angle where the arc starts, in counter-clockwise radians.
    pub start_angle: f32,
    /// The angle where the arc ends, in counter-clockwise radians.
    pub end_angle: f32,
    pub radius: f32,
}

impl CircularArc {
    /// Returns a new CircularArc from the start & end positions of the arc and the starting direction.
    ///
    /// - Returns None if the arc is perfectly straight, since there is no circle center or radius in this case.
    pub fn from_start_direction(
        start_position: Vec3,
        end_position: Vec3,
        start_direction: Dir3,
    ) -> Option<Self> {
        let inwards_start_transform =
            Transform::from_translation(start_position).looking_to(start_direction, Dir3::Y);
        let outwards_end_transform =
            calculate_end_transform_of_curved_section(inwards_start_transform, end_position);

        Self::from_transforms(inwards_start_transform, outwards_end_transform)
    }

    /// Returns a new CircularArc from the given transforms.
    ///
    /// - Returns None if the arc is perfectly straight, since there is no circle center or radius in this case.
    pub fn from_transforms(
        inwards_start_transform: Transform,
        outwards_end_transform: Transform,
    ) -> Option<Self> {
        let start_position = inwards_start_transform.translation;
        let end_position = outwards_end_transform.translation;
        let center_position = calculate_circle_center_from_start_and_end_transforms(
            inwards_start_transform,
            outwards_end_transform,
        )?;

        let radius = start_position.distance(center_position);
        let start_angle = calculate_angle_to_point(center_position, start_position);
        let end_angle = calculate_angle_to_point(center_position, end_position);

        Some(Self {
            position: center_position,
            start_angle,
            end_angle,
            radius,
        })
    }

    pub fn delta_angle(&self) -> f32 {
        self.end_angle - self.start_angle
    }

    pub fn curve_direction(&self) -> CurveDirection {
        match self.delta_angle().is_sign_positive() {
            true => CurveDirection::Right,
            false => CurveDirection::Left,
        }
    }

    pub fn rotation_towards_start(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, self.start_angle, 0.0, 0.0)
    }

    pub fn start_position(&self) -> Vec3 {
        self.rotation_towards_start() * (Vec3::NEG_Z * self.radius) + self.position
    }

    pub fn outwards_start_transform(&self) -> Transform {
        let outwards_rotation = match self.curve_direction() {
            CurveDirection::Right => Quat::from_axis_angle(Vec3::Y, PI / 2.0),
            CurveDirection::Left => Quat::from_axis_angle(Vec3::Y, -PI / 2.0),
        };

        Transform::from_translation(self.start_position())
            .with_rotation(self.rotation_towards_start() * outwards_rotation)
    }

    pub fn rotation_towards_end(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, self.end_angle, 0.0, 0.0)
    }

    pub fn end_position(&self) -> Vec3 {
        self.rotation_towards_end() * (Vec3::NEG_Z * self.radius) + self.position
    }

    pub fn outwards_end_transform(&self) -> Transform {
        let outwards_rotation = match self.curve_direction() {
            CurveDirection::Right => Quat::from_axis_angle(Vec3::Y, -PI / 2.0),
            CurveDirection::Left => Quat::from_axis_angle(Vec3::Y, PI / 2.0),
        };

        Transform::from_translation(self.end_position())
            .with_rotation(self.rotation_towards_end() * outwards_rotation)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CurveDirection {
    Right,
    Left,
}

fn calculate_end_transform_of_curved_section(
    inwards_start_transform: Transform,
    end_position: Vec3,
) -> Transform {
    let transform_looking_at_target = inwards_start_transform.looking_at(end_position, Dir3::Y);

    let start_to_target_rotation = delta_rotation(
        inwards_start_transform.rotation,
        transform_looking_at_target.rotation,
    );

    let delta_rotation = start_to_target_rotation * start_to_target_rotation;
    let end_rotation = inwards_start_transform.rotation * delta_rotation;

    Transform::from_translation(end_position).with_rotation(end_rotation)
}

fn calculate_circle_center_from_start_and_end_transforms(
    inwards_start_transform: Transform,
    outwards_end_transform: Transform,
) -> Option<Vec3> {
    let delta_y = delta_rotation(
        inwards_start_transform.rotation,
        outwards_end_transform.rotation,
    )
    .to_euler(EulerRot::YXZ)
    .0;

    let curve_direction = match delta_y.is_sign_positive() {
        true => CurveDirection::Right,
        false => CurveDirection::Left,
    };

    let inwards_direction_from_start = match curve_direction {
        CurveDirection::Right => inwards_start_transform.right(),
        CurveDirection::Left => inwards_start_transform.left(),
    };

    let inwards_direction_from_end = match curve_direction {
        CurveDirection::Right => outwards_end_transform.right(),
        CurveDirection::Left => outwards_end_transform.left(),
    };

    let inwards_ray_from_start = Ray3d::new(
        inwards_start_transform.translation,
        inwards_direction_from_start.as_vec3(),
    );

    let inwards_ray_from_end = Ray3d::new(
        outwards_end_transform.translation,
        inwards_direction_from_end.as_vec3(),
    );

    calculate_line_line_intersection_3d(inwards_ray_from_start, inwards_ray_from_end)
}

fn calculate_angle_to_point(center: Vec3, point: Vec3) -> f32 {
    Transform::from_translation(center)
        .looking_at(point, Dir3::Y)
        .rotation
        .to_euler(EulerRot::YXZ)
        .0
}

fn delta_rotation(from: Quat, to: Quat) -> Quat {
    to * from.inverse()
}

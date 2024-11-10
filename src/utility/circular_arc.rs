use std::f32::consts::{FRAC_PI_2, PI, TAU};

use bevy::prelude::*;

use super::line_intersection::calculate_line_line_intersection_3d;

// TODO: get the correct start & end angle values when creating struct
//      Current state:
//      - I've updated the struct & struct field docs, I'm pretty confident that sticking to the orientation described there can give consistent results
//      - An angle of 0.0 should/will ALWAYS point in the -Z direction because directions are always visualised through Quaternions. It might be the case that Euler 0.0 starts along X axis (not sure), but that data would only be visualised through a Quaternion which changes it back to -Z axis.
//      - I tried getting the start & end angles directly form the Transforms instead of from the circle center position, but this doesn't account for CurveDirection! So I think I do need to calculate it from the circle center
//      - Still not quite sure how to make sure the "forward" direction from the start Transform is used to prevent CurveDirection to flip at 180deg (it should stay in correct direction even above 180deg)

/// Describes a Circular arc, stores the position of the center of the circle and the shape of the arc.
///
/// - Can go either clockwise or counter-clockwise
/// - The delta angle will be positive for clockwise (looking along the Y axis), which is CurveDirection::Left (looking from above).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircularArc {
    /// The position of the center of the circle.
    pub position: Vec3,
    /// The angle where the arc starts.
    ///
    /// - 0.0 is along the negative Z axis ("forward" for Transforms).
    /// - This is always a value from 0.0 to TAU.
    pub start_angle: f32,
    /// The angle between the start and end of the arc.
    ///
    /// - When looking along the Y axis, positive values go clockwise, negative values go counter-clockwise.
    pub delta_angle: f32,
    pub radius: f32,
}

impl CircularArc {
    /// Returns a new CircularArc from the start & end positions of the arc and the starting direction.
    ///
    /// - Returns None if the arc is perfectly straight, since there is no circle center or radius in this case.
    pub fn from_start_transform(
        inwards_start_transform: Transform,
        end_position: Vec3,
    ) -> Option<Self> {
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
        let start_angle = calculate_positive_angle_to_point(center_position, start_position);
        let end_angle = calculate_positive_angle_to_point(center_position, end_position);

        // TODO: verify if delta angle is correct
        let delta_angle = end_angle - start_angle;

        // TODO:

        Some(Self {
            position: center_position,
            start_angle,
            delta_angle,
            radius,
        })
    }

    /// Returns the angle where the arc ends, in counter-clockwise radians.
    pub fn end_angle(&self) -> f32 {
        let end_angle = self.start_angle + self.delta_angle;

        wrap_angle_to_tau(end_angle)
    }

    pub fn curve_direction(&self) -> CurveDirection {
        curve_direction_from_angle(self.delta_angle)
    }

    // TODO: verify that start / end angles are actually from center to start / end, instead of along the direction of start / end
    /// Returns a Quaternion where "forward" points from the circle center to the start of this arc.
    pub fn rotation_towards_start(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, -self.start_angle, 0.0, 0.0)
    }

    pub fn start_position(&self) -> Vec3 {
        self.rotation_towards_start() * (Vec3::NEG_Z * self.radius) + self.position
    }

    pub fn outwards_start_transform(&self) -> Transform {
        let outwards_rotation = match self.curve_direction() {
            CurveDirection::Right => Quat::from_axis_angle(Vec3::Y, -PI / 2.0),
            CurveDirection::Left => Quat::from_axis_angle(Vec3::Y, PI / 2.0),
        };

        Transform::from_translation(self.start_position())
            .with_rotation(self.rotation_towards_start() * outwards_rotation)
    }

    /// Returns a Quaternion where "forward" points from the circle center to the end of this arc.
    pub fn rotation_towards_end(&self) -> Quat {
        Quat::from_euler(EulerRot::YXZ, -self.end_angle(), 0.0, 0.0)
    }

    pub fn end_position(&self) -> Vec3 {
        self.rotation_towards_end() * (Vec3::NEG_Z * self.radius) + self.position
    }

    pub fn outwards_end_transform(&self) -> Transform {
        let outwards_rotation = match self.curve_direction() {
            CurveDirection::Right => Quat::from_axis_angle(Vec3::Y, PI / 2.0),
            CurveDirection::Left => Quat::from_axis_angle(Vec3::Y, -PI / 2.0),
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
    // TODO: when the delta rotation passes 180deg, the current logic assumes the direction flips when it actually doesn't.
    //      What should happen is that the curve direction follows the start transform's forward direction
    let delta_y = delta_rotation(
        inwards_start_transform.rotation,
        outwards_end_transform.rotation,
    )
        .to_euler(EulerRot::YXZ)
        .0;

    let curve_direction = curve_direction_from_angle(delta_y);

    // println!("Curve direction: {:?}", curve_direction);

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

fn calculate_positive_angle_to_point(center: Vec3, point: Vec3) -> f32 {
    let angle = Transform::from_translation(center)
        .looking_at(point, Dir3::Y)
        .rotation
        .to_euler(EulerRot::YXZ)
        .0 - FRAC_PI_2;

    wrap_angle_to_tau(angle)
}

fn wrap_angle_to_tau(mut angle: f32) -> f32 {
    if angle.is_sign_negative() {
        angle += TAU;
    } else if angle >= TAU {
        angle -= TAU;
    }

    angle
}

fn delta_rotation(from: Quat, to: Quat) -> Quat {
    to * from.inverse()
}

/// Returns the curve direction (when looking from above) for the given angle (in clockwise radians along Y axis).
fn curve_direction_from_angle(angle: f32) -> CurveDirection {
    match angle.is_sign_positive() {
        true => CurveDirection::Left,
        false => CurveDirection::Right,
    }
}

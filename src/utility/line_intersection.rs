use bevy::prelude::*;

/// Maps the `calculate_line_line_intersection` function to 3D by ignoring the vertical axis.
pub fn calculate_line_line_intersection_3d(line1: Ray3d, line2: Ray3d) -> Option<Vec3> {
    let line1_2d = Ray2d::new(line1.origin.xz(), line1.direction.xz());
    let line2_2d = Ray2d::new(line2.origin.xz(), line2.direction.xz());

    calculate_line_line_intersection(line1_2d, line2_2d)
        .map(|vector| Vec3::new(vector.x, 0.0, vector.y))
}

/// Chat-gpt generated implementation to calculate intersection point from 2 lines
pub fn calculate_line_line_intersection(line1: Ray2d, line2: Ray2d) -> Option<Vec2> {
    // Calculate the denominator of the intersection formula
    let denominator = line1.direction.perp_dot(line2.direction.as_vec2());

    if denominator.abs() < f32::EPSILON {
        // Lines are parallel or coincident if the denominator is 0
        return None;
    }

    // Calculate the difference vector between the points
    let delta_origin = line2.origin - line1.origin;

    // Calculate the intersection scalar `t` for the first line
    let t = delta_origin.perp_dot(line2.direction.as_vec2()) / denominator;

    // Use `t` to find the intersection point along the first line
    Some(line1.origin + line1.direction * t)
}
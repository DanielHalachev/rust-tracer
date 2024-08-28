use crate::tracer::vector::{Coordinates, Point, Vector};

use super::triangle::Triangle;

pub struct Intersection {
    distance: f32,
    hit_point: Point,
    hit_normal: Vector,
    uv: Coordinates,
}

#[derive(PartialEq, Eq)]
enum RayType {
    CameraRay,
    ShadowRay,
    RelfectionRay,
    RefractionRay,
}

pub struct Ray {
    origin: Vector,
    direction: Vector,
    kind: RayType,
}

impl Ray {
    pub fn intersect_triangle(&self, triangle: &Triangle) -> Option<Intersection> {
        let triangle_normal: Vector = triangle.triangle_normal().clone();
        let normal_dot_direction = self.direction.dot_product(&triangle_normal);

        if self.kind == RayType::CameraRay && normal_dot_direction >= 0.0 {
            return None;
        }

        let distance_to_plane: f32 = -(triangle[0].position.dot_product(&triangle_normal));
        let t: f32 =
            -(triangle_normal.dot_product(&self.origin) + distance_to_plane) / normal_dot_direction;
        if t < 0.0 {
            return None;
        }

        let intersection_point: Point = self.origin.clone() + self.direction.clone() * t;
        if triangle.point_in_triangle(&intersection_point) == false {
            return None;
        }

        let hit_normal = triangle_normal;

        Option::Some(Intersection {
            distance: t,
            hit_point: intersection_point,
            hit_normal: hit_normal,
            uv: Coordinates(0.0, 0.0, 0.0),
        })
    }
}

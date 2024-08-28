use crate::tracer::vector::{Coordinates, Vector};

pub struct Vertex {
    pub position: Coordinates,
    pub normal: Vector,
    pub uv: Coordinates,
}

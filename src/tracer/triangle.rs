use crate::tracer::vector::{Coordinates, Point, Vector};
use crate::tracer::vertex::Vertex;
use std::ops::{Index, IndexMut};
pub struct Triangle<'a> {
    vertices: [&'a mut Vertex; 3],
    normal: Vector,
}

impl<'a> Index<usize> for Triangle<'a> {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        self.vertices[index]
    }
}

impl<'a> IndexMut<usize> for Triangle<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.vertices[index]
    }
}

impl<'a> Triangle<'a> {
    pub fn triangle_normal(&self) -> &Vector {
        &self.normal
    }

    fn calculate_triangle_normal(&self) -> Vector {
        let v1: Vector = self.vertices[1].position.clone() - self.vertices[0].position.clone();
        let v2: Vector = self.vertices[2].position.clone() - self.vertices[0].position.clone();
        v1.cross_product(&v2)
    }

    pub fn point_in_triangle(&self, point: &Point) -> bool {
        let e0: Vector = self.vertices[1].position.clone() - self.vertices[0].position.clone();
        let c0: Vector = point.clone() - self.vertices[0].position.clone();
        if self.normal.dot_product(&e0.cross_product(&c0)) < f32::EPSILON {
            return false;
        }

        let e1: Vector = self.vertices[2].position.clone() - self.vertices[1].position.clone();
        let c1: Vector = point.clone() - self.vertices[1].position.clone();
        if self.normal.dot_product(&e1.cross_product(&c1)) < f32::EPSILON {
            return false;
        }

        let e2: Vector = self.vertices[0].position.clone() - self.vertices[2].position.clone();
        let c2: Vector = point.clone() - self.vertices[2].position.clone();
        if self.normal.dot_product(&e2.cross_product(&c2)) < f32::EPSILON {
            return false;
        }

        true
    }

    pub fn area(&self) -> f32 {
        self.normal.get_length() / 2.0
    }

    pub fn get_barycentric_coordinates(&self, point: &Point) -> Coordinates {
        let v0p: Vector = point.clone() - self.vertices[0].position.clone();
        let v0v1: Vector = self.vertices[1].position.clone() - self.vertices[0].position.clone();
        let v0v2: Vector = self.vertices[2].position.clone() - self.vertices[0].position.clone();

        let area = v0v1.cross_product(&v0v2).get_length();
        let u = (v0p.cross_product(&v0v2).get_length()) / area;
        let v = (v0v1.cross_product(&v0p).get_length()) / area;

        Coordinates(u, v, 1.0 - u - v)
    }
}

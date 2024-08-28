use std::str::FromStr;

use image::{GenericImageView, Pixel};
use stb_image::stb_image::stbi_load;

use crate::tracer::{triangle::Triangle, vector::Coordinates};

use super::color::{Albedo, Color, PPMColor};

trait Texture {
    fn get_color(&self, triangle: &Triangle, barycentric_coordinates: &Coordinates) -> Color;
}

pub struct AlbedoTexture {
    name: String,
    albedo: Albedo,
}

impl Texture for AlbedoTexture {
    fn get_color(&self, _: &Triangle, _: &Coordinates) -> Color {
        Color::from(self.albedo.clone())
    }
}

pub struct EdgeTexture {
    name: String,
    inner_color: Color,
    edge_color: Color,
    width: f32,
}

impl Texture for EdgeTexture {
    fn get_color(&self, _: &Triangle, barycentric_coordinates: &Coordinates) -> Color {
        if barycentric_coordinates.0 < self.width
            || barycentric_coordinates.1 < self.width
            || barycentric_coordinates.2 < self.width
        {
            self.edge_color.clone();
        }
        self.inner_color.clone()
    }
}

pub struct CheckerTexture {
    first_color: Color,
    second_color: Color,
    square_size: f32,
}

impl Texture for CheckerTexture {
    fn get_color(&self, triangle: &Triangle, barycentric_coordinates: &Coordinates) -> Color {
        let interpolated_coordinates: Coordinates = barycentric_coordinates.0
            * triangle[1].uv.clone()
            + barycentric_coordinates.1 * triangle[2].uv.clone()
            + barycentric_coordinates.2 * triangle[0].uv.clone();
        let x = (interpolated_coordinates.0 / self.square_size) as u32;
        let y = (interpolated_coordinates.1 / self.square_size) as u32;
        if x % 2 == y % 2 {
            return self.first_color.clone();
        }
        return self.second_color.clone();
    }
}

pub struct BitmapTexture {
    name: String,
    width: u32,
    height: u32,
    buffer: Vec<Color>,
}

impl BitmapTexture {
    fn new(name: &str, path: &str) -> Self {
        let image = image::open(path).expect("Failed to open image");

        let (width, height) = image.dimensions();

        let mut buffer: Vec<Color> = Vec::with_capacity((width * height) as usize);

        for pixel in image.pixels() {
            let rgb_values = pixel.2.channels();
            buffer.push(Color(
                rgb_values[0] as f32 / 255.0,
                rgb_values[1] as f32 / 255.0,
                rgb_values[2] as f32 / 255.0,
            ));
        }

        BitmapTexture {
            name: name.to_string(),
            width,
            height,
            buffer,
        }
    }
}

impl Texture for BitmapTexture {
    fn get_color(&self, triangle: &Triangle, barycentric_coordinates: &Coordinates) -> Color {
        let interpolated_coordinates: Coordinates = barycentric_coordinates.0
            * triangle[1].uv.clone()
            + barycentric_coordinates.1 * triangle[2].uv.clone()
            + barycentric_coordinates.2 * triangle[0].uv.clone();
        let x = usize::clamp(
            (interpolated_coordinates.0 * self.width as f32) as usize,
            0,
            self.width as usize - 1,
        );
        let y = usize::clamp(
            ((1.0 - interpolated_coordinates.1) * self.height as f32) as usize,
            0,
            self.height as usize - 1,
        );

        self.buffer[y * self.width as usize + x].clone()
    }
}

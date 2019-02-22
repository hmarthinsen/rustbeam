use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Copy, Clone)]
/// A pixel containing RGBA data in floating point format. Values range from 0
/// to 1, where 0 means black, and 1 means max color. For the alpha channel, 0
/// means fully transparent, and 1 means fully opaque.
pub struct Pixel {
    /// Red channel.
    pub r: f64,
    /// Green channel.
    pub g: f64,
    /// Blue channel.
    pub b: f64,
    /// Alpha channel.
    pub a: f64,
}

/// An image containing `Pixel`s.
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

#[derive(Copy, Clone, Default)]
/// A 3D vector
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pixel {
    /// Make a black, opaque pixel.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

impl Image {
    /// Make a new `Image` of width `width` and height `height`.
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        let black_pixel = Pixel::new();

        for _ in 0..width * height {
            pixels.push(black_pixel);
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    /// Set pixel at coordinate (`x`, `y`).
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.pixels[self.width * y + x] = pixel;
    }

    /// Convert the image to a vector of gamma corrected SRGB data.
    pub fn to_srgba_vector(&self) -> Vec<u8> {
        let mut srgba_data = Vec::with_capacity(self.width * self.height * 4);

        for &pixel in self.pixels.iter() {
            srgba_data.push(Image::linear_to_srgb(pixel.r));
            srgba_data.push(Image::linear_to_srgb(pixel.g));
            srgba_data.push(Image::linear_to_srgb(pixel.b));
            srgba_data.push((pixel.a * 255.0).round() as u8);
        }

        srgba_data
    }

    /// Save the image as a png file.
    pub fn save_png(&self, filename: &str) {
        let srgba_vector = self.to_srgba_vector();
        let pixel_data = srgba_vector.as_slice();

        let png_file = File::create(filename).unwrap();
        let mut png_encoder = png::Encoder::new(
            BufWriter::new(png_file),
            self.width as u32,
            self.height as u32,
        );
        png_encoder
            .set(png::ColorType::RGBA)
            .set(png::BitDepth::Eight);
        let mut png_writer = png_encoder.write_header().unwrap();
        png_writer.write_image_data(pixel_data).unwrap();
    }

    /// Read a png file into a vector of SRGB data.
    pub fn read_png(filename: &str) -> Vec<u8> {
        let png_file = File::open(filename).unwrap();
        let decoder = png::Decoder::new(png_file);
        let (info, mut reader) = decoder.read_info().unwrap();
        // Allocate the output buffer.
        let mut buf = vec![0; info.buffer_size()];
        // Read the next frame. Currently this function should only called once.
        // The default options
        reader.next_frame(&mut buf).unwrap();

        buf
    }

    /// Convert color from linear color space to SRGB. `color` should be
    /// between 0 and 1.
    fn linear_to_srgb(color: f64) -> u8 {
        let srgb = if color < 0.003_130_8 {
            12.92 * color
        } else {
            1.055 * color.powf(1.0 / 2.4) - 0.055
        };

        (srgb * 255.0).round() as u8
    }

    pub fn render_sphere(&mut self) {
        // All lengths are in meters.

        // let aspect_ratio = window_width as f64 / window_height as f64;

        let screen_width = 0.64;

        // We assume square pixels.
        // let screen_height = screen_width / aspect_ratio;

        // Distance from the eye, assumed at the origin, to the middle of the
        // screen. The screen is oriented along the z-axis.
        let distance_to_screen = 0.5;

        let pixel_size = screen_width / self.width as f64;

        let sphere_center = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        };
        let sphere_radius: f64 = 0.5;

        for pixel_x in 0..self.width {
            for pixel_y in 0..self.height {
                let pixel_pos = Vector3 {
                    x: (pixel_x as f64 - 0.5 * (self.width - 1) as f64) * pixel_size,
                    y: (pixel_y as f64 - 0.5 * (self.height - 1) as f64) * pixel_size,
                    z: distance_to_screen,
                };

                let t = sphere_center.dot(&pixel_pos) / pixel_pos.norm2();

                let surface_fun = (pixel_pos * t - sphere_center).norm2() - sphere_radius.powi(2);

                let mut pixel = Pixel::new();
                if surface_fun.is_sign_negative() {
                    pixel.r = 1.0;
                }
                self.set_pixel(pixel_x, pixel_y, pixel);
            }
        }
    }
}

impl Vector3 {
    pub fn new() -> Self {
        Default::default()
    }

    // Dot product of two vectors.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Square of norm of the vector.
    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    // Norm of the vector.
    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_image() {
        let image = Image::new(640, 480);
        let filename = "test-data/test-data-out/test_make_image.png";
        let ref_filename = "test-data/test_make_image_ref.png";

        image.save_png(filename);

        let image_data = image.to_srgba_vector();
        let ref_image_data = Image::read_png(ref_filename);
        assert_eq!(image_data, ref_image_data);
    }

    #[test]
    fn render_sphere() {
        let mut image = Image::new(1280, 720);
        let filename = "test-data/test-data-out/test_render_sphere.png";
        let ref_filename = "test-data/test_render_sphere_ref.png";

        image.render_sphere();

        image.save_png(filename);

        let image_data = image.to_srgba_vector();
        let ref_image_data = Image::read_png(ref_filename);
        assert_eq!(image_data, ref_image_data);
    }
}

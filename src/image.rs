use png::HasParameters;
use std::fs::File;
use std::io::BufWriter;

/// A pixel containing RGBA data in floating point format. Values range from 0
/// to 1, where 0 means black, and 1 means max color. For the alpha channel, 0
/// means fully transparent, and 1 means fully opaque.
#[derive(Clone, Copy)]
pub struct Pixel {
    /// Red channel.
    r: f64,
    /// Green channel.
    g: f64,
    /// Blue channel.
    b: f64,
    /// Alpha channel.
    a: f64,
}

impl Pixel {
    fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }
}

impl Default for Pixel {
    /// Make a black, opaque pixel.
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl From<(f64, f64, f64)> for Pixel {
    fn from(rgb: (f64, f64, f64)) -> Self {
        Self::new(rgb.0, rgb.1, rgb.2, 1.0)
    }
}

/// An image containing `Pixel`s.
pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    /// Make a new `Image` of width `width` and height `height`.
    pub fn new(width: usize, height: usize) -> Self {
        let num_pixels = width * height;
        let mut pixels = Vec::with_capacity(num_pixels);
        let black_pixel = Pixel::default();

        for _ in 0..num_pixels {
            pixels.push(black_pixel);
        }

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Set pixel at coordinate (`x`, `y`).
    pub fn set_pixel<T: Into<Pixel>>(&mut self, x: usize, y: usize, pixel: T) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.pixels[self.width * y + x] = pixel.into();
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

    /// Find the minimum and maximum color values in the image, looking through
    /// R, G, and B channels.
    pub fn min_max(&self) -> (f64, f64) {
        let mut min = std::f64::INFINITY;
        let mut max = std::f64::NEG_INFINITY;

        for pixel in self.pixels.iter() {
            if pixel.r < min {
                min = pixel.r
            } else if pixel.r > max {
                max = pixel.r
            }
            if pixel.g < min {
                min = pixel.g
            } else if pixel.g > max {
                max = pixel.g
            }
            if pixel.b < min {
                min = pixel.b
            } else if pixel.b > max {
                max = pixel.b
            }
        }

        (min, max)
    }

    /// Map minimum color to 0 and maximum color to 1.
    pub fn normalize(&mut self) {
        let (min, max) = self.min_max();
        let recip_range = 1.0 / (max - min);

        for pixel in self.pixels.iter_mut() {
            pixel.r = (pixel.r - min) * recip_range;
            pixel.g = (pixel.g - min) * recip_range;
            pixel.b = (pixel.b - min) * recip_range;
        }
    }

    /// Clamp color to between 0 and 1.
    pub fn clamp(&mut self) {
        for pixel in self.pixels.iter_mut() {
            pixel.r = pixel.r.min(1.0).max(0.0);
            pixel.g = pixel.g.min(1.0).max(0.0);
            pixel.b = pixel.b.min(1.0).max(0.0);
        }
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
}

pub mod image;
pub mod lights;
pub mod math;
pub mod scene;
pub mod surfaces;

#[cfg(test)]
mod tests {
    use crate::image::Image;
    use crate::lights::Sun;
    use crate::scene::Scene;
    use crate::surfaces::{Plane, Sphere};
    use std::fs::File;

    /// Read a png file into a vector of SRGB data.
    fn read_png(filename: &str) -> Vec<u8> {
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

    #[test]
    fn make_image() {
        let image = Image::new(640, 480);
        let filename = "test-data/test-data-out/test_make_image.png";
        let ref_filename = "test-data/test_make_image_ref.png";

        image.save_png(filename);

        let image_data = image.to_srgba_vector();
        let ref_image_data = read_png(ref_filename);
        assert_eq!(image_data, ref_image_data);
    }

    #[test]
    fn render_sphere() {
        let mut image = Image::new(1280, 720);
        let filename = "test-data/test-data-out/test_render_sphere.png";
        let ref_filename = "test-data/test_render_sphere_ref.png";

        let mut scene = Scene::new();

        scene.add_surface(Box::new(Sphere::new((0.0, 2.0, 0.0), 0.5)));

        scene.add_light(Sun::new((1.0, 0.0, 0.0), (1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 1.0, 0.0), (-1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 0.0, 1.0), (0.0, 1.0, 1.0)));

        scene.render(&mut image);

        image.save_png(filename);

        let image_data = image.to_srgba_vector();
        let ref_image_data = read_png(ref_filename);
        assert_eq!(image_data, ref_image_data);
    }

    #[test]
    fn render_plane() {
        let mut image = Image::new(1280, 720);
        let filename = "test-data/test-data-out/test_render_plane.png";
        let ref_filename = "test-data/test_render_plane_ref.png";

        let mut scene = Scene::new();

        scene.add_surface(Box::new(Plane::new((0.0, 0.0, 1.0), -0.5)));

        scene.add_light(Sun::new((1.0, 0.0, 0.0), (1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 1.0, 0.0), (-1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 0.0, 1.0), (0.0, 1.0, 1.0)));

        scene.render(&mut image);

        image.save_png(filename);

        let image_data = image.to_srgba_vector();
        let ref_image_data = read_png(ref_filename);
        assert_eq!(image_data, ref_image_data);
    }
}

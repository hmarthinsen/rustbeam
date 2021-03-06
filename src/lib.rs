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
    use std::error::Error;
    use std::fs::File;

    /// Read a png file into a vector of SRGB data.
    fn read_png(filename: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let png_file = File::open(filename)?;
        let decoder = png::Decoder::new(png_file);
        let (info, mut reader) = decoder.read_info()?;
        // Allocate the output buffer.
        let mut buf = vec![0; info.buffer_size()];
        // Read the next frame. Currently this function should only called once.
        // The default options
        reader.next_frame(&mut buf)?;

        Ok(buf)
    }

    #[test]
    fn make_image() {
        let image = Image::new(640, 480);
        let filename = "test-data/test-data-out/test_make_image.png";
        let ref_filename = "test-data/test_make_image_ref.png";

        image.save_png(filename).unwrap();

        let image_data = image.get_srgba_vector();
        let ref_image_data = read_png(ref_filename).unwrap();
        assert_eq!(*image_data, ref_image_data);
    }

    #[test]
    fn render_sphere() {
        let image_width = 1280;
        let image_height = 720;
        let mut image = Image::new(image_width, image_height);
        let ref_filename = "test-data/test_render_sphere_ref.png";

        let mut scene = Scene::new();

        scene.add_surface(Sphere::new((0.0, 2.0, 0.0), 0.5));

        scene.add_light(Sun::new((1.0, 0.0, 0.0), (1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 1.0, 0.0), (-1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 0.0, 1.0), (0.0, 1.0, 1.0)));

        let receiver = scene.spawn_render_threads(image_width, image_height);
        image.update(receiver.iter());

        let image_data = image.get_srgba_vector();
        let ref_image_data = read_png(ref_filename).unwrap();
        assert_eq!(*image_data, ref_image_data);
    }

    #[test]
    fn render_plane() {
        let image_width = 1280;
        let image_height = 720;
        let mut image = Image::new(image_width, image_height);
        let ref_filename = "test-data/test_render_plane_ref.png";

        let mut scene = Scene::new();

        scene.add_surface(Plane::new((0.0, 0.0, 1.0), -0.5));

        scene.add_light(Sun::new((1.0, 0.0, 0.0), (1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 1.0, 0.0), (-1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 0.0, 1.0), (0.0, 1.0, 1.0)));

        let receiver = scene.spawn_render_threads(image_width, image_height);
        image.update(receiver.iter());

        let image_data = image.get_srgba_vector();
        let ref_image_data = read_png(ref_filename).unwrap();
        assert_eq!(*image_data, ref_image_data);
    }

    #[test]
    fn render_sphere_and_plane() {
        let image_width = 1280;
        let image_height = 720;
        let mut image = Image::new(image_width, image_height);
        let ref_filename = "test-data/test_render_sphere_and_plane_ref.png";

        let mut scene = Scene::new();

        scene.add_surface(Sphere::new((0.0, 2.0, 0.0), 0.5));
        scene.add_surface(Plane::new((0.0, 0.0, 1.0), -0.5));

        scene.add_light(Sun::new((1.0, 0.0, 0.0), (1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 1.0, 0.0), (-1.0, 1.0, -1.0)));
        scene.add_light(Sun::new((0.0, 0.0, 1.0), (0.0, 1.0, 1.0)));

        let receiver = scene.spawn_render_threads(image_width, image_height);
        image.update(receiver.iter());

        let image_data = image.get_srgba_vector();
        let ref_image_data = read_png(ref_filename).unwrap();
        assert_eq!(*image_data, ref_image_data);
    }
}

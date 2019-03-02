use crate::image::Image;
use crate::math::{Ray, UnitQuaternion, Vector3};
use crate::surfaces::Sphere;
use std::f64::INFINITY;

struct Camera {
    position: Vector3,
    orientation: UnitQuaternion,
    screen_width: f64,
    distance_to_screen: f64,
}

impl Default for Camera {
    /// The default camera is located at the origin, looking along the y-axis,
    /// with up along the z-axis.
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            orientation: UnitQuaternion::id(),
            screen_width: 0.64,
            distance_to_screen: 0.5,
        }
    }
}

impl Camera {
    fn up(&self) -> Vector3 {
        let ref_up = Vector3::k();
        ref_up.rotate(self.orientation)
    }

    fn direction(&self) -> Vector3 {
        let ref_dir = Vector3::j();
        ref_dir.rotate(self.orientation)
    }

    fn right(&self) -> Vector3 {
        self.direction().cross(self.up())
    }
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Sphere>,
    camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, sphere: Sphere) {
        self.objects.push(sphere);
    }

    pub fn render(self, image: &mut Image) {
        let (width, height) = image.get_size();
        let pixel_size = self.camera.screen_width / width as f64;

        let center_of_screen = self.camera.direction() * self.camera.distance_to_screen;

        for pixel_x in 0..width {
            let delta_x =
                (pixel_x as f64 - 0.5 * (width - 1) as f64) * pixel_size * self.camera.right();

            for pixel_y in 0..height {
                let delta_y =
                    -(pixel_y as f64 - 0.5 * (height - 1) as f64) * pixel_size * self.camera.up();

                let direction = center_of_screen + delta_x + delta_y;

                let ray = Ray::new(self.camera.position, direction);

                let mut global_closest_intersection = INFINITY;
                let mut closest_sphere: Option<&Sphere> = None;

                for sphere in self.objects.iter() {
                    let closest_intersection = sphere.closest_intersection(&ray);

                    match closest_intersection {
                        None => continue,
                        Some(distance) => {
                            // Ray intersects the sphere.
                            if distance < global_closest_intersection {
                                global_closest_intersection = distance;
                                closest_sphere = Some(sphere);
                            }
                        }
                    }
                }

                let mut rgb = (0.0, 0.0, 0.0);
                match closest_sphere {
                    None => (),
                    Some(sphere) => {
                        let dir_to_light1 = Vector3::from((-1.0, -1.0, 1.0)).normalize();
                        let dir_to_light2 = Vector3::from((1.0, -1.0, 1.0)).normalize();
                        let dir_to_light3 = Vector3::from((0.0, -1.0, -1.0)).normalize();

                        let surface_normal = (ray.origin
                            + global_closest_intersection * ray.direction
                            - sphere.center_pos)
                            .normalize();

                        rgb.0 = surface_normal.dot(dir_to_light1).max(0.0);
                        rgb.1 = surface_normal.dot(dir_to_light2).max(0.0);
                        rgb.2 = surface_normal.dot(dir_to_light3).max(0.0);
                    }
                }
                image.set_pixel(pixel_x, pixel_y, rgb);
            }
        }
    }
}

use crate::image::Pixel;
use crate::lights::Sun;
use crate::math::{Ray, UnitQuaternion, Vector3};
use crate::surfaces::Surface;
use std::{
    f64::{EPSILON, INFINITY},
    sync::mpsc,
};

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
pub struct Scene<'a> {
    surfaces: Vec<Box<Surface + Send + 'a>>,
    camera: Camera,
    lights: Vec<Sun>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_surface(&mut self, surface: impl Surface + Send + 'a) {
        self.surfaces.push(Box::new(surface));
    }

    pub fn add_light(&mut self, light: Sun) {
        self.lights.push(light);
    }

    pub fn render(self, width: usize, height: usize, sender: mpsc::Sender<(usize, usize, Pixel)>) {
        let pixel_size = self.camera.screen_width / width as f64;

        let center_of_screen = self.camera.direction() * self.camera.distance_to_screen;

        for pixel_y in 0..height {
            let delta_y =
                -(pixel_y as f64 - 0.5 * (height - 1) as f64) * pixel_size * self.camera.up();

            for pixel_x in 0..width {
                let delta_x =
                    (pixel_x as f64 - 0.5 * (width - 1) as f64) * pixel_size * self.camera.right();

                let direction = center_of_screen + delta_x + delta_y;

                let ray = Ray::new(self.camera.position, direction);

                let mut rgb = Vector3::zero();
                match self.trace(ray) {
                    None => (),
                    Some((intersection, normal)) => {
                        for light in self.lights.iter() {
                            let dir_to_light = -light.direction;
                            let shadow_ray = Ray::new(intersection, dir_to_light);
                            match self.trace(shadow_ray) {
                                Some(_) => (),
                                None => {
                                    // The light illuminates the intersection point.
                                    rgb += normal.dot(dir_to_light).max(0.0) * light.color;
                                }
                            }
                        }
                    }
                }
                sender.send((pixel_x, pixel_y, rgb.into())).unwrap();
            }
        }
    }

    /// Trace a ray until it intersects a surface in the scene. If nothing is
    /// hit, then `None` is returned. Else, a tuple is returned, where the first
    /// element is the intersection, and the second is the normal vector.
    fn trace(&self, ray: Ray) -> Option<(Vector3, Vector3)> {
        let mut closest_intersection = INFINITY;
        //let mut closest_surface: Option<&Box<Surface>> = None;
        let mut result = None;

        for surface in self.surfaces.iter() {
            let closest_intersection_of_surface = surface.closest_intersection(&ray);

            match closest_intersection_of_surface {
                None => continue,
                Some((distance, normal)) => {
                    if distance <= EPSILON.sqrt() {
                        // Don't intersect the same point that the ray is leaving from.
                        continue;
                    }
                    // Ray intersects the surface.
                    if distance < closest_intersection {
                        closest_intersection = distance;
                        result = Some((ray.origin + closest_intersection * ray.direction, normal));
                    }
                }
            }
        }
        result
    }
}

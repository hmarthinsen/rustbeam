//! Module containing the scene.
//!
//! This module performs the actual rendering.

use crate::image::Pixel;
use crate::lights::Sun;
use crate::math::{Ray, UnitQuaternion, Vector3};
use crate::surfaces::Surface;
use std::error::Error;
use std::{
    f64::{EPSILON, INFINITY},
    sync::{
        mpsc,
        mpsc::{Receiver, Sender},
        Arc,
    },
    thread,
};

/// The camera determines from which direction the scene is rendered. The
/// default camera is located at the origin, looking along the y-axis, with up
/// along the z-axis.
struct Camera {
    position: Vector3,
    orientation: UnitQuaternion,
    screen_width: f64,
    distance_to_screen: f64,
}

impl Default for Camera {
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
    /// Find the unit vector that points up when viewed through the camera.
    fn up(&self) -> Vector3 {
        let ref_up = Vector3::k();
        ref_up.rotate(self.orientation)
    }

    /// Find the unit vector that points through the middle of the camera.
    fn direction(&self) -> Vector3 {
        let ref_dir = Vector3::j();
        ref_dir.rotate(self.orientation)
    }

    /// Find the unit vector that points right when viewed through the camera.
    fn right(&self) -> Vector3 {
        self.direction().cross(self.up())
    }
}

/// A `Scene` contains the camera, light sources, and surfaces that are to be
/// rendered.
#[derive(Default)]
pub struct Scene {
    surfaces: Vec<Box<dyn Surface + Send + Sync>>,
    camera: Camera,
    lights: Vec<Sun>,
}

impl Scene {
    /// Make an empty scene with a default camera.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_surface(&mut self, surface: impl Surface + Send + Sync + 'static) {
        self.surfaces.push(Box::new(surface));
    }

    pub fn add_light(&mut self, light: Sun) {
        self.lights.push(light);
    }

    /// Render the scene to an image of size `width` x `height`. Only a
    /// part of the image is actually rendered, based on `thread_id` and
    /// `num_threads`. The function should be called in `num_threads` separate
    /// threads, where all the `sender`s send to the same receiver. The `sender`
    /// sends rendered pixels together with x-y-coordinates through a channel.
    pub fn render(
        &self,
        width: usize,
        height: usize,
        sender: Sender<(usize, usize, Pixel)>,
        thread_id: usize,
        num_threads: usize,
    ) -> Result<(), Box<dyn Error>> {
        let pixel_size = self.camera.screen_width / width as f64;

        let center_of_screen = self.camera.direction() * self.camera.distance_to_screen;

        for pixel_y in 0..height {
            if (pixel_y + thread_id) % num_threads != 0 {
                // Skip the line.
                continue;
            }

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
                sender.send((pixel_x, pixel_y, rgb.into()))?;
            }
        }

        Ok(())
    }

    /// Spawn multiple threads for rendering the scene. The number of threads
    /// spawned is one less than the number of CPU cores. Each thread renders a
    /// subset of the pixels of the image. When a pixel is finished, it is sent
    /// through a channel. The receiving end of the channel is returned from
    /// this function.
    pub fn spawn_render_threads(
        self,
        window_width: usize,
        window_height: usize,
    ) -> Receiver<(usize, usize, Pixel)> {
        let (sender, receiver) = mpsc::channel();
        let num_threads = num_cpus::get() - 1;
        let scene_arc = Arc::new(self);
        for thread_id in 1..num_threads {
            let scene_clone = scene_arc.clone();
            let sender_clone = sender.clone();

            thread::spawn(move || {
                scene_clone
                    .render(
                        window_width,
                        window_height,
                        sender_clone,
                        thread_id,
                        num_threads,
                    )
                    .unwrap();
            });
        }

        thread::spawn(move || {
            scene_arc
                .render(window_width, window_height, sender, 0, num_threads)
                .unwrap();
        });

        receiver
    }

    /// Trace a ray until it intersects a surface in the scene. If nothing is
    /// hit, then `None` is returned. Else, a tuple is returned, where the first
    /// element is the intersection, and the second is the normal vector.
    fn trace(&self, ray: Ray) -> Option<(Vector3, Vector3)> {
        let mut closest_intersection = INFINITY;
        let mut result = None;

        for surface in self.surfaces.iter() {
            let closest_intersection_of_surface = surface.closest_intersection(&ray);

            match closest_intersection_of_surface {
                None => continue,
                Some((distance, normal)) => {
                    if distance <= EPSILON.sqrt() {
                        // TODO: Is square root of machine epsilon a good choice?
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

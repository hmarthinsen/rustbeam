#![warn(clippy::all, clippy::pedantic)]

use rustbeam::image::Image;
use rustbeam::lights::Sun;
use rustbeam::scene::Scene;
use rustbeam::surfaces::{Plane, Sphere};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
};
use std::{
    sync::{mpsc, Arc},
    thread,
};

pub fn main() {
    // Initialize SDL and make a window that can be drawn into.
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window_width = 1280;
    let window_height = 720;

    let window = video_subsystem
        .window("rust-sdl2 demo", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    // Make a texture that is to be copied into the canvas every frame.
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ABGR8888, window_width, window_height)
        .unwrap();

    // Make a scene and add surfaces and lights to it.
    let mut scene = Scene::new();

    scene.add_surface(Sphere::new((-1.0, 5.0, 0.0), 1.5));
    scene.add_surface(Sphere::new((1.0, 5.0, 0.0), 1.0));
    scene.add_surface(Plane::new((0.0, 0.0, 1.0), -2.0));

    scene.add_light(Sun::new((1.0, 0.0, 0.0), (1.0, 1.0, -1.0)));
    scene.add_light(Sun::new((0.0, 1.0, 0.0), (-1.0, 1.0, -1.0)));
    scene.add_light(Sun::new((0.0, 0.0, 1.0), (0.0, 1.0, 1.0)));

    // The rendered pixels are written to this image.
    let mut image = Image::new(window_width as usize, window_height as usize);

    // Rendering of the scene is done in a separate thread. When each pixel is
    // complete, it is sent through a channel to the main thread and written
    // into the image.
    let (sender, receiver) = mpsc::channel();
    let num_threads = num_cpus::get();
    let scene_arc = Arc::new(scene);
    for thread_id in 0..num_threads {
        let sender_clone = sender.clone();
        let scene_clone = scene_arc.clone();
        thread::spawn(move || {
            scene_clone.render(
                window_width as usize,
                window_height as usize,
                sender_clone,
                thread_id,
                num_threads,
            );
        });
    }

    let mut event_pump = sdl_context.event_pump().unwrap();

    // SDL event loop.
    'render_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                // Exit the event loop if the user closes the window or presses
                // the escape key.
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'render_loop,
                _ => {}
            }
        }

        let mut received_pixels = receiver.try_iter().peekable();

        if received_pixels.peek().is_some() {
            // If there are any pixels that have been rendered and that have
            // been sent through the channel, write them to the image, and then
            // update the texture that is drawn on the screen.
            image.update(received_pixels);
            let srgba_vec = image.get_srgba_vector();
            texture
                .update(None, srgba_vec.as_slice(), 4 * window_width as usize)
                .unwrap();

            canvas.copy(&texture, None, None).unwrap();
        }

        canvas.present();
    }

    image.clamp();
    image.save_png("test-data/test-data-out/test.png");
}

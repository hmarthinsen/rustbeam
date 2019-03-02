#![warn(clippy::all, clippy::pedantic)]

use rustbeam::image::Image;
use rustbeam::scene::Scene;
use rustbeam::surfaces::Sphere;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
};

pub fn main() {
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

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ABGR8888, window_width, window_height)
        .unwrap();

    let mut scene = Scene::new();

    scene.add(Sphere::new((-1.0, 5.0, 0.0), 1.5));
    scene.add(Sphere::new((1.0, 5.0, 0.0), 1.0));

    let mut image = Image::new(window_width as usize, window_height as usize);
    scene.render(&mut image);

    image.clamp();

    let srgba_vec = image.to_srgba_vector();
    texture
        .update(None, srgba_vec.as_slice(), 4 * window_width as usize)
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'render_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'render_loop,
                _ => {}
            }
        }

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }

    image.save_png("test-data/test-data-out/test.png");
}

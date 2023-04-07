use std::time;

use manim_with_rust::object::vector::Vector;
use sdl2::{pixels::Color, rect::Point, sys::Window};

#[derive(Clone, Copy)]
struct asd {
    x: i32,
    y: i32,
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Teszt", 800, 600).build().unwrap();
    std::thread::sleep(time::Duration::from_secs(5));
    let mut canvas = window.into_canvas().build().unwrap();
    let points = [Vector::new(50 as f64, 40 as f64); 256];
    let points = points.map(|x| x.to_basic_sdl());
    canvas.set_draw_color(Color::BLUE);
    canvas.draw_points(points.as_slice()).unwrap();
    canvas.present();
    std::thread::sleep(time::Duration::from_secs(5));
}

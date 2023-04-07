use std::time;

use manim_with_rust::{
    object::{circle::Circle, vector::Vector, Object},
    scene::Scene,
};
use sdl2::{pixels::Color, rect::Point, sys::Window};
/// Ties together a Vector with a Point to render
struct RenderTie {
    calc_vector: Vector,
    rend_point: Point,
}
static WIDTH: i32 = 1000;
static HEIGHT: i32 = 1000;
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Teszt", 1000, 1000).build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut calc_mesh: Vec<RenderTie> = Vec::new(); //Ties vectors to calculate with pixels to render
    for w in 0..WIDTH {
        for h in 0..HEIGHT {
            let rend_point = Point::new(w, h);
            let calc_vector: Vector = rend_to_calc_transform(w, h, WIDTH, HEIGHT);
            calc_mesh.push(RenderTie {
                calc_vector,
                rend_point,
            });
        }
    }

    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    objects.push(Box::new(Circle::new(
        Vector::new(5.0, 5.0),
        f64::sqrt(50.0),
    )));

    let mut render_queue: Vec<Point> = Vec::new();
    for tie in calc_mesh {
        let calc_vector = tie.calc_vector;
        let rend_point = tie.rend_point;
        for obj in &objects {
            if obj.as_ref().contains(calc_vector) {
                render_queue.push(rend_point);
            }
        }
    }

    canvas.set_draw_color(Color::WHITE);
    canvas.draw_points(render_queue.as_slice()).unwrap();
    canvas.present();
    std::thread::sleep(time::Duration::from_secs(5));
}
/// Using render pixel coordinates returns a Vector representing where to render it.
/// For now it's a centered 20 x 20 coordinate system
fn rend_to_calc_transform(x: i32, y: i32, max_width: i32, max_height: i32) -> Vector {
    let max_width = max_width as f64;
    let max_height = max_height as f64;
    let x = x as f64;
    let y = y as f64;
    //Type bullshit
    let x = (x - max_width / 2.0) / max_width * 20.0;
    let y = (-y + max_height / 2.0) / max_height * 20.0;
    Vector { x, y }
}

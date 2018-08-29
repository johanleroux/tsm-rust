extern crate opengl_graphics;
extern crate piston_window;
extern crate num;

use piston_window::{OpenGL, PistonWindow, WindowSettings};
use opengl_graphics::GlGraphics;

mod config;
mod menu;
mod simulation;

fn main() {
    let title = "Travelling Salesman";
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        title,
        [config::WINDOW_SIZE.width, config::WINDOW_SIZE.height],
    ).opengl(opengl)
        .samples(4)
        .fullscreen(false)
        .resizable(false)
        .exit_on_esc(false)
        .build()
        .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error));

    let mut gl = GlGraphics::new(opengl);

    menu::run(&mut window, &mut gl);
}

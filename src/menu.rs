use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, RenderEvent, TextureSettings, Transformed};

use config::{color, font};
use num;
use simulation;

fn draw(context: Context, graphics: &mut GlGraphics, font: &mut GlyphCache, menu_lines: &mut Vec<String>, menu_selection: i32) {
    // Heading
    text(
        color::WHITE,
        font::TITLE_SIZE,
        "Travelling Salesman",
        font,
        context.transform.trans(
            font::PADDING,
            font::TITLE_PADDING,
        ),
        graphics,
    ).unwrap();

    // Menu
    for (index, line) in menu_lines.iter().enumerate() {
        let new_line_offset = 40.0;
        text(
            font::color(menu_selection, index),
            font::SIZE,
            line,
            font,
            context.transform.trans(
                font::PADDING,
                (index as f64 + 1.0) * new_line_offset + 192.0,
            ),
            graphics,
        ).unwrap();
    }
}

pub fn run(mut window: &mut PistonWindow, mut opengl: &mut GlGraphics) {
    let mut font = GlyphCache::new(
        "./assets/fonts/FiraSans-Regular.ttf",
        (),
        TextureSettings::new(),
    ).unwrap();
    let mut menu_selection: i32 = 0;
    let mut menu_lines: Vec<String> = vec![String::new()];
    menu_lines.clear();
    menu_lines.push(String::from("Simulate"));
    menu_lines.push(String::from("Settings"));
    menu_lines.push(String::from("Exit"));

    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            opengl.draw(args.viewport(), |context, graphics| {
                clear(color::BLACK, graphics);
                draw(context, graphics, &mut font, &mut menu_lines, menu_selection);
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W | Key::Up   => menu_selection = num::clamp(menu_selection - 1, 0, menu_lines.len() as i32 - 1),
                Key::S | Key::Down => menu_selection = num::clamp(menu_selection + 1, 0, menu_lines.len() as i32 - 1),
                Key::Space | Key::Return => {
                    match menu_selection {
                        0 => {
                            let mut simulation = simulation::Simulation::new();
                            
                            simulation.run(&mut window, &mut opengl, &mut font);
                        },
                        1 => {},
                        2 => break,
                        _ => {},
                    }
                },
                Key::Escape => break,
                _ => {}
            }
        }
    }
}

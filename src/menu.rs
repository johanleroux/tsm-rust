use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{
    clear, rectangle, text, Button, Context, Key, PistonWindow, PressEvent, RenderEvent,
    TextureSettings, Transformed,
};

use config;
use config::{color, font};
use num;
use simulation;

fn draw(
    context: Context,
    graphics: &mut GlGraphics,
    font: &mut GlyphCache,
    menu_lines: &mut Vec<String>,
    menu_selection: i32,
) {
    // Heading
    text(
        color::WHITE,
        font::TITLE_SIZE,
        "Travelling Salesman",
        font,
        context.transform.trans(font::PADDING, font::TITLE_PADDING),
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

    unsafe {
        match config::SELECTION_ALGORITHM_X {
            config::SelectionAlgorithm::Tournament => {
                rectangle(
                    color::RED,
                    [0.0, 0.0, 185.0, 36.0],
                    context.transform.trans(293.0, 243.0),
                    graphics,
                );
            }
            config::SelectionAlgorithm::Roulette => rectangle(
                color::RED,
                [0.0, 0.0, 133.0, 36.0],
                context.transform.trans(493.0, 243.0),
                graphics,
            ),
            config::SelectionAlgorithm::Random => rectangle(
                color::RED,
                [0.0, 0.0, 132.0, 36.0],
                context.transform.trans(643.0, 243.0),
                graphics,
            ),
        }
    }

    unsafe {
        match config::ELITISM {
            0 => {
                rectangle(
                    color::RED,
                    [0.0, 0.0, 53.0, 36.0],
                    context.transform.trans(293.0, 283.0),
                    graphics,
                );
            }
            1 => {
                rectangle(
                    color::RED,
                    [0.0, 0.0, 53.0, 36.0],
                    context.transform.trans(350.0, 283.0),
                    graphics,
                );
            }
            2 => {
                rectangle(
                    color::RED,
                    [0.0, 0.0, 53.0, 36.0],
                    context.transform.trans(407.0, 283.0),
                    graphics,
                );
            }
            5 => {
                rectangle(
                    color::RED,
                    [0.0, 0.0, 53.0, 36.0],
                    context.transform.trans(464.0, 283.0),
                    graphics,
                );
            }
            10 => {
                rectangle(
                    color::RED,
                    [0.0, 0.0, 68.0, 36.0],
                    context.transform.trans(521.0, 283.0),
                    graphics,
                );
            }
            _ => {}
        }
    }

    unsafe {
        match config::BENCH_MODE {
            true => {
                rectangle(
                    color::RED,
                    [0.0, 0.0, 72.0, 36.0],
                    context.transform.trans(293.0, 323.0),
                    graphics,
                );
            }
            false => {
                rectangle(
                    color::RED,
                    [0.0, 0.0, 80.0, 36.0],
                    context.transform.trans(370.0, 323.0),
                    graphics,
                );
            }
        }
    }

    text(
        color::WHITE,
        font::SIZE,
        "Tournament",
        font,
        context.transform.trans(300.0, 272.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "Roulette",
        font,
        context.transform.trans(500.0, 272.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "Random",
        font,
        context.transform.trans(650.0, 272.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "0%",
        font,
        context.transform.trans(300.0, 312.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "1%",
        font,
        context.transform.trans(356.0, 312.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "2%",
        font,
        context.transform.trans(413.0, 312.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "5%",
        font,
        context.transform.trans(470.0, 312.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "10%",
        font,
        context.transform.trans(527.0, 312.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "true",
        font,
        context.transform.trans(300.0, 352.0),
        graphics,
    ).unwrap();

    text(
        color::WHITE,
        font::SIZE,
        "false",
        font,
        context.transform.trans(375.0, 352.0),
        graphics,
    ).unwrap();
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
    menu_lines.push(String::from("Selection"));
    menu_lines.push(String::from("Elitism"));
    menu_lines.push(String::from("Bench Mode"));
    menu_lines.push(String::from("Exit"));

    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            opengl.draw(args.viewport(), |context, graphics| {
                clear(color::BLACK, graphics);
                draw(
                    context,
                    graphics,
                    &mut font,
                    &mut menu_lines,
                    menu_selection,
                );
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W | Key::Up => {
                    menu_selection = num::clamp(menu_selection - 1, 0, menu_lines.len() as i32 - 1)
                }
                Key::S | Key::Down => {
                    menu_selection = num::clamp(menu_selection + 1, 0, menu_lines.len() as i32 - 1)
                }
                Key::Space | Key::Return => match menu_selection {
                    0 => {
                        let mut simulation = simulation::Simulation::new();

                        simulation.run(&mut window, &mut opengl, &mut font);
                    }
                    1 => unsafe {
                        match config::SELECTION_ALGORITHM_X {
                            config::SelectionAlgorithm::Tournament => {
                                config::SELECTION_ALGORITHM_X = config::SelectionAlgorithm::Roulette
                            }
                            config::SelectionAlgorithm::Roulette => {
                                config::SELECTION_ALGORITHM_X = config::SelectionAlgorithm::Random
                            }
                            config::SelectionAlgorithm::Random => {
                                config::SELECTION_ALGORITHM_X =
                                    config::SelectionAlgorithm::Tournament
                            }
                        }
                    },
                    2 => unsafe {
                        match config::ELITISM {
                            0 => config::ELITISM = 1,
                            1 => config::ELITISM = 2,
                            2 => config::ELITISM = 5,
                            5 => config::ELITISM = 10,
                            10 => config::ELITISM = 0,
                            _ => config::ELITISM = 0,
                        }
                    },
                    3 => unsafe {
                        match config::BENCH_MODE {
                            true => config::BENCH_MODE = false,
                            false => config::BENCH_MODE = true,
                        }
                    },
                    4 => break,
                    _ => {}
                },
                Key::Escape => break,
                _ => {}
            }
        }
    }
}

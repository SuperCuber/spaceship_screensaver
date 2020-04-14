extern crate antidote;
extern crate nannou;
extern crate rayon;

use nannou::prelude::*;
use nannou::event::SimpleWindowEvent;

mod constants;
mod utils;

use constants::*;
use utils::*;

fn main() {
    nannou::app(model, event, view).run();
}

struct Line {
    head: Point2,
    tail: f32,
    color: Rgb,
    growth_rate: f32,
}

#[derive(Default)]
struct Model {
    lines: Vec<Line>,
    growing: Vec<(Line, f32)>,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .with_dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1)
        .build()
        .unwrap();
    let window = app.window(window).unwrap();
    window.inner_glium_display().gl_window().hide_cursor(true);
    let monitor = window.current_monitor();
    window.set_fullscreen(Some(monitor));
    Model::default()
}

fn event(app: &App, mut model: Model, event: Event) -> Model {
    match event {
        Event::WindowEvent {
            simple: Some(simple),
            ..
        } => {
            match simple {
                SimpleWindowEvent::KeyPressed(_) | SimpleWindowEvent::MouseMoved(_) => {
println!("reached");
                    if app.elapsed_frames() > 10 {
                        std::process::exit(0);
                    }
                }
                _ => {}
            }
        }
        Event::Update(_dt) => {
            model = zoom_in(app.window_rect(), model);
        }
        _ => {}
    }
    model
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(Rgb::new_u8(20, 20, 20));

    for line in model.lines.iter().chain(model.growing.iter().map(|t| &t.0)) {
        let tail_end = line.head.normalize() * (line.head.magnitude() - line.tail);
        draw.line().points(line.head, tail_end).color(line.color);
    }

    draw.to_frame(app, &frame).unwrap();
    frame
}

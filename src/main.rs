extern crate antidote;
extern crate nannou;
extern crate rayon;

use nannou::event::WindowEvent;
use nannou::prelude::*;

mod constants;
mod utils;

use constants::*;
use utils::*;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

#[derive(Debug, Clone)]
struct Line {
    head: Point2,
    tail: Point2,
    growth_rate: f32,
    color: Rgb,
}

#[derive(Default)]
struct Model {
    lines: Vec<Line>,
}

fn model(app: &App) -> Model {
    let window = app.main_window();
    window.set_cursor_visible(false);
    window.set_inner_size_pixels(SCREEN_SIZE.0, SCREEN_SIZE.1);
    window.set_fullscreen(true);
    app.set_loop_mode(LoopMode::rate_fps(2.0));
    Model::default()
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(simple),
            ..
        } => match simple {
            WindowEvent::KeyPressed(_) | WindowEvent::MouseMoved(_) => {
                if app.elapsed_frames() > 10 {
                    std::process::exit(0);
                }
            }
            _ => {}
        },
        Event::Update(dt) => {
            zoom_in(app.window_rect(), model, dt.since_last.as_millis());
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background()
        .color(Rgb::from_components((0u8, 0u8, 0u8)));

    for line in model.lines.iter() {
        draw.line().points(line.head, line.tail).color(line.color);
    }

    draw.to_frame(app, &frame).unwrap();
}

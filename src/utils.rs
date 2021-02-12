use crate::constants::*;
use crate::{Line, Model};

use antidote::Mutex;
use rayon::prelude::*;

use nannou::color::{FromColor, RgbHue};
use nannou::math::{Basis2, Deg, Rad};
use nannou::prelude::*;
use nannou::rand::distributions::{Distribution, Standard};
use nannou::rand::Rng;

impl Distribution<Line> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Line {
        let distance = random_f32().powf(LONG_LINE_BIAS);
        let head = random_unit_vector() * (random_f32() * CENTER_SIZE);
        Line {
            head,
            tail_len: 0.0,
            color: random_color(),
            growth_rate: map_range(distance, 1.0, 0.0, MIN_GROW_FACTOR, MAX_GROW_FACTOR),
        }
    }
}

pub(crate) fn random_unit_vector() -> Point2 {
    let angle = Deg::full_turn() * random_f32();
    let rotation = Basis2::from_angle(angle);
    rotation.rotate_point(Vector2::unit_x().into()).into()
}

pub(crate) fn random_color() -> Rgb {
    if USE_CLEAR_COLORS {
        let angle = Deg::<f32>::full_turn() * random();
        let radians: Rad<f32> = angle.into();
        Rgb::from_hsl(Hsl::new(RgbHue::from_radians(radians.0), 1.0, 0.5))
    } else {
        Rgb::from_components((random(), random(), random()))
    }
}

pub(crate) fn move_out(mut line: Line, millis: u128) -> Line {
    line.head *= (line.growth_rate - 1.0) * (millis as f32 / 15.0) + 1.0;
    line
}

pub(crate) fn in_window(window: Rect, line: &Line) -> bool {
    let tail = line.head.normalize() * (line.head.magnitude() - line.tail_len);
    window.contains(tail)
}

pub(crate) fn zoom_in(window: Rect, model: &mut Model, millis: u128) {
    // Zoom in effect
    model.lines = model
        .lines
        .clone()
        .into_par_iter()
        .map(|line| move_out(line, millis))
        .filter(|line| in_window(window, line))
        .collect();

    // Grow the baby lines
    let fully_grown = Mutex::new(Vec::new());
    model.growing = model
        .growing
        .clone()
        .into_par_iter()
        .map(|(line, target)| (move_out(line, millis), target))
        .filter_map(|(mut line, target)| {
            line.tail_len = line.head.magnitude() - CENTER_SIZE;
            if line.tail_len >= target {
                fully_grown.lock().push(line);
                None
            } else {
                Some((line, target))
            }
        })
        .collect();

    // Adult lines go to regular lines
    model.lines.extend(fully_grown.into_inner());

    // Create new lines
    for _ in 0..LINES_PER_FRAME {
        let line: Line = random();
        let tail = (line.growth_rate - 1.0) * GROWTH_TO_TAIL_LEN_RATIO;
        model.growing.push((line, tail));
    }
}

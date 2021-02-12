use crate::constants::*;
use crate::{Line, Model};

use rayon::prelude::*;

use nannou::color::{FromColor, RgbHue};
use nannou::math::{Basis2, Deg, Rad};
use nannou::prelude::*;
use nannou::rand::distributions::{Distribution, Standard};
use nannou::rand::Rng;

impl Distribution<Line> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Line {
        let distance = random_f32().powf(SHORT_LINE_BIAS);
        let source = random_unit_vector() * (random_f32() * CENTER_SIZE);
        Line {
            head: source,
            tail: source,
            color: random_color(),
            growth_rate: map_range(distance, 0.0, 1.0, MIN_GROW_FACTOR, MAX_GROW_FACTOR),
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

impl Line {
    fn move_out(self, millis: u128) -> Line {
        let current_len = (self.head-self.tail).magnitude();
        let target_len = self.growth_rate * GROWTH_TO_TAIL_LEN_RATIO;
        let new_head = self.head * (1.0 + self.growth_rate * millis as f32);
        if current_len < target_len {
            // Only move head
            Line {
                head: new_head,
                tail: self.tail,
                color: self.color,
                growth_rate: self.growth_rate,
            }
        } else {
            // Move both, preserving length
            Line {
                head: new_head,
                tail: self.tail.normalize_to(new_head.magnitude() - current_len),
                color: self.color,
                growth_rate: self.growth_rate,
            }
        }
    }
}

pub(crate) fn zoom_in(window: Rect, model: &mut Model, millis: u128) {
    // Zoom in effect
    model.lines = model
        .lines
        .clone()
        .into_par_iter()
        .map(|line| line.move_out(millis))
        .filter(|line| window.contains(line.tail))
        .collect();

    // Create new lines
    for _ in 0..LINES_PER_FRAME {
        model.lines.push(random());
    }
}

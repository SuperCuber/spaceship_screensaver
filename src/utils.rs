use crate::constants::*;
use crate::{Line, Model};

use rayon::prelude::*;

use nannou::color::RgbHue;
use nannou::math::{Basis2, Deg, Rad};
use nannou::prelude::*;
use nannou::rand::distributions::{Distribution, Standard};
use nannou::rand::Rng;

impl Distribution<Line> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Line {
        let distance = random_f32().powf(SHORT_LINE_BIAS);

        let growth_rate = map_range(distance, 0.0, 1.0, MIN_GROW_FACTOR, MAX_GROW_FACTOR);
        let length = growth_rate * GROWTH_TO_TAIL_LEN_RATIO;
        let head = random_unit_vector() * (random_f32() * CENTER_SIZE + length);

        Line {
            head,
            length,
            color: random_color(),
            growth_rate,
        }
    }
}

pub(crate) fn random_unit_vector() -> Point2 {
    let angle = Deg::full_turn() * random_f32();
    let rotation = Basis2::from_angle(angle);
    rotation.rotate_point(Vector2::unit_x().into()).into()
}

pub(crate) fn random_color() -> Rgba {
    let mut color = if USE_CLEAR_COLORS {
        let angle = Deg::<f32>::full_turn() * random();
        let radians: Rad<f32> = angle.into();
        Rgba::from(Hsl::new(RgbHue::from_radians(radians.0), 1.0, 0.5))
    } else {
        Rgba::from(Rgb::new(random(), random(), random()))
    };

    color.alpha = 0.0;
    color
}

impl Line {
    pub fn tail(&self) -> Point2 {
        self.head.normalize_to(self.head.magnitude() - self.length)
    }

    fn move_out(mut self, millis: u128) -> Line {
        let new_head = self.head * (1.0 + self.growth_rate * millis as f32);
        if self.color.alpha < 1.0 {
            self.color.alpha += 0.01;
        }

        Line {
            head: new_head,
            color: self.color,
            growth_rate: self.growth_rate,
            length: self.length * (1.0 + (self.growth_rate * GROWTH_TO_TAIL_GROWTH_RATIO)),
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
        .filter(|line| window.contains(line.tail()))
        .collect();

    // Create new lines
    for _ in 0..LINES_PER_FRAME {
        model.lines.push(random());
    }
}

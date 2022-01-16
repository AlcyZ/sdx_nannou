use std::time::{Duration, Instant};

use nannou::prelude::*;
use nannou::rand::prelude::SliceRandom;
use nannou::rand::thread_rng;

use crate::points_list_diagonals;

// Divisor of 360:
// 1 2 3 4 5 6 8 9 10 12 15 18 20 24 30 36 40 45 60 72 90 120 180
pub const DIVISORS_360: &[usize] = &[
    1, 2, 3, 4, 5, 6, 8, 9, 10, 12, 15, 18, 20, 24, 30, 36, 40, 45, 60, 72, 90, 120,
];

const _FANCY_DIVISORS_360: &[usize] = &[8, 60, 72, 90, 24, 40, 45];

struct Radius {
    base: f32,
    current: f32,
    offset: f32,
}

struct Color {
    color: Rgba,
    weight: f32,
}

pub struct Shape {
    pos: Point2,
    radius: Radius,
    step: usize,
    border_color: Color,
    stroke_color: Color,
    boundary: Option<Rect>,
    last_update: Instant,
    update_duration: u64,
}

impl Shape {
    pub fn new(pos: Point2) -> Shape {
        Shape {
            pos,
            radius: Radius {
                base: 100.0,
                current: 100.0,
                offset: 0.0,
            },
            step: rng_360_divisor(),
            border_color: Color {
                color: Rgba::new(0.8, 0.8, 0.8, 1.0),
                weight: 2.0,
            },
            stroke_color: Color {
                color: Rgba::new(0.8, 0.8, 0.8, 0.6),
                weight: 2.0,
            },
            boundary: None,
            last_update: Instant::now(),
            update_duration: 500,
        }
    }

    pub fn with_radius(self, radius: f32) -> Shape {
        Shape {
            pos: self.pos,
            radius: Radius {
                base: radius,
                current: radius,
                offset: self.radius.offset,
            },
            step: self.step,
            border_color: self.border_color,
            stroke_color: self.stroke_color,
            boundary: None,
            last_update: self.last_update,
            update_duration: self.update_duration,
        }
    }

    pub fn with_radius_offset(self, radius_offset: f32) -> Shape {
        Shape {
            pos: self.pos,
            radius: Radius {
                offset: radius_offset,
                ..self.radius
            },
            step: self.step,
            border_color: self.border_color,
            stroke_color: self.stroke_color,
            boundary: None,
            last_update: self.last_update,
            update_duration: self.update_duration,
        }
    }

    pub fn with_step(self, step: usize) -> Shape {
        Shape {
            pos: self.pos,
            radius: self.radius,
            step,
            border_color: self.border_color,
            stroke_color: self.stroke_color,
            boundary: None,
            last_update: self.last_update,
            update_duration: self.update_duration,
        }
    }

    pub fn with_update_duration(self, update_duration: u64) -> Shape {
        Shape {
            pos: self.pos,
            radius: self.radius,
            step: self.step,
            border_color: self.border_color,
            stroke_color: self.stroke_color,
            boundary: None,
            last_update: self.last_update,
            update_duration,
        }
    }

    pub fn with_boundary(self, boundary: Rect) -> Shape {
        Shape {
            pos: self.pos,
            radius: self.radius,
            step: self.step,
            border_color: self.border_color,
            stroke_color: self.stroke_color,
            boundary: Some(boundary),
            last_update: self.last_update,
            update_duration: self.update_duration,
        }
    }

    pub fn with_border_color(self, color: Rgba, width: f32) -> Shape {
        Shape {
            pos: self.pos,
            radius: self.radius,
            step: self.step,
            border_color: Color {
                color,
                weight: width,
            },
            stroke_color: self.stroke_color,
            boundary: None,
            last_update: self.last_update,
            update_duration: self.update_duration,
        }
    }

    pub fn with_stroke_color(self, color: Rgba, width: f32) -> Shape {
        Shape {
            pos: self.pos,
            radius: self.radius,
            step: self.step,
            border_color: self.border_color,
            stroke_color: Color {
                color,
                weight: width,
            },
            boundary: None,
            last_update: self.last_update,
            update_duration: self.update_duration,
        }
    }
}

impl Shape {
    pub fn update_radius(&mut self, app: &App) {
        let sine = app.time.sin();
        self.radius.current = map_range(
            sine,
            -1.0,
            1.0,
            self.radius.base - self.radius.offset,
            self.radius.base + self.radius.offset,
        );
    }

    pub fn update_step(&mut self) {
        if self.last_update.elapsed() > Duration::from_millis(self.update_duration) {
            self.step = rng_360_divisor();
            self.last_update = Instant::now();
        }
    }

    pub fn update_pos(&mut self, app: &App) {
        let sine = app.time.sin();
        let slower_sine = (app.time / 2.0).sin();

        let boundary = match self.boundary {
            Some(boundary) => boundary,
            None => app.window_rect(),
        };

        let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());
        let y = map_range(slower_sine, -1.0, 1.0, boundary.bottom(), boundary.top());

        self.pos = pt2(x, y);
    }
}

impl Shape {
    pub fn update_border_color_r(&mut self, app: &App) {
        let red = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);

        let color = Rgba::new(
            red,
            self.border_color.color.green,
            self.border_color.color.blue,
            self.border_color.color.alpha,
        );

        self.border_color.color = color;
    }

    pub fn update_border_color_g(&mut self, app: &App) {
        let green = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);

        let color = Rgba::new(
            self.border_color.color.red,
            green,
            self.border_color.color.blue,
            self.border_color.color.alpha,
        );

        self.border_color.color = color;
    }

    pub fn update_border_color_b(&mut self, app: &App) {
        let blue = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);

        let color = Rgba::new(
            self.border_color.color.red,
            self.border_color.color.green,
            blue,
            self.border_color.color.alpha,
        );

        self.border_color.color = color;
    }

    pub fn update_border_color_a(&mut self, app: &App) {
        let alpha = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);

        let color = Rgba::new(
            self.border_color.color.red,
            self.border_color.color.green,
            self.border_color.color.blue,
            alpha,
        );

        self.border_color.color = color;
    }
}

impl Shape {
    pub fn update_stroke_color_r(&mut self, app: &App) {
        let red = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);

        let color = Rgba::new(
            red,
            self.stroke_color.color.green,
            self.stroke_color.color.blue,
            self.stroke_color.color.alpha,
        );

        self.stroke_color.color = color;
    }

    pub fn update_stroke_color_g(&mut self, app: &App) {
        let green = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);

        let color = Rgba::new(
            self.stroke_color.color.red,
            green,
            self.stroke_color.color.blue,
            self.stroke_color.color.alpha,
        );

        self.stroke_color.color = color;
    }

    pub fn update_stroke_color_b(&mut self, app: &App) {
        let blue = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);

        let color = Rgba::new(
            self.stroke_color.color.red,
            self.stroke_color.color.green,
            blue,
            self.stroke_color.color.alpha,
        );

        self.stroke_color.color = color;
    }

    pub fn update_stroke_color_a(&mut self, app: &App) {
        let alpha = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);

        let color = Rgba::new(
            self.stroke_color.color.red,
            self.stroke_color.color.green,
            self.stroke_color.color.blue,
            alpha,
        );

        self.stroke_color.color = color;
    }
}

pub fn draw_circular_shape(draw: &Draw, shape: &Shape) {
    let points = circle_points(shape.radius.current, shape.pos.x, shape.pos.y, shape.step);
    let diagonal_points = points_list_diagonals(&points);
    for (start, end) in diagonal_points {
        draw.line()
            .start(start)
            .end(end)
            .color(shape.stroke_color.color)
            .weight(shape.border_color.weight);
    }

    let points_colored = points.iter().map(|p| (*p, shape.border_color.color));
    draw.polyline()
        .weight(shape.stroke_color.weight)
        .points_colored(points_colored);
}

fn circle_points(radius: f32, offset_x: f32, offset_y: f32, step: usize) -> Vec<Point2> {
    if !DIVISORS_360.contains(&step) {
        let divisors = DIVISORS_360
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>();

        panic!("step must be one of: {}", divisors.join(", "));
    }

    (0..=360)
        .step_by(step)
        .map(|i| {
            let radian = deg_to_rad(i as f32);

            let x = radian.sin() * radius + offset_x;
            let y = radian.cos() * radius + offset_y;

            pt2(x, y)
        })
        .collect()
}

pub fn rng_360_divisor() -> usize {
    *DIVISORS_360.choose(&mut thread_rng()).unwrap()
}

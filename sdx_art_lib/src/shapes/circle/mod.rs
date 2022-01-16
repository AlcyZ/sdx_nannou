use nannou::prelude::*;

#[derive(Clone)]
pub struct Circle {
    pub pos: Vec2,
    pub radius: f32,
    pub weight: f32,
    pub color: Rgba,
}

impl Circle {
    pub fn new(radius: f32) -> Circle {
        let pos = vec2(0.0, 0.0);

        Circle {
            pos,
            radius,
            weight: 1.0,
            color: Rgba::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn with_xy(&self, pos: Vec2) -> Circle {
        Circle {
            pos,
            ..self.clone()
        }
    }

    pub fn with_radius(&self, radius: f32) -> Circle {
        Circle {
            radius,
            ..self.clone()
        }
    }

    pub fn with_x_y(&self, x: f32, y: f32) -> Circle {
        Circle {
            pos: vec2(x, y),
            ..self.clone()
        }
    }

    pub fn with_color(&self, color: Rgba) -> Circle {
        Circle {
            color,
            ..self.clone()
        }
    }

    pub fn with_weight(&self, weight: f32) -> Circle {
        Circle {
            weight,
            ..self.clone()
        }
    }
}

impl Circle {
    pub fn draw(&self, draw: &Draw) {
        let points = (0..=360).map(|i| {
            let radian = deg_to_rad(i as f32);

            let x = radian.sin() * self.radius + self.pos.x;
            let y = radian.cos() * self.radius + self.pos.y;

            pt2(x, y)
        });

        draw.polyline()
            .color(self.color)
            .weight(self.weight)
            .points(points);
    }
}

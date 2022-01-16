use nannou::prelude::*;
use sdx_art_lib::shapes::circular::{draw_circular_shape, Shape};

pub fn present() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    green_shapes: GreenShapes,
}

struct GreenShapes {
    shapes: Vec<Shape>,
}

impl GreenShapes {
    fn new(app: &App) -> GreenShapes {
        let _win = app.window_rect();

        let mut shapes = vec![];

        for _i in 0..15 {
            let (shape, radius) = Self::rng_green_shape();

            let shape = shape.with_boundary(Rect::from_w_h(
                radius + nannou::rand::random_range(25.0, 600.0),
                radius + nannou::rand::random_range(25.0, 600.0),
            ));

            shapes.push(shape);
        }

        GreenShapes { shapes }
    }

    fn update(&mut self, app: &App) {
        self.shapes.iter_mut().for_each(|s| {
            s.update_pos(app);
            s.update_step();
            s.update_radius(app);
            s.update_border_color_g(app);
            s.update_stroke_color_g(app);
        });
    }

    fn rng_green_shape() -> (Shape, f32) {
        let radius = nannou::rand::random_range(50.0, 150.0);
        let radius_offset = nannou::rand::random_range(0.0, 30.0);

        let min = 0.5;
        let max = 1.0;

        let b_col = nannou::rand::random_range(min, max);
        let s_col = b_col - 0.1;

        (
            Shape::new(pt2(0.0, 0.0))
                .with_update_duration(nannou::rand::random_range(200, 800))
                .with_radius(radius)
                .with_radius_offset(radius_offset)
                .with_border_color(Rgba::new(0.0, b_col, 0.0, 1.0), 1.5)
                .with_stroke_color(Rgba::new(0.0, s_col, 0.0, 0.5), 1.0),
            radius,
        )
    }
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    Model {
        _window,
        green_shapes: GreenShapes::new(app),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.green_shapes.update(app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    frame.clear(DARKSLATEGREY);

    model
        .green_shapes
        .shapes
        .iter()
        .for_each(|s| draw_circular_shape(&draw, s));

    draw.to_frame(app, &frame).unwrap();
}

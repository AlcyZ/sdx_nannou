use nannou::prelude::*;

use sdx_art_lib::shapes::circle::Circle;

static mut LINE_WIDTH: f32 = 2.0;
static mut MIN_RADIUS: usize = 2;
static mut MAX_RADIUS: usize = 250;
static mut TOTAL_CIRCLES: usize = 1000;
static mut CREATE_CIRCLE_ATTEMPTS: usize = 500;

pub struct CirclePackingDescriptor {
    line_width: f32,
    min_radius: usize,
    max_radius: usize,
    total_circles: usize,
    create_circle_attempts: usize,
}

impl CirclePackingDescriptor {
    pub fn new(
        line_width: f32,
        min_radius: usize,
        max_radius: usize,
        total_circles: usize,
        create_circle_attempts: usize,
    ) -> CirclePackingDescriptor {
        CirclePackingDescriptor {
            line_width,
            min_radius,
            max_radius,
            total_circles,
            create_circle_attempts,
        }
    }

    fn from_statics() -> CirclePackingDescriptor {
        let (line_width, min_radius, max_radius, total_circles, create_circle_attempts) = unsafe {
            (
                LINE_WIDTH,
                MIN_RADIUS,
                MAX_RADIUS,
                TOTAL_CIRCLES,
                CREATE_CIRCLE_ATTEMPTS,
            )
        };

        CirclePackingDescriptor {
            line_width,
            min_radius,
            max_radius,
            total_circles,
            create_circle_attempts,
        }
    }
}

pub fn present(descriptor: CirclePackingDescriptor) {
    unsafe {
        LINE_WIDTH = descriptor.line_width;
        MIN_RADIUS = descriptor.min_radius;
        MAX_RADIUS = descriptor.max_radius;
        TOTAL_CIRCLES = descriptor.total_circles;
        CREATE_CIRCLE_ATTEMPTS = descriptor.create_circle_attempts;
    };

    nannou::app(model)
        .loop_mode(LoopMode::loop_ntimes(1))
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    descriptor: CirclePackingDescriptor,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model {
        _window,
        descriptor: CirclePackingDescriptor::from_statics(),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    draw.background().color(WHITE);

    let mut circles = vec![];

    for _ in 0..model.descriptor.total_circles {
        if let Some(circle) = try_create_circle(&window, &circles, model) {
            circle.draw(&draw);
            circles.push(circle);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn try_position_circle(window: &Rect, circles: &[Circle], model: &Model) -> Option<Circle> {
    for _ in 0..model.descriptor.create_circle_attempts {
        let circle = Circle::new(model.descriptor.min_radius as f32)
            .with_weight(model.descriptor.line_width)
            .with_x_y(
                random_range::<f32>(window.top_left().x, window.bottom_right().x),
                random_range::<f32>(window.top_left().y, window.bottom_right().y),
            );

        if !circle_has_collision(&circle, circles, window) {
            return Some(circle);
        }
    }

    None
}

fn try_create_circle(
    window: &Rect,
    circles: &[Circle],
    model: &Model,
) -> Option<Circle> {
    let mut circle = try_position_circle(window, circles, model)?;

    for radius in model.descriptor.min_radius..model.descriptor.max_radius {
        circle = circle.with_radius((radius) as f32);
        if circle_has_collision(&circle, circles, window) {
            circle = circle.with_radius((radius - 1) as f32);

            break;
        }
    }

    Some(circle)
}

fn circle_has_collision(circle: &Circle, circles: &[Circle], window: &Rect) -> bool {
    for other_circle in circles.iter() {
        let a = circle.radius + other_circle.radius;
        let x = circle.pos.x - other_circle.pos.x;
        let y = circle.pos.y - other_circle.pos.y;

        if a >= ((x * x) + (y * y)).sqrt() {
            return true;
        }
    }

    if circle.pos.x + circle.radius >= window.top_right().x
        || circle.pos.x - circle.radius <= window.top_left().x
    {
        return true;
    }

    if circle.pos.y + circle.radius >= window.top_left().y
        || circle.pos.y - circle.radius <= window.bottom_left().y
    {
        return true;
    }

    false
}

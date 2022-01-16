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
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    draw.background().color(WHITE);

    let mut circles = vec![];

    let total = unsafe { TOTAL_CIRCLES };
    for _ in 0..total {
        if let Some(circle) = create_and_draw_circle(&window, &draw, &circles) {
            circles.push(circle);
        }
    }

    // draw.ellipse().color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}

fn foo(window: &Rect, circles: &[Circle]) -> Option<Circle> {
    let (attempts, min_radius, line_width) =
        unsafe { (CREATE_CIRCLE_ATTEMPTS, MIN_RADIUS, LINE_WIDTH) };

    for _ in 0..attempts {
        let circle = Circle::new(min_radius as f32)
            .with_weight(line_width)
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

fn create_and_draw_circle(window: &Rect, draw: &Draw, circles: &[Circle]) -> Option<Circle> {
    let mut circle = foo(window, circles)?;

    let (min, max) = unsafe { (MIN_RADIUS, MAX_RADIUS) };

    for radius in min..max {
        circle = circle.with_radius((radius) as f32);
        if circle_has_collision(&circle, circles, window) {
            circle = circle.with_radius((radius - 1) as f32);

            break;
        }
    }

    circle.draw(draw);

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

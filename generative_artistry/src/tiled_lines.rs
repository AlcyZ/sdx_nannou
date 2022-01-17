use nannou::prelude::*;

static mut STEPS: i32 = 20;

pub fn present(size: u32, steps: i32) {
    unsafe {
        STEPS = steps;
    }

    nannou::app(model)
        .loop_mode(LoopMode::loop_ntimes(1))
        .update(update)
        .size(size, size)
        .run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .resizable(false)
        .view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();

    draw.background().color(WHITE);

    let width = window.top_right().x * 2.0;
    let height = window.top_right().y * 2.0;

    let steps = unsafe { STEPS };

    let mut x = 0;
    while x < width as i32 {
        let mut y = 0;
        while y < height as i32 {
            draw_line(x as f32, y as f32, steps as f32, steps as f32, app, &draw);
            y += steps;
        }
        x += steps;
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_line(x: f32, y: f32, width: f32, height: f32, app: &App, draw: &Draw) {
    let window = app.window_rect();

    let (start, end) = if random::<bool>() {
        let x_norm = window.top_left().x + x;
        let y_norm = window.top_left().y - y;

        let start = pt2(x_norm, y_norm);
        let end = pt2(x_norm + width, y_norm - height);

        (start, end)
    } else {
        let x_norm = window.bottom_left().x + x;
        let y_norm = window.bottom_left().y + y;

        let start = pt2(x_norm, y_norm);
        let end = pt2(x_norm + width, y_norm + height);

        (start, end)
    };

    draw.line()
        .start(start)
        .end(end)
        .weight(3.0)
        .color(BLACK);
}

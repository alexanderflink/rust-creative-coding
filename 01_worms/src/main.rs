use nannou::prelude::*;
use nannou::App;
use std::ops::AddAssign;
use std::ops::MulAssign;

const APP_LIFETIME_SECONDS: u64 = 10;
const NUM_BODIES: usize = 200;
const DEFAULT_DRAG: f32 = 0.95;
const DEFAULT_RADIUS: f32 = 1.0;
const DEFAULT_SPEED: f32 = 0.2;
const FADE_AMOUNT: f32 = 0.01;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Body {
    position: Point2,
    velocity: Vec2,
    drag: f32,
    radius: f32,
    color: Rgb,
}

struct Model {
    bodies: Vec<Body>,
    window_size: (u32, u32),
}

fn model(_app: &App) -> Model {
    let window_size = _app.main_window().inner_size_pixels();

    let mut bodies: Vec<Body> = Vec::new();

    for _ in 0..NUM_BODIES {
        let random_position = Point2::new(
            random_range(
                -1.0 * (window_size.0 / 5) as f32,
                (window_size.0 / 5) as f32,
            ),
            random_range(
                -1.0 * (window_size.1 / 5) as f32,
                (window_size.1 / 5) as f32,
            ),
        );

        bodies.push(Body {
            position: random_position,
            velocity: Vec2::new(0.0, 0.0),
            drag: DEFAULT_DRAG,
            radius: DEFAULT_RADIUS,
            color: rgb(0.1, 0.1, 0.1),
        });
    }

    Model {
        bodies,
        window_size,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.bodies.iter_mut().for_each(|body| {
        let random_direction = Vec2::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0));
        body.velocity.add_assign(random_direction * DEFAULT_SPEED);
        body.velocity.mul_assign(body.drag);
        body.position.add_assign(body.velocity);
    });

    if _app.duration.since_start.as_secs() >= APP_LIFETIME_SECONDS {
        _app.quit();
    }
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();

    if frame.nth() == 0 {
        draw.background().rgb(1.0, 1.0, 1.0);
    }
    _model.bodies.iter().for_each(|body| {
        draw.ellipse()
            .color(body.color)
            .radius(body.radius)
            .xy(body.position.clone());
    });
    draw.rect()
        .w_h(_model.window_size.0 as f32, _model.window_size.1 as f32)
        .rgba(1.0, 1.0, 1.0, FADE_AMOUNT);

    draw.to_frame(_app, &frame).unwrap();

    // capture frame
    _app.main_window()
        .capture_frame(format!("./render/frame{}.png", frame.nth()));
}

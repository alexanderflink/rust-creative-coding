use core::time::Duration;
use nannou::geom::path;
use nannou::prelude::*;

const APP_LIFETIME_SECONDS: f32 = 10.0;
const START_DISTANCE: f32 = 5.0;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    // paint frame white
    if frame.nth() == 0 {
        draw.background().rgb(1.0, 1.0, 1.0);
    }

    let grow_amount = 15.0;
    let time = app.time * 2.0;
    let sin = time.sin();
    let cos = time.cos();
    let distance = START_DISTANCE + (time * grow_amount);
    let tip_pos = Vec2::new(distance * sin, distance * cos);

    // draw tip position
    draw.ellipse()
        .radius(1.0)
        .rgba(0.0, 0.0, 0.0, 0.5)
        .xy(tip_pos);

    draw.line()
        .stroke_weight(1.0)
        .rgba(0.0, 0.0, 0.0, 0.5)
        .end(tip_pos);

    // let path = path()
    //     .line_to(tip_pos)
    //     .quadratic_bezier_to(tip_pos, tip_pos)
    //     .build();

    // draw.path()
    //     .stroke()
    //     // .weight(10.0)
    //     // .rgba(0.0, 0.0, 0.0, 1.0)
    //     .events(path.iter());

    draw.to_frame(app, &frame).unwrap();

    if app.time >= APP_LIFETIME_SECONDS {
        app.quit();
    }
}

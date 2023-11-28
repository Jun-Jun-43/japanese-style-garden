use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug)]
struct Model {
    left_upper_stones: Vec<Stone>,
    left_lower_stones: Vec<Stone>,
    right_upper_stones: Vec<Stone>,
    right_lower_stones: Vec<Stone>,
    random_n: RandomNumbers,
}

#[derive(Debug)]
struct Stone {
    point: Point2,
}

#[derive(Debug)]
struct RandomNumbers {
    left_upper: usize,
    left_lower: usize,
    right_upper: usize,
    right_lower: usize,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();

    let window = app.window_rect();
    let width = window.w() as usize;
    let height = window.h() as usize;

    let mut left_upper_stones = vec![];
    let mut left_lower_stones = vec![];
    let mut right_upper_stones = vec![];
    let mut right_lower_stones = vec![];

    let random_n_left_upper = random_range(3,100);
    let random_n_left_lower = random_range(3, 100);
    let random_n_right_upper = random_range(3, 100);
    let random_n_right_lower = random_range(3, 100);

    for x in 0..=width / 2 {
        for y in 0..=height / 2 {
            if x % 5 == 0 {
                if y % 5 == 0 {
                    let left_x = x as f32 - 400.0;
                    let left_upper_y = y as f32;
                    let left_lower_y = y as f32 - 300.0;

                    let right_x = x as f32;
                    let right_upper_y = y as f32;
                    let right_lower_y = y as f32 - 300.0;

                    let left_upper_stone = Stone {
                        point: pt2(left_x, left_upper_y),
                    };

                    let left_lower_stone = Stone {
                        point: pt2(left_x, left_lower_y),
                    };

                    let right_upper_stone = Stone {
                        point: pt2(right_x, right_upper_y),
                    };

                    let right_lower_stone = Stone {
                        point: pt2(right_x, right_lower_y),
                    };

                    left_upper_stones.push(left_upper_stone);
                    left_lower_stones.push(left_lower_stone);
                    right_upper_stones.push(right_upper_stone);
                    right_lower_stones.push(right_lower_stone);
                }
            }
        }
    }

    let random_n = RandomNumbers {
        left_upper: random_n_left_upper,
        left_lower: random_n_left_lower,
        right_upper: random_n_right_upper,
        right_lower: random_n_right_lower,
    };

    Model {
        left_upper_stones,
        left_lower_stones,
        right_upper_stones,
        right_lower_stones,
        random_n,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let time = app.duration.since_start.as_secs_f32();
    let frames = app.elapsed_frames() as usize;
    let contraction = time.sin().abs() * 0.5;

    draw.background().color(WHITE);

    for i in 0..model.left_upper_stones.len() {
        if i % (frames + model.random_n.left_upper) != 0 {
            draw.ellipse()
                .xy(model.left_upper_stones[i].point)
                .radius(2.5 + contraction)
                .hsla(
                    217.27 / 360.0,
                    0.6875 + (time.sin() * 0.25).abs(),
                    0.3765 - (time.sin() * 0.25).abs(),
                    1.0,
                );
        } else {
            draw.ellipse()
                .xy(model.left_upper_stones[i].point)
                .radius(2.5 + contraction)
                .hsla(
                    217.27 / 360.0,
                    0.6875 - (time.sin() * 0.25).abs(),
                    0.3765 + (time.sin() * 0.25).abs(),
                    1.0 - (time.sin() * 0.4).abs(),
                );
        }
    }

    for i in 0..model.left_lower_stones.len() {
        if i % (frames + model.random_n.left_lower) != 0 {
            draw.ellipse()
                .xy(model.left_lower_stones[i].point)
                .radius(2.5 + contraction)
                .hsla(
                    1.82 / 360.0,
                    0.7674 + (time.sin() * 0.25).abs(),
                    0.7471 - (time.sin() * 0.25).abs(),
                    1.0,
                );
        } else {
            draw.ellipse()
                .xy(model.left_lower_stones[i].point)
                .radius(2.5 + contraction)
                .hsla(
                    1.82 / 360.0,
                    0.7674 - (time.sin() * 0.25).abs(),
                    0.7471 - (time.sin() * 0.25).abs(),
                    1.0 - (time.sin() * 0.4).abs(),
                );
        }
    }

    for i in 0..model.right_upper_stones.len() {
        if i % (frames + model.random_n.right_upper) != 0 {
            draw.ellipse()
                .xy(model.right_upper_stones[i].point)
                .radius(2.5 + contraction)
                .hsla(343.81 / 360.0, 1.0, 0.4216 - (time.sin() * 0.25).abs(), 1.0);
        } else {
            draw.ellipse()
                .xy(model.right_upper_stones[i].point)
                .radius(2.5 + contraction)
                .hsla(
                    343.81 / 360.0,
                    1.0 - (time.sin() * 0.25).abs(),
                    0.4216 + (time.sin() * 0.25).abs(),
                    1.0 - (time.sin() * 0.4).abs(),
                );
        }
    }

    for i in 0..model.right_lower_stones.len() {
        if i % (frames + model.random_n.right_lower) != 0 {
            draw.ellipse()
                .xy(model.right_lower_stones[i].point)
                .radius(2.5 + contraction)
                .hsla(
                    268.29 / 360.0,
                    0.2966 + (time.sin() * 0.25).abs(),
                    0.4627 - (time.sin() * 0.25).abs(),
                    1.0,
                );
        } else {
            draw.ellipse()
                .xy(model.right_lower_stones[i].point)
                .radius(2.5 + contraction)
                .hsla(
                    268.29 / 360.0,
                    0.2966 - (time.sin() * 0.25).abs(),
                    0.4627 + (time.sin() * 0.25).abs(),
                    1.0 - (time.sin() * 0.4).abs(),
                );
        }
    }

    draw.to_frame(app, &frame).unwrap();

    if app.elapsed_frames().to_usize().unwrap() <= 200 {
        let file_path = capture_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {}

fn capture_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}

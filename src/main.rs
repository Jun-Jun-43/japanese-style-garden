use nannou::{
    color::Alpha, draw::properties::spatial::orientation, prelude::*, winit::dpi::Position,
};

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .update(update)
        .run();
}

#[derive(Debug)]
struct Model {
    stones: Vec<Stone>,
    bg_color: Alpha<Hsl, f32>,
    on_garden: Vec<usize>,
}

#[derive(Debug)]
struct Stone {
    point: Point2,
    width: f32,
    height: f32,
    color: Alpha<Hsl, f32>,
    step: usize,
    rotate: f32,
    alpha_max: bool,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();

    let alpha = 0.0;

    let hakuji = hsla(120.0 / 360.0, 0.2727, 0.9784, alpha);
    let usuzumi = hsla(60.0 / 360.0, 0.0054, 0.6373, alpha);
    let moegi = hsla(79.11 / 360.0, 0.7441, 0.4137, alpha);
    let karakurenai = hsla(347.18 / 360.0, 1.0, 0.4588, alpha);
    let mizu = hsla(193.8 / 360.0, 0.641, 0.6941, alpha);

    let colors = vec![usuzumi, moegi, karakurenai, mizu];

    let bg_color = hakuji;

    let mut stones = vec![];

    for _ in 0..(random_range(20, 30) as usize) {
        let stone_size = random_range(20, 120) as usize;
        let x = random_range(-400, 400) as f32;
        let y = random_range(-300, 300) as f32;
        let mut stone_x = 0.0 + x;
        let mut stone_y = 0.0 + y;

        if stone_x + stone_size as f32 > 400.0 {
            stone_x -= stone_size as f32;
        }
        if stone_y + stone_size as f32 > 300.0 {
            stone_y -= stone_size as f32;
        }

        let step = random_range(40, 60);
        // let step = random_range(6, 90);
        let rotate = 0.0;

        println!("{}, {}", x, y);
        println!("{}, {}", stone_size, stone_size);

        let stone = Stone {
            point: pt2(stone_x, stone_y),
            width: stone_size as f32,
            height: stone_size as f32,
            color: colors[random_range(0, colors.len())].clone(),
            step: step,
            rotate: rotate,
            alpha_max: false,
        };

        // println!("{:?}, {:?}", stone.point, (stone.width, stone.height));
        stones.push(stone);
    }

    let on_garden = vec![];

    Model { stones, bg_color, on_garden }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(model.bg_color);

    // println!("{}", model.stones.len());
    for i in 0..model.stones.len() {
        draw.polygon()
            .xy(model.stones[i].point)
            .rotate(model.stones[i].rotate)
            .points_colored((0..=360).step_by(model.stones[i].step).map(|n| {
                let radian = deg_to_rad(n as f32);
                let x = radian.sin() * model.stones[i].width;
                let y = radian.cos() * model.stones[i].height;
                (pt2(x, y), model.stones[i].color)
            }));
    }

    draw.to_frame(app, &frame).unwrap();

    if app.elapsed_frames().to_usize().unwrap() <= 600 {
        let file_path = capture_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // println!("{}", app.time.sin() * 0.001);
    // println!("{}", model.stones[0].rotate);
    let frames = app.elapsed_frames() as usize;
    let frequency = random_range(10, 20) as usize;

    if frames % frequency == 0 {
        let max = model.stones.len();
        let on_garden = random_range(0, max);

        let mut exist = false;
        for i in 0..model.on_garden.len() {
            if model.on_garden[i] == on_garden {
                exist = true;
            }
        }
        if exist == false {
            model.on_garden.push(on_garden);
        }
    }

    // println!("{:?}", model.on_garden);

    for i in 0..model.stones.len() {
        // model.stones[i].color.alpha += app.time.sin() * 0.008;
        model.stones[i].rotate -= deg_to_rad(0.2);
    }

    if !model.on_garden.is_empty() {
        for i in model.on_garden.iter() {
            if model.stones[*i].alpha_max {
                if model.stones[*i].color.alpha > 0.0 {
                    model.stones[*i].color.alpha -= 0.005;
                } else {
                    model.stones[*i].alpha_max = false;
                    model.stones[*i].point.x += (frames as f32).sin() * 20.0;
                    model.stones[*i].point.y += (frames as f32).cos() * 20.0;
                }
            } else {
                if model.stones[*i].color.alpha < 1.0 {
                    model.stones[*i].color.alpha += 0.02;
                } else {
                    model.stones[*i].alpha_max = true;
                }
            }
        }
    }
}

fn capture_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}

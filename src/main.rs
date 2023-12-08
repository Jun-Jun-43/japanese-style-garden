use derive_more::Add;
use nannou::{color::Alpha, prelude::*};

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
    color_count: ColorCount,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Stone {
    point: Point2,
    z: f32,
    width: f32,
    height: f32,
    color: Alpha<Hsl, f32>,
    color_number: usize,
    step: usize,
    rotate: f32,
    alpha_max: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Add)]
struct ColorCount {
    usuzumi: usize,
    moegi: usize,
    karakurenai: usize,
    mizu: usize,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();

    let alpha = 0.0;

    let unohana = hsla(60.0 / 360.0, 0.3846, 0.9745, alpha);
    let usuzumi = hsla(60.0 / 360.0, 0.0054, 0.6373, alpha);
    let moegi = hsla(79.11 / 360.0, 0.7441, 0.4137, alpha);
    let karakurenai = hsla(347.18 / 360.0, 1.0, 0.4588, alpha);
    let mizu = hsla(193.8 / 360.0, 0.641, 0.6941, alpha);

    let colors = vec![usuzumi, moegi, karakurenai, mizu];
    let bg_color = unohana;

    let quantity = random_range(20, 30) as usize;
    let mut stones: Vec<Stone> = vec![];

    for _ in 0..quantity {
        let color_number = random_range(0, 4) as usize;
        let mut stone = stone_init(&colors, color_number);

        if stones.is_empty() {
            stones.push(stone);
        } else {
            for exist_stone in stones.iter() {
                match exist_stone.color_number {
                    0 => {
                        if exist_stone.color_number == stone.color_number {
                            while (exist_stone.point.x - stone.point.x).abs() >= 200.0
                                || (exist_stone.point.y - stone.point.y).abs() >= 140.0
                            {
                                stone = stone_init(&colors, color_number);
                            }
                        }
                    }
                    1 => {
                        if exist_stone.color_number == stone.color_number {
                            while (exist_stone.point.x - stone.point.x).abs() >= 200.0
                                || (exist_stone.point.y - stone.point.y).abs() >= 140.0
                            {
                                stone = stone_init(&colors, color_number);
                            }
                        }
                    }
                    3 => {
                        if exist_stone.color_number == stone.color_number {
                            while (exist_stone.point.x - stone.point.x).abs() >= 200.0
                                || (exist_stone.point.y - stone.point.y).abs() >= 140.0
                            {
                                stone = stone_init(&colors, color_number);
                            }
                        }
                    }
                    _ => {}
                }
            }
            stones.push(stone);
        }
    }
    println!("{}", stones.len());

    let on_garden = vec![];
    let mut usuzumi_count = 0;
    let mut moegi_count = 0;
    let mut karakurenai_count = 0;
    let mut mizu_count = 0;
    for i in 0..stones.len() {
        match stones[i].color_number {
            0 => usuzumi_count += 1,
            1 => moegi_count += 1,
            2 => karakurenai_count += 1,
            3 => mizu_count += 1,
            _ => {}
        }
    }

    let color_count = ColorCount {
        usuzumi: usuzumi_count,
        moegi: moegi_count,
        karakurenai: karakurenai_count,
        mizu: mizu_count,
    };

    Model {
        stones,
        bg_color,
        on_garden,
        color_count,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(model.bg_color);

    for i in 0..model.stones.len() {
        match model.stones[i].color_number {
            2 => {
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
            _ => {
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
        }
    }

    draw.to_frame(app, &frame).unwrap();

    if app.elapsed_frames().to_usize().unwrap() <= 1000 {
        let file_path = capture_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let frames = app.elapsed_frames() as usize;
    let frequency = random_range(5, 10) as usize;
    let mut usuzumi_count = 0;
    let mut moegi_count = 0;
    let mut mizu_count = 0;
    for i in model.on_garden.iter() {
        match model.stones[*i].color_number {
            0 => usuzumi_count += 1,
            1 => moegi_count += 1,
            3 => mizu_count += 1,
            _ => {}
        }
    }

    println!(
        "{}",
        frames
    );
    model.bg_color.saturation += app.time.sin() * 0.02;

    if frames % frequency == 0 || frames % frequency == 5 || frames % frequency == 10 {
        let max = model.stones.len();
        let mut on_garden = random_range(0, max);


        if moegi_count == model.color_count.moegi
            && mizu_count == model.color_count.mizu
            && usuzumi_count == model.color_count.usuzumi
        {
            if model.stones[on_garden].color_number != 1
                && model.stones[on_garden].color_number != 3
            {
                let mut exist = false;
                for i in 0..model.on_garden.len() {
                    if model.on_garden[i] == on_garden {
                        exist = true;
                    }
                }
                if exist == false {
                    model.on_garden.push(on_garden);
                }
            } else {
                while model.stones[on_garden].color_number == 1
                    || model.stones[on_garden].color_number == 3
                {
                    on_garden = random_range(0, max);
                }

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
        } else {
            if model.stones[on_garden].color_number == 0
                || model.stones[on_garden].color_number == 1
                || model.stones[on_garden].color_number == 3
            {
                let mut exist = false;
                for i in 0..model.on_garden.len() {
                    if model.on_garden[i] == on_garden {
                        exist = true;
                    }
                }
                if exist == false {
                    model.on_garden.push(on_garden);
                }
            } else {
                while model.stones[on_garden].color_number == 2 {
                    on_garden = random_range(0, max);
                }

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
        }
    }

    for i in 0..model.stones.len() {
        if model.stones[i].color_number == 2 {
            model.stones[i].rotate -= deg_to_rad(0.2);
        }
    }

    if !model.on_garden.is_empty() {
        for i in model.on_garden.iter() {
            if model.stones[*i].alpha_max {
                if model.stones[*i].color.alpha > 0.0 {
                    match model.stones[*i].color_number {
                        2 => {
                            model.stones[*i].color.alpha -= 0.005;
                            model.stones[*i].point.y -= (frames as f32).sin().abs();
                        }
                        // _ => model.stones[*i].color.alpha -= 0.005,
                        _ => model.stones[*i].color.alpha -= 0.001,
                    }
                    // model.stones[*i].color.alpha -= 0.002;
                } else {
                    match model.stones[*i].color_number {
                        2 => {
                            model.stones[*i].point.x = random_range(-400, 400) as f32;
                            model.stones[*i].point.y = random_range(-300, 300) as f32;
                        }
                        _ => {},
                    }
                    model.stones[*i].alpha_max = false;
                }
            } else {
                match model.stones[*i].color_number {
                    2 => {
                        if model.stones[*i].color.alpha < 1.0 {
                            model.stones[*i].color.alpha += 0.01;
                            // model.stones[*i].color.alpha += 0.02;
                        } else {
                            model.stones[*i].alpha_max = true;
                        }
                    }
                    _ => {
                        if model.stones[*i].color.alpha < 0.8 {
                            model.stones[*i].color.alpha += 0.005;
                        }
                        // model.stones[*i].color.alpha += 0.02;
                        else {
                            model.stones[*i].alpha_max = true;
                        }
                    }
                }
            }
        }
    }
}

fn stone_init(colors: &Vec<Alpha<Hsl, f32>>, color_number: usize) -> Stone {
    let stone_size = match color_number {
        0 => random_range(80.0, 100.0),
        1 => random_range(100.0, 120.0),
        2 => random_range(30.0, 40.0),
        3 => random_range(60.0, 80.0),
        _ => 0.0,
    };
    // let stone_size = random_range(20, 120) as usize;
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

    let z = 0.0;
    let step = random_range(40, 60);
    let rotate = 0.0;

    let stone = Stone {
        point: pt2(stone_x, stone_y),
        z: z,
        width: stone_size as f32,
        height: stone_size as f32,
        color: colors[color_number].clone(),
        color_number: color_number,
        step: step,
        rotate: rotate,
        alpha_max: false,
    };

    stone
}

fn capture_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project path`")
        .join("frames")
        .join(format!("{:04}", frame.nth()))
        .with_extension("png")
}

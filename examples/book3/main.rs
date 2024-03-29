use clap::{App, Arg, ArgMatches};
use image;
use raytracer::{
    camera::Camera,
    math::{Color, Point3, Vec3},
};

mod scenes;
use scenes::*;

fn main() {
    let matches = match_args();

    // Parse camera, image args
    let image_height: usize = matches
        .value_of("height")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let image_width: usize = matches.value_of("width").unwrap().parse().unwrap();
    let aspect_ratio = image_width as f64 / image_height as f64;
    let samples_per_pixel: u64 = matches.value_of("samples").unwrap().parse().unwrap();
    let max_depth: u64 = matches.value_of("bounces").unwrap().parse().unwrap();
    let fov: f64 = matches.value_of("fov").unwrap().parse().unwrap();
    let aperture: f64 = matches.value_of("aperture").unwrap().parse().unwrap();
    let focus_distance: f64 = matches.value_of("focusdist").unwrap().parse().unwrap();

    // Parse filename
    let filename = matches.value_of("output").unwrap();

    // Parse scene
    let scene_num: i32 = matches.value_of("scene").unwrap().parse().unwrap();

    // Look directions for camera
    let look_from;
    let look_at;
    let v_up = Vec3::new(0.0, 1.0, 0.0);

    // Pick scene
    let scene_fn = match scene_num {
        1 => {
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            |camera| cornell_box(camera, Color::new(0.0, 0.0, 0.0))
        }
        _ => panic!("Unrecognized scene number - expected in range [1,1]"),
    };

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        fov,
        aspect_ratio,
        aperture,
        focus_distance,
        0.0,
        1.0,
    );

    println!(
        "Rendering scene to {}x{} image ({} pixels) with {} bounces/ray and {} samples/pixel",
        image_width,
        image_height,
        image_width * image_height,
        max_depth,
        samples_per_pixel,
    );

    // Render
    let scene = scene_fn(camera);
    let image = scene.render(image_width, image_height, samples_per_pixel, max_depth);
    image::save_buffer(
        filename,
        &image,
        image_width as u32,
        image_height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();
}

fn match_args() -> ArgMatches<'static> {
    App::new("Raytracer")
        .version("0.1.0")
        .about("Renders a raytraced scene")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .takes_value(true)
                .required(true)
                .help("Sets image width of output"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .takes_value(true)
                .required(true)
                .help("Sets image height of output"),
        )
        .arg(
            Arg::with_name("fov")
                .short("f")
                .long("fov")
                .value_name("FOV")
                .takes_value(true)
                .default_value("20.0")
                .help("Sets field of vision (fov)"),
        )
        .arg(
            Arg::with_name("aperture")
                .short("a")
                .long("aperture")
                .value_name("APERTURE")
                .takes_value(true)
                .default_value("0")
                .help("Sets diameter of camera aperture (controls amount of defocus blur)"),
        )
        .arg(
            Arg::with_name("focusdist")
                .short("d")
                .long("focus-dist")
                .value_name("FOCUS")
                .takes_value(true)
                .default_value("10.0")
                .help("Sets distance to the focus plane (controls distance of defocus blur)"),
        )
        .arg(
            Arg::with_name("samples")
                .short("s")
                .long("samples")
                .value_name("SAMPLES")
                .takes_value(true)
                .default_value("500")
                .help("Sets number of samples per pixel"),
        )
        .arg(
            Arg::with_name("bounces")
                .short("b")
                .long("bounces")
                .value_name("BOUNCES")
                .takes_value(true)
                .default_value("50")
                .help("Sets max bounces (depth) for each raycast"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .default_value("render.ppm")
                .help("File to save rendered image"),
        )
        .arg(
            Arg::with_name("scene")
                .long("scene")
                .value_name("SCENE")
                .takes_value(true)
                .default_value("1")
                .help("Scene number to render (default random scene)"),
        )
        .get_matches()
}

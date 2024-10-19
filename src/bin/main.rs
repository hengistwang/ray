use indicatif::ProgressBar;
use ndarray::{Array3, ArrayView3};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;
const COLOR_DEPTH: f32 = 255.999;

fn write_image(image: ArrayView3<f32>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("/tmp/image.ppm")?;
    let mut buffer = BufWriter::new(file);
    let pb = ProgressBar::new(IMAGE_HEIGHT as u64);
    write!(buffer, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for j in 0..IMAGE_HEIGHT {
        pb.inc(1);
        for i in 0..IMAGE_WIDTH {
            let ir = (COLOR_DEPTH * image[[i, j, 0]]) as i32;
            let ig = (COLOR_DEPTH * image[[i, j, 1]]) as i32;
            let ib = (COLOR_DEPTH * image[[i, j, 2]]) as i32;
            writeln!(buffer, "{} {} {}", ir, ig, ib)?;
        }
    }
    pb.finish_with_message("done");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = Array3::<f32>::zeros((IMAGE_WIDTH, IMAGE_HEIGHT, 3));
    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.0;
            image[[i, j, 0]] = r;
            image[[i, j, 1]] = g;
            image[[i, j, 2]] = b;
        }
    }
    write_image(image.view())?;
    Ok(())
}

use rand::{thread_rng, Rng};

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut rng = thread_rng();
    for _ in 0..image_height {
        for _ in 0..image_width {
            let r = rng.gen_range(0..=256);
            let g = rng.gen_range(0..=256);
            let b = rng.gen_range(0..=256);
            println!("{} {} {}", r, g, b);
        }
    }
}

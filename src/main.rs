fn main() {
    let image_width = 1024;
    let image_height = 1024;
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in 0..image_height {
        for j in (0..image_width).rev() {
            let r = i / 4;
            let g = j / 4;
            let b = 0;
            println!("{} {} {}", r, g, b);
        }
    }
}

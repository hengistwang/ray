use core::fmt;

fn main() {
    let image_width = 1024;
    let image_height = 1024;
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for i in 0..image_height {
        eprintln!(
            "Progress: {:.2}%",
            ((i as f64 / image_height as f64) * 100.0)
        );
        for j in (0..image_width).rev() {
            let r = i / 4;
            let g = j / 4;
            let b = 0;
            println!("{} {} {}", r, g, b);
        }
    }
    eprintln!("Progress: {}%", 100);
}

struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    fn delv(&mut self, r: &Vec3) {
        self.x -= r.x;
        self.y -= r.y;
        self.z -= r.z;
    }
    fn addv(&mut self, r: &Vec3) {
        self.x += r.x;
        self.y += r.y;
        self.z += r.z;
    }
    fn mulv(&mut self, r: &Vec3) {
        self.x *= r.x;
        self.y *= r.y;
        self.z *= r.z;
    }
    fn mulf(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
    fn divf(&mut self, t: f64) {
        self.mulf(1.0 / t);
    }
    fn neg(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn dot(&self, r: &Vec3) -> f64 {
        self.x * r.x + self.y * r.y + self.z * r.z
    }
    fn cross(self, r: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * r.z - self.z - r.y,
            y: self.z * r.x - self.x * r.z,
            z: self.x * r.y - self.y * r.x,
        }
    }
    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    fn unit_vector(&mut self) {
        self.divf(self.length())
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

//fn write_color()

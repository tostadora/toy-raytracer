mod space;
mod color;
mod ray;

use std::io::Write;
use space::Vec3;
use color::Color;

fn main() {
    let img_width: usize = 256;
    let img_height: usize = 256;

    println!("P3\n{} {}\n255\n", img_width, img_height);

    let _v = Vec3::origin();

    for j in 0..img_height {
        eprint!("\rScanlines remaining: {} ", img_height - j);
        for i in 0..img_width {
            let color = Color::new(i as f64 / (img_width-1) as f64 * 255.0,
                                   j as f64 / (img_height-1) as f64 * 255.0,
                                   0.0);
            color.write_color();
        }
    }
    eprintln!("\rDone");
    let _ = std::io::stderr().flush();
}

mod vec3;
mod color;

use color::Color;

fn main() {

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let color = Color::new(i as f64 / (image_width - 1) as f64,
                                    j as f64 / (image_width - 1) as f64,
                                    0.0);
            color.write_color();
        }
    }

    eprintln!("Done");
}

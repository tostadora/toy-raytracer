mod vec3;

fn main() {

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_width - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("Done");
}

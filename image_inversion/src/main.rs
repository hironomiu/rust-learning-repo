use image::{GenericImage, GenericImageView};
use std::env::args;
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("[Usage] inputfile");
        return;
    }

    let infile = args[1].clone();
    let outfile = format!("{}-out.jpg", infile);
    let mut img = image::open(infile).expect("input error");
    let (w, h) = img.dimensions();

    for y in 0..h {
        for x in 0..w {
            let c = img.get_pixel(x, y);
            let c = image::Rgba([255 - c[0], 255 - c[1], 255 - c[2], c[3]]);
            img.put_pixel(x, y, c);
        }
    }
    img.save(outfile).expect("outfile error");
}

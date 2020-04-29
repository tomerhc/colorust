extern crate image;
mod img_ops;
mod mcq;
use std::{env, process};
use image::imageops::FilterType;

//use std::path::Path;
//use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usege: colorust <file>");
        process::exit(1);
    }

    let path = &args[1];
    let img = image::open(path).unwrap();
    let rs_img = img.into_rgb();
    //    let rs_img = img.resize(1422, 800, FilterType::Nearest).into_rgb();
    
    let raw: Vec<&image::Rgb<u8>> = rs_img.pixels().collect();
    let palette: Vec<image::Rgb<u8>> = mcq::mcq(raw, 8.0);
    let printer: Vec<(u8,u8,u8)> = palette.iter().map(|a| (a[0],a[1],a[2])).collect();
    println!("{:?}", printer);
}

extern crate image;
use std::{env, process};
use std::collections::HashMap;
use image::GenericImageView;
use image::imageops::FilterType;
//use std::path::Path;
//use std::fs::File;


fn find_mode(v: &mut [image::Rgb<u8>], len_res: usize) -> Vec<(image::Rgb<u8>, i32)> {
    //    let comb = combine(v, 70);
    let mut hash: HashMap<image::Rgb<u8>, i32> = HashMap::new();
    for pix in v.iter(){
        let count = hash.entry(*pix).or_insert(0);
        *count += 1;
    }
    let mut sorted_vec: Vec<(image::Rgb<u8>,i32)> = hash.into_iter().collect();
    sorted_vec.sort_by(|a,b| b.1.cmp(&a.1));
    if sorted_vec.len() > len_res{
        return sorted_vec[..len_res].to_vec();
    }
    sorted_vec.to_vec()
}

fn find_closest(v: &[image::Rgb<u8>], pix: &image::Rgb<u8>) -> (image::Rgb<u8>, f64) {
    let mut closest: (image::Rgb<u8>, f64) = (image::Rgb([0,0,0]), 9999.0);
    for p in v{
        let dist = ((p[0] as f64 -pix[0] as f64).powi(2) + (p[1] as f64 -pix[0] as f64).powi(2) + (p[2] as f64 -pix[0] as f64 ).powi(2)).sqrt();
        if dist < closest.1 {
            closest.0 = *p;
            closest.1 = dist;
        }
    }
    closest
}

fn combine(v: &mut [&image:: Rgb<u8>], threshold: i32) -> Vec<image::Rgb<u8>> {
    let mut res: Vec<image::Rgb<u8>> = vec![*v[0]];
    for pix in v[1..].iter(){
        let (_, dist) = find_closest(&res[..], pix);
        if dist as i32 > threshold{
            res.push(**pix);
        }
    }
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usege: colorust <file>");
        process::exit(1);
    }

    let path = &args[1];
    let img = image::open(path).unwrap();
    let rs_img = img.resize(500,500, FilterType::Nearest).into_rgb();
    let mut final_res: Vec<(image::Rgb<u8>, i32)> = Vec::new();
    for row in rs_img.rows(){
        let mut mode = row.collect::<Vec<&image::Rgb<u8>>>();
        let mut r = combine(&mut mode, 65);
        final_res.append(&mut find_mode(&mut r[..], 8))
    }


    let mut dict: HashMap<image::Rgb<u8>, i32> = HashMap::new();
    for (rgb, count) in final_res.iter(){
        let c = dict.entry(*rgb).or_insert(0);
        *c += count;
    }

    let mut final_res: Vec<(image::Rgb<u8>, i32)> = dict.into_iter().collect();
    final_res.sort_by(|a,b| b.1.cmp(&a.1));
    final_res.truncate(12);
    let f: Vec<(u8,u8,u8)> = final_res.iter().map(|(rgb, _)| (rgb[0], rgb[1], rgb[2])).collect();
    println!("{:?}", f);
    //let raw: Vec<u8> = rs_img.into_raw();
    //println!("{:?}", &raw[3..6]);


}

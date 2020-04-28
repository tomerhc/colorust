extern crate image;
use std::{env, process};
use std::cmp;
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
        let dist: f64;

        if (p[0] as f64 + pix[0] as f64) / 2.0 > 128.0 {
            dist = ((p[0] as f64 -pix[0] as f64).powi(2) * 2.0 + (p[1] as f64 -pix[0] as f64).powi(2) * 4.0 + (p[2] as f64 -pix[0] as f64 ).powi(2) * 3.0).sqrt();
        } else {
            dist = ((p[0] as f64 -pix[0] as f64).powi(2) * 3.0 + (p[1] as f64 -pix[0] as f64).powi(2) * 4.0 + (p[2] as f64 -pix[0] as f64 ).powi(2) * 2.0).sqrt();
        }
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

fn calc_sat(pix: &image::Rgb<u8>) -> f64 {
    let image::Rgb([pr,pg,pb]) = pix;
    let r = *pr as f64 / 256.0;
    let g = *pg as f64 / 256.0;
    let b = *pb as f64 / 256.0;
    
    let lum = 0.5 * ( (r.max(g)).max(b) + (r.min(g)).min(b) );
    if lum < 1.0 {
        return ( (r.max(g)).max(b) - (r.min(g)).min(b)  ) / (1.0 - (2.0*lum - 1.0).abs()) 
    } else {
        return 0.0
    }
}


fn find_distinct(v: &mut [image::Rgb<u8>], avg_col: &image::Rgb<u8>, num: usize) -> Vec<image::Rgb<u8>> {
    let mut res: HashMap<image::Rgb<u8>, f64> = HashMap::new();
    for pix in v.iter(){
        match res.get(pix){
            Some(_) => (),
            None => {
                if (pix[0] as f64 + avg_col[0] as f64) / 2.0 > 128.0 {
                    let dist = ((pix[0] as f64 -avg_col[0] as f64).powi(2) * 2.0 + (pix[1] as f64 -avg_col[0] as f64).powi(2) * 4.0 + (pix[2] as f64 -avg_col[0] as f64 ).powi(2) * 3.0).sqrt();
                    res.insert(*pix, dist*calc_sat(pix).powi(4));
                } else {
                    let dist = ((pix[0] as f64 -avg_col[0] as f64).powi(2) * 3.0 + (pix[1] as f64 -avg_col[0] as f64).powi(2) * 4.0 + (pix[2] as f64 -avg_col[0] as f64 ).powi(2) * 2.0).sqrt();
                    res.insert(*pix, dist*calc_sat(pix).powi(4));
                }
            }
        }
    }
    let mut sorted_vec: Vec<(image::Rgb<u8>, f64)> = res.into_iter().collect();
    sorted_vec.sort_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
    let mut ret: Vec<image::Rgb<u8>> = sorted_vec.into_iter().map(|a| a.0).collect();
    ret.truncate(num);
    ret
}

fn find_avg(v: &mut [image::Rgb<u8>]) -> image::Rgb<u8>{
    let r: Vec<i32> = v.iter().map(|a| a[0] as i32).collect();
    let g: Vec<i32> = v.iter().map(|a| a[1] as i32).collect();
    let b: Vec<i32> = v.iter().map(|a| a[2] as i32).collect();
   
    let avg_r = r.iter().sum::<i32>() as f32 / r.len() as f32;
    let avg_g = g.iter().sum::<i32>() as f32 / g.len() as f32;
    let avg_b = b.iter().sum::<i32>() as f32 / b.len() as f32;
    image::Rgb([avg_r as u8, avg_g as u8, avg_b as u8])
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
    let mut avgs: Vec<image::Rgb<u8>> = Vec::new();
    for row in rs_img.rows(){
        let mut r = row.collect::<Vec<&image::Rgb<u8>>>();
        let mut comb = combine(&mut r, 65);
        avgs.push(find_avg(&mut comb[..]));
        final_res.append(&mut find_mode(&mut comb[..], 8));
    }

    let mut distincts: Vec<image::Rgb<u8>> = Vec::new(); 
    let total_avg = find_avg(&mut avgs[..]);
    for row in rs_img.rows(){
        let mut r = row.collect::<Vec<&image::Rgb<u8>>>();
        let mut comb = combine(&mut r, 50);
        distincts.append(&mut find_distinct(&mut comb[..], &total_avg, 5));
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
//    println!("{:?}", f);


}

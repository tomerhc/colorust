extern crate image;
use crate::img_ops;

pub fn mcq(v: Vec<&image::Rgb<u8>>, q: f32) -> Vec<image::Rgb<u8>> {
    assert!((q as u32).is_power_of_two());
    let iterations = q.log2() as i32;

    let mut buckets: Vec<Vec<&image::Rgb<u8>>> = vec![v];
    for _ in 0..iterations{
        for i in (0..buckets.len()){//.step_by(2) {
            let item = buckets.remove(i);
            if item.len() > 0 {
                let (v1,v2) = bucket_split(item);
                buckets.insert(i,v1);
                buckets.insert(i,v2);
            }
        }
    }
    
    //let res: Vec<image::Rgb<u8>> = buckets.into_iter().map(|mut a| img_ops::find_avg(&mut a[..])).collect();
    let res: Vec<image::Rgb<u8>> = buckets.into_iter().map(|mut a| get_sat(&mut a[..])).collect();
    res
}


fn bucket_split<'a>(mut v: Vec<&'a image::Rgb<u8>>) -> (Vec<&'a image::Rgb<u8>>,Vec<&'a image::Rgb<u8>>){
   let channel = largest_range(&v);
   v.sort_by(|a,b| a[channel].cmp(&b[channel]));
   //v.dedup();
   //let v2 = v.split_off(cut_index(&v,channel));
   let v2 = v.split_off(v.len() / 2);
   (v.to_vec(),v2.to_vec())
}


fn cut_index(v: &Vec<&image::Rgb<u8>>, channel: usize) -> usize {
    let col_vec: Vec<f32> = v.iter().map(|&a| a[channel] as f32).collect();
    let avg = col_vec.iter().sum::<f32>() / col_vec.len() as f32;
    for (index, i) in col_vec.iter().enumerate(){
        if i > &avg {
            return index;
        }
    }
    0
}


fn largest_range(v: &Vec<&image::Rgb<u8>>) -> usize {
    
    let r: Vec<u8> = v.iter().map(|&a| a[0]).collect();
    let g: Vec<u8> = v.iter().map(|&a| a[1]).collect();
    let b: Vec<u8> = v.iter().map(|&a| a[2]).collect();
    
    let range_r = r.iter().max().unwrap() - r.iter().min().unwrap();
    let range_g = g.iter().max().unwrap() - g.iter().min().unwrap();
    let range_b = b.iter().max().unwrap() - b.iter().min().unwrap();
    
    if range_r > range_g && range_r > range_b {
        return 0;
    } else if range_g > range_r && range_g > range_b {
        return 1;
    } else {
        return 2;
    }
}

fn get_sat(v: &mut [&image::Rgb<u8>]) -> image::Rgb<u8> {
    let mut sat_vec: Vec<(image::Rgb<u8>, f64)> = v.into_iter().map(|&mut a| (*a, img_ops::calc_sat(a))).collect();
    for i in sat_vec.iter(){
        match 2.0.partial_cmp(&i.1) {
            Some(_) => (),
            None => println!("{:?}", i)
        }
    }
    //sat_vec.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());
    sat_vec.last().unwrap().0
}

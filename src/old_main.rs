    let mut final_res: Vec<(image::Rgb<u8>, i32)> = Vec::new();
    let mut avgs: Vec<image::Rgb<u8>> = Vec::new();
    for row in rs_img.rows(){
        let mut r = row.collect::<Vec<&image::Rgb<u8>>>();
        let mut comb = img_ops::combine(&mut r, 65);
        avgs.push(img_ops::find_avg(&mut comb[..]));
        final_res.append(&mut img_ops::find_mode(&mut comb[..], 8));
    }

    let mut distincts: Vec<image::Rgb<u8>> = Vec::new(); 
    let total_avg = img_ops::find_avg(&mut avgs[..]);
    for row in rs_img.rows(){
        let mut r = row.collect::<Vec<&image::Rgb<u8>>>();
        let mut comb = img_ops::combine(&mut r, 50);
        distincts.append(&mut img_ops::find_distinct(&mut comb[..], &total_avg, 5));
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


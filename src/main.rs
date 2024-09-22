use image::{Rgb, RgbImage};
use std::env::args;
use std::io::{stdout, Write};
fn main() {
    let mut args = args().collect::<Vec<String>>();
    if args.len() <= 5
    {
        args.push(String::new());
        args.push(String::new());
        args.push(String::new());
        args.push(String::new());
        args.push(String::new());
    }
    let mut multi_face = Vec::new();
    let mut faces = 3;
    if let Ok(num) = args[1].clone().parse::<usize>() {
        faces = num;
    } else {
        multi_face = args[1].split(',').map(|a| a.parse::<usize>().unwrap_or(3)).collect::<Vec<usize>>();
    }
    let dice = args[2].clone().parse::<usize>().unwrap_or(3);
    let mut do_sum = false;
    let mut file = String::new();
    if let Ok(num) = args[3].clone().parse::<u8>()
    {
        do_sum = num == 1
    } else {
        file = args[3].clone()
    }
    let mut modulo = 0;
    if let Ok(num) = args[4].clone().parse::<usize>()
    {
        modulo = num
    }
    let mut signs = false;
    if let Ok(num) = args[5].clone().parse::<usize>()
    {
        signs = num == 1
    }
    //let faces = vec![faces; dice];
    if do_sum
    {
        for i in 2..=faces
        {
            print_dice(i, i.pow(dice as u32), do_sum, file.clone(), multi_face.clone(), modulo, signs)
        }
    } else {
        print_dice(faces, dice, do_sum, file, multi_face, modulo, signs)
    }
}

fn print_dice(mut faces: usize, dice: usize, do_sum: bool, file: String, multi_face: Vec<usize>, modulo: usize, signs: bool)
{
    let file_exists = !file.is_empty();
    let mut all_nums = vec![vec![1]];
    if !do_sum && !file_exists
    {
        println!("{:?}", vec![1]);
    }
    if dice == 1 && !do_sum
    {
        println!("{:?}", vec![
            1;
            faces
        ])
    } else {
        let multi = !multi_face.is_empty();
        if multi {
            faces = multi_face[0]
        }
        let mut last = vec![1; faces];
        if signs
        {
            for (i,n) in last.iter_mut().enumerate()
            {
                if i%2 == 1
                {
                    *n=if modulo == 0 {faces}else{modulo} as isize-1;
                }
            }
        }
        all_nums.push(last.clone());
        let mut current: Vec<isize> = last.clone();
        for i in 1..dice
        {
            if multi {
                faces = multi_face[i % multi_face.len()]
            }
            if do_sum
            {
                if is_power_of(i, faces)
                {
                    print!("{},\t", current.iter().sum::<isize>());
                    stdout().flush().unwrap();
                }
            } else if !file_exists {
                println!("{:?}", current);
            }
            current = Vec::new();
            for p in 0..=if multi {multi_face.iter().sum::<usize>() * (i/multi_face.len()) + multi_face[0..=i % multi_face.len()].iter().sum::<usize>()} else{faces * (i + 1)} - i - 1
            {
                let start = if (p + 1) > faces
                {
                    p + 1 - faces
                } else {
                    0
                };
                let value = last[start
                    ..=p.min(if multi {multi_face.iter().sum::<usize>() * (i/multi_face.len()) + multi_face[0..i % multi_face.len()].iter().sum::<usize>()} else {faces * i} - i)]
                    .iter().enumerate().map(|(j,a)| if signs && if (p + 1) <= faces && (faces-(p+1))%2 ==1 {j % 2 == 0} else{ j % 2 == 1} {-a}else{*a} )
                    .sum::<isize>();
                let mut n = value % if modulo == 0 {faces}else{modulo} as isize;
                if signs && n.is_negative()
                {
                    n += if modulo == 0 {faces}else{modulo} as isize;
                }
                current.push(n)
            }
            last.clone_from(&current);
            all_nums.push(last.clone());
        }
        if do_sum
        {
            println!("{}", current.iter().sum::<isize>());
        } else if !file_exists {
            println!("{:?}", current);
        } else {
            let width = all_nums[all_nums.len() - 1].len() as u32;
            let h = all_nums.len() as u32;
            let mut img = RgbImage::new(width, h);
            for x in 0..width
            {
                for y in 0..h
                {
                    if multi {
                        faces = multi_face[y as usize % multi_face.len()]
                    }
                    if x < all_nums[y as usize].len() as u32
                    {
                        let n = all_nums[y as usize][x as usize];
                        let no_zero = true;
                        if n != 0 || !no_zero
                        {
                            let mut v = hsv2rgb(6.0 * n as f64 / if modulo == 0 {faces}else{modulo} as f64, 1.0, 1.0);
                            if no_zero {
                                v = hsv2rgb(6.0 * (n - 1) as f64 / (if modulo == 0 {faces}else{modulo} - 1) as f64, 1.0, 1.0)
                            }
                            if (faces == 2 || !no_zero) && !multi
                            {
                                img.put_pixel(x, y, Rgb(v))
                            } else {
                                img.put_pixel((width - all_nums[y as usize].len() as u32) / 2 + x, y, Rgb(v))
                            }
                        }
                    }
                }
            }
            img.save(file).unwrap();
        }
    }
}
fn is_power_of(mut n: usize, base: usize) -> bool {
    while n > 1 {
        if n % base != 0 {
            return false;
        }
        n /= base;
    }
    true
}
fn hsv2rgb(hue: f64, sat: f64, val: f64) -> [u8; 3]
{
    if sat == 0.0
    {
        return rgb2val(val, val, val);
    }
    let i = hue.floor();
    let f = hue - i;
    let p = val * (1.0 - sat);
    let q = val * (1.0 - sat * f);
    let t = val * (1.0 - sat * (1.0 - f));
    match i as usize % 6
    {
        0 => rgb2val(val, t, p),
        1 => rgb2val(q, val, p),
        2 => rgb2val(p, val, t),
        3 => rgb2val(p, q, val),
        4 => rgb2val(t, p, val),
        _ => rgb2val(val, p, q),
    }
}
fn rgb2val(r: f64, g: f64, b: f64) -> [u8; 3]
{
    [(255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8]
}
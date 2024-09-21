use image::{Rgb, RgbImage};
use std::env::args;
use std::io::{stdout, Write};
fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 4
    {
        return
    }
    let faces = args[1].clone().parse::<usize>().unwrap();
    let dice = args[2].clone().parse::<usize>().unwrap();
    let mut do_sum = false;
    let mut file = String::new();
    if let Ok(num) = args[3].clone().parse::<u8>()
    {
        do_sum = num == 1
    } else {
        file = args[3].clone()
    }
    //let faces = vec![faces; dice];
    if do_sum
    {
        for i in 2..=faces
        {
            print_dice(i, i.pow(dice as u32), do_sum, file.clone())
        }
    } else {
        print_dice(faces, dice, do_sum, file)
    }
}

fn print_dice(faces: usize, dice: usize, do_sum: bool, file: String)
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
        let mut last = vec![1; faces];
        all_nums.push(last.clone());
        let mut current = last.clone();
        for i in 1..dice
        {
            if do_sum
            {
                if is_power_of(i, faces)
                {
                    print!("{},\t", current.iter().sum::<usize>());
                    stdout().flush().unwrap();
                }
            } else if !file_exists {
                println!("{:?}", current);
            }
            current = Vec::new();
            for p in 0..=faces * (i + 1) - i - 1
            {
                let value = last[if (p + 1) > faces
                {
                    p + 1 - faces
                } else {
                    0
                }
                    ..=p.min(faces * i - i)]
                    .iter()
                    .sum::<usize>();
                current.push(value % faces)
            }
            last.clone_from(&current);
            all_nums.push(last.clone());
        }
        if do_sum
        {
            println!("{}", current.iter().sum::<usize>());
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
                    if x < all_nums[y as usize].len() as u32
                    {
                        let n = all_nums[y as usize][x as usize];
                        let no_zero = true;
                        if n != 0 || !no_zero
                        {
                            let mut v = hsv2rgb(6.0 * n as f64 / faces as f64, 1.0, 1.0);
                            if no_zero {
                                v = hsv2rgb(6.0 * (n - 1) as f64 / (faces - 1) as f64, 1.0, 1.0)
                            }
                            if faces == 2 || !no_zero
                            {
                                img.put_pixel(x, y, Rgb(v))
                            } else {
                                img.put_pixel((width - y * (faces as u32 - 1)) / 2 + x, y, Rgb(v))
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
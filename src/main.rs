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
    let do_sum = args[3].clone().parse::<u8>().unwrap() == 1;
    //let faces = vec![faces; dice];
    if do_sum
    {
        for i in 2..=faces
        {
            print_dice(i, i.pow(dice as u32), do_sum)
        }
    } else {
        print_dice(faces, dice, do_sum)
    }
}

fn print_dice(faces: usize, dice: usize, do_sum: bool)
{
    if !do_sum
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
            } else {
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
            last.clone_from(&current)
        }
        if do_sum
        {
            println!("{}", current.iter().sum::<usize>());
        } else {
            println!("{:?}", current);
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
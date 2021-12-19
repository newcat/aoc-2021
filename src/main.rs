#![feature(mixed_integer_ops)]

use std::env;
mod readfile;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || (&args[1]).len() == 0 {
        println!("No day specified");
        std::process::exit(1);
    }

    let day = &args[1];

    match day.as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        _ => panic!("Unknown day: {}", day)
    }
}
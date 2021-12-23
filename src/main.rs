#![feature(mixed_integer_ops)]
#![feature(int_abs_diff)]
#![feature(drain_filter)]

use std::env;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day18;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod readfile;

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
        "6" => day6::run(),
        "7" => day7::run(),
        "8" => day8::run(),
        "9" => day9::run(),
        "10" => day10::run(),
        "11" => day11::run(),
        "12" => day12::run(),
        "13" => day13::run(),
        "18" => day18::run(),
        _ => panic!("Unknown day: {}", day),
    }
}

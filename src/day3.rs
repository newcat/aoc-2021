use crate::readfile;

fn get_length(lines: &readfile::Lines) -> usize {
    lines.lines().next().unwrap().len()
}

fn occurences(lines: &mut dyn std::iter::Iterator<Item = &str>, length: usize) -> Vec<[u32; 2]> {
    let mut occ: Vec<[u32; 2]> = Vec::new();
    for _ in 0..length {
        occ.push([0, 0]);
    }

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => occ[i][0] += 1,
                '1' => occ[i][1] += 1,
                _ => println!("Invalid char {}", c),
            }
        }
    }

    occ
}

fn part1(lines: &readfile::Lines) {
    let length = get_length(lines);
    let occ = occurences(&mut lines.lines(), length);
    let mut gamma: u32 = 0;
    for v in occ.iter() {
        gamma <<= 1;
        if v[1] > v[0] {
            gamma += 1;
        }
    }
    let epsilon = (!gamma) & (u32::pow(2, occ.len().try_into().unwrap()) - 1);
    println!("Part 1: {}", gamma * epsilon);
}

fn get_rating<C>(lines: &readfile::Lines, bit_criteria: C) -> &str
where
    C: Fn(u32, u32) -> char,
{
    let length = get_length(lines);
    let mut relevant_lines: Vec<&str> = lines.lines().collect();
    let mut index: usize = 0;
    while relevant_lines.len() > 1 {
        let mut iter = relevant_lines.clone().into_iter();
        let occ = occurences(&mut iter, length);
        let target = bit_criteria(occ[index][0], occ[index][1]);
        relevant_lines = relevant_lines
            .into_iter()
            .filter(|l| {
                let chars: Vec<char> = l.chars().collect();
                chars[index] == target
            })
            .collect();
        index += 1;
    }
    relevant_lines[0]
}

fn part2(lines: &readfile::Lines) {
    let oxygen_crit = |zeros: u32, ones: u32| if ones >= zeros { '1' } else { '0' };
    let oxygen_rating_str = get_rating(lines, oxygen_crit);
    let oxygen_rating = isize::from_str_radix(oxygen_rating_str, 2).unwrap();

    let co2_crit = |zeros: u32, ones: u32| if zeros <= ones { '0' } else { '1' };
    let co2_rating_str = get_rating(lines, co2_crit);
    let co2_rating = isize::from_str_radix(co2_rating_str, 2).unwrap();

    println!("Part 2: {}", oxygen_rating * co2_rating);
}

pub fn run() {
    let lines = readfile::Lines::new("day3.txt");
    part1(&lines);
    part2(&lines);
}

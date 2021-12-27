use crate::readfile::readfile;

#[derive(Debug)]
enum PacketType {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: usize,
    packet_type_id: usize,
    packet_type: PacketType,
}

impl Packet {
    pub fn value(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(v) => *v,
            PacketType::Operator(subpackets) => {
                let values: Vec<usize> = subpackets.iter().map(|p| p.value()).collect();
                match self.packet_type_id {
                    0 => values.iter().sum(),
                    1 => values.iter().fold(1, |p, c| p * c),
                    2 => values.into_iter().min().unwrap(),
                    3 => values.into_iter().max().unwrap(),
                    5 => {
                        if values[0] > values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if values[0] < values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if values[0] == values[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Invalid packet type id: {}", self.packet_type_id),
                }
            }
        }
    }
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn hex_to_bits(hex_str: &str) -> Vec<char> {
    let mut bits: Vec<char> = Vec::new();
    for hex_char in hex_str.chars() {
        bits.extend(to_binary(hex_char).chars());
    }
    return bits;
}

fn bits_to_usize<'a, I>(iter: I) -> usize
where
    I: Iterator<Item = &'a char>,
{
    return usize::from_str_radix(&String::from_iter(iter), 2).unwrap();
}

fn parse_literal<'a, I>(iter: &mut I) -> PacketType
where
    I: Iterator<Item = &'a char>,
{
    let mut value_bits: Vec<char> = Vec::new();
    let mut continue_bit = '1';
    while continue_bit == '1' {
        continue_bit = *iter.next().unwrap();
        value_bits.extend(iter.take(4));
    }
    let value = usize::from_str_radix(&String::from_iter(value_bits.iter()), 2).unwrap();
    return PacketType::Literal(value);
}

fn parse_operator<'a, I>(iter: &mut I) -> PacketType
where
    I: Iterator<Item = &'a char>,
{
    let length_type_id = *iter.next().unwrap();
    let mut subpackets: Vec<Packet> = Vec::new();
    if length_type_id == '0' {
        let total_subpacket_length = bits_to_usize(iter.take(15));
        let subpacket_bits: Vec<&char> = iter.take(total_subpacket_length).collect();
        let subpacket_iter = &mut subpacket_bits.into_iter();
        while !subpacket_iter.is_empty() {
            subpackets.push(parse_packet(subpacket_iter));
        }
    } else {
        let num_subpackets = bits_to_usize(iter.take(11));
        for _ in 0..num_subpackets {
            subpackets.push(parse_packet(iter));
        }
    }

    return PacketType::Operator(subpackets);
}

fn parse_packet<'a, I>(iter: &mut I) -> Packet
where
    I: Iterator<Item = &'a char>,
{
    let version = bits_to_usize(iter.take(3));
    let packet_type_id = bits_to_usize(iter.take(3));

    let packet_type: PacketType;
    if packet_type_id == 4 {
        packet_type = parse_literal(iter);
    } else {
        packet_type = parse_operator(iter);
    }

    return Packet {
        version: version,
        packet_type_id: packet_type_id,
        packet_type: packet_type,
    };
}

fn part1(lines: &readfile::Lines) {
    let bits = hex_to_bits(lines.lines().next().unwrap());
    let packet = parse_packet(&mut bits.iter());

    let mut packet_queue = vec![packet];
    let mut sum = 0;
    while !packet_queue.is_empty() {
        let p = packet_queue.pop().unwrap();
        sum += p.version;
        if let PacketType::Operator(subpackets) = p.packet_type {
            packet_queue.extend(subpackets);
        }
    }

    println!("Part 1: {}", sum);
}

fn part2(lines: &readfile::Lines) {
    let bits = hex_to_bits(lines.lines().next().unwrap());
    let packet = parse_packet(&mut bits.iter());
    println!("Part 2: {}", packet.value());
}

pub fn run() {
    let lines = readfile::Lines::new("day16.txt");
    part1(&lines);
    part2(&lines);
}

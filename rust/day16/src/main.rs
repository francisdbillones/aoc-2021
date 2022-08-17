#![feature(int_log)]

use std::fs;

use bitvec::{slice::BitSlice, vec::BitVec};
use take_until::TakeUntilExt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct LiteralPacket {
    version: u8,
    type_id: u8,
    literal: usize,
    bit_count: usize,
}

impl From<&BitSlice> for LiteralPacket {
    fn from(bits: &BitSlice) -> LiteralPacket {
        let (version, type_id) = Packet::get_version_and_type_id(bits);

        let mut bit_count: usize = 6;

        let literal = (6..bits.len())
            .step_by(5)
            .map(|i| {
                let group_bit = bits[i];
                let group = &bits[i + 1..i + 5];

                bit_count += 5;

                (group_bit, group)
            })
            .take_until(|&(group_bit, _)| !group_bit)
            .flat_map(|(_, group)| group)
            .fold(0, |acc, b| (acc << 1) + (*b as usize));

        LiteralPacket {
            version,
            type_id,
            literal,
            bit_count,
        }
    }
}

#[derive(Clone, Debug)]
struct OperatorPacket {
    version: u8,
    type_id: u8,
    length_type_id: bool,
    subpacket_metadata: u16,
    subpackets: Vec<Packet>,
    bit_count: usize,
}

impl OperatorPacket {
    fn value(&self) -> usize {
        let values = self.subpackets.iter().map(|sp| sp.value());
        match self.type_id {
            0 => values.sum(),
            1 => values.product(),
            2 => values.min().unwrap(),
            3 => values.max().unwrap(),
            5 | 6 | 7 => {
                let first = &self.subpackets[0];
                let second = &self.subpackets[1];

                (match self.type_id {
                    5 => first.value() > second.value(),
                    6 => first.value() < second.value(),
                    7 => first.value() == second.value(),
                    _ => panic!("For some odd reason, self.type_id is not 5, 6, or 7"),
                }) as usize
            }
            _ => panic!("Invalid self.type_id"),
        }
    }
}

impl From<&BitSlice> for OperatorPacket {
    fn from(bits: &BitSlice) -> OperatorPacket {
        let (version, type_id) = Packet::get_version_and_type_id(bits);

        let length_type_id = bits[6] as usize;
        let subpacket_metadata = get_usize(&bits[7..7 + [15, 11][length_type_id]]) as u16;

        let mut bit_count: usize = 7 + [15, 11][length_type_id];

        let mut subpackets: Vec<Packet> = Vec::new();

        if length_type_id == 0 {
            let subpacket_bit_count = get_usize(&bits[7..7 + 15]);

            let mut cursor = 0;

            loop {
                let packet = Packet::from(&bits[7 + 15 + cursor..]);

                match &packet {
                    Packet::Literal(packet) => {
                        cursor += packet.bit_count;
                        bit_count += packet.bit_count;
                    }
                    Packet::Operator(packet) => {
                        cursor += packet.bit_count;
                        bit_count += packet.bit_count;
                    }
                }

                subpackets.push(packet);

                if cursor == subpacket_bit_count {
                    break;
                }
            }
        } else {
            let num_subpackets = get_usize(&bits[7..7 + 11]) as u16;

            let mut cursor = 0;

            for _ in 0..num_subpackets {
                let packet = Packet::from(&bits[7 + 11 + cursor..]);

                match &packet {
                    Packet::Literal(packet) => {
                        cursor += packet.bit_count;
                        bit_count += packet.bit_count;
                    }
                    Packet::Operator(packet) => {
                        cursor += packet.bit_count;
                        bit_count += packet.bit_count;
                    }
                }

                subpackets.push(packet);
            }
        }

        OperatorPacket {
            version,
            type_id,
            length_type_id: length_type_id == 1,
            subpacket_metadata,
            subpackets,
            bit_count,
        }
    }
}

fn get_usize(bits: &BitSlice) -> usize {
    bits.iter().fold(0, |acc, b| (acc << 1) + (*b as usize))
}

#[derive(Debug, Clone)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

impl Packet {
    fn value(&self) -> usize {
        match self {
            Packet::Literal(packet) => packet.literal,
            Packet::Operator(packet) => packet.value(),
        }
    }

    fn get_version_and_type_id(bits: &BitSlice) -> (u8, u8) {
        let version = get_usize(&bits[0..3]) as u8;
        let type_id = get_usize(&bits[3..6]) as u8;

        (version, type_id)
    }
}

impl From<&BitSlice> for Packet {
    fn from(bits: &BitSlice) -> Packet {
        let (_, type_id) = Packet::get_version_and_type_id(bits);

        if type_id == 4 {
            Packet::Literal(LiteralPacket::from(bits))
        } else {
            Packet::Operator(OperatorPacket::from(bits))
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't read input.txt");

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let packet = parse(input);
    recursively_sum_version_numbers(&packet)
}

fn part2(input: &str) -> usize {
    let packet = parse(input);

    packet.value()
}

fn recursively_sum_version_numbers(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(packet) => packet.version as usize,
        Packet::Operator(packet) => {
            packet.version as usize
                + packet
                    .subpackets
                    .iter()
                    .map(recursively_sum_version_numbers)
                    .sum::<usize>()
        }
    }
}

fn parse(input: &str) -> Packet {
    let bits: BitVec = input
        .chars()
        .flat_map(|c| {
            BitVec::<u8>::from_element(
                u8::from_str_radix(c.to_string().as_str(), 16)
                    .unwrap()
                    .reverse_bits(),
            )[4..8]
                .to_bitvec()
        })
        .collect();

    Packet::from(&bits as &BitSlice)
}

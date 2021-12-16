use super::utils::read_input;

type Bits = Vec<bool>;
type BitsSlice<'a> = &'a [bool];

fn print_bits(b: BitsSlice) {
    for bit in b {
        print!("{}", if *bit { '1' } else { '0' })
    }
    println!();
}

fn hexdump2bits(hexdump: &String) -> Bits {
    fn hexdigit2bits(ch: char) -> Bits {
        let num = ch.to_digit(16).unwrap();
        (0..4)
            .rev()
            .map(|offset| (num >> offset) % 2 == 1)
            .collect()
    }

    hexdump.chars().flat_map(hexdigit2bits).collect()
}

fn bits2num(bits: BitsSlice) -> u64 {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, bit)| if *bit { (2 as u64).pow(i as u32) } else { 0 })
        .sum()
}

#[derive(Debug)]
struct PacketHeader {
    version: u8,
    type_id: u8,
}

#[derive(Debug)]
enum Packet {
    Literal(PacketHeader, u64),
    Op(PacketHeader, Vec<Packet>),
}

impl Packet {
    fn sum_versions(&self) -> u32 {
        match self {
            Packet::Literal(h, _) => h.version as u32,
            Packet::Op(h, subpackets) => {
                let subpackets_version_sum: u32 = subpackets.iter().map(|p| p.sum_versions()).sum();
                (h.version as u32) + subpackets_version_sum
            }
        }
    }

    fn value(&self) -> u64 {
        match self {
            Packet::Literal(_, v) => *v,
            Packet::Op(h, sub) => {
                let mut subpacket_values = sub.iter().map(|p| p.value());
                match h.type_id {
                    0 => subpacket_values.sum(),
                    1 => subpacket_values.product(),
                    2 => subpacket_values.min().unwrap(),
                    3 => subpacket_values.max().unwrap(),
                    5..=7 => {
                        let first = subpacket_values.next().unwrap();
                        let second = subpacket_values.next().unwrap();
                        if (first > second && h.type_id == 5)
                            || (first < second && h.type_id == 6)
                            || (first == second && h.type_id == 7)
                        {
                            1
                        } else {
                            0
                        }
                    }
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }
}

const DEBUG: bool = true;

fn parse_packet(bits: BitsSlice) -> Option<(Packet, BitsSlice)> {
    if DEBUG {
        println!("\ncurrent bits, len {}", bits.len());
    }
    if bits.len() < 10 {
        if DEBUG {
            print_bits(bits);
            println!("not a packet, exiting!")
        }
        return None;
    }

    if DEBUG {
        print_bits(bits);
    }

    let version = bits2num(&bits[0..=2]) as u8;
    let type_id = bits2num(&bits[3..=5]) as u8;
    if DEBUG {
        print!("VVVTTT")
    }
    let header = PacketHeader { version, type_id };
    if type_id == 4 {
        let mut number_bits: Bits = Vec::new();
        let mut group_start: usize = 6;
        loop {
            number_bits.extend(&bits[group_start + 1..group_start + 5]);
            if DEBUG {
                print!("FDDDD")
            }
            if !bits[group_start] {
                group_start += 5;
                break;
            }
            group_start += 5;
        }
        if DEBUG {
            println!("\nvalue");
        }
        return Some((
            Packet::Literal(header, bits2num(number_bits.as_slice())),
            &bits[group_start..],
        ));
    } else {
        let length_type_id = bits2num(&bits[6..=6]);
        if DEBUG {
            print!("I")
        }
        let mut subpackets: Vec<Packet> = Vec::new();
        if length_type_id == 0 {
            let length_bits = bits2num(&bits[7..=21]) as usize;
            if DEBUG {
                print!("LLLLLLLLLLLLLLL")
            }
            if DEBUG {
                println!("\nop packet with len in bits: {}", length_bits);
            }
            let subpackets_bits: Bits = bits
                .iter()
                .skip(22)
                .take(length_bits)
                .map(|b| *b)
                .collect::<Bits>();
            let mut subpackets: Vec<Packet> = Vec::new();

            let mut subpacket_bits = subpackets_bits.as_slice();
            while let Some((subpacket, left_bits)) = parse_packet(subpacket_bits) {
                subpackets.push(subpacket);
                subpacket_bits = left_bits;
            }
            return Some((Packet::Op(header, subpackets), &bits[22 + length_bits..]));
        } else if length_type_id == 1 {
            let n_subpackets = bits2num(&bits[7..=17]) as usize;

            if DEBUG {
                println!("LLLLLLLLLLL");
                println!("op packet with {} subpackets", n_subpackets);
            }
            let mut subpackets_start = 18;
            for _ in 0..n_subpackets {
                if let Some((subpacket, left_bits)) = parse_packet(&bits[subpackets_start..]) {
                    subpackets.push(subpacket);
                    subpackets_start = bits.len() - left_bits.len();
                } else {
                    panic!("Can't parse expected subpacket")
                }
            }
            return Some((Packet::Op(header, subpackets), &bits[subpackets_start..]));
        }
        panic!("");
    }
}

pub fn bits_decoding() {
    let input = read_input(16, false);

    let bits = hexdump2bits(&input);
    if let Some((packet, _)) = parse_packet(&bits) {
        println!();
        println!("versions sum: {}", packet.sum_versions());
        println!("value: {}", packet.value());
    }
}

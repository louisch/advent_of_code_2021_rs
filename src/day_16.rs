use bitvec::prelude::*;


pub fn part_1(lines: &Vec<String>) {
    let sum = sum_version_numbers(lines);
    println!("Sum of Version Numbers: {}", sum);
}

pub fn part_2(lines: &Vec<String>) {
    println!("Calculated: {}", calc_packets(lines));
}


struct Packet {
    version: BitVec,
    type_id: BitVec,
    subpackets: Vec<Box<Packet>>,
    value: Option<u64>,
}

impl Packet {
    fn new(version: &BitSlice, type_id: &BitSlice, subpackets: Vec<Box<Packet>>, value: Option<u64>) -> Self {
        Packet { version: BitVec::from_bitslice(version), type_id: BitVec::from_bitslice(type_id), subpackets: subpackets, value: value }
    }

    fn version_value(&self) -> u8 {
        self.version.load::<u8>().reverse_bits() >> 5
    }

    fn type_id_value(&self) -> u8 {
        self.type_id.load::<u8>().reverse_bits() >> 5
    }

    fn sum_versions(&self) -> u64 {
        self.subpackets.iter().fold(self.version_value() as u64, |acc, packet| acc + packet.sum_versions())
    }

    fn calc_packet(&self) -> u64 {
        let type_id_value = self.type_id_value();
        match type_id_value {
            0 => { // sum
                self.subpackets.iter().fold(0, |acc, packet| acc + packet.calc_packet())
            },
            1 => { // product
                self.subpackets.iter().fold(1, |acc, packet| acc * packet.calc_packet())
            },
            2 => { // min
                self.subpackets.iter().map(|packet| packet.calc_packet()).min().unwrap()
            },
            3 => { // max
                self.subpackets.iter().map(|packet| packet.calc_packet()).max().unwrap()
            },
            4 => { // literal
                self.value.unwrap()
            },
            5 => { // greater than
                let first = self.subpackets[0].calc_packet();
                let second = self.subpackets[1].calc_packet();
                if first > second { 1 } else { 0 }
            },
            6 => { // less than
                let first = self.subpackets[0].calc_packet();
                let second = self.subpackets[1].calc_packet();
                if first < second { 1 } else { 0 }
            },
            7 => { // equal to
                let first = self.subpackets[0].calc_packet();
                let second = self.subpackets[1].calc_packet();
                if first == second { 1 } else { 0 }
            },
            _ => { // this should never happen
                panic!("Type ID {} not supported!", type_id_value);
            }
        }
    }

    fn fmt_sub(&self, f: &mut std::fmt::Formatter, layer: usize) -> std::fmt::Result {
        let version_fmt = self.version_value();
        let type_id_fmt = match self.type_id_value() {
            0 => "Sum",
            1 => "Product",
            2 => "Minimum",
            3 => "Maximum",
            4 => "Literal",
            5 => "Greater Than",
            6 => "Less Than",
            7 => "Equal To",
            _ => "Unknown",
        };
        let _ = f.write_fmt(format_args!("{:indent$}Version {} TypeID {} Value {:?} / Raw: {} {}\n{:indent$}Sub Packets:\n", "", version_fmt, type_id_fmt, self.value, self.version, self.type_id, "", indent=2*layer))?;
        let subpacket_results = self.subpackets.iter().fold(Ok(()), |acc, packet: &Box<Packet>| {
            if acc.is_err() {
                acc
            } else {
                packet.fmt_sub(f, layer + 1)
            }
        })?;
        Ok(subpacket_results)
    }
}

impl std::fmt::Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_sub(f, 0)
    }
}

fn sum_version_numbers(lines: &Vec<String>) -> u64 {
    let packets = parse_transmission(lines);
    packets.iter().fold(0, |acc, packet| acc + packet.sum_versions())
}

fn calc_packets(lines: &Vec<String>) -> u64 {
    let packets = parse_transmission(lines);
    packets[0].calc_packet()
}

fn parse_transmission(lines: &Vec<String>) -> Vec<Box<Packet>> {
    let transmission = lines.iter().filter(|line| !line.trim().is_empty()).next().unwrap();
    let mut bv = BitVec::new();
    let as_half_bytes: Vec<u8> = transmission.chars().filter_map(|c| c.to_digit(16).map(|i| i as u8)).collect();
    bv.resize(as_half_bytes.len() * 4, false);
    for (i, half_byte) in as_half_bytes.iter().enumerate() {
        bv[i * 4..(i + 1) * 4].store(half_byte.reverse_bits() >> 4);
    }
    parse_packets(&bv)
}

fn parse_packets(bitstr: &BitSlice) -> Vec<Box<Packet>> {
    let mut all_packets = vec![];
    let mut remaining = BitVec::from_bitslice(bitstr);
    loop {
        if let Some((next_packet, remaining_)) = parse_packet(&remaining) {
            all_packets.push(Box::new(next_packet));
            remaining = remaining_;
        } else {
            break;
        }
    }
    all_packets
}

fn parse_packet(bitstr: &BitSlice) -> Option<(Packet, BitVec)> {
    if bitstr.is_empty() || bitstr.count_ones() == 0 {
        return None;
    }
    let (version, remaining_after_version) = bitstr.split_at(3);
    let (type_id, remaining_after_type_id) = remaining_after_version.split_at(3);
    let mut remaining = BitVec::from_bitslice(remaining_after_type_id);
    let mut subpackets = vec![];
    let mut literal_value = None;

    if type_id == bits![1, 0, 0] {
        let mut chunks: Vec<BitVec> = vec![];
        let mut skip_to_index = 0;
        for chunk in remaining.chunks(5) {
            let (not_last_chunk, value_chunk) = chunk.split_at(1);
            chunks.push(BitVec::from_bitslice(value_chunk));
            skip_to_index += 5;
            if !not_last_chunk[0] {
                break;
            }
        }
        let (_, remaining_after_literal) = remaining.split_at(skip_to_index);
        let mut literal = 0;
        for (i, chunk) in chunks.iter().rev().enumerate() {
            let chunk_as_value = ((chunk.load::<u8>().reverse_bits() >> 4) as u64) << (i * 4);
            literal += chunk_as_value;
        }
        literal_value = Some(literal);
        remaining = BitVec::from_bitslice(remaining_after_literal);
    } else {
        let (length_type_id, remaining_after_lti) = remaining.split_at(1);
        if !length_type_id[0] { // 15 bits
            let (length_in_bits, remaining_after_length) = remaining_after_lti.split_at(15);
            let length: usize = (length_in_bits.load::<u16>().reverse_bits() >> 1) as usize;
            let (subpackets_bitstr, remaining_after_subpackets) = remaining_after_length.split_at(length);
            subpackets = parse_packets(subpackets_bitstr);
            remaining = BitVec::from_bitslice(remaining_after_subpackets);
        } else { // 11 bits
            let (length_in_subpackets, remaining_after_length) = remaining_after_lti.split_at(11);
            let number_of_subpackets: u16 = length_in_subpackets.load::<u16>().reverse_bits() >> 5;
            let mut current_remaining = BitVec::from_bitslice(remaining_after_length);
            for _ in 0..number_of_subpackets {
                if let Some((subpacket, new_remaining)) = parse_packet(&current_remaining) {
                    subpackets.push(Box::new(subpacket));
                    current_remaining = new_remaining;
                }
            }
            remaining = current_remaining;
        }
    }

    Some((Packet::new(version, type_id, subpackets, literal_value), remaining))
}


#[cfg(test)]
mod tests {
    use crate::day_16::*;

    const TEST_INPUT_BASE: &str = "D2FE28";
    const TEST_INPUT_1: &str = r#"8A004A801A8002F478
"#;
    const TEST_INPUT_2: &str = "620080001611562C8802118E34";
    const TEST_INPUT_3: &str = "C0015000016115A2E0802F182340";
    const TEST_INPUT_4: &str = "A0016C880162017C3686B18A3D4780";

    fn get_test_input(s: &str) -> Vec<String> {
        s.split("\n").map(str::to_string).collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(sum_version_numbers(&get_test_input(TEST_INPUT_BASE)), 6);
        assert_eq!(sum_version_numbers(&get_test_input(TEST_INPUT_1)), 16);
        assert_eq!(sum_version_numbers(&get_test_input(TEST_INPUT_2)), 12);
        assert_eq!(sum_version_numbers(&get_test_input(TEST_INPUT_3)), 23);
        assert_eq!(sum_version_numbers(&get_test_input(TEST_INPUT_4)), 31);
    }

    const TEST_INPUT_5: &str = "C200B40A82";
    const TEST_INPUT_6: &str = "04005AC33890";
    const TEST_INPUT_7: &str = "880086C3E88112";
    const TEST_INPUT_8: &str = "CE00C43D881120";
    const TEST_INPUT_9: &str = "D8005AC2A8F0";
    const TEST_INPUT_10: &str = "F600BC2D8F";
    const TEST_INPUT_11: &str = "9C005AC2F8F0";
    const TEST_INPUT_12: &str = "9C0141080250320F1802104A08";
    const TEST_INPUT_13: &str = "9C005AC2F8F00000";

    #[test]
    fn test_part_2() {
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_BASE)), 2021);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_5)), 3);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_6)), 54);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_7)), 7);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_8)), 9);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_9)), 1);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_10)), 0);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_11)), 0);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_12)), 1);
        assert_eq!(calc_packets(&get_test_input(TEST_INPUT_13)), 0);
    }
}

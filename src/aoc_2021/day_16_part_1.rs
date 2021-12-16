use std::rc::Rc;
use std::cell::RefCell;

enum ParserState {
    Version,
    TypeID,
    LengthTypeID,
    LengthField11,
    LengthField15,
    LiteralValuePortion,
    End
}

pub struct Packet {
    pub version: u8,
    pub type_id: u8,
    pub length_type_id: u8,
    pub length_field: u16,
    pub total_accumulated_bits: u16,
    pub sub_packets: Vec<Rc<RefCell<Packet>>>,
    pub parent_packet: Option<Rc<RefCell<Packet>>>,
    pub literal_value: i64
}

fn get_total_accumulated_bits(packet: Rc<RefCell<Packet>>) -> u16 {
    let mut total_accumulated_bits: u16 = 0;
    for sub_packet in packet.borrow().sub_packets.iter() {
        total_accumulated_bits += get_total_accumulated_bits(sub_packet.clone());
    }
    total_accumulated_bits += packet.borrow().total_accumulated_bits;
    return total_accumulated_bits;
}

fn get_versions_sum(packet: Rc<RefCell<Packet>>) -> usize {
    let mut versions_sum: usize = 0;
    for sub_packet in packet.borrow().sub_packets.iter() {
        versions_sum += get_versions_sum(sub_packet.clone());
    }
    versions_sum += packet.borrow().version as usize;
    return versions_sum;
}

pub fn parse_input(input: &str) -> Rc<RefCell<Packet>> {
    let mut num_accumulated_bits: usize = 0;
    let mut accumulated_bits: u16 = 0;
    let mut state = ParserState::Version;

    let packet = Rc::new(RefCell::new(Packet {
        version: 0,
        type_id: 0,
        length_type_id: 0,
        length_field: 0,
        total_accumulated_bits: 0,
        sub_packets: Vec::<Rc<RefCell<Packet>>>::new(),
        parent_packet: None,
        literal_value: 0
    }));

    let mut current_packet = packet.clone();

    let mut indent = String::new();

    for ch in input.chars() {
        let num = ch.to_digit(16).unwrap() as u16;

        for i in (0..4).rev() {
            let new_bit: u16 = (num >> i) & 1u16;

            accumulated_bits <<= 1u16;
            accumulated_bits &= 0xFFFEu16;
            accumulated_bits |= new_bit;
            num_accumulated_bits += 1;

            match state {
                ParserState::Version => {
                    if num_accumulated_bits == 3 {
                        let version = accumulated_bits as u8;
                        println!("\n{}version = {}", indent, version);
                        current_packet.borrow_mut().version = version;
                        current_packet.borrow_mut().total_accumulated_bits += 3;

                        num_accumulated_bits = 0;
                        accumulated_bits = 0;

                        state = ParserState::TypeID;
                    }
                },
                ParserState::TypeID => {
                    if num_accumulated_bits == 3 {
                        let type_id = accumulated_bits as u8;
                        println!("{}type_id = {}", indent, type_id);
                        current_packet.borrow_mut().type_id = type_id;
                        current_packet.borrow_mut().total_accumulated_bits += 3;

                        num_accumulated_bits = 0;
                        accumulated_bits = 0;

                        if type_id == 4 {
                            state = ParserState::LiteralValuePortion;
                        }
                        else {
                            state = ParserState::LengthTypeID;
                        }
                    }
                },
                ParserState::LengthTypeID => {
                    if num_accumulated_bits == 1 {
                        let length_type_id = accumulated_bits as u8;
                        println!("{}length_type_id = {}", indent, length_type_id);
                        current_packet.borrow_mut().length_type_id = length_type_id;
                        current_packet.borrow_mut().total_accumulated_bits += 1;

                        num_accumulated_bits = 0;
                        accumulated_bits = 0;

                        if length_type_id == 1 {
                            state = ParserState::LengthField11;
                        }
                        else {
                            state = ParserState::LengthField15;
                        }
                    }
                },
                ParserState::LengthField11 => {
                    if num_accumulated_bits == 11 {
                        let num_sub_packets = accumulated_bits as u16;
                        println!("{}num_sub_packets = {}", indent, num_sub_packets);
                        current_packet.borrow_mut().length_field = num_sub_packets;
                        current_packet.borrow_mut().total_accumulated_bits += 11;

                        num_accumulated_bits = 0;
                        accumulated_bits = 0;

                        let new_packet = Rc::new(RefCell::new(Packet {
                            version: 0,
                            type_id: 0,
                            length_type_id: 0,
                            length_field: 0,
                            total_accumulated_bits: 0,
                            sub_packets: Vec::<Rc<RefCell<Packet>>>::new(),
                            parent_packet: Some(current_packet.clone()),
                            literal_value: 0
                        }));
                        current_packet.borrow_mut().sub_packets.push(new_packet.clone());
                        current_packet = new_packet;
                        indent.push('\t');

                        state = ParserState::Version;
                    }
                },
                ParserState::LengthField15 => {
                    if num_accumulated_bits == 15 {
                        let num_sub_packet_bits = accumulated_bits as u16;
                        println!("{}num_sub_packet_bits = {}", indent, num_sub_packet_bits);
                        current_packet.borrow_mut().length_field = num_sub_packet_bits;
                        current_packet.borrow_mut().total_accumulated_bits += 15;

                        num_accumulated_bits = 0;
                        accumulated_bits = 0;

                        let new_packet = Rc::new(RefCell::new(Packet {
                            version: 0,
                            type_id: 0,
                            length_type_id: 0,
                            length_field: 0,
                            total_accumulated_bits: 0,
                            sub_packets: Vec::<Rc<RefCell<Packet>>>::new(),
                            parent_packet: Some(current_packet.clone()),
                            literal_value: 0
                        }));
                        current_packet.borrow_mut().sub_packets.push(new_packet.clone());
                        current_packet = new_packet;
                        indent.push('\t');

                        state = ParserState::Version;
                    }
                },
                ParserState::LiteralValuePortion => {
                    if num_accumulated_bits == 5 {
                        let mut return_to_nearest_incomplete_parent = false;

                        {
                            let last_sub_value = ((accumulated_bits >> 4) & 1u16) != 1u16;
                            let sub_value = (accumulated_bits & 0xFu16) as i64;
                            println!("{}sub_value = {} (last_sub_value = {})", indent, sub_value, last_sub_value);
                            current_packet.borrow_mut().literal_value <<= 4;
                            current_packet.borrow_mut().literal_value |= sub_value;
                            current_packet.borrow_mut().total_accumulated_bits += 5;

                            num_accumulated_bits = 0;
                            accumulated_bits = 0;

                            let this_packet = current_packet.clone();
                            if last_sub_value {
                                println!("{}complete_literal_value = {}", indent, current_packet.borrow_mut().literal_value);
                                if let Some(parent_packet) = &this_packet.borrow_mut().parent_packet {
                                    current_packet = parent_packet.clone();
                                    indent.remove(indent.len() - 1);
                                    return_to_nearest_incomplete_parent = true;
                                }
                                else {
                                    state = ParserState::End;
                                    break;
                                }
                            }
                        }
                        if return_to_nearest_incomplete_parent {
                            loop {
                                let this_packet = current_packet.clone();
                                if this_packet.borrow().length_type_id == 1 {
                                    let num_sub_packets = current_packet.borrow().length_field;
                                    if current_packet.borrow().sub_packets.len() < num_sub_packets as usize {
                                        let new_packet = Rc::new(RefCell::new(Packet {
                                            version: 0,
                                            type_id: 0,
                                            length_type_id: 0,
                                            length_field: 0,
                                            total_accumulated_bits: 0,
                                            sub_packets: Vec::<Rc<RefCell<Packet>>>::new(),
                                            parent_packet: Some(current_packet.clone()),
                                            literal_value: 0
                                        }));
                                        current_packet.borrow_mut().sub_packets.push(new_packet.clone());
                                        current_packet = new_packet;
                                        indent.push('\t');
                
                                        state = ParserState::Version;
                                        break;
                                    }
                                    else {
                                        if let Some(parent_packet) = &this_packet.borrow().parent_packet {
                                            current_packet = parent_packet.clone();
                                            indent.remove(indent.len() - 1);
                                        }
                                        else {
                                            state = ParserState::End;
                                            break;
                                        }
                                    }
                                }
                                else {
                                    let expected_num_sub_packets_bits = current_packet.borrow().length_field;
                                    let total_num_sub_packet_bits = get_total_accumulated_bits(current_packet.clone()) - current_packet.borrow().total_accumulated_bits;
                                    if total_num_sub_packet_bits < expected_num_sub_packets_bits {
                                        let new_packet = Rc::new(RefCell::new(Packet {
                                            version: 0,
                                            type_id: 0,
                                            length_type_id: 0,
                                            length_field: 0,
                                            total_accumulated_bits: 0,
                                            sub_packets: Vec::<Rc<RefCell<Packet>>>::new(),
                                            parent_packet: Some(current_packet.clone()),
                                            literal_value: 0
                                        }));
                                        current_packet.borrow_mut().sub_packets.push(new_packet.clone());
                                        current_packet = new_packet;
                                        indent.push('\t');
                
                                        state = ParserState::Version;
                                        break;
                                    }
                                    else {
                                        if let Some(parent_packet) = &this_packet.borrow().parent_packet {
                                            current_packet = parent_packet.clone();
                                            indent.remove(indent.len() - 1);
                                        }
                                        else {
                                            state = ParserState::End;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                _ => ()
            }
        }
    }

    return packet;
}

#[allow(dead_code)]
fn day_16_part_1(input: &str) -> usize {
    let packet = parse_input(input);

    let ans = get_versions_sum(packet);

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_day_16_part_1() {
        let input =
r#"D2FE28"#;

        let ans = day_16_part_1(&input);

        assert_eq!(ans, 6);
    }

    #[test]
    fn example_2_day_16_part_1() {
        let input =
r#"38006F45291200"#;

        let ans = day_16_part_1(&input);

        assert_eq!(ans, 9);
    }

    #[test]
    fn example_3_day_16_part_1() {
        let input =
r#"EE00D40C823060"#;

        let ans = day_16_part_1(&input);

        assert_eq!(ans, 14);
    }

    #[test]
    fn example_4_day_16_part_1() {
        let input =
r#"8A004A801A8002F478"#;

        let ans = day_16_part_1(&input);

        assert_eq!(ans, 16);
    }

    #[test]
    fn example_5_day_16_part_1() {
        let input =
r#"620080001611562C8802118E34"#;

        let ans = day_16_part_1(&input);

        assert_eq!(ans, 12);
    }

    #[test]
    fn example_6_day_16_part_1() {
        let input =
r#"C0015000016115A2E0802F182340"#;

        let ans = day_16_part_1(&input);

        assert_eq!(ans, 23);
    }

    #[test]
    fn example_7_day_16_part_1() {
        let input =
r#"A0016C880162017C3686B18A3D4780"#;

        let ans = day_16_part_1(&input);

        assert_eq!(ans, 31);
    }

    #[test]
    fn test_day_16_part_1() {
        let input =
r#"E058F79802FA00A4C1C496E5C738D860094BDF5F3ED004277DD87BB36C8EA800BDC3891D4AFA212012B64FE21801AB80021712E3CC771006A3E47B8811E4C01900043A1D41686E200DC4B8DB06C001098411C22B30085B2D6B743A6277CF719B28C9EA11AEABB6D200C9E6C6F801F493C7FE13278FFC26467C869BC802839E489C19934D935C984B88460085002F931F7D978740668A8C0139279C00D40401E8D1082318002111CE0F460500BE462F3350CD20AF339A7BB4599DA7B755B9E6B6007D25E87F3D2977543F00016A2DCB029009193D6842A754015CCAF652D6609D2F1EE27B28200C0A4B1DFCC9AC0109F82C4FC17880485E00D4C0010F8D110E118803F0DA1845A932B82E200D41E94AD7977699FED38C0169DD53B986BEE7E00A49A2CE554A73D5A6ED2F64B4804419508B00584019877142180803715224C613009E795E58FA45EA7C04C012D004E7E3FE64C27E3FE64C24FA5D331CFB024E0064DEEB49D0CC401A2004363AC6C8344008641B8351B08010882917E3D1801D2C7CA0124AE32DD3DDE86CF52BBFAAC2420099AC01496269FD65FA583A5A9ECD781A20094CE10A73F5F4EB450200D326D270021A9F8A349F7F897E85A4020CF802F238AEAA8D22D1397BF27A97FD220898600C4926CBAFCD1180087738FD353ECB7FDE94A6FBCAA0C3794875708032D8A1A0084AE378B994AE378B9A8007CD370A6F36C17C9BFCAEF18A73B2028C0A004CBC7D695773FAF1006E52539D2CFD800D24B577E1398C259802D3D23AB00540010A8611260D0002130D23645D3004A6791F22D802931FA4E46B31FA4E4686004A8014805AE0801AC050C38010600580109EC03CC200DD40031F100B166005200898A00690061860072801CE007B001573B5493004248EA553E462EC401A64EE2F6C7E23740094C952AFF031401A95A7192475CACF5E3F988E29627600E724DBA14CBE710C2C4E72302C91D12B0063F2BBFFC6A586A763B89C4DC9A0"#;

        let ans = day_16_part_1(&input);

        assert_eq!(ans, 913);
    }
}

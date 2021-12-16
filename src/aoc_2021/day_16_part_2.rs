use std::rc::Rc;
use std::cell::RefCell;

use crate::aoc_2021::day_16_part_1::parse_input;
use crate::aoc_2021::day_16_part_1::Packet;

fn compute_result(packet: Rc<RefCell<Packet>>) -> i64 {
    let id = packet.borrow().type_id;
    let mut ans: i64 = 0;
    match id {
        1 => ans = 1,
        2 => ans = i64::MAX,
        4 => ans = packet.borrow().literal_value,
        5 => ans = i64::MAX,
        _ => ()
    }
    let mut prev_ans = ans;

    for sub_packet in packet.borrow().sub_packets.iter() {
        match id {
            0 => ans += compute_result(sub_packet.clone()),
            1 => ans *= compute_result(sub_packet.clone()),
            2 => ans = i64::min(ans, compute_result(sub_packet.clone())),
            3 => ans = i64::max(ans, compute_result(sub_packet.clone())),
            5 => {
                let this_ans = compute_result(sub_packet.clone());
                ans = if prev_ans > this_ans { 1 } else { 0 };
                prev_ans = this_ans;
            },
            6 => {
                let this_ans = compute_result(sub_packet.clone());
                ans = if prev_ans < this_ans { 1 } else { 0 };
                prev_ans = this_ans;
            },
            7 => {
                let this_ans = compute_result(sub_packet.clone());
                ans = if prev_ans == this_ans { 1 } else { 0 };
                prev_ans = this_ans;
            },
            _ => ()
        }
    }

    return ans;
}

#[allow(dead_code)]
fn day_16_part_2(input: &str) -> i64 {
    let packet = parse_input(input);

    let ans = compute_result(packet);

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_day_16_part_2() {
        let input =
r#"C200B40A82"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 3);
    }

    #[test]
    fn example_2_day_16_part_2() {
        let input =
r#"04005AC33890"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 54);
    }

    #[test]
    fn example_3_day_16_part_2() {
        let input =
r#"880086C3E88112"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 7);
    }

    #[test]
    fn example_4_day_16_part_2() {
        let input =
r#"CE00C43D881120"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 9);
    }

    #[test]
    fn example_5_day_16_part_2() {
        let input =
r#"D8005AC2A8F0"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 1);
    }

    #[test]
    fn example_6_day_16_part_2() {
        let input =
r#"F600BC2D8F"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 0);
    }

    #[test]
    fn example_7_day_16_part_2() {
        let input =
r#"9C005AC2F8F0"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 0);
    }

    #[test]
    fn example_8_day_16_part_2() {
        let input =
r#"9C0141080250320F1802104A08"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 1);
    }

    #[test]
    fn test_day_16_part_2() {
        let input =
r#"E058F79802FA00A4C1C496E5C738D860094BDF5F3ED004277DD87BB36C8EA800BDC3891D4AFA212012B64FE21801AB80021712E3CC771006A3E47B8811E4C01900043A1D41686E200DC4B8DB06C001098411C22B30085B2D6B743A6277CF719B28C9EA11AEABB6D200C9E6C6F801F493C7FE13278FFC26467C869BC802839E489C19934D935C984B88460085002F931F7D978740668A8C0139279C00D40401E8D1082318002111CE0F460500BE462F3350CD20AF339A7BB4599DA7B755B9E6B6007D25E87F3D2977543F00016A2DCB029009193D6842A754015CCAF652D6609D2F1EE27B28200C0A4B1DFCC9AC0109F82C4FC17880485E00D4C0010F8D110E118803F0DA1845A932B82E200D41E94AD7977699FED38C0169DD53B986BEE7E00A49A2CE554A73D5A6ED2F64B4804419508B00584019877142180803715224C613009E795E58FA45EA7C04C012D004E7E3FE64C27E3FE64C24FA5D331CFB024E0064DEEB49D0CC401A2004363AC6C8344008641B8351B08010882917E3D1801D2C7CA0124AE32DD3DDE86CF52BBFAAC2420099AC01496269FD65FA583A5A9ECD781A20094CE10A73F5F4EB450200D326D270021A9F8A349F7F897E85A4020CF802F238AEAA8D22D1397BF27A97FD220898600C4926CBAFCD1180087738FD353ECB7FDE94A6FBCAA0C3794875708032D8A1A0084AE378B994AE378B9A8007CD370A6F36C17C9BFCAEF18A73B2028C0A004CBC7D695773FAF1006E52539D2CFD800D24B577E1398C259802D3D23AB00540010A8611260D0002130D23645D3004A6791F22D802931FA4E46B31FA4E4686004A8014805AE0801AC050C38010600580109EC03CC200DD40031F100B166005200898A00690061860072801CE007B001573B5493004248EA553E462EC401A64EE2F6C7E23740094C952AFF031401A95A7192475CACF5E3F988E29627600E724DBA14CBE710C2C4E72302C91D12B0063F2BBFFC6A586A763B89C4DC9A0"#;

        let ans = day_16_part_2(&input);

        assert_eq!(ans, 1510977819698);
    }
}
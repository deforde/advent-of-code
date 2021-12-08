fn parse_input(input: &str) -> (Vec<u64>, Vec::<Vec<Vec<u64>>>) {
    let first_new_line_pos = input.find("\n\n").unwrap();
    let numbers_string = &input[..first_new_line_pos];
    let numbers: Vec<u64> = numbers_string.split(',').into_iter().map(|num_str| num_str.parse::<u64>().unwrap()).collect();

    let mut blocks = Vec::<Vec<Vec<u64>>>::new();
    let mut start_search_pos = Some(first_new_line_pos + 2);
    while start_search_pos != None {
        let start_pos = start_search_pos.unwrap();

        let next_line_break = input[start_pos..].find("\n\n");

        let end_pos: usize;
        match next_line_break {
            Some(val) => {
                end_pos = val + start_pos;
                start_search_pos = Some(end_pos + 2);
            }
            _ => {
                end_pos = input.len();
                start_search_pos = None
            }
        }

        let mut block = Vec::<Vec<u64>>::new();
        let block_string = &input[start_pos..end_pos];
        let block_row_strings: Vec<&str> = block_string.split('\n').collect();
        for block_row_string in block_row_strings {
            let row: Vec<u64> = block_row_string.split_whitespace().into_iter().map(|num_str| num_str.parse::<u64>().unwrap()).collect();
            block.push(row);
        }
        blocks.push(block);
    }

    return (numbers, blocks)
}

#[allow(dead_code)]
fn day_4_part_1(input: &str) -> u64 {
    let (numbers, blocks) = parse_input(input);

    println!("numbers = {:?}", numbers);
    println!("blocks = {:#?}", blocks);

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_4_part_1() {
        let input =
r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let ans = day_4_part_1(&input);

        assert_eq!(ans, 4512);
    }

    // #[test]
    // fn test_day_4_part_1() {
    // }
}

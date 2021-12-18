fn add_to_last_num(string: &mut String, val: i64) {
    let mut num_start_idx: usize = 0;
    let mut num_string = String::new();
    let mut accumulating_num_chars = false;
    for (idx, ch) in string.chars().enumerate() {
        if ch.is_digit(10) {
            if !accumulating_num_chars {
                accumulating_num_chars = true;
                num_start_idx = idx;
                num_string.clear();
            }
            num_string.push(ch);
        }
        else {
            accumulating_num_chars = false;
        }
    }
    if !num_string.is_empty() {
        let mut num = num_string.parse::<i64>().unwrap();
        num += val;
        string.replace_range(num_start_idx..(num_start_idx + num_string.len()), &num.to_string());
    }
}

fn add_to_first_num(string: &mut String, val: i64) {
    let mut num_start_idx: usize = 0;
    let mut num_string = String::new();
    let mut accumulating_num_chars = false;
    for (idx, ch) in string.chars().enumerate() {
        if ch.is_digit(10) {
            if !accumulating_num_chars {
                accumulating_num_chars = true;
                num_start_idx = idx;
                num_string.clear();
            }
            num_string.push(ch);
        }
        else {
            if accumulating_num_chars {
                break;
            }
        }
    }
    if !num_string.is_empty() {
        let mut num = num_string.parse::<i64>().unwrap();
        num += val;
        string.replace_range(num_start_idx..(num_start_idx + num_string.len()), &num.to_string());
    }
}

pub fn explode_snail(input: &mut String) -> bool {
    let mut output = String::new();
    let mut level: usize = 0;
    let mut last_bracket = ' ';

    for (idx, ch) in input.chars().enumerate() {
        output.push(ch);
        if ch == '[' {
            level += 1;
            last_bracket = ch;
        }
        else if ch == ']' {
            if last_bracket == '[' && level > 4 {
                let open_bracket_pos = output.len() - output.chars().rev().position(|result_ch| result_ch == '[').unwrap() - 1;
                let pair_string = &output[open_bracket_pos + 1..output.len()-1].split(',').collect::<Vec<_>>();
                let explosion_val_1 = pair_string[0].parse::<i64>().unwrap();
                let explosion_val_2 = pair_string[1].parse::<i64>().unwrap();

                output.truncate(open_bracket_pos);
                add_to_last_num(&mut output, explosion_val_1);
                output.push('0');

                let mut remainder = input[idx + 1..].chars().collect::<String>();
                add_to_first_num(&mut remainder, explosion_val_2);
                output.push_str(&remainder);
                break;
            }

            level -= 1;
            last_bracket = ch;
        }
    }

    let updated = *input != output;
    *input = output;
    return updated;
}

pub fn split_snail(input: &mut String) -> bool {
    let mut output = String::new();
    let mut num_start_idx: usize = 0;
    let mut accumulating_num_chars = false;
    let mut num_string = String::new();
    
    for (idx, ch) in input.chars().enumerate() {
        output.push(ch);
        if ch.is_digit(10) {
            if !accumulating_num_chars {
                accumulating_num_chars = true;
                num_start_idx = output.len() - 1;
                num_string.clear();
            }
            num_string.push(ch);
        }
        else {
            if accumulating_num_chars {
                accumulating_num_chars = false;
                if !num_string.is_empty() {
                    let num = num_string.parse::<i64>().unwrap();
                    if num > 9 {
                        let mut replacement_string = String::new();
                        replacement_string.push('[');
                        replacement_string.push_str(&(num / 2).to_string());
                        replacement_string.push(',');
                        replacement_string.push_str(&(f32::ceil(num as f32 / 2.0_f32) as i64).to_string());
                        replacement_string.push(']');
                        output.replace_range(num_start_idx..(num_start_idx + num_string.len()), &replacement_string);
                        output.push_str(&input[idx + 1..]);
                        break;
                    }
                }
            }
        }
    }

    let updated = *input != output;
    *input = output;
    return updated;
}

pub fn get_snail_magnitude(input: &mut String) -> i64 {
    let mut last_bracket = ' ';
    let mut start_pair_idx: usize = 0;
    let mut end_pair_idx: usize = 0;

    for (idx, ch) in input.chars().enumerate() {
        if ch == '[' {
            last_bracket = ch;
            start_pair_idx = idx;
        }
        else if ch == ']' {
            if last_bracket == '[' {
                end_pair_idx = idx;
                break;
            }
            last_bracket = ch;
        }
    }

    if end_pair_idx != 0 {
        let pair_string = &input[start_pair_idx + 1..end_pair_idx].split(',').collect::<Vec<_>>();
        let val_1 = pair_string[0].parse::<i64>().unwrap();
        let val_2 = pair_string[1].parse::<i64>().unwrap();
        let mag = 3 * val_1 + 2 * val_2;
        input.replace_range(start_pair_idx..end_pair_idx + 1, &mag.to_string());
        get_snail_magnitude(input);
    }

    return input.parse::<i64>().unwrap();
}

#[allow(dead_code)]
fn day_18_part_1(input: &str) -> i64 {
    let mut sum = String::new();

    for line in input.split('\n') {
        if sum.is_empty() {
            sum.push_str(line);
            continue;
        }

        sum.insert(0, '[');
        sum.push(',');
        sum.push_str(line);
        sum.push(']');

        while explode_snail(&mut sum) || split_snail(&mut sum) {
        }
    }

    let ans = get_snail_magnitude(&mut sum);

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_18_part_1() {
        let input =
r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

        let ans = day_18_part_1(&input);

        assert_eq!(ans, 4140);
    }

    #[test]
    fn test_day_18_part_1() {
        let input =
r#"[[7,[1,5]],[[5,7],[[0,8],2]]]
[[[[7,3],[2,2]],3],[[[7,1],[9,1]],2]]
[[[3,[0,2]],[5,2]],8]
[[[[1,5],8],[[0,5],[0,0]]],2]
[[[[9,9],1],[8,[3,2]]],[0,8]]
[[[[0,8],6],6],[[1,5],[[5,5],[4,6]]]]
[[3,[[9,0],[7,6]]],[7,4]]
[6,[[8,[9,7]],[[1,1],[2,6]]]]
[[[[2,2],6],[2,6]],[[8,[3,2]],[2,[6,5]]]]
[9,[1,[1,[4,6]]]]
[8,[[5,[7,7]],[2,2]]]
[[[[3,2],[4,9]],[6,8]],[[[7,9],9],[7,5]]]
[[[[0,4],[9,6]],0],[4,[6,[5,1]]]]
[[3,[4,5]],[1,[[2,8],4]]]
[[[9,[8,3]],[[0,0],2]],[[1,3],[[8,0],[5,3]]]]
[[[[2,4],4],[[5,8],4]],9]
[[1,[[0,8],[1,0]]],[1,[9,2]]]
[[[[7,3],5],[7,[3,1]]],[[3,1],[9,[2,0]]]]
[0,[[7,[9,3]],9]]
[[5,0],[[5,1],[2,3]]]
[[2,[[8,1],0]],[[0,[9,0]],[[3,4],[8,6]]]]
[[[7,5],8],[6,6]]
[[[2,4],[[4,7],[9,6]]],[6,[[3,5],0]]]
[7,[[3,2],[[9,3],9]]]
[[[[9,9],2],[[1,0],6]],[[[6,4],8],[[4,7],[5,6]]]]
[[[1,0],[[3,1],2]],[[[5,3],6],[2,[8,4]]]]
[[4,[[8,1],2]],[2,3]]
[[[[9,6],1],4],1]
[[5,[3,1]],[[[0,5],5],[[4,2],6]]]
[[6,[8,7]],[[[0,9],6],9]]
[[2,[[2,8],6]],[[[7,9],8],[6,[7,6]]]]
[[2,[[4,6],[7,5]]],[[[3,5],[6,4]],[[6,8],4]]]
[[[[1,8],[0,8]],[0,[5,0]]],[[[2,5],[0,6]],8]]
[[[[8,5],[2,3]],[[2,6],4]],[[0,9],1]]
[[2,[6,4]],8]
[[0,8],[[[1,6],[6,3]],[[2,2],8]]]
[0,[[[7,1],[4,7]],5]]
[[[7,[1,8]],[2,[1,3]]],9]
[[1,[9,1]],[[6,8],[6,[7,3]]]]
[[[[5,1],[6,9]],[0,5]],[0,[[6,5],3]]]
[[2,[[3,4],1]],[4,1]]
[[[[1,6],6],[[6,5],0]],[7,[0,4]]]
[[2,[[6,9],[9,4]]],3]
[[[1,3],[7,9]],3]
[1,[3,[[1,0],4]]]
[[[9,1],[4,[5,1]]],[[6,0],[1,2]]]
[[[3,[0,3]],9],[[6,[7,0]],[1,1]]]
[[2,[0,[1,5]]],[[0,[7,7]],[[4,9],[8,4]]]]
[[7,[[5,1],[6,4]]],[[2,1],9]]
[[5,6],[[8,6],[[9,1],[7,2]]]]
[[[[0,6],5],7],[[[6,7],9],1]]
[[4,[0,[6,9]]],[6,[[5,6],3]]]
[[[[0,0],5],[[1,3],0]],[[[4,1],6],7]]
[7,5]
[[[8,[9,5]],[0,1]],[3,[[0,1],9]]]
[[[[2,1],5],1],[[[8,1],[1,9]],1]]
[[[[0,9],[7,3]],[8,0]],[[4,[9,5]],7]]
[6,[[9,[4,7]],1]]
[[7,1],[[9,7],[7,[5,5]]]]
[[4,[6,[7,1]]],[[2,2],[[0,9],3]]]
[[[6,[5,4]],[9,4]],0]
[5,[[[7,8],5],7]]
[[[[1,8],[2,7]],[3,[4,4]]],[[[0,9],[4,5]],[[9,8],[0,6]]]]
[[[[0,2],6],2],[[[5,8],[1,3]],[[2,5],5]]]
[[3,[0,[2,3]]],[[[3,0],8],3]]
[[[[8,5],[7,2]],[8,7]],[[[8,1],[2,5]],5]]
[[[[6,0],[5,6]],[[7,8],[3,5]]],[[[8,3],9],[6,[5,8]]]]
[[7,0],[[[4,3],[4,3]],[[7,1],[8,9]]]]
[[[0,0],[7,0]],[3,[[3,6],1]]]
[[[6,6],1],[0,[1,[1,5]]]]
[[[1,[7,8]],[[7,7],[5,1]]],[0,6]]
[0,[[7,4],[7,7]]]
[[[4,[8,3]],[[2,9],6]],[2,[4,0]]]
[7,[8,8]]
[[[[1,4],1],3],[[3,2],3]]
[[7,[0,4]],[[[9,1],[4,4]],[4,8]]]
[4,[[1,8],9]]
[[[[5,7],[8,5]],[9,2]],[[7,5],7]]
[[[3,[3,2]],3],[8,[9,0]]]
[[2,0],9]
[[8,[[6,4],[2,2]]],[[[4,8],6],[2,[1,5]]]]
[[6,[[0,4],[3,0]]],[[2,5],[3,3]]]
[[[[6,8],[6,1]],[1,3]],[[[4,9],[0,8]],9]]
[[[1,[1,6]],[[2,8],5]],[[[8,1],9],6]]
[[[3,[9,6]],[1,[6,5]]],[[4,7],[[4,4],8]]]
[[[7,6],0],[[7,7],7]]
[[3,0],[1,[[2,1],9]]]
[[9,[[3,7],9]],[[[1,1],4],[0,[3,9]]]]
[[3,[1,[0,1]]],[[0,9],[4,[8,8]]]]
[[[8,[3,2]],[9,5]],[[[1,2],[0,4]],6]]
[[5,4],[[2,4],[0,[7,2]]]]
[[[[9,2],4],[[9,5],[8,8]]],[[7,[7,0]],3]]
[[[[8,2],[1,1]],[[9,3],3]],[[9,[5,2]],[[1,4],[0,6]]]]
[4,[[[2,7],6],[[0,8],0]]]
[[[4,4],3],[0,8]]
[[3,[3,[8,6]]],[5,[[0,6],3]]]
[[[2,[0,9]],[[5,0],[7,3]]],[[8,1],[5,2]]]
[[7,[[3,2],[6,8]]],[2,[[4,7],[3,1]]]]
[[[[6,3],6],4],[[[7,8],[5,1]],[[3,0],5]]]
[[3,[9,5]],[6,9]]"#;

        let ans = day_18_part_1(&input);

        assert_eq!(ans, 3987);
    }
}

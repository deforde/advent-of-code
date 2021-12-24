#[derive(Debug)]
struct ALU {
    x: i64,
    y: i64,
    z: i64,
    w: i64
}

fn execute_store(alu: &mut ALU, reg: &str, val: i64) {
    match reg {
        "x" => alu.x = val,
        "y" => alu.y = val,
        "z" => alu.z = val,
        "w" => alu.w = val,
        _ => panic!()
    }
}

fn get_val(alu: &ALU, reg: &str) -> i64 {
    match reg {
        "x" => return alu.x,
        "y" => return alu.y,
        "z" => return alu.z,
        "w" => return alu.w,
        _ => return reg.parse::<i64>().unwrap()
    }
}

fn execute_add(alu: &mut ALU, reg_a: &str, reg_b: &str) {
    let a = get_val(alu, reg_a);
    let b = get_val(alu, reg_b);
    let val = a + b;
    execute_store(alu, reg_a, val);
}

fn execute_mul(alu: &mut ALU, reg_a: &str, reg_b: &str) {
    let a = get_val(alu, reg_a);
    let b = get_val(alu, reg_b);
    let val = a * b;
    execute_store(alu, reg_a, val);
}

fn execute_div(alu: &mut ALU, reg_a: &str, reg_b: &str) {
    let a = get_val(alu, reg_a);
    let b = get_val(alu, reg_b);
    let val = a / b;
    execute_store(alu, reg_a, val);
}

fn execute_mod(alu: &mut ALU, reg_a: &str, reg_b: &str) {
    let a = get_val(alu, reg_a);
    let b = get_val(alu, reg_b);
    let val = a % b;
    execute_store(alu, reg_a, val);
}

fn execute_eql(alu: &mut ALU, reg_a: &str, reg_b: &str) {
    let a = get_val(alu, reg_a);
    let b = get_val(alu, reg_b);
    let val = if a == b { 1i64 } else { 0i64 };
    execute_store(alu, reg_a, val);
}

fn execute_program(prog: &str, input: Vec<i64>) -> ALU {
    let mut alu = ALU {
      x: 0,
      y: 0,
      z: 0,
      w: 0  
    };

    let mut input_iter = input.iter();
    
    for line in prog.split('\n') {
        let instr = line.split_whitespace().collect::<Vec<_>>();

        let op = instr[0];

        match op {
            "inp" => {
                if let Some(val) = input_iter.next() {
                    execute_store(&mut alu, instr[1], *val);
                }
                else {
                    panic!();
                }
            },
            "add" => execute_add(&mut alu, instr[1], instr[2]),
            "mul" => execute_mul(&mut alu, instr[1], instr[2]),
            "div" => execute_div(&mut alu, instr[1], instr[2]),
            "mod" => execute_mod(&mut alu, instr[1], instr[2]),
            "eql" => execute_eql(&mut alu, instr[1], instr[2]),
            _ => panic!()
        }
    }

    return alu;
}

fn decompose_program(prog: &str) -> Vec::<(i64, i64, i64)> {
    let mut variables = Vec::<(i64, i64, i64)>::new();

    for block in prog.split("inp w\n") {
        if block.is_empty() {
            continue;
        }
        let lines = block.split('\n').collect::<Vec<_>>();

        let a = lines[3].split_whitespace().collect::<Vec<_>>()[2].parse::<i64>().unwrap();
        let b = lines[4].split_whitespace().collect::<Vec<_>>()[2].parse::<i64>().unwrap();
        let c = lines[14].split_whitespace().collect::<Vec<_>>()[2].parse::<i64>().unwrap();

        variables.push((a, b, c));
    }

    return variables;
}

pub fn get_model_num_ranges(prog: &str) -> Vec::<Vec<i64>> {
    let vars = decompose_program(prog);

    let mut model_num_ranges: Vec::<Vec<i64>> = vec![Vec::<i64>::new(); 14];

    let mut stack = Vec::<usize>::new();
    for (idx, (a, b, _)) in vars.iter().enumerate() {
        if *a == 1 {
            stack.push(idx);
        }
        else {
            let other_idx = stack.pop().unwrap();
            let (_, _, other_c) = vars[other_idx];
            let diff = other_c + b;
            if diff < 0 {
                model_num_ranges[other_idx] = (1-diff..=9).collect();
                model_num_ranges[idx] = (1..=9+diff).collect();
            }
            else {
                model_num_ranges[other_idx] = (1..=9-diff).collect();
                model_num_ranges[idx] = (1+diff..=9).collect();
            }
        }
    }

    return model_num_ranges;
}

#[allow(dead_code)]
fn day_24_part_1(input: &str) -> i64 {
    let model_num_ranges = get_model_num_ranges(input);

    let mut model_num: i64 = 0;
    for model_num_digit_range in model_num_ranges.iter() {
        model_num *= 10;
        model_num += model_num_digit_range.last().unwrap();
    }

    return model_num;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_day_24_part_1() {
        let prog =
r#"inp x
mul x -1"#;

        let alu = execute_program(&prog, vec![3i64]);

        assert_eq!(alu.x, -3);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 0);
        assert_eq!(alu.w, 0);
    }

    #[test]
    fn example_2_day_24_part_1() {
        let prog =
r#"inp z
inp x
mul z 3
eql z x"#;

        let mut alu = execute_program(&prog, vec![2i64, 6i64]);

        assert_eq!(alu.x, 6);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 1);
        assert_eq!(alu.w, 0);

        alu.x = 0;
        alu.y = 0;
        alu.z = 0;
        alu.w = 0;
        alu = execute_program(&prog, vec![2i64, 4i64]);

        assert_eq!(alu.x, 4);
        assert_eq!(alu.y, 0);
        assert_eq!(alu.z, 0);
        assert_eq!(alu.w, 0);
    }

    #[test]
    fn example_3_day_24_part_1() {
        let prog =
r#"inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2"#;

        let alu = execute_program(&prog, vec![0b1010i64]);

        assert_eq!(alu.x, 0);
        assert_eq!(alu.y, 1);
        assert_eq!(alu.z, 0);
        assert_eq!(alu.w, 1);
    }

    #[test]
    fn test_day_24_part_1() {
        let input =
r#"inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 1
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -2
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -6
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x 0
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y"#;

        let ans = day_24_part_1(&input);

        assert_eq!(ans, 59998426997979)
    }
}
use std::collections::HashMap;

fn parse_input(input: &str) -> (char, HashMap::<(char, char), usize>, HashMap::<(char, char), char>) {
    let mut polymer = HashMap::<(char, char), usize>::new();
    let mut insertion_rules = HashMap::<(char, char), char>::new();

    let blocks: Vec<&str> = input.split("\n\n").collect();

    let first_char = blocks[0].chars().nth(0).unwrap();

    for idx in 0..blocks[0].len() - 1 {
        let char_pair = (blocks[0].chars().nth(idx).unwrap(), blocks[0].chars().nth(idx + 1).unwrap());
        if let Some(value) = polymer.get_mut(&char_pair) {
            *value += 1;
        }
        else {
            polymer.insert(char_pair, 1);
        }
    }

    for insertion_rule in blocks[1].split('\n') {
        let insertion_rule_strings: Vec::<&str> = insertion_rule.split(" -> ").collect::<Vec<&str>>();

        let pair = (insertion_rule_strings[0].chars().nth(0).unwrap(), insertion_rule_strings[0].chars().nth(1).unwrap());

        let insertion_char = insertion_rule_strings[1].chars().nth(0).unwrap();

        insertion_rules.insert(pair, insertion_char);
    }

    return (first_char, polymer, insertion_rules);
}

#[allow(dead_code)]
pub fn day_14_part_1(input: &str, n_steps: usize) -> usize {
    let (first_char, mut polymer, insertion_rules) = parse_input(&input);

    for _ in 0..n_steps {
        let mut new_polymer = HashMap::<(char, char), usize>::new();

        for (pair_key, count) in polymer {
            if let Some(insertion_char) = insertion_rules.get(&pair_key) {
                let new_pair_1 = (pair_key.0, *insertion_char);
                let new_pair_2 = (*insertion_char, pair_key.1);

                if let Some(value) = new_polymer.get_mut(&new_pair_1) {
                    *value += count;
                }
                else {
                    new_polymer.insert(new_pair_1, count);
                }

                if let Some(value) = new_polymer.get_mut(&new_pair_2) {
                    *value += count;
                }
                else {
                    new_polymer.insert(new_pair_2, count);
                }
            }
        }

        polymer = new_polymer;
    }

    let mut polymer_char_counts = HashMap::from([
        (first_char, 1),
    ]);
    for (pair_key, count) in polymer {
        if let Some(value) = polymer_char_counts.get_mut(&pair_key.1) {
            *value += count;
        }
        else {
            polymer_char_counts.insert(pair_key.1, count);
        }
    }

    let mut min_char_count: usize = usize::MAX;
    let mut max_char_count: usize = 0;
    for (_, count) in polymer_char_counts {
        min_char_count = usize::min(min_char_count, count);
        max_char_count = usize::max(max_char_count, count);
    }

    return max_char_count - min_char_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_day_14_part_1() {
        let input =
r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

        let ans = day_14_part_1(&input, 10);

        assert_eq!(ans, 1588);
    }

    #[test]
    fn test_day_14_part_1() {
        let input =
r#"SFBBNKKOHHHPFOFFSPFV

HB -> C
KO -> S
KK -> N
PF -> F
VB -> F
KC -> S
BP -> H
SS -> H
BS -> B
PB -> O
VH -> C
BK -> S
BO -> F
HN -> V
NN -> K
PV -> C
NH -> P
KP -> N
NB -> V
NF -> V
PP -> O
PN -> B
VN -> K
SC -> O
NS -> N
SV -> B
BV -> P
FV -> F
OK -> H
HF -> F
CV -> K
KB -> C
OB -> B
NO -> V
OF -> B
HP -> C
BB -> F
FB -> H
OC -> K
NV -> H
OV -> S
OP -> N
SP -> N
FK -> F
VV -> B
VK -> H
OS -> F
CO -> F
CH -> V
HV -> V
FN -> B
CS -> F
PS -> F
HS -> F
VO -> K
NP -> F
FP -> B
KF -> P
CC -> N
BF -> S
VP -> F
HO -> H
FC -> F
BH -> K
NK -> S
BN -> V
SH -> K
CP -> B
VS -> K
ON -> S
FS -> P
HK -> F
PC -> O
KN -> H
CK -> N
HH -> N
CN -> S
BC -> K
PH -> N
OO -> B
FO -> O
SK -> B
FF -> V
VC -> N
SF -> N
KH -> V
SO -> F
KS -> H
SB -> K
VF -> V
PK -> O
OH -> N
HC -> F
PO -> O
NC -> F
FH -> V
KV -> V
CB -> C
CF -> O
SN -> H"#;

        let ans = day_14_part_1(&input, 10);

        assert_eq!(ans, 4517);
    }
}
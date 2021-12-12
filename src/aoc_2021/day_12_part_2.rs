use crate::aoc_2021::day_12_part_1::CaveCategory;
use crate::aoc_2021::day_12_part_1::Cave;
use crate::aoc_2021::day_12_part_1::parse_input;

fn get_all_small_cave_indices_in_path(caves: &Vec::<Cave>, current_path: &Vec<(usize, String)>) -> Vec::<usize> {
    let mut small_cave_indices = Vec::<usize>::new();
    for cave_pair in current_path {
        match caves[cave_pair.0].category {
            CaveCategory::Small => {
                small_cave_indices.push(cave_pair.0);
            },
            _ => (),
        }
    }
    small_cave_indices.sort();
    return small_cave_indices;
}

fn get_all_paths(caves: &Vec::<Cave>, complete_paths: &mut Vec::<Vec<(usize, String)>>, current_path: &Vec<(usize, String)>, current_cave_idx: usize) {
    let current_cave = &caves[current_cave_idx];

    if current_cave.name == "end" {
        complete_paths.push(current_path.clone()); //Path is complete, add it to completed paths and return
        return;
    }

    for &connected_cave_idx in &current_cave.connections {
        if caves[connected_cave_idx].name == "start" {
            continue;
        }

        match caves[connected_cave_idx].category {
            CaveCategory::Small => {
                let small_cave_indices = get_all_small_cave_indices_in_path(caves, current_path);
                let mut duplicated_small_cave_indices = small_cave_indices.clone();
                duplicated_small_cave_indices.retain(|idx| small_cave_indices.iter().filter(|&other_idx| other_idx == idx).count() > 1);
                duplicated_small_cave_indices.dedup();

                // println!("current_path = {:?}", current_path);
                // println!("small_cave_indices = {:?}", small_cave_indices);
                // println!("duplicated_small_cave_indices = {:?}", duplicated_small_cave_indices);

                if !(duplicated_small_cave_indices.is_empty() || (duplicated_small_cave_indices.len() == 1 && duplicated_small_cave_indices[0] != connected_cave_idx)) {
                    continue; //Can enter up to 1 small cave twice, but all other small caves no more than once
                }
            },
            _ => (),
        }

        let mut new_path = current_path.clone();
        new_path.push((connected_cave_idx, caves[connected_cave_idx].name.clone()));
        get_all_paths(&caves, complete_paths, &new_path, connected_cave_idx);
    }
}

#[allow(dead_code)]
fn day_12_part_2(input: &str) -> usize {
    let caves = parse_input(&input);

    let mut complete_paths = Vec::<Vec<(usize, String)>>::new();
    let start_cave_index = caves.iter().position(|cave| cave.name == "start").unwrap();
    let current_path: Vec::<(usize, String)> = vec![(start_cave_index, "start".into())];

    get_all_paths(&caves, &mut complete_paths, &current_path, start_cave_index);

    //println!("{:?}", caves);
    //println!("{:?}", complete_paths);

    return complete_paths.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_day_12_part_2() {
        let input =
r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

        let ans = day_12_part_2(&input);

        assert_eq!(ans, 36);
    }

    #[test]
    fn example_2_day_12_part_2() {
        let input =
r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

        let ans = day_12_part_2(&input);

        assert_eq!(ans, 103);
    }

    #[test]
    fn example_3_day_12_part_2() {
        let input =
r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

        let ans = day_12_part_2(&input);

        assert_eq!(ans, 3509);
    }

    #[test]
    fn test_day_12_part_2() {
        let input =
r#"DA-xn
KD-ut
gx-ll
dj-PW
xn-dj
ll-ut
xn-gx
dg-ak
DA-start
ut-gx
YM-ll
dj-DA
ll-xn
dj-YM
start-PW
dj-start
PW-gx
YM-gx
xn-ak
PW-ak
xn-PW
YM-end
end-ll
ak-end
ak-DA"#;

        let ans = day_12_part_2(&input);

        assert_eq!(ans, 85883);
    }
}
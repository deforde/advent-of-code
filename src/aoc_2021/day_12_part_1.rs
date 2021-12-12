#[derive(Debug)]
pub enum CaveCategory {
    Large,
    Small
}

#[derive(Debug)]
pub struct Cave {
    pub connections: Vec::<usize>,
    pub name: String,
    pub category: CaveCategory,
}

pub fn get_cave_index(cave_string: &str, caves: &mut Vec::<Cave>) -> usize {
    if let Some(extant_cave_index) = caves.into_iter().position(|cave| cave.name == cave_string) {
        return extant_cave_index;
    }

    let category = if cave_string.chars().nth(0).unwrap().is_uppercase() {
        CaveCategory::Large
    } else {
        CaveCategory::Small
    };

    let new_cave = Cave {
        connections: Vec::<usize>::new(),
        name: String::from(cave_string),
        category: category,
    };

    caves.push(new_cave);

    return caves.len() - 1;
}

pub fn parse_input(input: &str) -> Vec::<Cave> {
    let mut caves = Vec::<Cave>::new();

    for line in input.split('\n') {
        let cave_strings: Vec<&str> = line.split('-').collect();

        let first_cave_string = cave_strings[0];
        let second_cave_string = cave_strings[1];

        let first_cave_idx = get_cave_index(first_cave_string, &mut caves);
        let second_cave_idx = get_cave_index(second_cave_string, &mut caves);

        let first_cave = &mut caves[first_cave_idx];
        first_cave.connections.push(second_cave_idx);
        first_cave.connections.sort();
        first_cave.connections.dedup();

        let second_cave = &mut caves[second_cave_idx];
        second_cave.connections.push(first_cave_idx);
        second_cave.connections.sort();
        second_cave.connections.dedup();

        //println!("line: \"{}\" - connecting caves #{} and #{}", line, first_cave_idx, second_cave_idx);
    }

    return caves;
}

fn get_all_paths(caves: &Vec::<Cave>, complete_paths: &mut Vec::<Vec<(usize, String)>>, current_path: &Vec<(usize, String)>, current_cave_idx: usize) {
    let current_cave = &caves[current_cave_idx];

    if current_cave.name == "end" {
        complete_paths.push(current_path.clone()); //Path is complete, add it to completed paths and return
        return;
    }

    for &connected_cave_idx in &current_cave.connections {
        match caves[connected_cave_idx].category {
            CaveCategory::Small => {
                if current_path.iter().find(|&cave_pair| cave_pair.0 == connected_cave_idx) != None {
                    continue; //Can't enter a small cave more than once, so don't continue down this path
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
fn day_12_part_1(input: &str) -> usize {
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
    fn example_1_day_12_part_1() {
        let input =
r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

        let ans = day_12_part_1(&input);

        assert_eq!(ans, 10);
    }

    #[test]
    fn example_2_day_12_part_1() {
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

        let ans = day_12_part_1(&input);

        assert_eq!(ans, 19);
    }

    #[test]
    fn example_3_day_12_part_1() {
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

        let ans = day_12_part_1(&input);

        assert_eq!(ans, 226);
    }

    #[test]
    fn test_day_12_part_1() {
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

        let ans = day_12_part_1(&input);

        assert_eq!(ans, 3369);
    }
}
use std::fs;

fn main() {
    let data = fs::read_to_string("/home/flarga/Projects/aoc24/day_2/input.txt").unwrap();
//     let data = "7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9
// ";

    let mut safe_reports = 0;

    for line in data.lines() {
        let line_split = line.split_whitespace();
        let mut levels = line_split.map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let safe_forwards = process(&levels);
        levels.reverse();
        let safe_backwards = process(&levels);

        if safe_forwards || safe_backwards {
            safe_reports += 1;
        }

    }
    println!("{}", safe_reports);
}

fn process(levels: &[u32]) -> bool {
    let mut safe = true;
    let mut last_level = levels[0];
    let mut last_order = levels[1].cmp(&levels[0]);

    let mut freebie_used: bool = false;

    for level in levels[1..].iter() {
        let new_order = level.cmp(&last_level);
        let difference = level.abs_diff(last_level);

        if new_order != last_order || !(1..=3).contains(&difference) {
            if !freebie_used {
                freebie_used = true;
                continue;
            }

            safe = false;
            break;
        }

        last_level = *level;
        last_order = new_order;
    }

    safe
}

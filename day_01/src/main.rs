use std::fs;
use std::iter;

fn main() {
    let data = fs::read_to_string("day_01/input.txt").unwrap();

    let size = data.lines().count();

    let mut left_list = Vec::with_capacity(size);
    let mut right_list = Vec::with_capacity(size);

    for line in data.lines() {
        let mut line_split = line.split_whitespace();

        let left = line_split.next().unwrap().parse::<u32>().unwrap();
        left_list.push(left);

        let right = line_split.next().unwrap().parse::<u32>().unwrap();
        right_list.push(right);
    }

    left_list.sort();
    right_list.sort();

    let total_distance: u32 = iter::zip(&left_list, &right_list)
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("{}", total_distance);

    let similarity: u32 = left_list
        .iter()
        .map(|left| *left * right_list.iter().filter(|right| **right == *left).count() as u32)
        .sum();

    println!("{}", similarity);
}

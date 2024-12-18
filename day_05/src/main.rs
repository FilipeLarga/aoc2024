use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Div;

fn main() {
    let data = fs::read_to_string("day_05/src/input.txt").unwrap();
    //     let data = "47|53
    // 97|13
    // 97|61
    // 97|47
    // 75|29
    // 61|13
    // 75|53
    // 29|13
    // 97|29
    // 53|29
    // 61|53
    // 97|53
    // 61|29
    // 47|13
    // 75|47
    // 97|75
    // 47|61
    // 75|61
    // 47|29
    // 75|13
    // 53|13
    //
    // 75,47,61,53,29
    // 97,61,53,29,13
    // 75,29,13
    // 75,97,47,61,53
    // 61,13,29
    // 97,13,75,29,47
    // ";
    let mut rules: HashMap<u16, HashSet<u16>> = HashMap::new();

    let mut finished_rules = false;

    let mut correct_updates: Vec<Vec<u16>> = Vec::new();
    let mut wrong_updates: Vec<Vec<u16>> = Vec::new();

    'outer: for line in data.lines() {
        if line.is_empty() {
            finished_rules = true;
            continue;
        }

        if !finished_rules {
            let mut split = line.split('|');
            let page = split.next().unwrap().parse::<u16>().unwrap();
            let successor = split.next().unwrap().parse::<u16>().unwrap();

            rules.entry(page).or_default().insert(successor);
            continue;
        }

        let update: Vec<u16> = line.split(',').map(|x| x.parse::<u16>().unwrap()).collect();

        for i in 0..update.len() {
            let page_rules = rules.get(&update[i]);
            if let Some(rules) = page_rules {
                for j in 0..i {
                    if rules.contains(&update[j]) {
                        wrong_updates.push(update);
                        continue 'outer;
                    }
                }
            }
        }

        correct_updates.push(update);
    }

    let sum_middle_pages: u16 = correct_updates
        .iter()
        .map(|update| update[update.len().div(2)])
        .sum();

    println!("Part 1: {}", sum_middle_pages);

    for update in wrong_updates.iter_mut() {
        update.sort_by(|a, b| { 
            let a_rules = rules.get(a);
            let b_rules = rules.get(b);
            
            if a_rules.is_none() && b_rules.is_none() {
                return Ordering::Equal;
            }
            
            if let Some(a_rules) = a_rules {
                if a_rules.contains(b) {
                    return Ordering::Less;
                }
            }

            if let Some(b_rules) = b_rules {
                if b_rules.contains(a) {
                    return Ordering::Greater;
                }
            }
            
            Ordering::Equal
        });
    }

    let sum_middle_pages: u16 = wrong_updates
        .iter()
        .map(|update| update[update.len().div(2)])
        .sum();
    println!("Part 2: {}", sum_middle_pages);
}

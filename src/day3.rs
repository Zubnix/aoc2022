use std::collections::HashSet;

const PRIORITIES: [char; 52] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

#[allow(dead_code)]
pub fn a() {
    let input = include_str!("3.txt");
    let sum: usize = input.lines()
        .flat_map(|line| -> Vec<char> {
            let split_at = line.len() / 2;
            let compartments = line.split_at(split_at);

            let compartment0: HashSet<char> = HashSet::from_iter(compartments.0.chars());
            let compartment1: HashSet<char> = HashSet::from_iter(compartments.1.chars());

            compartment0.intersection(&compartment1).cloned().collect::<Vec<char>>()
        })
        .map(|common_item| PRIORITIES.iter().position(|&x| x == common_item).unwrap() + 1)
        .sum();
    println!("3a: {sum}");
}

#[allow(dead_code)]
pub fn b() {
    let input = include_str!("3.txt");
    let sum: usize = input.lines()
        .collect::<Vec<&str>>()
        .as_slice().chunks(3)
        .map(|triplet| {
            let elf0: HashSet<char> = HashSet::from_iter(triplet[0].chars());
            let elf1: HashSet<char> = HashSet::from_iter(triplet[1].chars());
            let elf2: HashSet<char> = HashSet::from_iter(triplet[2].chars());

            let first_intersect: HashSet<char> = elf0.intersection(&elf1).cloned().collect();
            let badge: HashSet<char> = first_intersect.intersection(&elf2).cloned().collect();

            return badge.into_iter().next().unwrap();
        })
        .map(|common_item| PRIORITIES.iter().position(|&x| x == common_item).unwrap() + 1)
        .sum();
    println!("3b: {sum}");
}
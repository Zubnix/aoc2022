use std::collections::HashSet;

pub fn a() {
    let input = include_str!("6.txt");
    let position: usize = input.as_bytes()
        .windows(4)
        .enumerate()
        .find(|(index, window)| {
            let window_hash_set: HashSet<&u8> = HashSet::from_iter(window.iter());
            return window_hash_set.len() == window.len();
        })
        .map(|(index, window)| index + 4)
        .unwrap();
    println!("6a: {position}")
}

pub fn b() {
    let input = include_str!("6.txt");
    let position: usize = input.as_bytes()
        .windows(14)
        .enumerate()
        .find(|(index, window)| {
            let window_hash_set: HashSet<&u8> = HashSet::from_iter(window.iter());
            return window_hash_set.len() == window.len();
        })
        .map(|(index, window)| index + 14)
        .unwrap();
    println!("6b: {position}")
}
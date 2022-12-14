pub fn a() {
    let input = include_str!("5.txt");
    let crates_and_operations: Vec<&str> = input.split("\n\n").collect();
    let crate_operations: Vec<&str> = crates_and_operations[1].split("\n").collect();

    let iterable_crate_lines = crates_and_operations[0].replace("\n", " \n");
    let mut crate_lines: Vec<&str> = iterable_crate_lines.split("\n").collect();
    let last_crate_line = crate_lines.last().unwrap();
    let mut crate_piles: Vec<Vec<String>> = last_crate_line.split_whitespace().map(|_| Vec::new()).collect();
    crate_lines.pop();

    crate_lines.iter().for_each(|crate_line| {
        crate_line.chars().collect::<Vec<char>>().chunks(4).enumerate().for_each(|(index, elf_crate)| {
            if !(elf_crate[0] == ' ') {
                let elf_crate_string = String::from_iter(&elf_crate[..3]);
                crate_piles[index].insert(0, elf_crate_string);
            }
        });
    });

    crate_operations.iter().for_each(|crate_operation| {
        let operation: Vec<&str> = crate_operation.split_whitespace().collect();
        let moves: u32 = operation[1].parse().unwrap();
        let from: usize = operation[3].parse().unwrap();
        let to: usize = operation[5].parse().unwrap();

        for _i in 0..moves {
            let crate_ = crate_piles[from - 1].pop().unwrap();
            crate_piles[to - 1].push(crate_);
        }
    });

    let top_crates_vec: Vec<char> = crate_piles.iter()
        .map(|crate_pile| crate_pile.last().unwrap().chars().collect::<Vec<char>>()[1])
        .collect();
    let top_crates = String::from_iter(top_crates_vec);
    println!("5a: {top_crates}");
}

pub fn b() {
    let input = include_str!("5.txt");
    let crates_and_operations: Vec<&str> = input.split("\n\n").collect();
    let crate_operations: Vec<&str> = crates_and_operations[1].split("\n").collect();

    let iterable_crate_lines = crates_and_operations[0].replace("\n", " \n");
    let mut crate_lines: Vec<&str> = iterable_crate_lines.split("\n").collect();
    let last_crate_line = crate_lines.last().unwrap();
    let mut crate_piles: Vec<Vec<String>> = last_crate_line.split_whitespace().map(|_| Vec::new()).collect();
    crate_lines.pop();

    crate_lines.iter().for_each(|crate_line| {
        crate_line.chars().collect::<Vec<char>>().chunks(4).enumerate().for_each(|(index, elf_crate)| {
            if !(elf_crate[0] == ' ') {
                let elf_crate_string = String::from_iter(&elf_crate[..3]);
                crate_piles[index].insert(0, elf_crate_string);
            }
        });
    });

    crate_operations.iter().for_each(|crate_operation| {
        let operation: Vec<&str> = crate_operation.split_whitespace().collect();
        let quant: usize = operation[1].parse().unwrap();
        let from: usize = operation[3].parse().unwrap();
        let to: usize = operation[5].parse().unwrap();

        let from_crates_size = crate_piles[from - 1].len();
        let mut crates: Vec<String> = crate_piles[from - 1].drain(from_crates_size-quant..).collect();
        crate_piles[to - 1].append(&mut crates);
    });

    let top_crates_vec: Vec<char> = crate_piles.iter()
        .map(|crate_pile| crate_pile.last().unwrap().chars().collect::<Vec<char>>()[1])
        .collect();
    let top_crates = String::from_iter(top_crates_vec);
    println!("5b: {top_crates}");
}
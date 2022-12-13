#[allow(dead_code)]
pub fn a() {
    let input = include_str!("1.txt");
    let max: u32 = input.split("\n\n")
        .map(|elf_inventory| {
            return elf_inventory
                .lines()
                .map(|c| c.parse::<u32>().unwrap())
                .sum();
        })
        .max()
        .unwrap();
    println!("1a: {max}");
}

#[allow(dead_code)]
pub fn b() {
    let input = include_str!("1.txt");
    let mut maximums: Vec<u32> = input.split("\n\n")
        .map(|elf_inventory| -> u32 {
            return elf_inventory
                .lines()
                .map(|c| c.parse::<u32>().unwrap())
                .sum();
        })
        .collect();
    maximums.sort_by(|a, b| b.cmp(a));
    let top_3 = &maximums[..3];
    let sum: u32 = top_3.iter().sum();
    println!("1b: {sum}");
}
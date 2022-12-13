pub fn a() {
    let input = include_str!("4.txt");
    let count = input.lines()
        .map(|line| -> Vec<u32> {
            return line.split(|c| c == ',' || c == '-')
                .map(|range_coord| range_coord.parse::<u32>().unwrap())
                .collect();
        })
        .filter(|range_coords| {
            (range_coords[0] <= range_coords[2] && range_coords[1] >= range_coords[3]) || (range_coords[2] <= range_coords[0] && range_coords[3] >= range_coords[1])
        })
        .count();
    println!("4a: {count}");
}

pub fn b() {
    let input = include_str!("4.txt");
    let count = input.lines()
        .map(|line| -> Vec<u32> {
            return line.split(|c| c == ',' || c == '-')
                .map(|range_coord| range_coord.parse::<u32>().unwrap())
                .collect();
        })
        .filter(|range_coords| {
            return range_coords[0] >= range_coords[2] && range_coords[0] <= range_coords[3] ||
                range_coords[1] >= range_coords[2] && range_coords[0] <= range_coords[3] ||
                range_coords[2] >= range_coords[0] && range_coords[2] <= range_coords[1] ||
                range_coords[3] >= range_coords[0] && range_coords[3] <= range_coords[1]
        })
        .count();
    println!("4b: {count}");
}
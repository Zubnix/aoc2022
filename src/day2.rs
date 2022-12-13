#[allow(dead_code)]
pub fn a() {
    // A, X = Rock (1pt); B, Y = Paper (2pt); C, Z = Scissors (3pt)
    // Lost (0pt), Draw (3pt), Win (6pt)
    let input = include_str!("2.txt");
    let points: u32 = input.lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|split_line| {
            let opponent = split_line[0];
            let mine = split_line[1];
            let mut points: u32 = 0;
            if mine == "X" {
                points += 1;
                if opponent == "A" {
                    // draw
                    points += 3;
                } else if opponent == "B" {
                    // lost
                    points += 0;
                } else if opponent == "C" {
                    // win
                    points += 6;
                }
            } else if mine == "Y" {
                points += 2;
                if opponent == "A" {
                    // win
                    points += 6;
                } else if opponent == "B" {
                    // draw
                    points += 3;
                } else if opponent == "C" {
                    // lost
                    points += 0;
                }
            } else if mine == "Z" {
                points += 3;
                if opponent == "A" {
                    // lost
                    points += 0;
                } else if opponent == "B" {
                    // win
                    points += 6;
                } else if opponent == "C" {
                    // draw
                    points += 3;
                }
            }
            return points;
        })
        .sum();
    println!("2a: {points}");
}

#[allow(dead_code)]
pub fn b() {
    // A = Rock (1pt); B = Paper (2pt); C = Scissors (3pt)
    // X = Lost; Y = Draw; Z = Win
    // Lost (0pt), Draw (3pt), Win (6pt)
    let input = include_str!("2.txt");
    let points: u32 = input.lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|split_line| {
            let opponent = split_line[0];
            let mine = split_line[1];
            let mut points: u32 = 0;
            if mine == "X" {
                // lost
                points += 0;
                if opponent == "A" {
                    // scissors
                    points += 3
                } else if opponent == "B" {
                    // rock
                    points += 1
                } else if opponent == "C" {
                    // paper
                    points += 2
                }
            } else if mine == "Y" {
                // draw
                points += 3;
                if opponent == "A" {
                    // rock
                    points += 1
                } else if opponent == "B" {
                    // paper
                    points += 2
                } else if opponent == "C" {
                    // scissors
                    points += 3
                }
            } else if mine == "Z" {
                // win
                points += 6;
                if opponent == "A" {
                    // paper
                    points += 2
                } else if opponent == "B" {
                    // scissors
                    points += 3
                } else if opponent == "C" {
                    // rock
                    points += 1
                }
            }
            return points;
        })
        .sum();
    println!("2b: {points}");
}
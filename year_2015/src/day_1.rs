pub fn day1_part1() {
    let input = lib::handle_input_file("input/year_2015/day_1.txt");
    let nb_floor = input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });

    println!("Day 1 - Part 1: Floor: {}", nb_floor);
}

pub fn day1_part2() {
    let input = lib::handle_input_file("input/year_2015/day_1.txt");
    let mut position = 0;
    let mut nb_floor = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            nb_floor += 1;
        } else if c == ')' {
            nb_floor -= 1;
            if nb_floor == -1 {
                position = i + 1;
                break;
            }
        }
    }

    println!("Day 1 - Part 2: Position: {}", position);
}

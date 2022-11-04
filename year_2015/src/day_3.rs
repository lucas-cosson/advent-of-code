use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

pub fn day3_part1() {
    let input = lib::handle_input_file("input/year_2015/day_3.txt");
    let nb_houses = input
        .chars()
        .fold((Position { x: 0, y: 0 }, HashSet::new()), |mut acc, c| {
            match c {
                '>' => acc.0.x += 1,
                '<' => acc.0.x -= 1,
                '^' => acc.0.y += 1,
                'v' => acc.0.y -= 1,
                _ => (),
            }
            acc.1.insert(acc.0);
            acc
        })
        .1
        .len();
    println!(
        "Day 3 - Part 1: Number of house with at least one present: {}",
        nb_houses
    );
}

pub fn day3_part2() {
    let input = lib::handle_input_file("input/year_2015/day_3.txt");
    let nb_houses = input
        .chars()
        .fold(
            (
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                false,
                HashSet::new(),
            ),
            |mut acc, c| {
                if acc.2 {
                    match c {
                        '>' => acc.0.x += 1,
                        '<' => acc.0.x -= 1,
                        '^' => acc.0.y += 1,
                        'v' => acc.0.y -= 1,
                        _ => (),
                    }
                    acc.3.insert(acc.0);
                } else {
                    match c {
                        '>' => acc.1.x += 1,
                        '<' => acc.1.x -= 1,
                        '^' => acc.1.y += 1,
                        'v' => acc.1.y -= 1,
                        _ => (),
                    }
                    acc.3.insert(acc.1);
                }
                acc.2 = !acc.2;
                acc
            },
        )
        .3
        .len();
    println!(
        "Day 3 - Part 2: Number of house with at least one present: {}",
        nb_houses
    );
}

pub fn day5_part1() {
    let input = lib::handle_input_file("input/year_2015/day_5.txt");
    let nb_nice_words = input
        .lines()
        .filter(|line| {
            !line.contains("ab")
                && !line.contains("cd")
                && !line.contains("pq")
                && !line.contains("xy")
        })
        .filter(|line| line.chars().filter(|c| "aeiou".contains(*c)).count() >= 3)
        .filter(|line| line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b))
        .count();
    println!("Day 5 - Part 1: Number of nice words: {}", nb_nice_words);
}

pub fn day5_part2() {
    let input = lib::handle_input_file("input/year_2015/day_5.txt");
    let nb_nice_words = input
        .lines()
        .filter(|line| {
            line.chars()
                .zip(line.chars().skip(1))
                .zip(line.chars().skip(2))
                .any(|((a, _), c)| a == c)
        })
        .filter(|line| line.chars().zip(line.chars().skip(2)).any(|(a, b)| a == b))
        .count();
    println!("Day 5 - Part 1: Number of nice words: {}", nb_nice_words);
}

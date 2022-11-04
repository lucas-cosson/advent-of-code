pub fn day4_part1() {
    let input = "iwrupvqb";

	let mut i = 0;
	loop {
		let hash = md5::compute(format!("{}{}", input, i));
		if hash[0] == 0 && hash[1] == 0 && hash[2] < 16 {
			println!("Day 4 - Part 1: Number hash: {}", i);
			break;
		}
		i += 1;
	}
}

pub fn day4_part2() {
	let input = "iwrupvqb";

	let mut i = 0;
	loop {
		let hash = md5::compute(format!("{}{}", input, i));
		if hash[0] == 0 && hash[1] == 0 && hash[2] == 0 {
			println!("Day 4 - Part 2: Number hash: {}", i);
			break;
		}
		i += 1;
	}
}

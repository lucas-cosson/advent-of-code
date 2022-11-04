pub fn day2_part1() {
	let input = lib::handle_input_file("input/year_2015/day_2.txt");
	let split_string: Vec<&str> = input.split("\n").collect();
	let mut total = 0;

	for str in split_string {
		let dimensions: Vec<&str> = str.split("x").collect();
		let l = dimensions[0].parse::<i32>().unwrap();
		let w = dimensions[1].parse::<i32>().unwrap();
		let h = dimensions[2].parse::<i32>().unwrap();

		let mut sides = vec![l*w, w*h, h*l];
		sides.sort();

		total += sides.iter().fold(0, |acc, x| acc + 2*x);
		total += sides[0];
	}

	println!("Day 2 - Part 1: Square feet of wrapping paper: {}", total);
}

pub fn day2_part2() {
	let input = lib::handle_input_file("input/year_2015/day_2.txt");
	let split_string: Vec<&str> = input.split("\n").collect();
	let mut total = 0;

	for str in split_string {
		let mut dimensions = str.split("x")
			.collect::<Vec<&str>>()
			.iter()
			.map(|x| x.parse::<i32>().unwrap())
			.collect::<Vec<i32>>();
		dimensions.sort();

		total += 2*dimensions[0] + 2*dimensions[1] + dimensions.iter().fold(1, |acc, x| acc * x);
	}

	println!("Day 2 - Part 2: Feet of ribbon: {}", total);
}
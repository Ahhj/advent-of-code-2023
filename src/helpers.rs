use textwrap::dedent;

pub(crate) struct Example {
    pub input_data: String,
    pub expected_part1: u64,
    pub expected_part2: u64,
}

pub(crate) struct Solution {
    pub input_data: String,
    pub get_solution_part1: fn(&str) -> u64,
    pub get_solution_part2: fn(&str) -> u64,
}

impl Solution {
    pub fn print_solutions(self, day: u32, example: Example) {
        // Example part 1
        let example_solution_part1 = (self.get_solution_part1)(&example.input_data);
        assert_eq!(example.expected_part1, example_solution_part1);

        // Example part 2
        let example_solution_part2 = (self.get_solution_part2)(&example.input_data);
        assert_eq!(example.expected_part2, example_solution_part2);

        // Part 1
        let solution_part1 = (self.get_solution_part1)(&self.input_data);
        println!("Day {day}, part 1: {solution_part1}");

        // Part 2
        let solution_part2 = (self.get_solution_part2)(&self.input_data);
        println!("Day {day}, part 2: {solution_part2}");
    }
}

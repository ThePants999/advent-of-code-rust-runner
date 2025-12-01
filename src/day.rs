use std::time::{Duration, Instant};

pub struct DayResult<'a> {
    pub(crate) part_1_result: &'a str,
    pub(crate) part_1_time: Duration,
    pub(crate) part_2_result: &'a str,
    pub(crate) part_2_time: Duration
}

pub struct TestResult<'a> {
    pub(crate) test_result: DayResult<'a>,
    pub(crate) part_1_correct: bool,
    pub(crate) part_2_correct: bool
}

pub trait DayImplementation {
    type Context<'a>;

    fn day(&self) -> u8;
    fn example_input(&self) -> &'static str;
    fn example_part_1_result(&self) -> &'static str;
    fn example_part_2_result(&self) -> &'static str;

    fn execute_part_1<'a>(&self, input: &'a str) -> (&'a str, Self::Context<'a>);
    fn execute_part_2<'a>(&self, input: &'a str, context: Self::Context<'a>) -> &'a str;
}

pub trait Day {
    fn day(&self) -> u8;
    fn example_input(&self) -> &'static str;
    fn example_part_1_result(&self) -> &'static str;
    fn example_part_2_result(&self) -> &'static str;

    fn test_day<'a>(&'a self) -> TestResult<'a>;
    fn execute_day<'a>(&'a self) -> DayResult<'a>;
    fn run_with_input<'a>(&'a self, input: &'a str) -> DayResult<'a>;
}

impl<T: DayImplementation> Day for T {
    fn day(&self) -> u8 { DayImplementation::day(self) }
    fn example_input(&self) -> &'static str { DayImplementation::example_input(self) }
    fn example_part_1_result(&self) -> &'static str { DayImplementation::example_part_1_result(self) }
    fn example_part_2_result(&self) -> &'static str { DayImplementation::example_part_2_result(self) }

    fn test_day<'a>(&'a self) -> TestResult<'a> {
        log::info!("Running tests for day {}", self.day());
        let result = self.run_with_input(self.example_input());
        let part_1_correct = result.part_1_result == self.example_part_1_result();
        let part_2_correct = result.part_2_result == self.example_part_2_result();

        TestResult {
            test_result: result,
            part_1_correct,
            part_2_correct
        }
    }

    fn execute_day<'a>(&'a self) -> DayResult<'a> {
        // Temp hack before we implement input fetching
        let input = self.example_input();
        log::info!("Executing day {} with actual input", self.day());
        self.run_with_input(input)
    }

    fn run_with_input<'a>(&'a self, input: &'a str) -> DayResult<'a> {
        log::debug!("Starting part 1 for day {}", self.day());
        let start_part_1 = Instant::now();
        let (part_1_result, context) = self.execute_part_1(input);
        let part_1_time = start_part_1.elapsed();
        log::info!("Part 1 completed in {:?}, result: {}", part_1_time, part_1_result);

        log::debug!("Starting part 2 for day {}", self.day());
        let start_part_2 = Instant::now();
        let part_2_result = self.execute_part_2(input, context);
        let part_2_time = start_part_2.elapsed();
        log::info!("Part 2 completed in {:?}, result: {}", part_2_time, part_2_result);

        DayResult {
            part_1_result,
            part_1_time,
            part_2_result,
            part_2_time
        }
    }
}
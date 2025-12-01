use std::time::{Duration, Instant};

pub(crate) struct DayResult {
    part_1_result: String,
    part_1_time: Duration,
    part_2_result: String,
    part_2_time: Duration
}

pub(crate) struct TestResult {
    test_result: DayResult,
    part_1_correct: bool,
    part_2_correct: bool
}

pub trait Day {
    fn day(&self) -> u8;
    fn example_input(&self) -> &'static str;
    fn example_part_1_result(&self) -> &'static str;
    fn example_part_2_result(&self) -> &'static str;

    fn execute_part_1(&mut self, input: &str) -> String;
    fn execute_part_2(&mut self, input: &str) -> String;
}

pub(crate) fn run_day_with_input(day: &mut dyn Day, input: &str) -> DayResult {
    log::debug!("Starting part 1 for day {}", day.day());
    let start_part_1 = Instant::now();
    let part_1_result = day.execute_part_1(input);
    let part_1_time = start_part_1.elapsed();
    log::info!("Part 1 completed in {:?}, result: {}", part_1_time, part_1_result);

    log::debug!("Starting part 2 for day {}", day.day());
    let start_part_2 = Instant::now();
    let part_2_result = day.execute_part_2(input);
    let part_2_time = start_part_2.elapsed();
    log::info!("Part 2 completed in {:?}, result: {}", part_2_time, part_2_result);

    DayResult {
        part_1_result,
        part_1_time,
        part_2_result,
        part_2_time
    }
}
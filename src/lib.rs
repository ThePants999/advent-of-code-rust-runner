mod day;
mod environment;

pub use day::{Day, DayImplementation};
use day::TestCaseResult;

pub fn run_all(year: &str, implementations: &[Box<dyn Day>]) {
    let environment = match environment::AOCEnvironment::initialize(year) {
        Ok(env) => env,
        Err(e) => {
            log::error!("Failed to initialize Advent of Code environment: {}", e);
            return;
        }
    };

    for day_impl in implementations {
        let input = match environment.fetch_input(day_impl.day()) {
            Ok(input) => input,
            Err(e) => {
                log::error!("Day {}: failed to fetch input: {}", day_impl.day(), e);
                continue;
            }
        };

        match day_impl.test_day() {
            Ok(test_result) => {
                let part1_passed = !matches!(test_result.part1, TestCaseResult::Failed(_, _));
                let part2_passed = !matches!(test_result.part2, TestCaseResult::Failed(_, _));

                if part1_passed && part2_passed {
                    // Log any passed tests
                    if let TestCaseResult::Passed(t1) = test_result.part1 {
                        log::debug!("Day {}: part 1 test passed in {:?}", day_impl.day(), t1);
                    }
                    if let TestCaseResult::Passed(t2) = test_result.part2 {
                        log::debug!("Day {}: part 2 test passed in {:?}", day_impl.day(), t2);
                    }

                    match day_impl.execute_day(input.as_str()) {
                        Ok(result) => {
                            log::info!(
                                "Day {}: part 1 result: {} (took {:?}), part 2 result: {} (took {:?})",
                                day_impl.day(),
                                result.part_1_result,
                                result.part_1_time,
                                result.part_2_result,
                                result.part_2_time
                            );
                        }
                        Err(e) => {
                            log::error!("Day {}: execution failed: {}", day_impl.day(), e);
                        }
                    }
                } else {
                    log::warn!("Day {}: tests failed!", day_impl.day());
                    if let TestCaseResult::Failed(expected, actual) = &test_result.part1 {
                        log::warn!("  Part 1: expected {}, got {}", expected, actual);
                    }
                    if let TestCaseResult::Failed(expected, actual) = &test_result.part2 {
                        log::warn!("  Part 2: expected {}, got {}", expected, actual);
                    }
                }
            }
            Err(e) => {
                log::error!("Day {}: tests failed to run: {}", day_impl.day(), e);
            }
        }
    }
}

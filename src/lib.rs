mod day;
mod environment;

pub use day::{Day, DayImplementation};

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
            Ok(test_result) if test_result.part_1_correct && test_result.part_2_correct => {
                log::warn!("Day {}: tests passed, part 1 in {:?}, part 2 in {:?}", day_impl.day(), test_result.part_1_time, test_result.part_2_time);
                match day_impl.execute_day(input.as_str()) {
                    Ok(result) => {
                        log::warn!(
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
            }
            Ok(test_result) => {
                log::error!(
                    "Day {}: tests failed! Part 1 correct: {}, part 2 correct: {}",
                    day_impl.day(),
                    test_result.part_1_correct,
                    test_result.part_2_correct);
            }
            Err(e) => {
                log::error!("Day {}: tests failed to run: {}", day_impl.day(), e);
            }
        };
    }
}

mod day;

pub use day::{Day, DayImplementation};

pub fn run_all(implementations: &[Box<dyn Day>]) {
    for day_impl in implementations {
        match day_impl.test_day() {
            Ok(test_result) if test_result.part_1_correct && test_result.part_2_correct => {
                log::warn!("Day {}: tests passed, part 1 in {:?}, part 2 in {:?}", day_impl.day(), test_result.part_1_time, test_result.part_2_time);
                match day_impl.execute_day() {
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

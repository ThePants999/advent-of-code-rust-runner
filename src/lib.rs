mod day;

pub use day::{Day, DayImplementation};

pub fn run_all(implementations: &[Box<dyn Day>]) {
    for day_impl in implementations {
        let test_result = day_impl.test_day();
        if !test_result.part_1_correct || !test_result.part_2_correct {
            if !test_result.part_1_correct {
                log::error!(
                    "Day {}: part 1 test failed! Expected {}, got {}",
                    day_impl.day(),
                    day_impl.example_part_1_result(),
                    test_result.test_result.part_1_result
                );
            }
            if !test_result.part_2_correct {
                log::error!(
                    "Day {}: part 2 test failed! Expected {}, got {}",
                    day_impl.day(),
                    day_impl.example_part_2_result(),
                    test_result.test_result.part_2_result
                );
            }
        } else {
            let result = day_impl.execute_day();
            log::warn!(
                "Day {}: part 1 result: {} (took {:?}), part 2 result: {} (took {:?})",
                day_impl.day(),
                result.part_1_result,
                result.part_1_time,
                result.part_2_result,
                result.part_2_time
            );
        }
    }
}

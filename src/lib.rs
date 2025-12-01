mod day;

pub use day::Day;

pub fn run_all(implementations: &mut [Box<dyn Day>]) {
    for day_impl in implementations.iter_mut() {
        let input = day_impl.example_input();
        let result = day::run_day_with_input(day_impl.as_mut(), input);
    }
}

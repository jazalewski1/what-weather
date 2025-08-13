use crate::domain::port::Reporter;

pub fn run(reporter: Box<dyn Reporter>) {
    reporter.report_current_weather();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::port::mocks::MockReporter;

    #[test]
    fn report_with_reporter() {
        let mut reporter = Box::new(MockReporter::new());
        reporter
            .expect_report_current_weather()
            .times(1)
            .return_const(());
        run(reporter);
    }
}

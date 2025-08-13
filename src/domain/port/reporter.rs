#[mockall::automock]
pub trait Reporter {
    fn report_current_weather(&self);
}

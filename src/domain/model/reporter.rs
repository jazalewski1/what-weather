pub struct Parameters;

pub trait Reporter {
    fn fetch_and_report(&self, parameters: &Parameters);
}

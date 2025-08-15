use crate::output::View;

pub struct ConsoleView;

impl View for ConsoleView {
    fn display(&self, data: &str) {
        println!("{data}")
    }
}

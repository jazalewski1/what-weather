use crate::view::View;

pub struct ConsoleView;

impl View for ConsoleView {
    fn display(&self, data: String) {
        println!("{data}")
    }
}

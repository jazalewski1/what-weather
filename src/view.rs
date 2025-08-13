mod console;

pub use console::ConsoleView;

#[mockall::automock]
pub trait View {
    fn display(&self, data: String);
}

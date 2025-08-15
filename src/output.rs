mod console;
pub mod formatters;

pub use console::ConsoleView;

#[mockall::automock]
pub trait View {
    fn display(&self, data: &str);
}

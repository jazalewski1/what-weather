pub mod console;
pub mod format;

pub use console::ConsoleView;

#[mockall::automock]
pub trait View {
    fn display(&self, data: &str);
}

mod console;
mod format;

pub use console::ConsoleView;
pub use format::Format;

#[mockall::automock]
pub trait View {
    fn display(&self, data: &str);
}

use std::fmt::{Display, Write};

pub fn write_param<Stream: Write>(stream: &mut Stream, label: impl Display, value: impl Display) {
    writeln!(stream, "{label}: {value}").expect("Failed to write parameter")
}

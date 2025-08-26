#[derive(Default)]
pub struct ListBuilder {
    string: String,
}

impl ListBuilder {
    pub fn add(&mut self, label: &str, value: &str) -> &mut Self {
        if self.string.is_empty() {
            self.string.push_str(&format!("{label}: {value}"));
        } else {
            self.string.push_str(&format!("\n{label}: {value}"));
        }
        self
    }

    pub fn string(self) -> String {
        self.string
    }
}

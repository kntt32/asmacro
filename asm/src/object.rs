use crate::line::label::{Label, Location};

#[derive(Clone, Debug)]
pub struct Object {
    pub code: Vec<u8>,
    pub label: Vec<Label>,
    pub location: Vec<Location>,
}

impl Object {
    pub fn new() -> Self {
        Object {
            code: Vec::new(),
            label: Vec::new(),
            location: Vec::new(),
        }
    }

    pub fn add_label(&mut self, label: Label) {
        self.label.push(label);
    }

    pub fn code_len(&self) -> usize {
        self.code.len()
    }
}
